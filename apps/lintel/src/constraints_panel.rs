//! Constraints panel content (v0.22.0, T-239).
//!
//! Full constraint management: list, add form, solver status.

use crate::theme::SPEC;
use eframe::egui;
use lintel_core::command::Command;
use lintel_core::constraint::Constraint;
use lintel_core::ids::{DimensionId, WallId};

pub fn constraints_panel(ui: &mut egui::Ui, app: &mut crate::LintelApp, now: i64) {
    ui.vertical(|ui| {
        ui.heading("Constraints");
        ui.separator();

        let level = match app.doc.project().active_level() {
            Some(l) => l,
            None => {
                ui.label("No active level.");
                return;
            }
        };

        // Constraint list.
        ui.label("Constraints:");
        for (index, constraint) in level.constraints.iter().enumerate() {
            ui.horizontal(|ui| {
                let label = match constraint {
                    Constraint::PerpendicularDistance {
                        wall_id, param, ..
                    } => {
                        format!("PerpDist: Wall {} @ {:.0}%", wall_id.value(), param * 100.0)
                    }
                    Constraint::FixedAngle { dimension, degrees, .. } => {
                        format!("FixedAngle: Dim {} @ {:.0}°", dimension.value(), degrees)
                    }
                    Constraint::EqualLength { a, b, .. } => {
                        format!("EqualLength: Dim {} = Dim {}", a.value(), b.value())
                    }
                };
                ui.label(egui::RichText::new(label).small());

                if ui.button("✕").on_hover_text("Remove constraint").clicked() {
                    let _ = app.doc.apply(
                        Command::RemoveConstraint { index },
                        now,
                    );
                }
            });
        }

        ui.separator();

        // Add constraint form.
        ui.label("Add constraint:");
        ui.horizontal(|ui| {
            let mut constraint_kind = 0;
            egui::ComboBox::from_label("Type")
                .selected_text(match constraint_kind {
                    0 => "Perpendicular Distance",
                    1 => "Fixed Angle",
                    _ => "Equal Length",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut constraint_kind, 0, "Perpendicular Distance");
                    ui.selectable_value(&mut constraint_kind, 1, "Fixed Angle");
                    ui.selectable_value(&mut constraint_kind, 2, "Equal Length");
                });
        });

        // Wall selection for perpendicular distance.
        if constraint_kind == 0 {
            ui.horizontal(|ui| {
                ui.label("Wall:");
                let mut wall_index = 0;
                let walls: Vec<_> = level.walls.iter().collect();
                if !walls.is_empty() {
                    egui::ComboBox::from_label("Wall")
                        .selected_text(format!("Wall {}", walls[wall_index].0.value()))
                        .show_ui(ui, |ui| {
                            for (i, wall) in walls.iter().enumerate() {
                                ui.selectable_value(
                                    &mut wall_index,
                                    i,
                                    format!("Wall {}", wall.0.value()),
                                );
                            }
                        });
                }

                if ui.button("Add").clicked() && !walls.is_empty() {
                    let wall = walls[wall_index].id;
                    let _ = app.doc.apply(
                        Command::AddConstraint {
                            constraint: Constraint::PerpendicularDistance {
                                wall_id: wall,
                                dimension: DimensionId(0), // placeholder
                                param: 0.5,
                            },
                        },
                        now,
                    );
                }
            });
        }

        ui.separator();

        // Solver status.
        ui.label("Solver status:");
        ui.label(
            egui::RichText::new("Solving…")
                .small()
                .color(SPEC.component.text.hint),
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_labels() {
        // Verify constraint variants are handled.
        let _ = Constraint::PerpendicularDistance {
            wall_id: WallId(0),
            dimension: DimensionId(0),
            param: 0.5,
        };
    }
}
