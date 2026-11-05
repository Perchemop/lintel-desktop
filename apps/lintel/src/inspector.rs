//! Right-hand inspector: properties of the selected element plus global
//! view controls. All edits flow through `lintel_core::Command` values
//! returned here and applied by the shell.

use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    ids::{DimensionId, LevelId, OpeningId, WallId},
    join::Hit,
    opening::OpeningKind,
    snap::SnapSettings,
};

pub fn inspector(
    ui: &mut egui::Ui,
    doc: &Document,
    selection: &Option<Hit>,
    snap: &mut SnapSettings,
) -> Option<Command> {
    let element_cmd = egui::CollapsingHeader::new("Element")
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
                wall_fields(ui, *id, &wall)
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
                opening_fields(ui, doc, *id, &opening)
            }
            Some(Hit::Dimension(id)) => {
                let dimension = {
                    let level = doc.project().active_level().expect("level");
                    level.dimension(*id).expect("dimension").clone()
                };
                ui.label(format!(
                    "{} {} — {:.0} mm",
                    dimension.kind.label(),
                    id.value(),
                    dimension.length()
                ));
                ui.separator();
                dimension_fields(ui, *id, &dimension)
            }
            None => {
                ui.label("Select a wall or opening to edit it.");
                None
            }
        })
        .body_returned
        .flatten();

    ui.separator();

    let view_cmd = egui::CollapsingHeader::new("View")
        .default_open(false)
        .show(ui, |ui| {
            ui.checkbox(&mut snap.grid_enabled, "Show grid")
                .on_hover_text("Show a background grid on the canvas.");
            ui.horizontal(|ui| {
                ui.label("Snap tolerance:");
                ui.add(
                    egui::DragValue::new(&mut snap.anchor_tolerance)
                        .speed(1.0)
                        .range(10.0..=1000.0)
                        .suffix(" mm"),
                )
                .on_hover_text("Maximum distance in mm to snap to wall anchors.");
            });
            ui.horizontal(|ui| {
                ui.label("Grid pitch:");
                ui.add(
                    egui::DragValue::new(&mut snap.grid)
                        .speed(10.0)
                        .range(50.0..=5000.0)
                        .suffix(" mm"),
                )
                .on_hover_text("Grid spacing in mm.");
            });
            ui.checkbox(&mut snap.ortho, "Orthogonal drawing")
                .on_hover_text("Constrain new walls to horizontal or vertical.");
            None::<Command>
        })
        .body_returned
        .flatten();

    ui.separator();

    let storeys_cmd = egui::CollapsingHeader::new("Storeys")
        .default_open(false)
        .show(ui, |ui| {
            let active = doc.project().active_level;
            let mut changed: Option<LevelId> = None;
            for level in &doc.project().levels {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(level.id == active, &level.name)
                        .clicked()
                        && level.id != active
                    {
                        changed = Some(level.id);
                    }
                    ui.label(format!("{} mm", level.elevation));
                });
            }
            changed.map(|id| Command::SetActiveLevel { id })
        })
        .body_returned
        .flatten();

    element_cmd.or(storeys_cmd).or(view_cmd)
}

fn wall_fields(ui: &mut egui::Ui, id: WallId, wall: &lintel_core::wall::Wall) -> Option<Command> {
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
        )
        .on_hover_text("Wall thickness in mm (20–2000). Press Apply to commit.");
        if ui
            .button("Apply")
            .on_hover_text("Apply the new wall thickness.")
            .clicked()
            && (thickness - wall.thickness).abs() > f64::EPSILON
        {
            command = Some(Command::SetWallThickness { id, thickness });
        }
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(
            egui::DragValue::new(&mut height)
                .speed(50.0)
                .range(lintel_core::wall::MIN_WALL_HEIGHT..=lintel_core::wall::MAX_WALL_HEIGHT)
                .suffix(" mm"),
        )
        .on_hover_text("Wall height in mm (300–20000). Press Apply to commit.");
        if ui
            .button("Apply")
            .on_hover_text("Apply the new wall height.")
            .clicked()
            && (height - wall.height).abs() > f64::EPSILON
        {
            command = Some(Command::SetWallHeight { id, height });
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
    let command: Option<Command> = None;
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
        ui.radio_value(&mut kind, OpeningKind::Door, "Door")
            .on_hover_text("A hinged door opening in the wall.");
        ui.radio_value(&mut kind, OpeningKind::Window, "Window")
            .on_hover_text("A fixed window opening in the wall.");
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
        )
        .on_hover_text("Opening width in mm (200–6000).");
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
        )
        .on_hover_text("Opening height in mm (200–6000).");
    });
    ui.horizontal(|ui| {
        ui.label("Sill:");
        ui.add(
            egui::DragValue::new(&mut sill)
                .speed(50.0)
                .range(0.0..=20000.0)
                .suffix(" mm"),
        )
        .on_hover_text("Sill height above floor level in mm (0–20000).");
    });
    ui.horizontal(|ui| {
        ui.label("Position along wall:");
        ui.add(
            egui::DragValue::new(&mut offset)
                .speed(10.0)
                .range(0.0..=wall.length())
                .suffix(" mm"),
        )
        .on_hover_text("Distance from the wall start to the opening centre in mm (0–wall length).");
    });
    ui.checkbox(&mut flipped, "Flip door hinge")
        .on_hover_text("Mirror the door swing to the other side of the wall.");

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

fn dimension_fields(
    ui: &mut egui::Ui,
    id: DimensionId,
    dimension: &lintel_core::dimension::Dimension,
) -> Option<Command> {
    let mut command: Option<Command> = None;

    // Kind selector.
    let mut kind = dimension.kind;
    ui.horizontal(|ui| {
        ui.label("Kind:");
        ui.radio_value(
            &mut kind,
            lintel_core::dimension::DimensionKind::LinearHorizontal,
            lintel_core::dimension::DimensionKind::LinearHorizontal.label(),
        )
        .on_hover_text("A horizontal dimension line.");
        ui.radio_value(
            &mut kind,
            lintel_core::dimension::DimensionKind::LinearVertical,
            lintel_core::dimension::DimensionKind::LinearVertical.label(),
        )
        .on_hover_text("A vertical dimension line.");
        ui.radio_value(
            &mut kind,
            lintel_core::dimension::DimensionKind::Aligned,
            lintel_core::dimension::DimensionKind::Aligned.label(),
        )
        .on_hover_text("A dimension line aligned with the measured segment.");
    });
    if kind != dimension.kind {
        return Some(Command::SetDimensionKind { id, kind });
    }

    let mut offset = dimension.offset;
    ui.horizontal(|ui| {
        ui.label("Offset:");
        ui.add(
            egui::DragValue::new(&mut offset)
                .speed(10.0)
                .range(1.0..=100000.0)
                .suffix(" mm"),
        )
        .on_hover_text("Extension distance from the measured points to the dimension line in mm (1–100000). Press Apply to commit.");
        if ui
            .button("Apply")
            .on_hover_text("Apply the new dimension offset.")
            .clicked()
            && (offset - dimension.offset).abs() > f64::EPSILON
        {
            command = Some(Command::SetDimensionOffset { id, offset });
        }
    });

    let mut extension_lines = dimension.extension_lines;
    if ui
        .checkbox(&mut extension_lines, "Extension lines")
        .on_hover_text("Draw witness lines from the measured points to the dimension line.")
        .changed()
        && extension_lines != dimension.extension_lines
    {
        return Some(Command::SetDimensionExtensionLines {
            id,
            extension_lines,
        });
    }

    command
}
