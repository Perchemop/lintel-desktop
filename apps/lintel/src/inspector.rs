//! Right-hand inspector: properties of the selected element plus global
//! view controls. All edits flow through `lintel_core::Command` values
//! returned here and applied by the shell.

use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    ids::{Hit, OpeningId, WallId},
    opening::OpeningKind,
    snap::SnapSettings,
};

pub fn inspector(
    ui: &mut egui::Ui,
    doc: &Document,
    selection: &Option<Hit>,
    snap: &mut SnapSettings,
) -> Option<Command> {
    let mut command: Option<Command> = None;

    egui::CollapsingHeader::new("Element")
        .default_open(true)
        .show(ui, |ui| match selection {
            Some(Hit::Wall(id)) => {
                let wall = {
                    let level = doc.project().active_level().expect("level");
                    level.wall(*id).expect("wall").clone()
                };
                ui.label(format!(
                    "Wall {} — {:.0} mm long",
                    id.value(),
                    wall.length()
                ));
                ui.separator();
                command = command.or(wall_fields(ui, doc, *id, &wall));
            }
            Some(Hit::Opening(id)) => {
                let opening = {
                    let level = doc.project().active_level().expect("level");
                    level.opening(*id).expect("opening").clone()
                };
                ui.label(format!(
                    "{} {} — {:.0} × {:.0} mm",
                    opening.kind_label(),
                    id.value(),
                    opening.width,
                    opening.height
                ));
                ui.separator();
                command = command.or(opening_fields(ui, doc, *id, &opening));
            }
            None => {
                ui.label("Select a wall or opening to edit it.");
            }
        });

    ui.separator();
    egui::CollapsingHeader::new("View")
        .default_open(false)
        .show(ui, |ui| {
            ui.checkbox(&mut snap.grid_enabled, "Show grid");
            ui.horizontal(|ui| {
                ui.label("Snap tolerance:");
                ui.add(
                    egui::DragValue::new(&mut snap.anchor_tolerance)
                        .speed(1.0)
                        .range(10.0..=1000.0)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Grid pitch:");
                ui.add(
                    egui::DragValue::new(&mut snap.grid)
                        .speed(10.0)
                        .range(50.0..=5000.0)
                        .suffix(" mm"),
                );
            });
            ui.checkbox(&mut snap.ortho, "Orthogonal drawing");
        });

    ui.separator();
    egui::CollapsingHeader::new("Storeys")
        .default_open(false)
        .show(ui, |ui| {
            let active = doc.project().active_level;
            for level in &doc.project().levels {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(level.id == active, &level.name)
                        .clicked()
                        && level.id != active
                    {
                        command = Some(Command::SetActiveLevel { id: level.id });
                    }
                    ui.label(format!("{} mm", level.elevation));
                });
            }
            ui.horizontal(|ui| {
                if ui.button("Add storey…").clicked() {
                    command = Some(Command::AddLevel {
                        name: format!("Level {}", doc.project().levels.len() + 1),
                        elevation: doc.project().levels.len() as f64 * 3000.0,
                    });
                }
            });
        });

    command
}

fn wall_fields(
    ui: &mut egui::Ui,
    doc: &Document,
    id: WallId,
    wall: &lintel_core::wall::Wall,
) -> Option<Command> {
    let mut command: Option<Command> = None;
    let mut thickness = wall.thickness;
    let mut height = wall.height;

    ui.horizontal(|ui| {
        ui.label("Thickness:");
        ui.add(
            egui::DragValue::new(&mut thickness)
                .speed(5.0)
                .range(
                    lintel_core::wall::MIN_WALL_THICKNESS..=lintel_core::wall::MAX_WALL_THICKNESS,
                )
                .suffix(" mm"),
        );
        if ui.button("Apply").clicked() && (thickness - wall.thickness).abs() > f64::EPSILON {
            return Some(Command::SetWallThickness { id, thickness });
        }
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(
            egui::DragValue::new(&mut height)
                .speed(50.0)
                .range(lintel_core::wall::MIN_WALL_HEIGHT..=lintel_core::wall::MAX_WALL_HEIGHT)
                .suffix(" mm"),
        );
        if ui.button("Apply").clicked() && (height - wall.height).abs() > f64::EPSILON {
            return Some(Command::SetWallHeight { id, height });
        }
    });
    command
}

fn opening_fields(
    ui: &mut egui::Ui,
    doc: &Document,
    id: OpeningId,
    opening: &lintel_core::opening::Opening,
) -> Option<Command> {
    let mut command: Option<Command> = None;
    let level = doc.project().active_level().expect("level");
    let wall = level.wall(opening.host).expect("host wall");

    let mut kind = opening.kind;
    let mut width = opening.width;
    let mut height = opening.height;
    let mut sill = opening.sill;
    let mut offset = opening.offset;
    let mut flipped = opening.flipped;

    ui.horizontal(|ui| {
        ui.label("Kind:");
        ui.radio_value(&mut kind, OpeningKind::Door, "Door");
        ui.radio_value(&mut kind, OpeningKind::Window, "Window");
    });
    if kind != opening.kind {
        return Some(Command::SetOpeningKind { id, kind });
    }

    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(
            egui::DragValue::new(&mut width)
                .speed(50.0)
                .range(
                    lintel_core::opening::MIN_OPENING_WIDTH
                        ..=lintel_core::opening::MAX_OPENING_WIDTH,
                )
                .suffix(" mm"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(
            egui::DragValue::new(&mut height)
                .speed(50.0)
                .range(
                    lintel_core::opening::MIN_OPENING_HEIGHT
                        ..=lintel_core::opening::MAX_OPENING_HEIGHT,
                )
                .suffix(" mm"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Sill:");
        ui.add(
            egui::DragValue::new(&mut sill)
                .speed(50.0)
                .range(0.0..=20000.0)
                .suffix(" mm"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Position along wall:");
        ui.add(
            egui::DragValue::new(&mut offset)
                .speed(10.0)
                .range(0.0..=wall.length())
                .suffix(" mm"),
        );
    });
    ui.checkbox(&mut flipped, "Flip door hinge");

    let size_changed = (width - opening.width).abs() > f64::EPSILON
        || (height - opening.height).abs() > f64::EPSILON
        || (sill - opening.sill).abs() > f64::EPSILON;
    if size_changed {
        return Some(Command::SetOpeningSize {
            id,
            width,
            height,
            sill,
        });
    }
    if (offset - opening.offset).abs() > f64::EPSILON {
        return Some(Command::MoveOpening { id, offset });
    }
    if flipped != opening.flipped {
        return Some(Command::FlipOpening { id });
    }

    command
}
