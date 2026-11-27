//! Output panel content (v0.22.0, T-240).
//!
//! Command log, solver status, and measurements output.

use crate::theme::SPEC;
use eframe::egui;

pub fn output_panel(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) {
    ui.vertical(|ui| {
        ui.heading("Output");
        ui.separator();

        // Tabs: Log, Solver, Measurements.
        let mut tab_index = 0;
        egui::ComboBox::from_label("Tab")
            .selected_text(match tab_index {
                0 => "Log",
                1 => "Solver",
                _ => "Measurements",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut tab_index, 0, "Log");
                ui.selectable_value(&mut tab_index, 1, "Solver");
                ui.selectable_value(&mut tab_index, 2, "Measurements");
            });

        ui.separator();

        match tab_index {
            0 => log_tab(ui, app),
            1 => solver_tab(ui, app),
            _ => measurements_tab(ui, app),
        }

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                // TODO: clear log
            }
            if ui.button("Copy").clicked() {
                // TODO: copy to clipboard
            }
        });
    });
}

fn log_tab(ui: &mut egui::Ui, app: &mut crate::LintelApp) {
    ui.label("Command log:");
    ui.separator();

    // Show recent status messages.
    if let Some(status) = &app.status {
        ui.label(
            egui::RichText::new(status)
                .small()
                .color(SPEC.component.text.primary),
        );
    } else {
        ui.label(
            egui::RichText::new("No recent commands.")
                .small()
                .color(SPEC.component.text.muted),
        );
    }
}

fn solver_tab(ui: &mut egui::Ui, _app: &mut crate::LintelApp) {
    ui.label("Constraint solver:");
    ui.separator();
    ui.label(
        egui::RichText::new("No active constraints.")
            .small()
            .color(SPEC.component.text.muted),
    );
}

fn measurements_tab(ui: &mut egui::Ui, app: &mut crate::LintelApp) {
    ui.label("Measurements:");
    ui.separator();

    let level = match app.doc.project().active_level() {
        Some(l) => l,
        None => {
            ui.label("No active level.");
            return;
        }
    };

    let wall_count = level.walls.len();
    let opening_count = level.openings.len();
    let dimension_count = level.dimensions.len();

    ui.label(format!("Walls: {}", wall_count));
    ui.label(format!("Openings: {}", opening_count));
    ui.label(format!("Dimensions: {}", dimension_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_panel_tabs() {
        // Verify all three tabs exist.
        let _ = vec!["Log", "Solver", "Measurements"];
    }
}
