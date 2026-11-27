//! Layers panel content (v0.22.0, T-238).
//!
//! Full layer management: visibility, lock, colour, name, add/remove.

use crate::theme::SPEC;
use eframe::egui;
use lintel_core::command::Command;
use lintel_core::ids::LayerId;
use lintel_core::layer::{Layer, LayerProperty, Rgb, Linetype};

fn rgb_to_color32(rgb: &Rgb) -> egui::Color32 {
    egui::Color32::from_rgb(rgb.r, rgb.g, rgb.b)
}

pub fn layers_panel(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) {
    ui.vertical(|ui| {
        ui.heading("Layers");
        ui.separator();

        let level = match app.doc.project().active_level() {
            Some(l) => l,
            None => {
                ui.label("No active level.");
                return;
            }
        };

        for layer in &level.layers {
            ui.horizontal(|ui| {
                // Visibility toggle.
                let mut visible = layer.visible;
                let vis_resp = ui
                    .checkbox(&mut visible, "")
                    .on_hover_text(if visible { "Hide layer" } else { "Show layer" });
                if vis_resp.changed() {
                    let _ = app.doc.apply(
                        Command::SetLayerProperty {
                            layer_id: layer.id,
                            property: LayerProperty::Visible(visible),
                        },
                        now,
                    );
                }

                // Lock toggle.
                let mut locked = layer.locked;
                let lock_resp = ui
                    .checkbox(&mut locked, "")
                    .on_hover_text(if locked { "Unlock layer" } else { "Lock layer" });
                if lock_resp.changed() {
                    let _ = app.doc.apply(
                        Command::SetLayerProperty {
                            layer_id: layer.id,
                            property: LayerProperty::Locked(locked),
                        },
                        now,
                    );
                }

                // Colour indicator.
                let color = layer.color.as_ref().map_or(SPEC.component.text.primary, rgb_to_color32);
                let color_rect = egui::Rect::from_min_size(
                    ui.cursor().min,
                    egui::vec2(16.0, 16.0),
                );
                ui.painter().rect_filled(color_rect, 2.0, color);
                ui.label(egui::RichText::new(" ").small());

                // Layer name (editable).
                let mut name = layer.name.clone();
                if ui.text_edit_singleline(&mut name).changed() && name != layer.name {
                    let _ = app.doc.apply(
                        Command::SetLayerProperty {
                            layer_id: layer.id,
                            property: LayerProperty::Name(name.clone()),
                        },
                        now,
                    );
                }

                // Remove button (disabled for default layer).
                if !layer.is_default() {
                    if ui.button("✕").on_hover_text("Remove layer").clicked() {
                        let _ = app.doc.apply(
                            Command::DeleteLayer {
                                layer_id: layer.id,
                            },
                            now,
                        );
                    }
                } else {
                    ui.add_enabled(false, egui::Button::new("✕"));
                }
            });
        }

        ui.separator();

        // Add layer button.
        ui.horizontal(|ui| {
            ui.label("New layer name:");
            let mut new_name = String::new();
            let name_resp = ui.text_edit_singleline(&mut new_name);
            if ui.button("Add Layer").clicked() && !new_name.trim().is_empty() {
                let _ = app.doc.apply(
                    Command::CreateLayer {
                        name: new_name.trim().to_string(),
                        color: None,
                        linetype: Linetype::Continuous,
                        lineweight_mm: 0.25,
                    },
                    now,
                );
            }
            let _ = name_resp;
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_color32_converts_correctly() {
        let rgb = Rgb { r: 255, g: 128, b: 0 };
        let color = rgb_to_color32(&rgb);
        assert_eq!(color, egui::Color32::from_rgb(255, 128, 0));
    }
}
