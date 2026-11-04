//! Shared update checking for Lintel suite apps.
//!
//! Behaviour contract:
//!
//! * Queries the **GitHub Releases API** list endpoint for a repository's
//!   releases, chooses the newest published (non-draft, non-prerelease) release
//!   and compares it with the running version.
//! * **Fails soft offline** — any network, HTTP or parse problem becomes an
//!   [`UpdateStatus::Unknown`], never a panic and never a retry loop.
//! * **No API abuse** — a successful result is cached in memory for
//!   [`CACHE_TTL`]; callers run a check on an explicit user action or at most
//!   once per session.
//! * The comparison is *semantic*, so `0.2.0` correctly beats `0.10.0`.
//!
//! The crate is deliberately UI-free: an app runs [`UpdateChecker::check`] on a
//! worker thread and posts the resulting [`UpdateStatus`] back to its UI loop.

use serde::Deserialize;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// How long a successful check stays valid before it may be refreshed.
pub const CACHE_TTL: Duration = Duration::from_secs(60 * 60);

/// GitHub Releases API base for a repository (`{RELEASES_API}/{repo}/releases`).
pub const RELEASES_API: &str = "https://api.github.com/repos";

/// Overall request timeout: a check must never hang the caller's worker.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// `User-Agent` sent with every request (the GitHub API requires one).
pub const USER_AGENT: &str = "lintel-update-check";

/// Outcome of a check. `Unknown` covers every failure mode so the UI can show a
/// single honest "couldn't check" state instead of a stack of error variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    /// The running version is the newest published release.
    UpToDate {
        /// The running version that was compared, echoed back for the UI.
        current: String,
    },
    /// A newer published release exists; carries the parsed [`Release`].
    UpdateAvailable(Release),
    /// The check could not be completed (offline, rate-limited, bad payload…).
    ///
    /// The string is a short, non-fatal explanation for logs and diagnostics.
    Unknown(String),
}

/// One published release, as far as this crate cares.
///
/// Fields map onto the GitHub Releases payload: `tag` ← `tag_name`, `url` ←
/// `html_url` and `notes` ← `body`. Unknown fields are ignored, so the payload
/// may grow without breaking this crate.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Release {
    /// Git tag of the release (e.g. `v0.2.0`).
    #[serde(rename = "tag_name", default)]
    pub tag: String,
    /// Release title, when the API provides one.
    #[serde(default)]
    pub name: Option<String>,
    /// Browser URL of the release page.
    #[serde(rename = "html_url", default)]
    pub url: String,
    /// Release notes / body text.
    #[serde(rename = "body", default)]
    pub notes: String,
    /// ISO-8601 publication timestamp, when present.
    #[serde(default)]
    pub published_at: Option<String>,
    /// GitHub draft flag; drafts are never offered to the user.
    #[serde(default)]
    pub draft: bool,
    /// GitHub pre-release flag; pre-releases are never offered to the user.
    #[serde(default)]
    pub prerelease: bool,
}

/// Parsed semantic version (major.minor.patch, optional pre-release suffix).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    /// Major component.
    pub major: u64,
    /// Minor component.
    pub minor: u64,
    /// Patch component.
    pub patch: u64,
}

impl Version {
    /// Parse `v?MAJOR.MINOR.PATCH(-suffix)?`. Pre-release and build suffixes are
    /// ignored for ordering; unparseable input yields `None` rather than a wrong
    /// answer.
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        let raw = raw.strip_prefix('v').unwrap_or(raw);
        let core = raw.split(['-', '+']).next()?;
        let mut parts = core.split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        let patch = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

/// Compare two versions: `Ordering::Greater` when `a` is newer than `b`.
///
/// Exposed for tests and for callers that want to show a diff direction.
pub fn compare(a: &str, b: &str) -> Option<Ordering> {
    Some(Version::parse(a)?.cmp(&Version::parse(b)?))
}

/// The record a successful check leaves behind for the cache.
#[derive(Debug, Clone)]
struct CachedCheck {
    status: UpdateStatus,
    at: Instant,
}

/// Cached, rate-limit-friendly checker.
///
/// Blocking: call [`check`](Self::check) from a worker thread, never from the
/// UI loop (the app spawns it and posts the result back).
#[derive(Debug, Default)]
pub struct UpdateChecker {
    cache: Option<CachedCheck>,
}

impl UpdateChecker {
    /// Create an empty checker with no cached result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check for updates. `repo` is `owner/name`; `current` is the running
    /// version. Returns a cached result while [`CACHE_TTL`] is unexpired.
    ///
    /// Any failure is returned as [`UpdateStatus::Unknown`] and is *not* cached,
    /// so a later user-initiated retry is possible.
    pub fn check(&mut self, repo: &str, current: &str) -> UpdateStatus {
        if let Some(cached) = self.cached() {
            return cached.clone();
        }
        let status = check_now(repo, current);
        if !matches!(status, UpdateStatus::Unknown(_)) {
            self.cache = Some(CachedCheck {
                status: status.clone(),
                at: Instant::now(),
            });
        }
        status
    }

    /// Test/CLI hook: run one check with no caching.
    pub fn check_uncached(&self, repo: &str, current: &str) -> UpdateStatus {
        check_now(repo, current)
    }

    /// Forget any cached result (used by "check again now").
    pub fn invalidate(&mut self) {
        self.cache = None;
    }

    /// The cached status, if still fresh. Pure; never touches the network.
    pub fn cached(&self) -> Option<&UpdateStatus> {
        self.cache
            .as_ref()
            .filter(|c| c.at.elapsed() < CACHE_TTL)
            .map(|c| &c.status)
    }
}

/// One network round trip plus the comparison. Fails soft on every error.
fn check_now(repo: &str, current: &str) -> UpdateStatus {
    let url = format!("{RELEASES_API}/{repo}/releases");
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(REQUEST_TIMEOUT))
        .build();
    let agent = ureq::Agent::new_with_config(config);
    let mut response = match agent
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
    {
        Ok(response) => response,
        // Offline, DNS failure, timeout…: fail soft.
        Err(error) => return UpdateStatus::Unknown(format!("request failed: {error}")),
    };
    let status = response.status().as_u16();
    if status != 200 {
        // 404 (no releases yet), rate-limited and other HTTP conditions all mean
        // the same thing to the user: "could not check".
        return UpdateStatus::Unknown(format!("GitHub returned HTTP {status}"));
    }
    let releases: Vec<Release> = match response.body_mut().read_json() {
        Ok(releases) => releases,
        Err(error) => return UpdateStatus::Unknown(format!("bad release payload: {error}")),
    };
    evaluate(current, &releases)
}

/// The comparison half of a check, split out so it is testable without a
/// network. Pure: no clock, no I/O.
fn evaluate(current: &str, releases: &[Release]) -> UpdateStatus {
    let Some(running) = Version::parse(current) else {
        return UpdateStatus::Unknown(format!("unparseable running version: {current}"));
    };
    let Some(latest) = pick_latest(releases) else {
        // No published releases at all: nothing is newer than what we run.
        return UpdateStatus::UpToDate {
            current: current.to_string(),
        };
    };
    let Some(latest_version) = Version::parse(&latest.tag) else {
        return UpdateStatus::Unknown(format!("unparseable release tag: {}", latest.tag));
    };
    if latest_version > running {
        UpdateStatus::UpdateAvailable(latest)
    } else {
        UpdateStatus::UpToDate {
            current: current.to_string(),
        }
    }
}

/// Choose the newest published release from a pre-fetched list.
///
/// Drafts, pre-releases and releases whose tag is not a semantic version are
/// skipped, so callers can hand over the raw API list (or a fixture) unchanged.
pub fn pick_latest(releases: &[Release]) -> Option<Release> {
    releases
        .iter()
        .filter(|release| !release.draft && !release.prerelease)
        .filter_map(|release| Version::parse(&release.tag).map(|version| (version, release)))
        .max_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, release)| release.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(tag: &str) -> Release {
        Release {
            tag: tag.to_string(),
            name: None,
            url: format!("https://github.com/x/y/releases/tag/{tag}"),
            notes: String::new(),
            published_at: None,
            draft: false,
            prerelease: false,
        }
    }

    #[test]
    fn version_parses_plain_and_prefixed() {
        assert_eq!(
            Version::parse("1.2.3"),
            Some(Version {
                major: 1,
                minor: 2,
                patch: 3
            })
        );
        assert_eq!(
            Version::parse("v0.1.0"),
            Some(Version {
                major: 0,
                minor: 1,
                patch: 0
            })
        );
        assert_eq!(Version::parse("  v2.10.0 "), Version::parse("2.10.0"));
    }

    #[test]
    fn version_handles_prerelease_and_build_suffixes() {
        assert_eq!(Version::parse("1.2.3-rc.1"), Version::parse("1.2.3"));
        assert_eq!(Version::parse("1.2.3+build.9"), Version::parse("1.2.3"));
    }

    #[test]
    fn version_rejects_garbage() {
        assert_eq!(Version::parse(""), None);
        assert_eq!(Version::parse("abc"), None);
        assert_eq!(Version::parse("1.2"), None);
        assert_eq!(Version::parse("1.2.3.4"), None);
        assert_eq!(Version::parse("1.2.x"), None);
    }

    #[test]
    fn compare_is_semantic_not_lexicographic() {
        assert_eq!(compare("0.2.0", "0.10.0"), Some(Ordering::Less));
        assert_eq!(compare("0.10.0", "0.2.0"), Some(Ordering::Greater));
        assert_eq!(compare("1.0.0", "1.0.0"), Some(Ordering::Equal));
        assert_eq!(compare("0.1.1", "0.1.0"), Some(Ordering::Greater));
        assert_eq!(compare("nope", "0.1.0"), None);
    }

    #[test]
    fn pick_latest_ignores_drafts_prereleases_and_bad_tags() {
        let mut draft = rel("1.4.0");
        draft.draft = true;
        let mut prerelease = rel("9.9.9");
        prerelease.prerelease = true;
        let releases = [
            rel("0.1.0"),
            draft,
            prerelease,
            rel("0.2.0"),
            rel("garbage"),
        ];
        assert_eq!(
            pick_latest(&releases).map(|r| r.tag),
            Some("0.2.0".to_string())
        );
    }

    #[test]
    fn pick_latest_empty_is_none() {
        assert!(pick_latest(&[]).is_none());
    }

    #[test]
    fn release_payload_deserialises_from_github_shape() {
        let json = r#"{
            "tag_name": "v0.2.0",
            "name": "Lintel 0.2.0",
            "html_url": "https://github.com/x/y/releases/tag/v0.2.0",
            "body": "Notes here",
            "published_at": "2026-11-01T00:00:00Z",
            "draft": false,
            "prerelease": false,
            "id": 123
        }"#;
        let release: Release = serde_json::from_str(json).expect("deserialise");
        assert_eq!(release.tag, "v0.2.0");
        assert_eq!(release.name.as_deref(), Some("Lintel 0.2.0"));
        assert_eq!(release.notes, "Notes here");
        assert_eq!(
            release.published_at.as_deref(),
            Some("2026-11-01T00:00:00Z")
        );
        assert!(!release.draft);
    }

    #[test]
    fn release_payload_missing_fields_default_safely() {
        let release: Release = serde_json::from_str("{}").expect("deserialise");
        assert_eq!(release.tag, "");
        assert_eq!(release.name, None);
        assert!(!release.draft);
        assert!(!release.prerelease);
    }

    #[test]
    fn evaluate_reports_newer_release() {
        let status = evaluate("0.1.0", &[rel("v0.2.0")]);
        match status {
            UpdateStatus::UpdateAvailable(release) => {
                assert_eq!(release.tag, "v0.2.0");
                assert!(release.url.contains("/tag/v0.2.0"));
            }
            other => panic!("expected UpdateAvailable, got {other:?}"),
        }
    }

    #[test]
    fn evaluate_reports_up_to_date_when_running_is_newest() {
        assert_eq!(
            evaluate("0.2.0", &[rel("v0.2.0")]),
            UpdateStatus::UpToDate {
                current: "0.2.0".to_string()
            }
        );
        assert_eq!(
            evaluate("0.3.0", &[rel("v0.2.0")]),
            UpdateStatus::UpToDate {
                current: "0.3.0".to_string()
            }
        );
        // Semantic, not lexicographic: running 0.10.0 beats published 0.2.0.
        assert_eq!(
            evaluate("0.10.0", &[rel("v0.2.0")]),
            UpdateStatus::UpToDate {
                current: "0.10.0".to_string()
            }
        );
    }

    #[test]
    fn evaluate_ignores_drafts_prereleases_and_empty_tags() {
        let mut draft = rel("v9.0.0");
        draft.draft = true;
        assert_eq!(
            evaluate("0.1.0", &[draft]),
            UpdateStatus::UpToDate {
                current: "0.1.0".to_string()
            }
        );
        let mut prerelease = rel("v9.0.0");
        prerelease.prerelease = true;
        assert_eq!(
            evaluate("0.1.0", &[prerelease]),
            UpdateStatus::UpToDate {
                current: "0.1.0".to_string()
            }
        );
        assert_eq!(
            evaluate("0.1.0", &[rel("")]),
            UpdateStatus::UpToDate {
                current: "0.1.0".to_string()
            }
        );
    }

    #[test]
    fn evaluate_is_unknown_for_unparseable_running_version() {
        match evaluate("dev-build", &[rel("v0.2.0")]) {
            UpdateStatus::Unknown(reason) => assert!(reason.contains("dev-build")),
            other => panic!("expected Unknown, got {other:?}"),
        }
    }

    #[test]
    fn checker_caches_within_ttl_and_respects_invalidate() {
        let mut checker = UpdateChecker::new();
        assert!(checker.cached().is_none());
        // Seed the cache directly: no network in unit tests.
        checker.cache = Some(CachedCheck {
            status: UpdateStatus::UpToDate {
                current: "0.1.0".to_string(),
            },
            at: Instant::now(),
        });
        assert_eq!(
            checker.cached(),
            Some(&UpdateStatus::UpToDate {
                current: "0.1.0".to_string()
            })
        );
        checker.invalidate();
        assert!(checker.cached().is_none());
    }
}
