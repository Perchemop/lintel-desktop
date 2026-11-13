//! Every edit the user can make is a `Command`.
//!
//! The UI never mutates the model directly: it sends a command and renders the
//! result (addendum K.3). That is what makes the workflow tests in
//! `tests/workflows.rs` exercise the *same* path the app uses.

use serde::{Deserialize, Serialize};

use crate::constraint::Constraint;
use crate::construction::{ConstructionEntity, ConstructionKind};
use crate::dimension::{Dimension, DimensionAnchor, DimensionKind};
use crate::error::CoreError;
use crate::geom::{Bounds, Point2};
use crate::ids::{ConstructionId, DimensionId, LabelId, LayerId, LevelId, OpeningId, WallId};
use crate::label::{MAX_LABEL_HEIGHT_MM, MIN_LABEL_HEIGHT_MM, PlanLabel};
use crate::layer::{
    EntitySelector, Layer, LayerProperty, Linetype, MAX_LAYER_NAME_CHARS, MAX_LINEWEIGHT_MM,
    MIN_LINEWEIGHT_MM, Rgb,
};
use crate::level::Level;
use crate::opening::{
    MAX_OPENING_HEIGHT, MAX_OPENING_WIDTH, MIN_OPENING_HEIGHT, MIN_OPENING_WIDTH, Opening,
    OpeningKind,
};
use crate::project::Project;
use crate::title_block::TitleBlock;
use crate::units::UnitSystem;
use crate::viewport::{MAX_CAPTION_CHARS, Viewport};
use crate::wall::{
    MAX_WALL_HEIGHT, MAX_WALL_THICKNESS, MIN_WALL_HEIGHT, MIN_WALL_LENGTH, MIN_WALL_THICKNESS, Wall,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    AddWall {
        start: Point2,
        end: Point2,
        thickness: f64,
        height: f64,
    },
    MoveWall {
        id: WallId,
        start: Point2,
        end: Point2,
    },
    SetWallThickness {
        id: WallId,
        thickness: f64,
    },
    SetWallHeight {
        id: WallId,
        height: f64,
    },
    DeleteWall {
        id: WallId,
    },
    PlaceOpening {
        kind: OpeningKind,
        host: WallId,
        offset: f64,
    },
    MoveOpening {
        id: OpeningId,
        offset: f64,
    },
    SetOpeningKind {
        id: OpeningId,
        kind: OpeningKind,
    },
    SetOpeningSize {
        id: OpeningId,
        width: f64,
        height: f64,
        sill: f64,
    },
    FlipOpening {
        id: OpeningId,
    },
    DeleteOpening {
        id: OpeningId,
    },
    PlaceDimension {
        kind: DimensionKind,
        start: Point2,
        end: Point2,
        offset: f64,
    },
    MoveDimension {
        id: DimensionId,
        start: Point2,
        end: Point2,
    },
    SetDimensionOffset {
        id: DimensionId,
        offset: f64,
    },
    SetDimensionKind {
        id: DimensionId,
        kind: DimensionKind,
    },
    SetDimensionExtensionLines {
        id: DimensionId,
        extension_lines: bool,
    },
    SetDimensionStartAnchor {
        id: DimensionId,
        anchor: Option<DimensionAnchor>,
    },
    SetDimensionEndAnchor {
        id: DimensionId,
        anchor: Option<DimensionAnchor>,
    },
    SetDimensionWitnessGap {
        id: DimensionId,
        gap: f64,
    },
    SetDimensionWitnessOverrun {
        id: DimensionId,
        overrun: f64,
    },
    DeleteDimension {
        id: DimensionId,
    },
    PlaceLabel {
        anchor: Point2,
        text: String,
    },
    SetLabelText {
        id: LabelId,
        text: String,
    },
    MoveLabel {
        id: LabelId,
        anchor: Point2,
    },
    DeleteLabel {
        id: LabelId,
    },
    CreateLayer {
        name: String,
        color: Option<Rgb>,
        linetype: Linetype,
        lineweight_mm: f64,
    },
    DeleteLayer {
        layer_id: LayerId,
    },
    SetLayerProperty {
        layer_id: LayerId,
        property: LayerProperty,
    },
    AssignEntityToLayer {
        entity: EntitySelector,
        layer_id: LayerId,
    },
    CreateConstruction {
        kind: ConstructionKind,
    },
    DeleteConstruction {
        id: ConstructionId,
    },
    SetConstructionVisible {
        id: ConstructionId,
        visible: bool,
    },
    AddLevel {
        name: String,
        elevation: f64,
    },
    DeleteLevel {
        id: LevelId,
    },
    SetActiveLevel {
        id: LevelId,
    },
    SetUnits {
        units: UnitSystem,
    },
    RenameProject {
        name: String,
    },
    AddConstraint {
        constraint: Constraint,
    },
    RemoveConstraint {
        index: usize,
    },
    SetTitleBlock {
        title_block: TitleBlock,
    },
    AddViewport {
        level: LevelId,
        region: Bounds,
        scale_denominator: u32,
        caption: String,
    },
    RemoveViewport {
        index: usize,
    },
}

/// What a successful command created, so the UI can select it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Applied {
    pub wall: Option<WallId>,
    pub opening: Option<OpeningId>,
    pub dimension: Option<DimensionId>,
    pub label: Option<LabelId>,
    pub level: Option<LevelId>,
    pub layer: Option<LayerId>,
    pub construction: Option<ConstructionId>,
}

pub(crate) fn apply_command(project: &mut Project, command: Command) -> Result<Applied, CoreError> {
    match command {
        Command::AddWall {
            start,
            end,
            thickness,
            height,
        } => add_wall(project, start, end, thickness, height),
        Command::MoveWall { id, start, end } => move_wall(project, id, start, end),
        Command::SetWallThickness { id, thickness } => {
            validate_thickness(thickness)?;
            wall_mut(project, id)?.thickness = thickness;
            Ok(Applied::default())
        }
        Command::SetWallHeight { id, height } => {
            validate_height(height)?;
            wall_mut(project, id)?.height = height;
            Ok(Applied::default())
        }
        Command::DeleteWall { id } => delete_wall(project, id),
        Command::PlaceOpening { kind, host, offset } => place_opening(project, kind, host, offset),
        Command::MoveOpening { id, offset } => move_opening(project, id, offset),
        Command::SetOpeningSize {
            id,
            width,
            height,
            sill,
        } => set_opening_size(project, id, width, height, sill),
        Command::SetOpeningKind { id, kind } => {
            let level = active_level_mut(project)?;
            let opening = level
                .opening_mut(id)
                .ok_or(CoreError::OpeningNotFound(id))?;
            opening.kind = kind;
            Ok(Applied::default())
        }
        Command::FlipOpening { id } => {
            let level = active_level_mut(project)?;
            let opening = level
                .opening_mut(id)
                .ok_or(CoreError::OpeningNotFound(id))?;
            opening.flipped = !opening.flipped;
            Ok(Applied::default())
        }
        Command::DeleteOpening { id } => {
            let level = active_level_mut(project)?;
            let before = level.openings.len();
            level.openings.retain(|o| o.id != id);
            if level.openings.len() == before {
                return Err(CoreError::OpeningNotFound(id));
            }
            Ok(Applied::default())
        }
        Command::PlaceDimension {
            kind,
            start,
            end,
            offset,
        } => place_dimension(project, kind, start, end, offset),
        Command::MoveDimension { id, start, end } => move_dimension(project, id, start, end),
        Command::SetDimensionOffset { id, offset } => {
            validate_dimension_offset(offset)?;
            let level = active_level_mut(project)?;
            let dimension = level
                .dimension_mut(id)
                .ok_or(CoreError::DimensionNotFound(id))?;
            dimension.offset = offset;
            Ok(Applied::default())
        }
        Command::SetDimensionKind { id, kind } => {
            let level = active_level_mut(project)?;
            let dimension = level
                .dimension_mut(id)
                .ok_or(CoreError::DimensionNotFound(id))?;
            let mut candidate = dimension.clone();
            candidate.kind = kind;
            validate_dimension(&candidate)?;
            dimension.kind = kind;
            Ok(Applied::default())
        }
        Command::SetDimensionExtensionLines {
            id,
            extension_lines,
        } => {
            let level = active_level_mut(project)?;
            let dimension = level
                .dimension_mut(id)
                .ok_or(CoreError::DimensionNotFound(id))?;
            dimension.extension_lines = extension_lines;
            Ok(Applied::default())
        }
        Command::SetDimensionStartAnchor { id, anchor } => {
            validate_anchor(anchor.as_ref())?;
            dimension_mut(project, id)?.start_anchor = anchor;
            Ok(Applied::default())
        }
        Command::SetDimensionEndAnchor { id, anchor } => {
            validate_anchor(anchor.as_ref())?;
            dimension_mut(project, id)?.end_anchor = anchor;
            Ok(Applied::default())
        }
        Command::SetDimensionWitnessGap { id, gap } => {
            validate_witness_margin(gap, "gap")?;
            dimension_mut(project, id)?.witness_gap = gap;
            Ok(Applied::default())
        }
        Command::SetDimensionWitnessOverrun { id, overrun } => {
            validate_witness_margin(overrun, "overrun")?;
            dimension_mut(project, id)?.witness_overrun = overrun;
            Ok(Applied::default())
        }
        Command::DeleteDimension { id } => {
            let level = active_level_mut(project)?;
            let before = level.dimensions.len();
            level.dimensions.retain(|d| d.id != id);
            if level.dimensions.len() == before {
                return Err(CoreError::DimensionNotFound(id));
            }
            Ok(Applied::default())
        }
        Command::PlaceLabel { anchor, text } => place_label(project, anchor, text),
        Command::SetLabelText { id, text } => {
            let level = active_level(project)?;
            let existing = level.label(id).ok_or(CoreError::LabelNotFound(id))?;
            let mut candidate = existing.clone();
            candidate.text = text.trim().to_string();
            validate_label(&candidate)?;
            let level = active_level_mut(project)?;
            let label = level.label_mut(id).ok_or(CoreError::LabelNotFound(id))?;
            label.text = candidate.text;
            Ok(Applied::default())
        }
        Command::MoveLabel { id, anchor } => {
            let level = active_level(project)?;
            let existing = level.label(id).ok_or(CoreError::LabelNotFound(id))?;
            let mut candidate = existing.clone();
            candidate.anchor = anchor;
            validate_label(&candidate)?;
            let level = active_level_mut(project)?;
            let label = level.label_mut(id).ok_or(CoreError::LabelNotFound(id))?;
            label.anchor = anchor;
            Ok(Applied::default())
        }
        Command::DeleteLabel { id } => {
            let level = active_level_mut(project)?;
            let before = level.labels.len();
            level.labels.retain(|l| l.id != id);
            if level.labels.len() == before {
                return Err(CoreError::LabelNotFound(id));
            }
            Ok(Applied::default())
        }
        Command::CreateLayer {
            name,
            color,
            linetype,
            lineweight_mm,
        } => create_layer(project, name, color, linetype, lineweight_mm),
        Command::DeleteLayer { layer_id } => delete_layer(project, layer_id),
        Command::SetLayerProperty { layer_id, property } => {
            set_layer_property(project, layer_id, property)
        }
        Command::AssignEntityToLayer { entity, layer_id } => {
            assign_entity_to_layer(project, entity, layer_id)
        }
        Command::CreateConstruction { kind } => create_construction(project, kind),
        Command::DeleteConstruction { id } => delete_construction(project, id),
        Command::SetConstructionVisible { id, visible } => {
            let level = active_level_mut(project)?;
            let entity = level
                .construction_mut(id)
                .ok_or(CoreError::ConstructionNotFound(id))?;
            entity.visible = visible;
            Ok(Applied::default())
        }
        Command::AddLevel { name, elevation } => add_level(project, name, elevation),
        Command::DeleteLevel { id } => delete_level(project, id),
        Command::SetActiveLevel { id } => {
            if project.level(id).is_none() {
                return Err(CoreError::LevelNotFound(id));
            }
            project.active_level = id;
            Ok(Applied::default())
        }
        Command::SetUnits { units } => {
            project.units = units;
            Ok(Applied::default())
        }
        Command::RenameProject { name } => {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err(CoreError::invalid("a project needs a name"));
            }
            project.name = trimmed.to_string();
            Ok(Applied::default())
        }
        Command::AddConstraint { constraint } => add_constraint(project, constraint),
        Command::RemoveConstraint { index } => remove_constraint(project, index),
        Command::SetTitleBlock { title_block } => set_title_block(project, title_block),
        Command::AddViewport {
            level,
            region,
            scale_denominator,
            caption,
        } => add_viewport(project, level, region, scale_denominator, caption),
        Command::RemoveViewport { index } => remove_viewport(project, index),
    }
}

pub(crate) fn active_level_mut(project: &mut Project) -> Result<&mut Level, CoreError> {
    let active = project.active_level;
    project
        .level_mut(active)
        .ok_or(CoreError::LevelNotFound(active))
}

fn active_level(project: &Project) -> Result<&Level, CoreError> {
    let active = project.active_level;
    project
        .level(active)
        .ok_or(CoreError::LevelNotFound(active))
}

fn wall_mut(project: &mut Project, id: WallId) -> Result<&mut Wall, CoreError> {
    active_level_mut(project)?
        .wall_mut(id)
        .ok_or(CoreError::WallNotFound(id))
}

fn dimension_mut(project: &mut Project, id: DimensionId) -> Result<&mut Dimension, CoreError> {
    active_level_mut(project)?
        .dimension_mut(id)
        .ok_or(CoreError::DimensionNotFound(id))
}

fn validate_thickness(thickness: f64) -> Result<(), CoreError> {
    if thickness.is_finite() && (MIN_WALL_THICKNESS..=MAX_WALL_THICKNESS).contains(&thickness) {
        Ok(())
    } else {
        Err(CoreError::invalid(format!(
            "a wall thickness must be between {MIN_WALL_THICKNESS} mm and {MAX_WALL_THICKNESS} mm"
        )))
    }
}

fn validate_height(height: f64) -> Result<(), CoreError> {
    if height.is_finite() && (MIN_WALL_HEIGHT..=MAX_WALL_HEIGHT).contains(&height) {
        Ok(())
    } else {
        Err(CoreError::invalid(format!(
            "a wall height must be between {MIN_WALL_HEIGHT} mm and {MAX_WALL_HEIGHT} mm"
        )))
    }
}

fn add_wall(
    project: &mut Project,
    start: Point2,
    end: Point2,
    thickness: f64,
    height: f64,
) -> Result<Applied, CoreError> {
    if !start.is_finite() || !end.is_finite() {
        return Err(CoreError::invalid("a wall needs two finite points"));
    }
    if start.distance(end) < MIN_WALL_LENGTH {
        return Err(CoreError::invalid(format!(
            "a wall needs a length of at least {MIN_WALL_LENGTH} mm"
        )));
    }
    validate_thickness(thickness)?;
    validate_height(height)?;
    let id = WallId(project.allocate_id());
    let wall = Wall::new(id, start, end, thickness, height);
    active_level_mut(project)?.walls.push(wall);
    Ok(Applied {
        wall: Some(id),
        ..Applied::default()
    })
}

fn move_wall(
    project: &mut Project,
    id: WallId,
    start: Point2,
    end: Point2,
) -> Result<Applied, CoreError> {
    if !start.is_finite() || !end.is_finite() {
        return Err(CoreError::invalid("a wall needs two finite points"));
    }
    if start.distance(end) < MIN_WALL_LENGTH {
        return Err(CoreError::invalid(format!(
            "a wall needs a length of at least {MIN_WALL_LENGTH} mm"
        )));
    }
    let level = active_level(project)?;
    let existing = level.wall(id).ok_or(CoreError::WallNotFound(id))?;
    let candidate = Wall::new(id, start, end, existing.thickness, existing.height);
    for opening in level.openings_on(id) {
        if !opening.fits_on(&candidate) {
            return Err(CoreError::invalid(format!(
                "{} would no longer fit on {}",
                opening.id, id
            )));
        }
    }
    let level = active_level_mut(project)?;
    let wall = level.wall_mut(id).ok_or(CoreError::WallNotFound(id))?;
    wall.start = start;
    wall.end = end;
    Ok(Applied::default())
}

fn delete_wall(project: &mut Project, id: WallId) -> Result<Applied, CoreError> {
    let level = active_level_mut(project)?;
    let before = level.walls.len();
    level.walls.retain(|w| w.id != id);
    if level.walls.len() == before {
        return Err(CoreError::WallNotFound(id));
    }
    // A hosted opening cannot outlive its wall.
    level.openings.retain(|o| o.host != id);
    Ok(Applied::default())
}

fn validate_dimension_offset(offset: f64) -> Result<(), CoreError> {
    if offset.is_finite() && offset > 0.0 {
        Ok(())
    } else {
        Err(CoreError::invalid(
            "a dimension offset must be a distance greater than zero",
        ))
    }
}

fn validate_anchor(anchor: Option<&DimensionAnchor>) -> Result<(), CoreError> {
    if anchor.is_some_and(|anchor| !anchor.is_valid()) {
        return Err(CoreError::invalid(
            "a dimension anchor must sit between the ends of its wall",
        ));
    }
    Ok(())
}

fn validate_witness_margin(margin: f64, what: &str) -> Result<(), CoreError> {
    if margin.is_finite() && margin >= 0.0 {
        Ok(())
    } else {
        Err(CoreError::invalid(format!(
            "a witness-line {what} must be a distance of zero or more"
        )))
    }
}

fn validate_dimension(dimension: &Dimension) -> Result<(), CoreError> {
    if !dimension.start.is_finite() || !dimension.end.is_finite() {
        return Err(CoreError::invalid("a dimension needs two finite points"));
    }
    let length = dimension.length();
    if !length.is_finite() || length <= 0.0 {
        return Err(CoreError::invalid(
            "a dimension needs two different measured points",
        ));
    }
    validate_dimension_offset(dimension.offset)
}

fn place_dimension(
    project: &mut Project,
    kind: DimensionKind,
    start: Point2,
    end: Point2,
    offset: f64,
) -> Result<Applied, CoreError> {
    // Validate before allocating so a rejected command does not burn an id.
    let candidate = Dimension::new(DimensionId(0), kind, start, end, offset);
    validate_dimension(&candidate)?;
    let id = DimensionId(project.allocate_id());
    let dimension = Dimension::new(id, kind, start, end, offset);
    active_level_mut(project)?.dimensions.push(dimension);
    Ok(Applied {
        dimension: Some(id),
        ..Applied::default()
    })
}

fn move_dimension(
    project: &mut Project,
    id: DimensionId,
    start: Point2,
    end: Point2,
) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let existing = level
        .dimension(id)
        .ok_or(CoreError::DimensionNotFound(id))?;
    let mut candidate = existing.clone();
    candidate.start = start;
    candidate.end = end;
    validate_dimension(&candidate)?;
    let level = active_level_mut(project)?;
    let dimension = level
        .dimension_mut(id)
        .ok_or(CoreError::DimensionNotFound(id))?;
    dimension.start = start;
    dimension.end = end;
    Ok(Applied::default())
}

fn validate_label(label: &PlanLabel) -> Result<(), CoreError> {
    if !label.anchor.is_finite() {
        return Err(CoreError::invalid("a label needs a finite position"));
    }
    if label.text.trim().is_empty() {
        return Err(CoreError::invalid("a label needs some text"));
    }
    if !label.height_mm.is_finite()
        || !(MIN_LABEL_HEIGHT_MM..=MAX_LABEL_HEIGHT_MM).contains(&label.height_mm)
    {
        return Err(CoreError::invalid(format!(
            "a label height must be between {MIN_LABEL_HEIGHT_MM} mm and {MAX_LABEL_HEIGHT_MM} mm"
        )));
    }
    Ok(())
}

fn place_label(project: &mut Project, anchor: Point2, text: String) -> Result<Applied, CoreError> {
    // Validate before allocating so a rejected command does not burn an id.
    let text = text.trim().to_string();
    let candidate = PlanLabel::new(LabelId(0), anchor, text.clone());
    validate_label(&candidate)?;
    let id = LabelId(project.allocate_id());
    let label = PlanLabel::new(id, anchor, text);
    active_level_mut(project)?.labels.push(label);
    Ok(Applied {
        label: Some(id),
        ..Applied::default()
    })
}

fn place_opening(
    project: &mut Project,
    kind: OpeningKind,
    host: WallId,
    offset: f64,
) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let wall = level.wall(host).ok_or(CoreError::WallNotFound(host))?;
    let id = OpeningId(project.next_id);
    let candidate = Opening::new(id, kind, host, offset);
    if !candidate.fits_on(wall) {
        return Err(CoreError::invalid(format!(
            "a {} mm {} does not fit on {}",
            candidate.width.round(),
            kind.label().to_ascii_lowercase(),
            host
        )));
    }
    if let Some(clash) = overlap_on(
        level,
        host,
        None,
        candidate.start_offset(),
        candidate.end_offset(),
    ) {
        return Err(CoreError::invalid(format!(
            "the new {} would overlap {}",
            kind.label().to_ascii_lowercase(),
            clash
        )));
    }
    let id = OpeningId(project.allocate_id());
    let opening = Opening::new(id, kind, host, offset);
    active_level_mut(project)?.openings.push(opening);
    Ok(Applied {
        opening: Some(id),
        ..Applied::default()
    })
}

fn move_opening(project: &mut Project, id: OpeningId, offset: f64) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let existing = level.opening(id).ok_or(CoreError::OpeningNotFound(id))?;
    let host = level
        .wall(existing.host)
        .ok_or(CoreError::WallNotFound(existing.host))?;
    let mut candidate = existing.clone();
    candidate.offset = offset;
    if !candidate.fits_on(host) {
        return Err(CoreError::invalid(format!(
            "{} would no longer fit on {}",
            id, existing.host
        )));
    }
    if let Some(clash) = overlap_on(
        level,
        existing.host,
        Some(id),
        candidate.start_offset(),
        candidate.end_offset(),
    ) {
        return Err(CoreError::invalid(format!("{id} would overlap {clash}")));
    }
    let level = active_level_mut(project)?;
    level
        .opening_mut(id)
        .ok_or(CoreError::OpeningNotFound(id))?
        .offset = offset;
    Ok(Applied::default())
}

fn set_opening_size(
    project: &mut Project,
    id: OpeningId,
    width: f64,
    height: f64,
    sill: f64,
) -> Result<Applied, CoreError> {
    if !width.is_finite() || !(MIN_OPENING_WIDTH..=MAX_OPENING_WIDTH).contains(&width) {
        return Err(CoreError::invalid(format!(
            "an opening width must be between {MIN_OPENING_WIDTH} mm and {MAX_OPENING_WIDTH} mm"
        )));
    }
    if !height.is_finite() || !(MIN_OPENING_HEIGHT..=MAX_OPENING_HEIGHT).contains(&height) {
        return Err(CoreError::invalid(format!(
            "an opening height must be between {MIN_OPENING_HEIGHT} mm and {MAX_OPENING_HEIGHT} mm"
        )));
    }
    if !sill.is_finite() || sill < 0.0 {
        return Err(CoreError::invalid("a sill cannot be below the floor"));
    }

    let level = active_level(project)?;
    let existing = level.opening(id).ok_or(CoreError::OpeningNotFound(id))?;
    let host = level
        .wall(existing.host)
        .ok_or(CoreError::WallNotFound(existing.host))?;
    let mut candidate = existing.clone();
    candidate.width = width;
    candidate.height = height;
    candidate.sill = sill;
    if !candidate.fits_on(host) {
        return Err(CoreError::invalid(format!(
            "{id} would no longer fit on {} at {width} mm wide",
            existing.host
        )));
    }
    if let Some(clash) = overlap_on(
        level,
        existing.host,
        Some(id),
        candidate.start_offset(),
        candidate.end_offset(),
    ) {
        return Err(CoreError::invalid(format!("{id} would overlap {clash}")));
    }
    let level = active_level_mut(project)?;
    let opening = level
        .opening_mut(id)
        .ok_or(CoreError::OpeningNotFound(id))?;
    opening.width = width;
    opening.height = height;
    opening.sill = sill;
    Ok(Applied::default())
}

fn overlap_on(
    level: &Level,
    host: WallId,
    exclude: Option<OpeningId>,
    start: f64,
    end: f64,
) -> Option<OpeningId> {
    level
        .openings_on(host)
        .filter(|o| Some(o.id) != exclude)
        .find(|o| start < o.end_offset() && end > o.start_offset())
        .map(|o| o.id)
}

fn validate_layer_name(name: &str) -> Result<(), CoreError> {
    if name.is_empty() {
        return Err(CoreError::invalid("a layer needs a name"));
    }
    if name.chars().count() > MAX_LAYER_NAME_CHARS {
        return Err(CoreError::invalid(format!(
            "a layer name must be at most {MAX_LAYER_NAME_CHARS} characters"
        )));
    }
    Ok(())
}

fn validate_lineweight(weight: f64) -> Result<(), CoreError> {
    if weight.is_finite() && (MIN_LINEWEIGHT_MM..=MAX_LINEWEIGHT_MM).contains(&weight) {
        Ok(())
    } else {
        Err(CoreError::invalid(format!(
            "a lineweight must be between {MIN_LINEWEIGHT_MM} mm and {MAX_LINEWEIGHT_MM} mm"
        )))
    }
}

fn validate_linetype(linetype: Linetype) -> Result<(), CoreError> {
    if linetype.is_valid() {
        Ok(())
    } else {
        Err(CoreError::invalid(
            "a linetype needs a positive dash and a non-negative gap",
        ))
    }
}

fn create_layer(
    project: &mut Project,
    name: String,
    color: Option<Rgb>,
    linetype: Linetype,
    lineweight_mm: f64,
) -> Result<Applied, CoreError> {
    let name = name.trim().to_string();
    validate_layer_name(&name)?;
    validate_lineweight(lineweight_mm)?;
    validate_linetype(linetype)?;

    let level = active_level(project)?;
    if level.layer_by_name(&name).is_some() {
        return Err(CoreError::invalid(format!(
            "a layer named \"{name}\" already exists"
        )));
    }

    let id = LayerId(project.allocate_id());
    let mut layer = Layer::new(id, name);
    layer.color = color;
    layer.linetype = linetype;
    layer.lineweight_mm = lineweight_mm;
    active_level_mut(project)?.layers.push(layer);
    Ok(Applied {
        layer: Some(id),
        ..Applied::default()
    })
}

fn delete_layer(project: &mut Project, layer_id: LayerId) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let layer = level
        .layer(layer_id)
        .ok_or(CoreError::LayerNotFound(layer_id))?;
    if layer.is_default() {
        return Err(CoreError::DefaultLayer);
    }
    // Moving the contents automatically would silently change what is drawn;
    // the user is asked to move them first instead.
    if level.entity_count_on_layer(layer_id) > 0 {
        return Err(CoreError::LayerInUse(layer_id));
    }
    let level = active_level_mut(project)?;
    level.layers.retain(|layer| layer.id != layer_id);
    Ok(Applied::default())
}

fn set_layer_property(
    project: &mut Project,
    layer_id: LayerId,
    property: LayerProperty,
) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let existing = level
        .layer(layer_id)
        .ok_or(CoreError::LayerNotFound(layer_id))?;
    let mut candidate = existing.clone();

    match property {
        LayerProperty::Name(name) => {
            let name = name.trim().to_string();
            validate_layer_name(&name)?;
            let duplicate = level
                .layers
                .iter()
                .any(|layer| layer.id != layer_id && layer.name.eq_ignore_ascii_case(&name));
            if duplicate {
                return Err(CoreError::invalid(format!(
                    "a layer named \"{name}\" already exists"
                )));
            }
            candidate.name = name;
        }
        LayerProperty::Color(color) => candidate.color = color,
        LayerProperty::Linetype(linetype) => {
            validate_linetype(linetype)?;
            candidate.linetype = linetype;
        }
        LayerProperty::Lineweight(weight) => {
            validate_lineweight(weight)?;
            candidate.lineweight_mm = weight;
        }
        LayerProperty::Visible(visible) => candidate.visible = visible,
        LayerProperty::Locked(locked) => candidate.locked = locked,
        LayerProperty::Printable(printable) => candidate.printable = printable,
    }

    if !candidate.is_valid() {
        return Err(CoreError::invalid(
            "that change would leave the layer unusable",
        ));
    }

    let level = active_level_mut(project)?;
    if let Some(layer) = level.layer_mut(layer_id) {
        *layer = candidate;
    }
    Ok(Applied::default())
}

fn assign_entity_to_layer(
    project: &mut Project,
    entity: EntitySelector,
    layer_id: LayerId,
) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    if level.layer(layer_id).is_none() {
        return Err(CoreError::LayerNotFound(layer_id));
    }
    if level.entity_layer(entity).is_none() {
        return Err(missing_entity(entity));
    }
    let level = active_level_mut(project)?;
    level.set_entity_layer(entity, layer_id);
    Ok(Applied::default())
}

fn missing_entity(entity: EntitySelector) -> CoreError {
    match entity {
        EntitySelector::Wall(id) => CoreError::WallNotFound(id),
        EntitySelector::Opening(id) => CoreError::OpeningNotFound(id),
        EntitySelector::Dimension(id) => CoreError::DimensionNotFound(id),
        EntitySelector::Label(id) => CoreError::LabelNotFound(id),
    }
}

fn create_construction(
    project: &mut Project,
    kind: ConstructionKind,
) -> Result<Applied, CoreError> {
    if let Some(reason) = kind.invalid_reason() {
        return Err(CoreError::invalid(reason));
    }
    let id = ConstructionId(project.allocate_id());
    let entity = ConstructionEntity::new(id, kind);
    active_level_mut(project)?.construction.push(entity);
    Ok(Applied {
        construction: Some(id),
        ..Applied::default()
    })
}

fn delete_construction(project: &mut Project, id: ConstructionId) -> Result<Applied, CoreError> {
    let level = active_level_mut(project)?;
    let before = level.construction.len();
    level.construction.retain(|entity| entity.id != id);
    if level.construction.len() == before {
        return Err(CoreError::ConstructionNotFound(id));
    }
    Ok(Applied::default())
}

fn add_level(project: &mut Project, name: String, elevation: f64) -> Result<Applied, CoreError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(CoreError::invalid("a level needs a name"));
    }
    if !elevation.is_finite() {
        return Err(CoreError::invalid("a level needs a finite elevation"));
    }
    let id = LevelId(project.allocate_id());
    project.levels.push(Level::new(id, trimmed, elevation));
    project.active_level = id;
    Ok(Applied {
        level: Some(id),
        ..Applied::default()
    })
}

fn delete_level(project: &mut Project, id: LevelId) -> Result<Applied, CoreError> {
    if project.levels.len() <= 1 {
        return Err(CoreError::LastLevel);
    }
    let before = project.levels.len();
    project.levels.retain(|l| l.id != id);
    if project.levels.len() == before {
        return Err(CoreError::LevelNotFound(id));
    }
    if project.active_level == id {
        if let Some(first) = project.levels.first() {
            project.active_level = first.id;
        }
    }
    Ok(Applied::default())
}

fn add_constraint(project: &mut Project, constraint: Constraint) -> Result<Applied, CoreError> {
    let level = active_level(project)?;
    let mut candidate = level.clone();
    candidate.constraints.push(constraint.clone());
    crate::constraint::validate_constraints(&candidate)?;
    let level = active_level_mut(project)?;
    level.constraints.push(constraint);
    Ok(Applied::default())
}

fn remove_constraint(project: &mut Project, index: usize) -> Result<Applied, CoreError> {
    let level = active_level_mut(project)?;
    if index >= level.constraints.len() {
        return Err(CoreError::invalid("constraint index out of range"));
    }
    level.constraints.remove(index);
    Ok(Applied::default())
}

fn set_title_block(project: &mut Project, title_block: TitleBlock) -> Result<Applied, CoreError> {
    let title_block = TitleBlock {
        company: title_block.company.trim().to_string(),
        project_name: title_block.project_name.trim().to_string(),
        sheet_title: title_block.sheet_title.trim().to_string(),
        revision: title_block.revision.trim().to_string(),
        date: title_block.date.trim().to_string(),
        drawn_by: title_block.drawn_by.trim().to_string(),
        sheet_number: title_block.sheet_number,
    };
    title_block.validate()?;
    project.title_block = title_block;
    Ok(Applied::default())
}

fn validate_viewport(project: &Project, viewport: &Viewport) -> Result<(), CoreError> {
    if project.level(viewport.level).is_none() {
        return Err(CoreError::LevelNotFound(viewport.level));
    }
    if !viewport.is_valid() {
        return Err(CoreError::invalid(
            "a viewport needs a positive scale and a region with an area",
        ));
    }
    if !viewport.is_within_sheet() {
        return Err(CoreError::invalid("a viewport region must lie on the sheet"));
    }
    if viewport.caption.chars().count() > MAX_CAPTION_CHARS {
        return Err(CoreError::invalid(format!(
            "a viewport caption must be at most {MAX_CAPTION_CHARS} characters"
        )));
    }
    Ok(())
}

fn add_viewport(
    project: &mut Project,
    level: LevelId,
    region: Bounds,
    scale_denominator: u32,
    caption: String,
) -> Result<Applied, CoreError> {
    let caption = caption.trim().to_string();
    let viewport = Viewport::new(level, region, scale_denominator, caption);
    validate_viewport(project, &viewport)?;
    project.viewports.push(viewport);
    Ok(Applied::default())
}

fn remove_viewport(project: &mut Project, index: usize) -> Result<Applied, CoreError> {
    if index >= project.viewports.len() {
        return Err(CoreError::invalid("viewport index out of range"));
    }
    project.viewports.remove(index);
    Ok(Applied::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::DEFAULT_WITNESS_GAP;
    use crate::project::Project;

    fn empty_project() -> Project {
        Project::new("Test plan", 1_760_000_000)
    }

    fn add(project: &mut Project, start: (f64, f64), end: (f64, f64)) -> WallId {
        apply_command(
            project,
            Command::AddWall {
                start: Point2::new(start.0, start.1),
                end: Point2::new(end.0, end.1),
                thickness: 200.0,
                height: 2800.0,
            },
        )
        .expect("add wall")
        .wall
        .expect("wall id")
    }

    #[test]
    fn adding_a_wall_allocates_an_id() {
        let mut project = empty_project();
        let id = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        assert_eq!(project.wall_count(), 1);
        assert!(project.active_level().expect("level").wall(id).is_some());
    }

    #[test]
    fn zero_length_walls_are_rejected() {
        let mut project = empty_project();
        let error = apply_command(
            &mut project,
            Command::AddWall {
                start: Point2::new(500.0, 500.0),
                end: Point2::new(505.0, 500.0),
                thickness: 200.0,
                height: 2800.0,
            },
        )
        .expect_err("too short");
        assert!(error.to_string().contains("at least 10 mm"));
    }

    #[test]
    fn deleting_a_wall_removes_its_openings() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        apply_command(
            &mut project,
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: wall,
                offset: 2000.0,
            },
        )
        .expect("place door");
        assert_eq!(project.opening_count(), 1);
        apply_command(&mut project, Command::DeleteWall { id: wall }).expect("delete wall");
        assert_eq!(project.wall_count(), 0);
        assert_eq!(
            project.opening_count(),
            0,
            "hosted openings go with the wall"
        );
    }

    #[test]
    fn moving_a_wall_that_would_strand_an_opening_is_refused() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        apply_command(
            &mut project,
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: wall,
                offset: 3550.0,
            },
        )
        .expect("door near the end");
        let error = apply_command(
            &mut project,
            Command::MoveWall {
                id: wall,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(3000.0, 0.0),
            },
        )
        .expect_err("door would fall off the end");
        assert!(error.to_string().contains("no longer fit"));
        // The wall is untouched after a rejected command.
        let length = project
            .active_level()
            .expect("level")
            .wall(wall)
            .expect("wall")
            .length();
        assert!((length - 4000.0).abs() < 1e-9);
    }

    #[test]
    fn overlapping_openings_are_refused() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        apply_command(
            &mut project,
            Command::PlaceOpening {
                kind: OpeningKind::Window,
                host: wall,
                offset: 1500.0,
            },
        )
        .expect("first window");
        let error = apply_command(
            &mut project,
            Command::PlaceOpening {
                kind: OpeningKind::Window,
                host: wall,
                offset: 1600.0,
            },
        )
        .expect_err("overlap");
        assert!(error.to_string().contains("overlap"));
        assert_eq!(project.opening_count(), 1);
    }

    #[test]
    fn the_last_level_cannot_be_deleted() {
        let mut project = empty_project();
        let active = project.active_level;
        let error = apply_command(&mut project, Command::DeleteLevel { id: active })
            .expect_err("last level");
        assert!(matches!(error, CoreError::LastLevel));
    }

    #[test]
    fn a_second_level_becomes_active() {
        let mut project = empty_project();
        let applied = apply_command(
            &mut project,
            Command::AddLevel {
                name: "Level 1".into(),
                elevation: 3000.0,
            },
        )
        .expect("add level");
        assert_eq!(project.levels.len(), 2);
        assert_eq!(project.active_level, applied.level.expect("level id"));
    }

    #[test]
    fn opening_kind_can_be_switched() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        apply_command(
            &mut project,
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: wall,
                offset: 2000.0,
            },
        )
        .expect("place door");
        let id = project
            .active_level()
            .expect("level")
            .openings
            .first()
            .expect("door")
            .id;
        apply_command(
            &mut project,
            Command::SetOpeningKind {
                id,
                kind: OpeningKind::Window,
            },
        )
        .expect("switch kind");
        let opening = project
            .active_level()
            .expect("level")
            .opening(id)
            .expect("opening");
        assert_eq!(opening.kind, OpeningKind::Window);
        assert!(project.validate().is_ok());
    }

    fn place_horizontal(project: &mut Project) -> DimensionId {
        apply_command(
            project,
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(3000.0, 0.0),
                offset: 500.0,
            },
        )
        .expect("place dimension")
        .dimension
        .expect("dimension id")
    }

    #[test]
    fn placing_a_dimension_returns_its_id() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        assert_eq!(project.dimension_count(), 1);
        assert!(
            project
                .active_level()
                .expect("level")
                .dimension(id)
                .is_some()
        );
    }

    #[test]
    fn zero_length_dimensions_are_rejected() {
        let mut project = empty_project();
        let before = project.next_id;
        let error = apply_command(
            &mut project,
            Command::PlaceDimension {
                kind: DimensionKind::Aligned,
                start: Point2::new(500.0, 500.0),
                end: Point2::new(500.0, 500.0),
                offset: 500.0,
            },
        )
        .expect_err("zero length");
        assert!(error.to_string().contains("different measured points"));
        assert_eq!(project.dimension_count(), 0);
        assert_eq!(project.next_id, before, "a rejected command burns no id");
    }

    #[test]
    fn dimension_offsets_must_be_positive() {
        let mut project = empty_project();
        let error = apply_command(
            &mut project,
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(3000.0, 0.0),
                offset: 0.0,
            },
        )
        .expect_err("zero offset");
        assert!(error.to_string().contains("greater than zero"));
        assert_eq!(project.dimension_count(), 0);
    }

    #[test]
    fn a_dimension_can_be_moved_and_deleted() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::MoveDimension {
                id,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
            },
        )
        .expect("move dimension");
        let dimension = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension");
        assert!((dimension.length() - 4000.0).abs() < 1e-9);

        apply_command(&mut project, Command::DeleteDimension { id }).expect("delete dimension");
        assert_eq!(project.dimension_count(), 0);
        let error =
            apply_command(&mut project, Command::DeleteDimension { id }).expect_err("already gone");
        assert!(matches!(error, CoreError::DimensionNotFound(_)));
    }

    #[test]
    fn a_dimension_offset_can_be_changed() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::SetDimensionOffset { id, offset: 800.0 },
        )
        .expect("set offset");
        let offset = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .offset;
        assert_eq!(offset, 800.0);

        let error = apply_command(
            &mut project,
            Command::SetDimensionOffset { id, offset: 0.0 },
        )
        .expect_err("zero offset");
        assert!(error.to_string().contains("greater than zero"));
        let offset = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .offset;
        assert_eq!(offset, 800.0, "a rejected change leaves the offset alone");
    }

    #[test]
    fn a_placed_dimension_can_be_undone_and_redone() {
        use crate::document::Document;

        let mut document = Document::new("Test plan", 1_760_000_000);
        let id = document
            .apply(
                Command::PlaceDimension {
                    kind: DimensionKind::LinearVertical,
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(0.0, 2000.0),
                    offset: 500.0,
                },
                1_760_000_100,
            )
            .expect("place dimension")
            .dimension
            .expect("dimension id");
        assert!(document.undo(1_760_000_200));
        assert_eq!(document.project().dimension_count(), 0);
        assert!(document.redo(1_760_000_300));
        assert_eq!(document.project().dimension_count(), 1);
        assert!(
            document
                .project()
                .active_level()
                .expect("level")
                .dimension(id)
                .is_some()
        );
    }

    #[test]
    fn switching_a_dimension_kind_revalidates() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        // The two points share a y, so a vertical measure of them is zero and
        // the switch must be refused.
        let error = apply_command(
            &mut project,
            Command::SetDimensionKind {
                id,
                kind: DimensionKind::LinearVertical,
            },
        )
        .expect_err("a vertical measure of a horizontal pair is zero");
        assert!(error.to_string().contains("different measured points"));
        let kind = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .kind;
        assert_eq!(
            kind,
            DimensionKind::LinearHorizontal,
            "a rejected change leaves the kind alone"
        );

        apply_command(
            &mut project,
            Command::SetDimensionKind {
                id,
                kind: DimensionKind::Aligned,
            },
        )
        .expect("aligned always measures something");
        let kind = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .kind;
        assert_eq!(kind, DimensionKind::Aligned);
    }

    #[test]
    fn dimension_extension_lines_can_be_toggled() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::SetDimensionExtensionLines {
                id,
                extension_lines: false,
            },
        )
        .expect("hide extension lines");
        let extension_lines = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .extension_lines;
        assert!(!extension_lines);

        apply_command(
            &mut project,
            Command::SetDimensionExtensionLines {
                id,
                extension_lines: true,
            },
        )
        .expect("show extension lines");
        let extension_lines = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .extension_lines;
        assert!(extension_lines);
    }

    fn dimension_anchor(project: &Project, id: DimensionId) -> Option<DimensionAnchor> {
        project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .start_anchor
    }

    #[test]
    fn a_dimension_can_be_anchored_to_a_wall() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let id = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::SetDimensionStartAnchor {
                id,
                anchor: Some(DimensionAnchor::new(wall, 0.25)),
            },
        )
        .expect("anchor the start");
        let anchor = dimension_anchor(&project, id).expect("start anchor");
        assert_eq!(anchor.wall_id, wall);
        assert_eq!(anchor.param, 0.25);
        assert!(!anchor.perpendicular);
        assert!(
            project
                .active_level()
                .expect("level")
                .dimension(id)
                .expect("dimension")
                .end_anchor
                .is_none(),
            "only the requested end is anchored"
        );

        apply_command(
            &mut project,
            Command::SetDimensionStartAnchor { id, anchor: None },
        )
        .expect("clear the start anchor");
        assert!(dimension_anchor(&project, id).is_none());
    }

    #[test]
    fn an_anchor_parameter_must_lie_on_its_wall() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let id = place_horizontal(&mut project);
        let error = apply_command(
            &mut project,
            Command::SetDimensionEndAnchor {
                id,
                anchor: Some(DimensionAnchor::new(wall, 1.5)),
            },
        )
        .expect_err("past the end of the wall");
        assert!(error.to_string().contains("between the ends"));

        let error = apply_command(
            &mut project,
            Command::SetDimensionEndAnchor {
                id,
                anchor: Some(DimensionAnchor::new(wall, f64::NAN)),
            },
        )
        .expect_err("not a number");
        assert!(error.to_string().contains("between the ends"));

        assert!(
            project
                .active_level()
                .expect("level")
                .dimension(id)
                .expect("dimension")
                .end_anchor
                .is_none(),
            "a rejected anchor leaves the dimension alone"
        );
    }

    #[test]
    fn witness_margins_can_be_set() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::SetDimensionWitnessGap { id, gap: 150.0 },
        )
        .expect("set the gap");
        apply_command(
            &mut project,
            Command::SetDimensionWitnessOverrun { id, overrun: 75.0 },
        )
        .expect("set the overrun");
        let dimension = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension");
        assert_eq!(dimension.witness_gap, 150.0);
        assert_eq!(dimension.witness_overrun, 75.0);
    }

    #[test]
    fn witness_margins_must_be_distances() {
        let mut project = empty_project();
        let id = place_horizontal(&mut project);
        let error = apply_command(
            &mut project,
            Command::SetDimensionWitnessGap { id, gap: -1.0 },
        )
        .expect_err("a negative gap");
        assert!(error.to_string().contains("zero or more"));

        let error = apply_command(
            &mut project,
            Command::SetDimensionWitnessOverrun {
                id,
                overrun: f64::INFINITY,
            },
        )
        .expect_err("an infinite overrun");
        assert!(error.to_string().contains("zero or more"));

        let gap = project
            .active_level()
            .expect("level")
            .dimension(id)
            .expect("dimension")
            .witness_gap;
        assert_eq!(gap, DEFAULT_WITNESS_GAP, "a rejected margin is ignored");
    }

    #[test]
    fn anchoring_a_missing_dimension_reports_it_gone() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let missing = DimensionId(999);
        let error = apply_command(
            &mut project,
            Command::SetDimensionStartAnchor {
                id: missing,
                anchor: Some(DimensionAnchor::new(wall, 0.5)),
            },
        )
        .expect_err("no such dimension");
        let CoreError::DimensionNotFound(id) = error else {
            panic!("expected a missing-dimension error");
        };
        assert_eq!(id, missing);

        let error = apply_command(
            &mut project,
            Command::SetDimensionEndAnchor {
                id: missing,
                anchor: None,
            },
        )
        .expect_err("no such dimension");
        assert!(matches!(error, CoreError::DimensionNotFound(_)));

        let error = apply_command(
            &mut project,
            Command::SetDimensionWitnessGap {
                id: missing,
                gap: 100.0,
            },
        )
        .expect_err("no such dimension");
        assert!(matches!(error, CoreError::DimensionNotFound(_)));
    }

    #[test]
    fn the_new_dimension_commands_round_trip_through_json() {
        let commands = [
            Command::SetDimensionStartAnchor {
                id: DimensionId(3),
                anchor: Some(DimensionAnchor::new(WallId(7), 0.5)),
            },
            Command::SetDimensionEndAnchor {
                id: DimensionId(3),
                anchor: None,
            },
            Command::SetDimensionWitnessGap {
                id: DimensionId(3),
                gap: 150.0,
            },
            Command::SetDimensionWitnessOverrun {
                id: DimensionId(3),
                overrun: 50.0,
            },
        ];
        for command in commands {
            let json = serde_json::to_string(&command).expect("serialize");
            let back: Command = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, command);
        }
    }

    fn add_label(project: &mut Project, text: &str) -> LabelId {
        apply_command(
            project,
            Command::PlaceLabel {
                anchor: Point2::new(1000.0, 500.0),
                text: text.into(),
            },
        )
        .expect("place label")
        .label
        .expect("label id")
    }

    #[test]
    fn placing_a_label_returns_its_id() {
        use crate::label::DEFAULT_LABEL_HEIGHT_MM;

        let mut project = empty_project();
        let id = add_label(&mut project, "  Living room  ");
        let level = project.active_level().expect("level");
        assert_eq!(level.labels.len(), 1);
        let label = level.label(id).expect("label");
        assert_eq!(label.text, "Living room", "text is trimmed on placement");
        assert_eq!(label.anchor, Point2::new(1000.0, 500.0));
        assert_eq!(label.height_mm, DEFAULT_LABEL_HEIGHT_MM);
    }

    #[test]
    fn a_rejected_label_burns_no_id() {
        let mut project = empty_project();
        let before = project.next_id;
        let error = apply_command(
            &mut project,
            Command::PlaceLabel {
                anchor: Point2::new(1000.0, 500.0),
                text: "   ".into(),
            },
        )
        .expect_err("blank text");
        assert!(error.to_string().contains("some text"));
        assert_eq!(project.active_level().expect("level").labels.len(), 0);
        assert_eq!(project.next_id, before, "a rejected command burns no id");
    }

    #[test]
    fn setting_label_text_rejects_blank() {
        let mut project = empty_project();
        let id = add_label(&mut project, "Hall");
        let error = apply_command(
            &mut project,
            Command::SetLabelText {
                id,
                text: " ".into(),
            },
        )
        .expect_err("blank text");
        assert!(error.to_string().contains("some text"));
        let text = project
            .active_level()
            .expect("level")
            .label(id)
            .expect("label")
            .text
            .clone();
        assert_eq!(text, "Hall", "a rejected change leaves the text alone");

        apply_command(
            &mut project,
            Command::SetLabelText {
                id,
                text: "Corridor".into(),
            },
        )
        .expect("set text");
        let text = project
            .active_level()
            .expect("level")
            .label(id)
            .expect("label")
            .text
            .clone();
        assert_eq!(text, "Corridor");
    }

    #[test]
    fn moving_a_label_moves_its_anchor() {
        let mut project = empty_project();
        let id = add_label(&mut project, "Hall");
        apply_command(
            &mut project,
            Command::MoveLabel {
                id,
                anchor: Point2::new(2500.0, 1750.0),
            },
        )
        .expect("move label");
        let anchor = project
            .active_level()
            .expect("level")
            .label(id)
            .expect("label")
            .anchor;
        assert_eq!(anchor, Point2::new(2500.0, 1750.0));
    }

    #[test]
    fn deleting_a_label_removes_it_once() {
        let mut project = empty_project();
        let id = add_label(&mut project, "Hall");
        apply_command(&mut project, Command::DeleteLabel { id }).expect("delete label");
        assert_eq!(project.active_level().expect("level").labels.len(), 0);
        let error =
            apply_command(&mut project, Command::DeleteLabel { id }).expect_err("already gone");
        assert!(matches!(error, CoreError::LabelNotFound(_)));
    }

    #[test]
    fn a_deleted_label_can_be_undone_and_redone() {
        use crate::document::Document;

        let mut document = Document::new("Test plan", 1_760_000_000);
        let id = document
            .apply(
                Command::PlaceLabel {
                    anchor: Point2::new(1000.0, 500.0),
                    text: "Hall".into(),
                },
                1_760_000_100,
            )
            .expect("place label")
            .label
            .expect("label id");
        document
            .apply(Command::DeleteLabel { id }, 1_760_000_200)
            .expect("delete label");
        assert!(
            document
                .project()
                .active_level()
                .expect("level")
                .label(id)
                .is_none()
        );
        assert!(document.undo(1_760_000_300));
        assert!(
            document
                .project()
                .active_level()
                .expect("level")
                .label(id)
                .is_some()
        );
        assert!(document.redo(1_760_000_400));
        assert!(
            document
                .project()
                .active_level()
                .expect("level")
                .label(id)
                .is_none()
        );
    }

    fn add_layer(project: &mut Project, name: &str) -> LayerId {
        let command = Command::CreateLayer {
            name: name.into(),
            color: None,
            linetype: Linetype::Continuous,
            lineweight_mm: 0.25,
        };
        apply_command(project, command)
            .expect("create layer")
            .layer
            .expect("layer id")
    }

    #[test]
    fn creating_a_layer_returns_its_id() {
        let mut project = empty_project();
        let id = add_layer(&mut project, "Walls");
        assert_ne!(id, LayerId::DEFAULT);
        let layer = project
            .active_level()
            .expect("level")
            .layer(id)
            .expect("layer");
        assert_eq!(layer.name, "Walls");
        assert!(layer.visible);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn a_rejected_layer_burns_no_id() {
        let mut project = empty_project();
        let before = project.next_id;
        let applied = apply_command(
            &mut project,
            Command::CreateLayer {
                name: "Walls".into(),
                color: None,
                linetype: Linetype::Continuous,
                lineweight_mm: 0.25,
            },
        )
        .expect("first layer");
        assert!(applied.layer.is_some());
        let error = apply_command(
            &mut project,
            Command::CreateLayer {
                name: "walls".into(),
                color: None,
                linetype: Linetype::Continuous,
                lineweight_mm: 0.25,
            },
        )
        .expect_err("duplicate name");
        assert!(error.to_string().contains("already exists"));
        assert_eq!(project.next_id, before + 1, "only the first created an id");
    }

    #[test]
    fn a_zero_lineweight_layer_is_refused() {
        let mut project = empty_project();
        let error = apply_command(
            &mut project,
            Command::CreateLayer {
                name: "Bad".into(),
                color: None,
                linetype: Linetype::Continuous,
                lineweight_mm: 0.0,
            },
        )
        .expect_err("lineweight too small");
        assert!(error.to_string().contains("lineweight"));
        assert_eq!(
            project.active_level().expect("level").layers.len(),
            1,
            "only the default layer remains"
        );
    }

    #[test]
    fn the_default_layer_cannot_be_deleted() {
        let mut project = empty_project();
        let error = apply_command(
            &mut project,
            Command::DeleteLayer {
                layer_id: LayerId::DEFAULT,
            },
        )
        .expect_err("default layer");
        assert!(matches!(error, CoreError::DefaultLayer));
        assert!(
            project
                .active_level()
                .expect("level")
                .layer(LayerId::DEFAULT)
                .is_some()
        );
    }

    #[test]
    fn a_layer_with_entities_cannot_be_deleted() {
        let mut project = empty_project();
        let layer = add_layer(&mut project, "Walls");
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        apply_command(
            &mut project,
            Command::AssignEntityToLayer {
                entity: EntitySelector::Wall(wall),
                layer_id: layer,
            },
        )
        .expect("assign wall");

        let error = apply_command(&mut project, Command::DeleteLayer { layer_id: layer })
            .expect_err("layer in use");
        assert!(matches!(error, CoreError::LayerInUse(_)));

        apply_command(
            &mut project,
            Command::AssignEntityToLayer {
                entity: EntitySelector::Wall(wall),
                layer_id: LayerId::DEFAULT,
            },
        )
        .expect("move the wall back");
        apply_command(&mut project, Command::DeleteLayer { layer_id: layer })
            .expect("now the layer is empty");
        assert!(
            project
                .active_level()
                .expect("level")
                .layer(layer)
                .is_none()
        );
    }

    #[test]
    fn layer_properties_change_one_at_a_time() {
        let mut project = empty_project();
        let layer = add_layer(&mut project, "Walls");

        apply_command(
            &mut project,
            Command::SetLayerProperty {
                layer_id: layer,
                property: LayerProperty::Visible(false),
            },
        )
        .expect("hide");
        let level = project.active_level().expect("level");
        assert!(!level.layer(layer).expect("layer").visible);

        apply_command(
            &mut project,
            Command::SetLayerProperty {
                layer_id: layer,
                property: LayerProperty::Color(Some(Rgb::new(10, 20, 30))),
            },
        )
        .expect("colour");
        assert_eq!(
            project
                .active_level()
                .expect("level")
                .layer(layer)
                .expect("layer")
                .color,
            Some(Rgb::new(10, 20, 30))
        );

        let error = apply_command(
            &mut project,
            Command::SetLayerProperty {
                layer_id: layer,
                property: LayerProperty::Lineweight(99.0),
            },
        )
        .expect_err("lineweight out of range");
        assert!(error.to_string().contains("lineweight"));
        assert_eq!(
            project
                .active_level()
                .expect("level")
                .layer(layer)
                .expect("layer")
                .lineweight_mm,
            0.25,
            "a rejected change leaves the layer alone"
        );
    }

    #[test]
    fn a_layer_can_be_renamed_but_not_onto_an_existing_name() {
        let mut project = empty_project();
        let first = add_layer(&mut project, "Walls");
        let second = add_layer(&mut project, "Doors");

        apply_command(
            &mut project,
            Command::SetLayerProperty {
                layer_id: second,
                property: LayerProperty::Name("Openings".into()),
            },
        )
        .expect("rename");

        let error = apply_command(
            &mut project,
            Command::SetLayerProperty {
                layer_id: second,
                property: LayerProperty::Name("Walls".into()),
            },
        )
        .expect_err("duplicate");
        assert!(error.to_string().contains("already exists"));

        let level = project.active_level().expect("level");
        assert_eq!(level.layer(first).expect("layer").name, "Walls");
        assert_eq!(level.layer(second).expect("layer").name, "Openings");
    }

    #[test]
    fn assigning_an_entity_moves_it_and_its_style_copy() {
        let mut project = empty_project();
        let layer = add_layer(&mut project, "Walls");
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));

        apply_command(
            &mut project,
            Command::AssignEntityToLayer {
                entity: EntitySelector::Wall(wall),
                layer_id: layer,
            },
        )
        .expect("assign");
        let level = project.active_level().expect("level");
        let wall = level.wall(wall).expect("wall");
        assert_eq!(wall.layer_id, layer);
        assert_eq!(wall.style.layer_id, layer);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn assigning_to_a_missing_layer_or_entity_is_reported() {
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));

        let error = apply_command(
            &mut project,
            Command::AssignEntityToLayer {
                entity: EntitySelector::Wall(wall),
                layer_id: LayerId(999),
            },
        )
        .expect_err("missing layer");
        assert!(matches!(error, CoreError::LayerNotFound(_)));

        let error = apply_command(
            &mut project,
            Command::AssignEntityToLayer {
                entity: EntitySelector::Wall(WallId(999)),
                layer_id: LayerId::DEFAULT,
            },
        )
        .expect_err("missing wall");
        assert!(matches!(error, CoreError::WallNotFound(_)));
    }

    #[test]
    fn construction_guides_can_be_created_retargeted_and_deleted() {
        let mut project = empty_project();
        let applied = apply_command(
            &mut project,
            Command::CreateConstruction {
                kind: ConstructionKind::InfiniteLine {
                    point: Point2::new(0.0, 0.0),
                    direction: crate::geom::Vec2::new(1.0, 0.0),
                },
            },
        )
        .expect("create guide");
        let id = applied.construction.expect("construction id");
        assert_eq!(
            project.active_level().expect("level").construction_count(),
            1
        );

        apply_command(
            &mut project,
            Command::SetConstructionVisible { id, visible: false },
        )
        .expect("hide guide");
        assert!(
            !project
                .active_level()
                .expect("level")
                .construction_entity(id)
                .expect("guide")
                .visible
        );
        assert_eq!(
            project
                .active_level()
                .expect("level")
                .visible_construction()
                .count(),
            0
        );

        apply_command(&mut project, Command::DeleteConstruction { id }).expect("delete guide");
        assert_eq!(
            project.active_level().expect("level").construction_count(),
            0
        );
        let error = apply_command(&mut project, Command::DeleteConstruction { id })
            .expect_err("already gone");
        assert!(matches!(error, CoreError::ConstructionNotFound(_)));
    }

    #[test]
    fn an_invalid_construction_guide_burns_no_id() {
        let mut project = empty_project();
        let before = project.next_id;
        let error = apply_command(
            &mut project,
            Command::CreateConstruction {
                kind: ConstructionKind::Circle {
                    center: Point2::new(0.0, 0.0),
                    radius: 0.0,
                },
            },
        )
        .expect_err("zero radius");
        assert!(error.to_string().contains("positive radius"));
        assert_eq!(project.next_id, before);
    }

    #[test]
    fn a_construction_guide_never_prints() {
        let mut project = empty_project();
        let applied = apply_command(
            &mut project,
            Command::CreateConstruction {
                kind: ConstructionKind::Line {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(1000.0, 0.0),
                },
            },
        )
        .expect("create guide");
        let id = applied.construction.expect("construction id");
        let guide = project
            .active_level()
            .expect("level")
            .construction_entity(id)
            .expect("guide");
        assert!(!guide.print_enabled);
    }

    #[test]
    fn adding_a_constraint_pushes_it_onto_the_active_level() {
        use crate::constraint::Constraint;
        let mut project = empty_project();
        // Need a dimension and wall for a perpendicular constraint.
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let dim = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::AddConstraint {
                constraint: Constraint::PerpendicularDistance {
                    dimension: dim,
                    wall_id: wall,
                    param: 0.5,
                },
            },
        )
        .expect("add constraint");
        assert_eq!(project.active_level().expect("level").constraints.len(), 1);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn adding_an_invalid_constraint_is_rejected() {
        use crate::constraint::Constraint;
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let dim = place_horizontal(&mut project);
        let error = apply_command(
            &mut project,
            Command::AddConstraint {
                constraint: Constraint::PerpendicularDistance {
                    dimension: dim,
                    wall_id: wall,
                    param: 1.5,
                },
            },
        )
        .expect_err("param out of range");
        assert!(error.to_string().contains("outside 0..1"));
        assert!(
            project
                .active_level()
                .expect("level")
                .constraints
                .is_empty()
        );
    }

    #[test]
    fn removing_a_constraint_by_index_works() {
        use crate::constraint::Constraint;
        let mut project = empty_project();
        let wall = add(&mut project, (0.0, 0.0), (4000.0, 0.0));
        let dim = place_horizontal(&mut project);
        apply_command(
            &mut project,
            Command::AddConstraint {
                constraint: Constraint::PerpendicularDistance {
                    dimension: dim,
                    wall_id: wall,
                    param: 0.5,
                },
            },
        )
        .expect("add constraint");
        apply_command(&mut project, Command::RemoveConstraint { index: 0 }).expect("remove");
        assert!(
            project
                .active_level()
                .expect("level")
                .constraints
                .is_empty()
        );
        let error = apply_command(&mut project, Command::RemoveConstraint { index: 0 })
            .expect_err("out of bounds");
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn an_added_constraint_can_be_undone() {
        use crate::constraint::Constraint;
        use crate::document::Document;
        let mut document = Document::new("Test plan", 1_760_000_000);
        // set up wall + dimension via commands
        let wall = document
            .apply(
                Command::AddWall {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(4000.0, 0.0),
                    thickness: 200.0,
                    height: 2800.0,
                },
                1_760_000_010,
            )
            .expect("wall")
            .wall
            .expect("wall id");
        let dim = document
            .apply(
                Command::PlaceDimension {
                    kind: DimensionKind::LinearHorizontal,
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(3000.0, 0.0),
                    offset: 500.0,
                },
                1_760_000_020,
            )
            .expect("dimension")
            .dimension
            .expect("dim id");
        document
            .apply(
                Command::AddConstraint {
                    constraint: Constraint::PerpendicularDistance {
                        dimension: dim,
                        wall_id: wall,
                        param: 0.5,
                    },
                },
                1_760_000_030,
            )
            .expect("add constraint");
        assert_eq!(
            document
                .project()
                .active_level()
                .expect("level")
                .constraints
                .len(),
            1
        );
        assert!(document.undo(1_760_000_040));
        assert!(
            document
                .project()
                .active_level()
                .expect("level")
                .constraints
                .is_empty()
        );
    }

    fn viewport_region() -> Bounds {
        Bounds {
            min: Point2::new(10.0, 40.0),
            max: Point2::new(150.0, 140.0),
        }
    }

    #[test]
    fn setting_the_title_block_replaces_it_and_trims_whitespace() {
        let mut project = empty_project();
        apply_command(
            &mut project,
            Command::SetTitleBlock {
                title_block: TitleBlock {
                    company: "  Acme Drafting  ".into(),
                    project_name: "  Riverside House  ".into(),
                    sheet_title: "Ground floor".into(),
                    revision: " A ".into(),
                    date: "2026-11-14".into(),
                    drawn_by: " JD ".into(),
                    sheet_number: Some(101),
                },
            },
        )
        .expect("set title block");
        let block = &project.title_block;
        assert_eq!(block.company, "Acme Drafting");
        assert_eq!(block.project_name, "Riverside House");
        assert_eq!(block.revision, "A");
        assert_eq!(block.drawn_by, "JD");
        assert_eq!(block.sheet_number, Some(101));
        assert!(project.validate().is_ok());
    }

    #[test]
    fn a_rejected_title_block_leaves_the_old_one_alone() {
        let mut project = empty_project();
        let before = project.title_block.clone();
        let error = apply_command(
            &mut project,
            Command::SetTitleBlock {
                title_block: TitleBlock {
                    revision: "WAY-TOO-LONG".into(),
                    ..TitleBlock::default()
                },
            },
        )
        .expect_err("long revision");
        assert!(error.to_string().contains("at most 8 characters"));
        assert_eq!(project.title_block, before, "the old block is untouched");

        let error = apply_command(
            &mut project,
            Command::SetTitleBlock {
                title_block: TitleBlock {
                    sheet_number: Some(0),
                    ..TitleBlock::default()
                },
            },
        )
        .expect_err("zero sheet number");
        assert!(error.to_string().contains("1 or more"));
    }

    #[test]
    fn a_set_title_block_can_be_undone_and_redone() {
        use crate::document::Document;

        let mut document = Document::new("Test plan", 1_760_000_000);
        document
            .apply(
                Command::SetTitleBlock {
                    title_block: TitleBlock {
                        project_name: "Riverside House".into(),
                        ..TitleBlock::default()
                    },
                },
                1_760_000_100,
            )
            .expect("set title block");
        assert_eq!(
            document.project().title_block.project_name,
            "Riverside House"
        );
        assert!(document.undo(1_760_000_200));
        assert_eq!(
            document.project().title_block,
            TitleBlock::default(),
            "undo restores the default block"
        );
        assert!(document.redo(1_760_000_300));
        assert_eq!(
            document.project().title_block.project_name,
            "Riverside House"
        );
    }

    #[test]
    fn adding_a_viewport_appends_it() {
        let mut project = empty_project();
        let level = project.active_level;
        apply_command(
            &mut project,
            Command::AddViewport {
                level,
                region: viewport_region(),
                scale_denominator: 50,
                caption: "  Ground floor unit  ".into(),
            },
        )
        .expect("add viewport");
        assert_eq!(project.viewports.len(), 1);
        assert_eq!(project.viewports[0].caption, "Ground floor unit");
        assert_eq!(project.viewports[0].level, level);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn a_viewport_bound_to_a_missing_level_is_refused() {
        let mut project = empty_project();
        let error = apply_command(
            &mut project,
            Command::AddViewport {
                level: LevelId(999),
                region: viewport_region(),
                scale_denominator: 50,
                caption: String::new(),
            },
        )
        .expect_err("missing level");
        assert!(matches!(error, CoreError::LevelNotFound(_)));
        assert!(project.viewports.is_empty());
    }

    #[test]
    fn a_viewport_with_a_zero_scale_or_bad_region_is_refused() {
        let mut project = empty_project();
        let level = project.active_level;
        let error = apply_command(
            &mut project,
            Command::AddViewport {
                level,
                region: viewport_region(),
                scale_denominator: 0,
                caption: String::new(),
            },
        )
        .expect_err("zero scale");
        assert!(error.to_string().contains("positive scale"));

        let flat = Bounds {
            min: Point2::new(10.0, 10.0),
            max: Point2::new(10.0, 100.0),
        };
        let error = apply_command(
            &mut project,
            Command::AddViewport {
                level,
                region: flat,
                scale_denominator: 50,
                caption: String::new(),
            },
        )
        .expect_err("flat region");
        assert!(error.to_string().contains("region with an area"));

        let off_sheet = Bounds {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(5000.0, 5000.0),
        };
        let error = apply_command(
            &mut project,
            Command::AddViewport {
                level,
                region: off_sheet,
                scale_denominator: 50,
                caption: String::new(),
            },
        )
        .expect_err("region off the sheet");
        assert!(error.to_string().contains("lie on the sheet"));
        assert!(project.viewports.is_empty());
    }

    #[test]
    fn removing_a_viewport_by_index_works_and_is_bounds_checked() {
        let mut project = empty_project();
        let level = project.active_level;
        for _ in 0..2 {
            apply_command(
                &mut project,
                Command::AddViewport {
                    level,
                    region: viewport_region(),
                    scale_denominator: 50,
                    caption: String::new(),
                },
            )
            .expect("add viewport");
        }
        apply_command(&mut project, Command::RemoveViewport { index: 0 }).expect("remove");
        assert_eq!(project.viewports.len(), 1);
        let error = apply_command(&mut project, Command::RemoveViewport { index: 5 })
            .expect_err("out of bounds");
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn an_added_viewport_can_be_undone_and_redone() {
        use crate::document::Document;

        let mut document = Document::new("Test plan", 1_760_000_000);
        let level = document.project().active_level;
        document
            .apply(
                Command::AddViewport {
                    level,
                    region: viewport_region(),
                    scale_denominator: 50,
                    caption: "Unit A".into(),
                },
                1_760_000_100,
            )
            .expect("add viewport");
        assert_eq!(document.project().viewports.len(), 1);
        assert!(document.undo(1_760_000_200));
        assert!(document.project().viewports.is_empty());
        assert!(document.redo(1_760_000_300));
        assert_eq!(document.project().viewports.len(), 1);
    }

    #[test]
    fn the_sheet_commands_round_trip_through_json() {
        let commands = [
            Command::SetTitleBlock {
                title_block: TitleBlock {
                    company: "Acme".into(),
                    revision: "B".into(),
                    sheet_number: Some(101),
                    ..TitleBlock::default()
                },
            },
            Command::AddViewport {
                level: LevelId(1),
                region: Bounds {
                    min: Point2::new(10.0, 40.0),
                    max: Point2::new(150.0, 140.0),
                },
                scale_denominator: 50,
                caption: "Unit A".into(),
            },
            Command::RemoveViewport { index: 2 },
        ];
        for command in commands {
            let json = serde_json::to_string(&command).expect("serialize");
            let back: Command = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, command);
        }
    }
}
