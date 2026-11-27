//! Bottom status bar: coordinate readout, active tool, snap toggles, selection
//! count, zoom level, unit system and the suite-standard version link.
//!
//! Pure presentation: it mutates only UI state (snap toggles) and dispatches
//! core `Command`s for unit changes; it never owns geometry or history.

use crate::canvas::{SnapModeKind, SnapModes};
use crate::theme::{self, SPEC};
use eframe::egui;
use lintel_core::command::Command;
use lintel_core::units::UnitSystem;

pub fn render(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) -> egui::Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        // Coordinate readout (bottom-left of the bar, first thing the eye
        // lands on when checking a point), with the snap that is winning.
        let units = app.doc.project().units;
        let coord_text = match app.last_snap.map(|snap| snap.point).or(app.pointer_world) {
            Some(p) => format!("X: {}   Y: {}", units.format(p.x), units.format(p.y)),
            None => "X: —   Y: —".to_string(),
        };
        ui.label(egui::RichText::new(coord_text).small());
        let snap_text = match app.last_snap {
            Some(snap) if snap.kind == SnapModeKind::Free => "Snap: free".to_string(),
            Some(snap) => format!("Snap: {}", snap.kind.label()),
            None => "Snap: free".to_string(),
        };
        ui.separator();
        ui.label(
            egui::RichText::new(snap_text)
                .small()
                .color(SPEC.component.text.hint),
        );
        ui.separator();

        // Active tool name.
        ui.label(egui::RichText::new(format!("Tool: {}", app.tool.label())).small());
        ui.separator();

        // Contextual hint for the active tool.
        let hint_text = match app.tool {
            crate::app::tool::Tool::Select => {
                if app.selection.is_empty() {
                    "Click wall to select • Shift+Click multi-select • Esc clear"
                } else {
                    "Drag to move • Click dimension to edit • Del delete • Esc clear"
                }
            }
            crate::app::tool::Tool::Wall => {
                "Click to start • Move to preview • Click to place • Esc cancel"
            }
            crate::app::tool::Tool::Door => "Click wall to place door • Esc cancel",
            crate::app::tool::Tool::Window => "Click wall to place window • Esc cancel",
            crate::app::tool::Tool::Dimension => {
                "Click first point • Click second point • Esc cancel"
            }
            crate::app::tool::Tool::Label => {
                "Click to place label • Type text • Enter confirm • Esc cancel"
            }
            crate::app::tool::Tool::Measurements => "Click room to measure • Esc cancel",
            crate::app::tool::Tool::Block => "Click to place instance • Esc cancel",
            crate::app::tool::Tool::ConstructionLine => {
                "Click first point • Click second point • Esc cancel"
            }
            crate::app::tool::Tool::ConstructionRay => {
                "Click first point • Click second point • Esc cancel"
            }
            crate::app::tool::Tool::ConstructionCircle => {
                "Click first point • Click second point • Esc cancel"
            }
            crate::app::tool::Tool::ConstructionArc => {
                "Click first point • Click second point • Esc cancel"
            }
        };
        ui.label(
            egui::RichText::new(hint_text)
                .small()
                .color(SPEC.component.text.hint),
        );
        ui.separator();

        // Snap mode chips. The active ones are drawn in the marker colour of
        // their kind, so the bar and the canvas agree; clicking one toggles it,
        // exactly as its key does.
        let grid_on = app.snap.grid_enabled;
        chip(
            ui,
            "Grid",
            grid_on,
            SPEC.component.text.hint,
            "Toggle the drawing grid (G).",
            || {
                app.snap.grid_enabled = !grid_on;
            },
        );
        let ortho_on = app.snap.ortho;
        chip(
            ui,
            "Ortho",
            ortho_on,
            SPEC.component.text.hint,
            "Toggle orthogonal drawing (O).",
            || {
                app.snap.ortho = !ortho_on;
            },
        );
        for kind in SnapModes::KINDS {
            let active = app.snap_modes.enabled(kind);
            let hint = format!("{} snap ({})", kind.label(), SnapModes::key_hint(kind));
            chip(
                ui,
                SnapModes::label(kind),
                active,
                kind.colour(),
                &hint,
                || app.snap_modes.toggle(kind),
            );
        }
        ui.separator();

        // Selection count.
        let count = app.selection.len();
        let selected_text = match count {
            0 => "Nothing selected".to_string(),
            1 => "1 selected".to_string(),
            n => format!("{n} selected"),
        };
        ui.label(egui::RichText::new(selected_text).small());
        ui.separator();

        // Zoom level.
        ui.label(egui::RichText::new(format!("Zoom: {}%", app.view.zoom_percent())).small());
        ui.separator();

        // Unit system indicator (click to flip).
        let next_units = match units {
            UnitSystem::Metric => UnitSystem::Imperial,
            UnitSystem::Imperial => UnitSystem::Metric,
        };
        if ui
            .add(
                egui::Button::new(units.label())
                    .fill(SPEC.component.button.primary_sm.fill)
                    .min_size(SPEC.component.button.primary_sm.min_size)
                    .stroke(egui::Stroke::new(2.0, SPEC.component.button.ring)),
            )
            .on_hover_text("Click to switch the unit system.")
            .clicked()
        {
            let _ = app.doc.apply(Command::SetUnits { units: next_units }, now);
        }

        // Version link stays in the bottom-right corner per suite standard.
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let version = ui.link(
                egui::RichText::new(format!("Lintel v{}", env!("CARGO_PKG_VERSION")))
                    .small()
                    .color(SPEC.component.text.hint),
            );
            if version
                .on_hover_text("Release notes and update status")
                .clicked()
            {
                app.showing_notes = true;
            }

            // Theme selector: Light / Dark / High Contrast / System.
            ui.separator();
            let mut theme_mode = app.theme_mode;
            egui::ComboBox::from_label("")
                .selected_text(theme_mode.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut theme_mode, theme::ThemeMode::Light, "Light");
                    ui.selectable_value(&mut theme_mode, theme::ThemeMode::Dark, "Dark");
                    ui.selectable_value(
                        &mut theme_mode,
                        theme::ThemeMode::HighContrast,
                        "High Contrast",
                    );
                    ui.selectable_value(&mut theme_mode, theme::ThemeMode::System, "System");
                });
            if theme_mode != app.theme_mode {
                app.theme_mode = theme_mode;
            }
        });
    })
    .response
}

/// One toggle chip: the label, coloured by its kind when it is active.
fn chip(
    ui: &mut egui::Ui,
    label: &str,
    active: bool,
    active_colour: egui::Color32,
    hint: &str,
    toggle: impl FnOnce(),
) {
    let text = egui::RichText::new(label).small().color(if active {
        active_colour
    } else {
        SPEC.component.text.hint
    });
    let response = ui.selectable_label(active, text).on_hover_text(hint);
    if response.clicked() {
        toggle();
    }
}
