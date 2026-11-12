//! In-app About, release-notes and update UI (guide section 9).
//!
//! All of this is presentation: the update *check* lives in the shared
//! `update-check` crate and is run on a worker thread by the shell; this module
//! only renders the result and opens links.

use crate::release_notes;
use eframe::egui;
use std::path::PathBuf;
use update_check::UpdateStatus;

/// GitHub repository that publishes Lintel releases.
pub const REPO: &str = "Perchemop/lintel-desktop";

/// Directory the crash-report panic hook writes crash logs into.
///
/// Prefers a per-user data directory and falls back to the system temp
/// directory, so a panic is always captured even in a sandbox.
pub fn crash_log_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("XDG_DATA_HOME").map(PathBuf::from))
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local").join("share"))
        })
        .unwrap_or_else(std::env::temp_dir);
    base.join("lintel").join("crashes")
}

/// A one-line, human-readable summary of the last update check.
pub fn status_line(status: Option<&UpdateStatus>) -> String {
    match status {
        None => "Not checked yet.".to_string(),
        Some(UpdateStatus::UpToDate { current }) => {
            format!("You are on the newest release (v{current}).")
        }
        Some(UpdateStatus::UpdateAvailable(release)) => {
            format!("A newer release is available: {}.", release.tag)
        }
        Some(UpdateStatus::Unknown(reason)) => {
            format!("Could not check for updates ({reason}). Everything still works offline.")
        }
    }
}

/// The full release-notes window: every version, with token metrics.
pub fn release_notes_window(ui: &mut egui::Ui, open: &mut bool) {
    egui::Window::new("Release notes")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .default_size([580.0, 480.0])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Lintel release history").strong());
            ui.label(egui::RichText::new(release_notes::TOKEN_METRIC_NOTE).small());
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for release in release_notes::RELEASES {
                    let heading = format!("v{} — {}", release.version, release.date);
                    ui.collapsing(heading, |ui| {
                        ui.label(egui::RichText::new(release.summary).italics());
                        ui.add_space(4.0);
                        section(ui, "Added", release.added);
                        section(ui, "Changed", release.changed);
                        section(ui, "Fixed", release.fixed);
                        section(ui, "Known limitations", release.known_limitations);
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Approx. tokens this release: {}",
                                release.tokens_this_release
                            ))
                            .small(),
                        );
                    });
                }
                ui.separator();
                ui.label(format!(
                    "Approx. total tokens since development began: {}",
                    release_notes::TOTAL_TOKENS
                ));
                ui.separator();
                ui.hyperlink_to(
                    "View releases on GitHub",
                    format!("https://github.com/{REPO}/releases"),
                );
                ui.add_space(4.0);
                if ui
                    .button("Update")
                    .on_hover_text(
                        "Opens the release page — Lintel never installs updates for you.",
                    )
                    .clicked()
                {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(
                        "https://github.com/Perchemop/lintel-desktop/releases",
                    ));
                }
            });
        });
}

fn section(ui: &mut egui::Ui, title: &str, items: &[&str]) {
    if items.is_empty() {
        return;
    }
    ui.label(egui::RichText::new(title).strong());
    for item in items {
        ui.label(format!("- {item}"));
    }
}

/// The non-intrusive "update available" prompt.
///
/// Renders only when a newer release exists; the caller clears `open` when the
/// user dismisses it. Never blocks normal use.
pub fn update_window(ui: &mut egui::Ui, open: &mut bool, status: Option<&UpdateStatus>) {
    let Some(UpdateStatus::UpdateAvailable(release)) = status else {
        return;
    };
    let mut visible = *open;
    let mut dismiss = false;
    egui::Window::new("Update available")
        .open(&mut visible)
        .collapsible(false)
        .resizable(false)
        .show(ui, |ui| {
            ui.label(format!("Lintel {} is available.", release.tag));
            if !release.notes.trim().is_empty() {
                let preview = release.notes.lines().take(6).collect::<Vec<_>>().join("\n");
                ui.label(egui::RichText::new(preview).small());
            }
            // Surface the published checksum manifest and, when GitHub supplies
            // one, its SHA-256, so the download can be verified by hand. A
            // release with no manifest shows nothing extra — no dead UI.
            if let Some(asset) = release.checksum_asset() {
                let mut line = asset.name.clone();
                if let Some(sha256) = asset.sha256() {
                    line = format!("{line} — SHA-256 {sha256}");
                }
                ui.label(
                    egui::RichText::new(line)
                        .small()
                        .color(crate::theme::TEXT_MUTED),
                );
            }
            ui.hyperlink_to(
                "How to verify the download",
                format!("https://github.com/{REPO}/blob/main/docs/release-verification.md"),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("Open download page").clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(&release.url));
                }
                if ui.button("Later").clicked() {
                    dismiss = true;
                }
            });
        });
    *open = visible && !dismiss;
}
