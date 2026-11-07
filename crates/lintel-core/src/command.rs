//! Every edit the user can make is a `Command`.
//!
//! The UI never mutates the model directly: it sends a command and renders the
//! result (addendum K.3). That is what makes the workflow tests in
//! `tests/workflows.rs` exercise the *same* path the app uses.

use serde::{Deserialize, Serialize};

use crate::dimension::{Dimension, DimensionKind};
use crate::error::CoreError;
use crate::geom::Point2;
use crate::ids::{DimensionId, LabelId, LevelId, OpeningId, WallId};
use crate::label::{MAX_LABEL_HEIGHT_MM, MIN_LABEL_HEIGHT_MM, PlanLabel};
use crate::level::Level;
use crate::opening::{
    MAX_OPENING_HEIGHT, MAX_OPENING_WIDTH, MIN_OPENING_HEIGHT, MIN_OPENING_WIDTH, Opening,
    OpeningKind,
};
use crate::project::Project;
use crate::units::UnitSystem;
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
}

/// What a successful command created, so the UI can select it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Applied {
    pub wall: Option<WallId>,
    pub opening: Option<OpeningId>,
    pub dimension: Option<DimensionId>,
    pub label: Option<LabelId>,
    pub level: Option<LevelId>,
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
