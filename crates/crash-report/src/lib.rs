//! Shared crash and bug reporting for Lintel suite apps.
//!
//! Design rules, all enforced here:
//!
//! * **Never embed a privileged token.** This crate ships no credential. If an
//!   app offers in-app submission at all, it uses a *user-supplied* token held
//!   only in memory ([`submit`]).
//! * **Consent first, never auto-send.** A report is only *captured* by the
//!   panic hook; turning it into a network call takes explicit user action. The
//!   default path is [`prefill_issue_url`], a pre-filled "New Issue" page the
//!   user opens and submits themselves.
//! * **Scrub before sharing.** Absolute home directories and anything that
//!   looks like a token are redacted from every captured line ([`scrub`]).
//!
//! The crate is deliberately UI-free: an app renders the [`Report`] with its own
//! toolkit and calls [`submit`] only after the user confirms.

use serde::{Deserialize, Serialize};
use std::panic::PanicHookInfo;
use std::path::Path;

/// `User-Agent` sent when submitting an issue (the GitHub API requires one).
pub const USER_AGENT: &str = "lintel-crash-report";

/// Labels requested on a crash issue.
pub const ISSUE_LABELS: [&str; 3] = ["crash", "bug", "user-report"];

/// Maximum characters retained for the summary line.
pub const MAX_MESSAGE_CHARS: usize = 200;

/// Placeholder that replaces a user's home directory.
const HOME_PLACEHOLDER: &str = "<home>";

/// Replacement marker for anything redacted from a captured line.
const REDACTED: &str = "[REDACTED]";

/// A captured crash or bug report: what happened, where, and the scrubbed
/// context. All fields are public so a UI can build a report by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    /// Application name (e.g. `lintel`).
    pub app: String,
    /// Application version (usually `CARGO_PKG_VERSION`).
    pub version: String,
    /// Operating system identifier (`std::env::consts::OS`).
    pub os: String,
    /// CPU architecture identifier (`std::env::consts::ARCH`).
    pub arch: String,
    /// Capture time as Unix epoch seconds.
    pub timestamp: u64,
    /// One-line reason (the panic message, truncated).
    pub message: String,
    /// Scrubbed backtrace, when one could be captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<String>,
    /// Free-text notes the user chose to add. Optional and never fabricated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Report {
    /// Build a report from a panic payload and the current backtrace.
    pub fn from_panic(app: &str, version: &str, info: &PanicHookInfo<'_>) -> Self {
        Self {
            app: app.to_string(),
            version: version.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            timestamp: now_epoch(),
            message: truncate(&panic_message(info), MAX_MESSAGE_CHARS),
            backtrace: Some(scrubbed_backtrace()),
            notes: None,
        }
    }

    /// Build a report for a non-panic bug the user reports by hand.
    pub fn manual(app: &str, version: &str, message: &str) -> Self {
        Self {
            app: app.to_string(),
            version: version.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            timestamp: now_epoch(),
            message: truncate(message, MAX_MESSAGE_CHARS),
            backtrace: None,
            notes: None,
        }
    }

    /// Human-readable issue title.
    pub fn title(&self) -> String {
        format!("[crash] {}: {}", self.os, self.message)
    }

    /// The full issue body: report plus optional notes, as markdown.
    pub fn body(&self) -> String {
        let mut out = String::new();
        out.push_str("## Crash report\n\n");
        out.push_str(&format!(
            "- App: {} {}\n- OS: {}\n- Arch: {}\n- Captured (unix): {}\n\n",
            self.app, self.version, self.os, self.arch, self.timestamp
        ));
        out.push_str("### Summary\n\n```\n");
        out.push_str(&self.message);
        out.push_str("\n```\n\n");
        if let Some(backtrace) = &self.backtrace {
            out.push_str("### Backtrace (scrubbed)\n\n```\n");
            out.push_str(backtrace);
            out.push_str("\n```\n\n");
        }
        if let Some(notes) = &self.notes {
            out.push_str("### User notes\n\n");
            out.push_str(notes);
            out.push('\n');
        }
        out
    }

    /// Serialise the report (used by a review dialog and by tests).
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Install the panic hook, writing a crash log into `log_dir`.
///
/// The previously installed hook still runs afterwards, so the usual stderr
/// output (and any hook the host app had set) is preserved. Enabling frames is
/// `std`'s job: set `RUST_BACKTRACE=full` in the environment.
pub fn install_panic_hook(app: &str, version: &str, log_dir: &Path) {
    let app = app.to_string();
    let version = version.to_string();
    let log_dir = log_dir.to_path_buf();
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let report = Report::from_panic(&app, &version, info);
        if let Err(error) = write_crash_log(&log_dir, &report) {
            // A failed log write must never mask the original panic.
            eprintln!("crash-report: could not write crash log: {error}");
        }
        previous(info);
    }));
}

/// Write `report` as JSON to a uniquely named file inside `log_dir`.
fn write_crash_log(log_dir: &Path, report: &Report) -> std::io::Result<()> {
    std::fs::create_dir_all(log_dir)?;
    let name = format!("crash-{}-{}.log", report.timestamp, uuid::Uuid::new_v4());
    std::fs::write(log_dir.join(name), report.to_json())
}

/// Extract a readable message from a panic payload.
pub fn panic_message(info: &PanicHookInfo<'_>) -> String {
    if let Some(text) = info.payload().downcast_ref::<&str>() {
        (*text).to_string()
    } else if let Some(text) = info.payload().downcast_ref::<String>() {
        text.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

/// Collect the current backtrace, scrubbed, as one newline-joined string.
pub fn scrubbed_backtrace() -> String {
    std::backtrace::Backtrace::force_capture()
        .to_string()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(scrub)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Redact home directories and credential-looking material from one line.
///
/// `/home/<user>` and `/Users/<user>` become `<home>`; GitHub token prefixes
/// (`ghp_…`, `github_pat_…`), `Bearer` values, long hex runs and `key=value`
/// secrets become `[REDACTED]`. Pure and deterministic: it never reads the
/// environment, so a shared report cannot leak a username or secret.
pub fn scrub(input: &str) -> String {
    let without_home = redact_home(input);
    let without_prefixes = redact_prefixed_tokens(&without_home);
    let without_bearer = redact_bearer(&without_prefixes);
    let without_hex = redact_long_hex(&without_bearer);
    redact_key_values(&without_hex)
}

/// Replace `/home/<user>` and `/Users/<user>` with `<home>`.
fn redact_home(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if let Some(after) = input[i..].strip_prefix("/home/") {
            i += "/home/".len() + segment_len(after);
            out.push_str(HOME_PLACEHOLDER);
        } else if let Some(after) = input[i..].strip_prefix("/Users/") {
            i += "/Users/".len() + segment_len(after);
            out.push_str(HOME_PLACEHOLDER);
        } else {
            let Some(ch) = input[i..].chars().next() else {
                break;
            };
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

/// Length in bytes of the user-name segment that follows a home marker.
fn segment_len(after: &str) -> usize {
    after
        .find(|c: char| c == '/' || c.is_whitespace())
        .unwrap_or(after.len())
}

/// GitHub token prefixes that are redacted wherever they appear.
const TOKEN_PREFIXES: [&str; 6] = ["github_pat_", "ghp_", "gho_", "ghu_", "ghs_", "ghr_"];

/// Redact the body of `ghp_…`-style tokens while keeping the prefix visible.
fn redact_prefixed_tokens(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    loop {
        let Some((index, prefix_len)) = find_token_prefix(rest) else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..index]);
        out.push_str(&rest[index..index + prefix_len]);
        let after = &rest[index + prefix_len..];
        let end = after
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(after.len());
        out.push_str(REDACTED);
        rest = &after[end..];
    }
    out
}

/// Earliest token prefix in `input`, with its byte length.
fn find_token_prefix(input: &str) -> Option<(usize, usize)> {
    let lower = input.to_ascii_lowercase();
    TOKEN_PREFIXES
        .iter()
        .filter_map(|prefix| lower.find(prefix).map(|index| (index, prefix.len())))
        .min_by_key(|(index, _)| *index)
}

/// Minimum length of a word after `Bearer` before it is treated as a token.
const MIN_BEARER_TOKEN: usize = 8;

/// Redact the word following `Bearer` when it looks like a credential.
fn redact_bearer(input: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < lower.len() {
        let Some(pos) = lower[i..].find("bearer") else {
            out.push_str(&input[i..]);
            break;
        };
        let at = i + pos;
        out.push_str(&input[i..at]);
        out.push_str(&input[at..at + "bearer".len()]);
        let mut j = at + "bearer".len();
        let space_start = j;
        while j < input.len() && input[j..].starts_with(|c: char| c.is_whitespace()) {
            j += 1;
        }
        if j > space_start {
            out.push_str(&input[space_start..j]);
            let token_end = input[j..]
                .find(|c: char| c.is_whitespace())
                .map(|p| j + p)
                .unwrap_or(input.len());
            if token_end - j >= MIN_BEARER_TOKEN {
                out.push_str(REDACTED);
                j = token_end;
            }
        }
        i = j;
    }
    out
}

/// Minimum run of hex characters treated as a secret.
const MIN_HEX_RUN: usize = 32;

/// Redact runs of `MIN_HEX_RUN` or more hex digits (hashes, API keys).
fn redact_long_hex(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_hexdigit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                i += 1;
            }
            if i - start >= MIN_HEX_RUN {
                out.push_str(REDACTED);
            } else {
                out.push_str(&input[start..i]);
            }
        } else {
            let Some(ch) = input[i..].chars().next() else {
                break;
            };
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

/// Keys whose `key=value` / `key: value` content is a secret.
const SECRET_KEYS: [&str; 6] = ["token", "api_key", "apikey", "secret", "password", "bearer"];

/// Redact the value assigned to a secret-looking key.
fn redact_key_values(input: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < lower.len() {
        let rest = &lower[i..];
        let Some(pos) = SECRET_KEYS.iter().filter_map(|key| rest.find(key)).min() else {
            out.push_str(&input[i..]);
            break;
        };
        let at = i + pos;
        out.push_str(&input[i..at]);
        let after = &input[at..];
        let (key_end, has_separator) = match after.find(['=', ':']) {
            Some(offset) if offset < 16 => (at + offset, true),
            _ => (at, false),
        };
        out.push_str(&input[at..key_end]);
        let mut value_start = key_end;
        if has_separator {
            out.push('=');
            value_start += 1;
        }
        while value_start < input.len()
            && input[value_start..].starts_with(|c: char| c.is_whitespace())
        {
            value_start += 1;
        }
        let value_end = input[value_start..]
            .find(|c: char| c.is_whitespace())
            .map(|p| value_start + p)
            .unwrap_or(input.len());
        out.push_str(REDACTED);
        i = value_end;
    }
    out
}

/// Current time as Unix epoch seconds (0 if the clock is before the epoch).
fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

/// Truncate on a char boundary, appending an ellipsis when shortened.
fn truncate(text: &str, max: usize) -> String {
    if text.len() <= max {
        return text.to_string();
    }
    let mut cut = max;
    while cut > 0 && !text.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{}…", &text[..cut])
}

/// Build the pre-filled GitHub "New Issue" URL for a report.
///
/// Nothing is sent until the user presses Submit in their browser; this is the
/// no-token fallback and never performs a network call itself.
pub fn prefill_issue_url(repo: &str, report: &Report) -> String {
    format!(
        "https://github.com/{}/issues/new?title={}&body={}&labels={}",
        repo.trim_end_matches('/'),
        urlencode(&report.title()),
        urlencode(&report.body()),
        urlencode(&ISSUE_LABELS.join(",")),
    )
}

/// Submit a report as a GitHub issue using a **user-supplied** token.
///
/// Passing `None` or a blank token returns an error rather than making any
/// network call: this crate never posts a report the user has not reviewed and
/// confirmed. The token is used once and never stored.
pub fn submit(repo: &str, report: &Report, token: Option<&str>) -> Result<(), String> {
    let Some(token) = token
        .filter(|value| !value.trim().is_empty())
        .map(str::trim)
    else {
        return Err("no user token supplied; open the pre-filled issue URL instead".to_string());
    };
    let url = format!(
        "https://api.github.com/repos/{}/issues",
        repo.trim_end_matches('/')
    );
    let payload = serde_json::json!({
        "title": report.title(),
        "body": report.body(),
        "labels": ISSUE_LABELS,
    });
    let response = ureq::post(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .send_json(payload)
        .map_err(|error| format!("network error: {error}"))?;
    match response.status().as_u16() {
        200..=299 => Ok(()),
        401 => Err("GitHub rejected the token (401). Check it and retry.".to_string()),
        403 => Err(
            "GitHub refused issue creation (403). The token may lack the `repo` scope.".to_string(),
        ),
        status => Err(format!("GitHub returned {status}")),
    }
}

/// Percent-encode a string for use in a query parameter.
fn urlencode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Report {
        Report {
            app: "lintel".to_string(),
            version: "0.1.0".to_string(),
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            timestamp: 1_700_000_000,
            message: "boom".to_string(),
            backtrace: Some("frame one\nframe two".to_string()),
            notes: None,
        }
    }

    #[test]
    fn title_names_os_and_message() {
        assert_eq!(sample().title(), "[crash] linux: boom");
    }

    #[test]
    fn body_contains_report_fields() {
        let mut report = sample();
        report.notes = Some("happens when zooming".to_string());
        let body = report.body();
        assert!(body.starts_with("## Crash report"));
        assert!(body.contains("lintel 0.1.0"));
        assert!(body.contains("linux"));
        assert!(body.contains("x86_64"));
        assert!(body.contains("boom"));
        assert!(body.contains("frame two"));
        assert!(body.contains("happens when zooming"));
    }

    #[test]
    fn json_round_trips() {
        let report = sample();
        let json = report.to_json();
        let back: Report = serde_json::from_str(&json).expect("valid json");
        assert_eq!(back, report);
    }

    #[test]
    fn optional_fields_are_omitted_from_json_when_absent() {
        let mut report = sample();
        report.backtrace = None;
        report.notes = None;
        let json = report.to_json();
        assert!(!json.contains("backtrace"));
        assert!(!json.contains("notes"));
    }

    #[test]
    fn scrub_redacts_linux_and_macos_home_paths() {
        assert_eq!(
            scrub("/home/runner/projects/lintel/src/lib.rs"),
            "<home>/projects/lintel/src/lib.rs"
        );
        assert_eq!(scrub("/Users/runner/lintel"), "<home>/lintel");
        // A bare home path with no trailing segment is still redacted.
        assert_eq!(scrub("/home/runner"), "<home>");
    }

    #[test]
    fn scrub_redacts_github_token_prefixes() {
        let line = "token ghp_abcdefghijklmnop and github_pat_0123456789abcdef";
        let scrubbed = scrub(line);
        assert!(!scrubbed.contains("ghp_abcdefghijklmnop"));
        assert!(!scrubbed.contains("github_pat_0123456789abcdef"));
        assert!(scrubbed.contains("[REDACTED]"));
    }

    #[test]
    fn scrub_redacts_bearer_values() {
        let scrubbed = scrub("Authorization: Bearer abcdef123456");
        assert!(!scrubbed.contains("abcdef123456"));
        assert!(scrubbed.contains("[REDACTED]"));
    }

    #[test]
    fn scrub_redacts_long_hex_runs() {
        let secret = "0123456789abcdef0123456789abcdef";
        let scrubbed = scrub(&format!("digest {secret} end"));
        assert!(!scrubbed.contains(secret));
        assert!(scrubbed.contains("[REDACTED]"));
    }

    #[test]
    fn scrub_redacts_key_value_secrets() {
        let scrubbed = scrub("password=hunter2 token=abc123 secret: swordfish");
        assert!(!scrubbed.contains("hunter2"));
        assert!(!scrubbed.contains("abc123"));
        assert!(!scrubbed.contains("swordfish"));
    }

    #[test]
    fn scrub_leaves_normal_code_lines_alone() {
        let line = "12: lintel_core::wall::length";
        assert_eq!(scrub(line), line);
    }

    #[test]
    fn prefill_issue_url_is_prefilled_and_percent_encoded() {
        let url = prefill_issue_url("Perchemop/lintel-desktop", &sample());
        assert!(url.starts_with("https://github.com/Perchemop/lintel-desktop/issues/new?"));
        assert!(url.contains("title=%5Bcrash%5D"));
        assert!(url.contains("labels=crash%2Cbug%2Cuser-report"));
        assert!(!url.contains(' '));
    }

    #[test]
    fn submit_without_token_refuses_to_send() {
        let error =
            submit("Perchemop/lintel-desktop", &sample(), None).expect_err("expected error");
        assert!(error.contains("no user token"));
    }

    #[test]
    fn submit_with_blank_token_refuses_too() {
        assert!(submit("Perchemop/lintel-desktop", &sample(), Some("   ")).is_err());
    }
}
