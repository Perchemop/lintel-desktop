//! Constraints panel content (v0.22.0, T-239).
//!
//! Stub implementation — full constraint management (list, add form, solver
//! status) is implemented here in a follow-up slice.

use crate::theme::SPEC;
use eframe::egui;

pub fn constraints_panel(ui: &mut egui::Ui, _app: &mut crate::LintelApp, _now: i64) {
    ui.vertical(|ui| {
        ui.heading("Constraints");
        ui.separator();
        ui.label("Constraint management — TODO: full constraints panel");
        ui.separator();
        if ui.button("Add Constraint").clicked() {
            // TODO: add constraint command
        }
    });
    ui.label(
        egui::RichText::new("Constraint list, add form and solver status coming soon.")
            .small()
            .color(SPEC.component.text.hint),
    );
}
