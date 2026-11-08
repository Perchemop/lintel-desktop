//! Bottom status bar: coordinate readout, active tool, snap toggles, selection
//! count, zoom level, unit system and the suite-standard version link.
//!
//! Pure presentation: it mutates only UI state (snap toggles) and dispatches
//! core `Command`s for unit changes; it never owns geometry or history.

use eframe::egui;

use lintel_core::command::Command;
use lintel_core::units::UnitSystem;

pub fn render(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) {
    ui.horizontal(|ui| {
        // Coordinate readout (bottom-left of the bar, first thing the eye
        // lands on when checking a point).
        let coord_text = match app.pointer_world {
            Some(p) => {
                let units = app.doc.project().units;
                format!(
                    "X: {}   Y: {}",
                    units.format(p.x),
                    units.format(p.y)
                )
            }
            None => "X: —   Y: —".to_string(),
        };
        ui.label(egui::RichText::new(coord_text).small());
        ui.separator();

        // Active tool name.
        ui.label(egui::RichText::new(format!("Tool: {}", app.tool.label())).small());
        ui.separator();

        // Snap / grid / ortho toggle indicators (clickable).
        let grid_resp = ui
            .selectable_label(app.snap.grid_enabled, "Grid")
            .on_hover_text("Toggle the drawing grid (G).");
        if grid_resp.clicked() {
            app.snap.grid_enabled = !app.snap.grid_enabled;
        }
        let ortho_resp = ui
            .selectable_label(app.snap.ortho, "Ortho")
            .on_hover_text("Toggle orthogonal drawing (O).");
        if ortho_resp.clicked() {
            app.snap.ortho = !app.snap.ortho;
        }
        ui.separator();

        // Selection count.
        let selected = if app.selection.is_some() {
            1usize
        } else {
            0usize
        };
        ui.label(egui::RichText::new(format!("{} selected", selected)).small());
        ui.separator();

        // Zoom level.
        ui.label(egui::RichText::new(format!("Zoom: {}%", app.view.zoom_percent())).small());
        ui.separator();

        // Unit system indicator (click to flip).
        let units = app.doc.project().units;
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
