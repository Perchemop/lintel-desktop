//! Output panel content (v0.22.0, T-240).
//!
//! Stub implementation — full output panel (command log, solver status,
//! measurements) is implemented here in a follow-up slice.

use crate::theme::SPEC;
use eframe::egui;

pub fn output_panel(ui: &mut egui::Ui, _app: &mut crate::LintelApp, _now: i64) {
    ui.vertical(|ui| {
        ui.heading("Output");
        ui.separator();
        ui.label("Command log, solver status and measurements output.");
        ui.separator();
        if ui.button("Clear").clicked() {
            // TODO: clear log
        }
    });
    ui.label(
        egui::RichText::new("Log, solver status and measurements coming soon.")
            .small()
            .color(SPEC.component.text.hint),
    );
}
