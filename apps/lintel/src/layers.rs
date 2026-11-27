//! Layers panel content (v0.22.0, T-238).
//!
//! Stub implementation — full layer management (visibility, lock, colour,
//! add/remove) is implemented here in a follow-up slice.

use crate::theme::SPEC;
use eframe::egui;

#[allow(dead_code)]
pub fn layers_panel(ui: &mut egui::Ui, _app: &mut crate::LintelApp, _now: i64) {
    ui.vertical(|ui| {
        ui.heading("Layers");
        ui.separator();
        ui.label("Layer management — TODO: full layer panel");
        ui.separator();
        if ui.button("Add Layer").clicked() {
            // TODO: add layer command
        }
    });
    ui.label(
        egui::RichText::new("Visibility, lock, colour and reordering coming soon.")
            .small()
            .color(SPEC.component.text.hint),
    );
}
