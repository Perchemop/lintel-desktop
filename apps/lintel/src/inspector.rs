//! Right-hand inspector: properties of the selected element plus global
//! view controls. All edits flow through `lintel_core::Command` values
//! returned here and applied by the shell.

use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    geom::Point2,
    ids::{DimensionId, LabelId, OpeningId, WallId},
    join::Hit,
    opening::OpeningKind,
    snap::SnapSettings,
    units::UnitSystem,
};

/// Transient drafts the Inspector keeps between frames (edits that are only
/// committed when the user presses Apply / Add).
#[derive(Debug, Default)]
pub struct InspectorDraft {
    /// Name being typed for a new storey.
    pub new_level_name: String,
    /// Elevation being typed for a new storey, in millimetres.
    pub new_level_elevation: f64,
}

pub fn inspector(
    ui: &mut egui::Ui,
    doc: &Document,
    selection: &Option<Hit>,
    snap: &mut SnapSettings,
    draft: &mut InspectorDraft,
    measurements_active: bool,
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
            Some(Hit::Label(id)) => {
                let label = {
                    let level = doc.project().active_level().expect("level");
                    level.label(*id).expect("label").clone()
                };
                let truncated = if label.text.chars().count() > 20 {
                    format!("{}…", label.text.chars().take(20).collect::<String>())
                } else {
                    label.text.clone()
                };
                ui.label(format!("Label {} — {}", id.value(), truncated));
                ui.separator();
                label_fields(ui, *id, &label)
            }
            None => {
                ui.label("Select a wall or opening to edit it.");
                None
            }
        })
        .body_returned
        .flatten();

    ui.separator();

    let project_cmd = egui::CollapsingHeader::new("Project")
        .default_open(false)
        .show(ui, |ui| {
            let mut command: Option<Command> = None;
            let project = doc.project();

            let mut name = project.name.clone();
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut name)
                    .on_hover_text("The project name stored in the saved file.");
                if ui
                    .button("Apply")
                    .on_hover_text("Apply the new project name.")
                    .clicked()
                    && !name.trim().is_empty()
                    && name.trim() != project.name.as_str()
                {
                    command = Some(Command::RenameProject {
                        name: name.trim().to_string(),
                    });
                }
            });

            let mut units = project.units;
            ui.horizontal(|ui| {
                ui.label("Units:");
                ui.radio_value(&mut units, UnitSystem::Metric, "Metric (mm)")
                    .on_hover_text("Show lengths in millimetres and metres.");
                ui.radio_value(&mut units, UnitSystem::Imperial, "Imperial (ft-in)")
                    .on_hover_text("Show lengths in feet and inches.");
            });
            if units != project.units {
                command = Some(Command::SetUnits { units });
            }

            command
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
            let project = doc.project();
            let active = project.active_level;
            // The core refuses to delete the last storey, so the button is
            // disabled rather than failing with a status message.
            let only_one = project.levels.len() <= 1;
            let mut command: Option<Command> = None;

            for level in &project.levels {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(level.id == active, &level.name)
                        .on_hover_text("Make this storey active.")
                        .clicked()
                        && level.id != active
                    {
                        command = Some(Command::SetActiveLevel { id: level.id });
                    }
                    ui.label(format!("{} mm", level.elevation));
                    if ui
                        .add_enabled(!only_one, egui::Button::new("Delete"))
                        .on_hover_text("Delete this storey (the last storey cannot be deleted).")
                        .clicked()
                    {
                        command = Some(Command::DeleteLevel { id: level.id });
                    }
                });
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut draft.new_level_name)
                        .desired_width(90.0)
                        .hint_text("New storey"),
                )
                .on_hover_text("Name for the new storey.");
                ui.add(
                    egui::DragValue::new(&mut draft.new_level_elevation)
                        .speed(100.0)
                        .range(-100_000.0..=100_000.0)
                        .suffix(" mm"),
                )
                .on_hover_text("Elevation of the new storey above the project datum.");
                if ui
                    .button("Add")
                    .on_hover_text("Add the new storey and make it active.")
                    .clicked()
                    && !draft.new_level_name.trim().is_empty()
                {
                    command = Some(Command::AddLevel {
                        name: draft.new_level_name.trim().to_string(),
                        elevation: draft.new_level_elevation,
                    });
                }
            });

            command
        })
        .body_returned
        .flatten();

    ui.separator();

    let measurements_cmd = if measurements_active {
        egui::CollapsingHeader::new("Measurements")
            .default_open(true)
            .show(ui, |ui| {
                let project = doc.project();
                let measurements = lintel_core::measurements::Measurements::new(project);

                // Cache rooms for this frame to avoid repeated detection
                let level_rooms = project.active_level().map(|lvl| lintel_core::room::detect_rooms(&lvl.walls));

                // Selected room (if the current selection sits inside one).
                if let Some(Hit::Wall(wall_id)) = selection {
                    let level = project.active_level().expect("level");
                    if let Some(wall) = level.wall(*wall_id) {
                        let mid = lintel_core::geom::Point2::new(
                            (wall.start.x + wall.end.x) * 0.5,
                            (wall.start.y + wall.end.y) * 0.5,
                        );
                        // Midpoint lies on the wall centreline; nudge it inward a little so containment works.
                        let probe = [mid, lintel_core::geom::Point2::new(mid.x + 10.0, mid.y), lintel_core::geom::Point2::new(mid.x - 10.0, mid.y), lintel_core::geom::Point2::new(mid.x, mid.y + 10.0), lintel_core::geom::Point2::new(mid.x, mid.y - 10.0)];
                        if let Some(rooms) = &level_rooms {
                            if let Some(room) = rooms.iter().find(|r| probe.iter().any(|p| r.contains(*p))) {
                                ui.label(format!(
                                    "Selected room — area {}  perimeter {}",
                                    room.area_label(),
                                    project.units.format(room.perimeter_mm)
                                ));
                                ui.separator();
                            }
                        }
                    }
                }

                if let Some(level) = project.active_level() {
                    let totals = measurements.level_totals(level.id);
                    ui.label(format!("Level: {}", level.name));
                    ui.label(format!(
                        "Wall length total: {}",
                        project.units.format(totals.wall_length_mm)
                    ));
                    ui.label(format!(
                        "Opening length total: {}",
                        project.units.format(totals.opening_length_mm)
                    ));
                    ui.label(format!("Rooms: {}", totals.room_count));
                    ui.label(format!(
                        "Room area total: {:.2} m²",
                        totals.total_room_area_mm2 / 1_000_000.0
                    ));
                    ui.label(format!(
                        "Room perimeter total: {}",
                        project.units.format(totals.total_room_perimeter_mm)
                    ));
                    ui.separator();
                    if let Some(rooms) = &level_rooms {
                        if rooms.is_empty() {
                            ui.label("No enclosed rooms.");
                        } else {
                            for (index, room) in rooms.iter().enumerate() {
                                ui.label(format!(
                                    "Room {} — area {}  perimeter {}",
                                    index + 1,
                                    room.area_label(),
                                    project.units.format(room.perimeter_mm)
                                ));
                            }
                        }
                    }
                }

                ui.separator();
                let project_totals = measurements.project_totals();
                ui.label("Project totals:");
                ui.label(format!(
                    "Wall length: {}",
                    project.units.format(project_totals.wall_length_mm)
                ));
                ui.label(format!(
                    "Opening length: {}",
                    project.units.format(project_totals.opening_length_mm)
                ));
                ui.label(format!("Rooms: {}", project_totals.room_count));
                ui.label(format!(
                    "Room area: {:.2} m²",
                    project_totals.total_room_area_mm2 / 1_000_000.0
                ));
                ui.label(format!(
                    "Room perimeter: {}",
                    project.units.format(project_totals.total_room_perimeter_mm)
                ));

                None::<Command>
            })
            .body_returned
            .flatten()
    } else {
        None
    };

    ui.separator();

    let rooms_cmd = egui::CollapsingHeader::new("Rooms")
        .default_open(false)
        .show(ui, |ui| {
            let level = doc.project().active_level().expect("level");
            let rooms = lintel_core::room::detect_rooms(&level.walls);
            if rooms.is_empty() {
                ui.label("No enclosed areas yet.");
            } else {
                for (index, room) in rooms.iter().enumerate() {
                    ui.label(format!("{} — {}", index + 1, room.area_label()));
                }
            }
            None::<Command>
        })
        .body_returned
        .flatten();

    ui.separator();

    let schedule_cmd = egui::CollapsingHeader::new("Schedule")
        .default_open(false)
        .show(ui, |ui| {
            let level = doc.project().active_level().expect("level");
            let rows = lintel_core::schedule::opening_schedule(level);
            if rows.is_empty() {
                ui.label("No doors or windows.");
            } else {
                for row in &rows {
                    ui.label(format!(
                        "{}  {}  x{}",
                        row.mark,
                        row.size_label(),
                        row.count
                    ));
                }
            }
            None::<Command>
        })
        .body_returned
        .flatten();

    element_cmd
        .or(project_cmd)
        .or(storeys_cmd)
        .or(view_cmd)
        .or(rooms_cmd)
        .or(schedule_cmd)
        .or(measurements_cmd)
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

fn label_fields(
    ui: &mut egui::Ui,
    id: LabelId,
    label: &lintel_core::label::PlanLabel,
) -> Option<Command> {
    let mut command: Option<Command> = None;

    let mut text = label.text.clone();
    ui.horizontal(|ui| {
        ui.label("Text:");
        ui.text_edit_singleline(&mut text);
        if ui.button("Apply").clicked() && !text.trim().is_empty() && text.trim() != label.text {
            command = Some(Command::SetLabelText {
                id,
                text: text.trim().to_string(),
            });
        }
    });

    let mut x = label.anchor.x;
    let mut y = label.anchor.y;
    ui.horizontal(|ui| {
        ui.label("X:");
        ui.add(egui::DragValue::new(&mut x).speed(10.0).suffix(" mm"));
        ui.label("Y:");
        ui.add(egui::DragValue::new(&mut y).speed(10.0).suffix(" mm"));
        if ui.button("Apply").clicked()
            && ((x - label.anchor.x).abs() > f64::EPSILON
                || (y - label.anchor.y).abs() > f64::EPSILON)
        {
            command = Some(Command::MoveLabel {
                id,
                anchor: Point2::new(x, y),
            });
        }
    });

    if ui.button("Delete").clicked() {
        command = Some(Command::DeleteLabel { id });
    }

    command
}
