//! Bottom status bar: coordinate readout, active tool, snap toggles, selection
//! count, zoom level, unit system and the suite-standard version link.
//!
//! Pure presentation: it mutates only UI state (snap toggles) and dispatches
//! core `Command`s for unit changes; it never owns geometry or history.

use crate::canvas::{SnapModeKind, SnapModes};
use eframe::egui;
use lintel_core::command::Command;
use lintel_core::units::UnitSystem;

pub fn render(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        // Coordinate readout (bottom-left of the bar, first thing the eye
        // lands on when checking a point), with the snap that is winning.
        let units = app.doc.project().units;
        let coord_text = match app.pointer_world {
            Some(p) => format!("X: {}   Y: {}", units.format(p.x), units.format(p.y)),
            None => "X: —   Y: —".to_string(),
        };
        ui.label(egui::RichText::new(coord_text).small());
        let snap_text = match app.last_snap {
            Some(SnapModeKind::Free) | None => "Snap: free".to_string(),
            Some(kind) => format!("Snap: {}", kind.label()),
        };
        ui.separator();
        ui.label(egui::RichText::new(snap_text).small().color(egui::Color32::GRAY));
        ui.separator();

        // Active tool name.
        ui.label(egui::RichText::new(format!("Tool: {}", app.tool.label())).small());
        ui.separator();

        // Snap mode chips. The active ones are drawn in the marker colour of
        // their kind, so the bar and the canvas agree; clicking one toggles it,
        // exactly as its key does.
        let grid_on = app.snap.grid_enabled;
        chip(
            ui,
            "Grid",
            grid_on,
            egui::Color32::GRAY,
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
            egui::Color32::GRAY,
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
                hint,
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
            .button(units.label())
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
                    .color(egui::Color32::GRAY),
            );
            if version
                .on_hover_text("Release notes and update status")
                .clicked()
            {
                app.showing_notes = true;
            }
        });
    });
}

/// One toggle chip: the label, coloured by its kind when it is active.
fn chip(
    ui: &mut egui::Ui,
    label: &str,
    active: bool,
    active_colour: egui::Color32,
    hint: String,
    toggle: impl FnOnce(),
) {
    let text = egui::RichText::new(label).small().color(if active {
        active_colour
    } else {
        egui::Color32::GRAY
    });
    let response = ui.selectable_label(active, text).on_hover_text(hint);
    if response.clicked() {
        toggle();
    }
}
