//! Rendered-evidence generator (ticket T-008, spec AC-09, guide II.30.8).
//!
//! Env-gated: every `evidence_*` test returns immediately unless the
//! environment variable `LINTEL_GENERATE_EVIDENCE` is set. The `kittest` CI
//! job sets it, renders the PNGs into `docs/qc/<suite>-evidence/`, and
//! uploads those folders as workflow artefacts for the pull-request
//! description. PNGs are never committed (guide II.30.8 — artefacts go to
//! the PR, not the tree); `.gitignore` excludes them.
//!
//! Rendering is headless: the `egui_kittest` harness with its wgpu renderer
//! — no window server required (guide K.13; the CI job configures
//! swiftshader).

use crate::components::buttons::{ButtonRole, ButtonSize, StyledButton};
use crate::components::forms::{Field, FieldSet};
use crate::layout::helpers::{card, card_compact, hstack, tool_group, vstack, wrap_row};
use crate::theme;
use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use std::path::PathBuf;

/// Render evidence only when the CI job (or a reviewer) asks for it.
fn evidence_enabled() -> bool {
    std::env::var_os("LINTEL_GENERATE_EVIDENCE").is_some()
}

/// `docs/qc/<suite>/` relative to the workspace root, created on demand.
fn evidence_dir(suite: &str) -> PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/qc")
        .join(suite);
    if let Err(error) = std::fs::create_dir_all(&dir) {
        panic!(
            "could not create evidence directory {}: {error}",
            dir.display()
        );
    }
    dir
}

/// Render the harness's last frame to `<suite>/<file>.png`.
fn render_to(harness: &mut Harness, suite: &str, file: &str) {
    let image = harness
        .render()
        .expect("the kittest renderer must produce an image");
    let path = evidence_dir(suite).join(format!("{file}.png"));
    image
        .save(&path)
        .expect("the evidence PNG must be writable");
    println!("lintel evidence: wrote {}", path.display());
}

// ---------------------------------------------------------------------------
// Sheets — the actual screens the evidence depicts
// ---------------------------------------------------------------------------

/// All twelve button variants, grouped by role in cards.
fn buttons_sheet_ui(ui: &mut egui::Ui) {
    let roles = [
        (ButtonRole::Primary, "Primary"),
        (ButtonRole::Secondary, "Secondary"),
        (ButtonRole::Ghost, "Ghost"),
        (ButtonRole::Danger, "Danger"),
    ];
    for (role, name) in roles {
        card(ui, |ui| {
            ui.label(egui::RichText::new(name).strong().color(theme::TEXT_STRONG));
            hstack(ui, theme::SPACING_MD, |ui| {
                for (size, tag) in [
                    (ButtonSize::Sm, "sm"),
                    (ButtonSize::Md, "md"),
                    (ButtonSize::Lg, "lg"),
                ] {
                    StyledButton::new(format!("{name}-{tag}"))
                        .role(role)
                        .size(size)
                        .show(ui);
                }
            });
        });
    }
}

/// Disabled, loading, selected and loading-everything states, one per cell.
fn button_states_sheet_ui(ui: &mut egui::Ui) {
    type Paint = Box<dyn Fn(&mut egui::Ui)>;
    type Cell = (&'static str, &'static str, Paint);
    let cells: Vec<Cell> = vec![
        (
            "Disabled",
            "hover explains why (II.30.11)",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Disabled")
                    .role(ButtonRole::Primary)
                    .enabled(false)
                    .show(ui);
            }),
        ),
        (
            "Disabled + reason",
            "tooltip text attached",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Locked")
                    .role(ButtonRole::Secondary)
                    .enabled(false)
                    .show(ui);
            }),
        ),
        (
            "Loading",
            "spinner, size stable",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Loading")
                    .role(ButtonRole::Primary)
                    .loading(true)
                    .show(ui);
            }),
        ),
        (
            "Selected",
            "pressed fill",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Wall")
                    .role(ButtonRole::Ghost)
                    .selected(true)
                    .show(ui);
            }),
        ),
        (
            "Danger loading",
            "large",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Delete")
                    .role(ButtonRole::Danger)
                    .size(ButtonSize::Lg)
                    .loading(true)
                    .show(ui);
            }),
        ),
        (
            "Normal",
            "reference",
            Box::new(|ui: &mut egui::Ui| {
                StyledButton::new("Normal")
                    .role(ButtonRole::Secondary)
                    .show(ui);
            }),
        ),
    ];
    card(ui, |ui| {
        ui.label(
            egui::RichText::new("Button states")
                .strong()
                .color(theme::TEXT_STRONG),
        );
        wrap_row(ui, theme::SPACING_LG, theme::SPACING_MD, |ui| {
            for (title, note, paint) in cells {
                vstack(ui, theme::SPACING_XS, |ui| {
                    ui.label(egui::RichText::new(title).small().color(theme::TEXT_MUTED));
                    paint(ui);
                    ui.label(egui::RichText::new(note).weak());
                });
            }
        });
    });
}

/// A representative form: valid fields, an invalid field and actions.
fn form_sheet_ui(ui: &mut egui::Ui) {
    let mut name = String::from("Ground floor");
    let mut elevation = String::from("0");
    let mut length = String::from("-1");
    FieldSet::new("Storey")
        .fields(vec![
            Field::new("Name")
                .input(|ui| ui.text_edit_singleline(&mut name))
                .help_text("Shown in the level list"),
            Field::new("Elevation")
                .input(|ui| ui.text_edit_singleline(&mut elevation))
                .help_text("Millimetres above the site datum"),
        ])
        .show(ui);
    Field::new("Wall length")
        .input(|ui| ui.text_edit_singleline(&mut length))
        .error_text("Length must be positive")
        .show(ui);
    ui.add_space(theme::SPACING_SM);
    hstack(ui, theme::SPACING_MD, |ui| {
        StyledButton::new("Save storey")
            .role(ButtonRole::Primary)
            .show(ui);
        StyledButton::new("Cancel")
            .role(ButtonRole::Secondary)
            .show(ui);
    });
}

/// Layout helpers at whatever width the harness currently is: cards, a
/// wrapping button row, and a rail-style tool group.
fn layout_sheet_ui(ui: &mut egui::Ui) {
    vstack(ui, theme::SPACING_MD, |ui| {
        card(ui, |ui| {
            ui.label(
                egui::RichText::new("Wrapping row")
                    .strong()
                    .color(theme::TEXT_STRONG),
            );
            wrap_row(ui, theme::SPACING_SM, theme::SPACING_SM, |ui| {
                for label in [
                    "Select",
                    "Wall",
                    "Door",
                    "Window",
                    "Dimension",
                    "Label",
                    "Block",
                    "Measure",
                ] {
                    StyledButton::new(label)
                        .role(ButtonRole::Secondary)
                        .size(ButtonSize::Sm)
                        .show(ui);
                }
            });
        });
        hstack(ui, theme::SPACING_MD, |ui| {
            card_compact(ui, |ui| {
                ui.label(egui::RichText::new("Compact card").small());
            });
            card_compact(ui, |ui| {
                ui.label(egui::RichText::new("Second card").small());
            });
        });
        tool_group(ui, "Draw", |ui| {
            StyledButton::new("Line").role(ButtonRole::Ghost).show(ui);
            StyledButton::new("Ray").role(ButtonRole::Ghost).show(ui);
        });
    });
}

// ---------------------------------------------------------------------------
// Evidence tests (env-gated)
// ---------------------------------------------------------------------------

#[test]
fn evidence_buttons_light() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(buttons_sheet_ui);
    harness.set_size(egui::vec2(540.0, 440.0));
    harness.run();
    render_to(&mut harness, "buttons-evidence", "buttons-light");
}

#[test]
fn evidence_buttons_dark() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(buttons_sheet_ui);
    harness.set_size(egui::vec2(540.0, 440.0));
    harness.ctx.set_theme(egui::Theme::Dark);
    harness.run();
    render_to(&mut harness, "buttons-evidence", "buttons-dark");
}

#[test]
fn evidence_buttons_states() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(button_states_sheet_ui);
    harness.set_size(egui::vec2(540.0, 300.0));
    // The Loading state paints a spinner, which repaints forever — run a
    // fixed step budget instead of run()'s settle-until-stable loop.
    harness.run_steps(3);
    render_to(&mut harness, "buttons-evidence", "buttons-states");
}

#[test]
fn evidence_buttons_focus_ring() {
    if !evidence_enabled() {
        return;
    }
    // Tab once: the first button holds keyboard focus and paints its ring —
    // the visible half of the keyboard story (guide II.30.7.3).
    let mut harness = Harness::new_ui(buttons_sheet_ui);
    harness.set_size(egui::vec2(540.0, 440.0));
    harness.run();
    harness.key_press(egui::Key::Tab);
    harness.run();
    render_to(&mut harness, "buttons-evidence", "buttons-focus");
}

#[test]
fn evidence_form_light() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(form_sheet_ui);
    harness.set_size(egui::vec2(420.0, 460.0));
    harness.run();
    render_to(&mut harness, "form-evidence", "form-light");
}

#[test]
fn evidence_form_dark() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(form_sheet_ui);
    harness.set_size(egui::vec2(420.0, 460.0));
    harness.ctx.set_theme(egui::Theme::Dark);
    harness.run();
    render_to(&mut harness, "form-evidence", "form-dark");
}

#[test]
fn evidence_form_error_state() {
    if !evidence_enabled() {
        return;
    }
    let mut harness = Harness::new_ui(form_sheet_ui);
    harness.set_size(egui::vec2(420.0, 460.0));
    harness.run();
    render_to(&mut harness, "form-evidence", "form-error");
}

#[test]
fn evidence_layout_narrow_and_wide() {
    if !evidence_enabled() {
        return;
    }
    // II.30.8 reflow evidence: the same sheet at 400 px and at 1200 px.
    let mut harness = Harness::new_ui(layout_sheet_ui);
    harness.set_size(egui::vec2(400.0, 340.0));
    harness.run();
    render_to(&mut harness, "layout-evidence", "layout-narrow-400");

    harness.set_size(egui::vec2(1200.0, 340.0));
    harness.run();
    render_to(&mut harness, "layout-evidence", "layout-wide-1200");

    harness.set_size(egui::vec2(800.0, 420.0));
    harness.run();
    render_to(&mut harness, "layout-evidence", "layout-cards");
}

// ---------------------------------------------------------------------------
// Always-on assertions (the gate, not the pictures — ticket T-008)
// ---------------------------------------------------------------------------

#[test]
fn kittest_layout_resizes_without_overlap_or_clipping() {
    // AC-04 / II.30.7.6: resize 400 → 1200 and prove no two controls
    // intersect at either width.
    let mut harness = Harness::new_ui(layout_sheet_ui);
    for width in [400.0, 1200.0] {
        harness.set_size(egui::vec2(width, 340.0));
        harness.run();

        let rects: Vec<egui::Rect> = harness
            .get_all_by_role(egui::accesskit::Role::Button)
            .map(|node| node.rect())
            .collect();
        assert!(
            rects.len() >= 10,
            "the wrapping row and tool group must expose their buttons at {width}px"
        );
        for index in 0..rects.len() {
            let rect = rects[index];
            assert!(
                rect.width() >= 24.0 && rect.height() >= 24.0,
                "button {index} at {width}px is below the 24pt floor: {rect:?}"
            );
            for other in (index + 1)..rects.len() {
                assert!(
                    !rect.intersects(rects[other]),
                    "buttons {index} and {other} overlap at {width}px: {:?} vs {:?}",
                    rects[index],
                    rects[other]
                );
            }
        }
    }
}
