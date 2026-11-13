//! The project document: metadata plus a list of storeys.
//!
//! Everything the user creates lives here. The file is a plain, versioned
//! JSON document (`<name>.lintel.json`) so a user can read it, diff it, keep
//! it under version control, and reopen it in five years without this app.

use serde::{Deserialize, Serialize};

use crate::constraint::apply_constraints_to_anchors;
use crate::error::CoreError;
use crate::geom::Bounds;
use crate::ids::{LayerId, LevelId, OpeningId};
use crate::layer::Layer;
use crate::level::Level;
use crate::title_block::TitleBlock;
use crate::units::UnitSystem;
use crate::viewport::Viewport;

/// Current on-disk format. A file written by a newer build is refused rather
/// than half-understood.
pub const FORMAT_VERSION: u32 = 1;

/// Untitled project name used when the caller does not supply one.
pub const UNTITLED: &str = "Untitled plan";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub format_version: u32,
    pub name: String,
    pub units: UnitSystem,
    pub levels: Vec<Level>,
    pub active_level: LevelId,
    pub created_epoch_s: i64,
    pub modified_epoch_s: i64,
    pub notes: String,
    /// Title block printed on every exported sheet. Old files load with a
    /// default block, so a sheet always carries one.
    #[serde(default)]
    pub title_block: TitleBlock,
    /// Sheet viewports: framed windows onto a level at their own scale.
    /// `serde(default)` keeps files written before viewports existed readable.
    #[serde(default)]
    pub viewports: Vec<Viewport>,
    /// Monotonic identity source. Stored so a reopened file keeps allocating
    /// fresh ids instead of reusing a deleted wall's number.
    pub next_id: u64,
}

impl Project {
    /// A new project with one empty ground level.
    pub fn new(name: impl Into<String>, now_epoch_s: i64) -> Self {
        let name = name.into();
        let name = if name.trim().is_empty() {
            UNTITLED.to_string()
        } else {
            name
        };
        let level_id = LevelId(1);
        Self {
            format_version: FORMAT_VERSION,
            name,
            units: UnitSystem::Metric,
            levels: vec![Level::new(level_id, "Level 0", 0.0)],
            active_level: level_id,
            created_epoch_s: now_epoch_s,
            modified_epoch_s: now_epoch_s,
            notes: String::new(),
            title_block: TitleBlock::default(),
            viewports: Vec::new(),
            next_id: 2,
        }
    }

    /// Allocate a new identity. Ids are never reused within a project.
    pub fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    /// Bring a project written by an older build up to the current shape.
    ///
    /// Every level gains the default layer, and every entity that references a
    /// missing layer is moved onto the default one (or onto the layer its style
    /// already names, when that layer exists). The loader runs this before
    /// [`Project::validate`], so old files open instead of being refused.
    pub fn migrate(&mut self) {
        for level in &mut self.levels {
            if level.layer(LayerId::DEFAULT).is_none() {
                level.layers.insert(0, Layer::default_layer());
            }
            let known: Vec<LayerId> = level.layers.iter().map(|layer| layer.id).collect();

            for wall in &mut level.walls {
                Self::sync_entity_layer(&mut wall.layer_id, &mut wall.style.layer_id, &known);
            }
            for opening in &mut level.openings {
                Self::sync_entity_layer(&mut opening.layer_id, &mut opening.style.layer_id, &known);
            }
            for dimension in &mut level.dimensions {
                Self::sync_entity_layer(
                    &mut dimension.layer_id,
                    &mut dimension.style.layer_id,
                    &known,
                );
            }
            for label in &mut level.labels {
                Self::sync_entity_layer(&mut label.layer_id, &mut label.style.layer_id, &known);
            }
        }
    }

    /// Point both copies of an entity's layer association at a layer that
    /// actually exists. If the entity's layer is unknown, fall back to the
    /// default layer so validation passes.
    fn sync_entity_layer(layer_id: &mut LayerId, style_layer_id: &mut LayerId, known: &[LayerId]) {
        if !known.contains(layer_id) {
            *layer_id = LayerId::DEFAULT;
        }
        *style_layer_id = *layer_id;
    }

    pub fn active_level(&self) -> Option<&Level> {
        self.levels.iter().find(|l| l.id == self.active_level)
    }

    pub fn active_level_mut(&mut self) -> Option<&mut Level> {
        self.levels.iter_mut().find(|l| l.id == self.active_level)
    }

    pub fn level(&self, id: LevelId) -> Option<&Level> {
        self.levels.iter().find(|l| l.id == id)
    }

    pub fn level_mut(&mut self, id: LevelId) -> Option<&mut Level> {
        self.levels.iter_mut().find(|l| l.id == id)
    }

    /// Bounds of the active level, for zoom-to-fit and sheet layout.
    pub fn extent(&self) -> Option<Bounds> {
        self.active_level().and_then(Level::extent)
    }

    pub fn wall_count(&self) -> usize {
        self.levels.iter().map(Level::wall_count).sum()
    }

    pub fn opening_count(&self) -> usize {
        self.levels.iter().map(Level::opening_count).sum()
    }

    pub fn dimension_count(&self) -> usize {
        self.levels.iter().map(Level::dimension_count).sum()
    }

    /// Re-solve every level's persisted constraints against its own geometry,
    /// so an anchored dimension snaps back onto the wall it measures. Returns
    /// how many dimension ends moved. Levels with no constraints are untouched.
    pub fn solve_constraints(&mut self) -> usize {
        self.levels
            .iter_mut()
            .map(|level| {
                // `apply_constraints_to_anchors` borrows the level mutably, so
                // the rules the level owns are read out first.
                let constraints = level.constraints.clone();
                apply_constraints_to_anchors(level, &constraints)
            })
            .sum()
    }

    /// Structural integrity check, used on load and before a save.
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.format_version == 0 || self.format_version > FORMAT_VERSION {
            return Err(CoreError::UnsupportedVersion {
                found: self.format_version,
                supported: FORMAT_VERSION,
            });
        }
        if self.name.trim().is_empty() {
            return Err(CoreError::InvalidProject("the project has no name".into()));
        }
        if self.levels.is_empty() {
            return Err(CoreError::InvalidProject(
                "the project has no levels".into(),
            ));
        }
        if !self.levels.iter().any(|l| l.id == self.active_level) {
            return Err(CoreError::InvalidProject(
                "the active level does not exist".into(),
            ));
        }

        let mut seen_levels = std::collections::HashSet::new();
        let mut seen_walls = std::collections::HashSet::new();
        let mut seen_openings = std::collections::HashSet::new();
        let mut seen_dimensions = std::collections::HashSet::new();
        let mut highest_id = 0_u64;

        for level in &self.levels {
            if !seen_levels.insert(level.id) {
                return Err(CoreError::InvalidProject(format!(
                    "{} appears twice",
                    level.id
                )));
            }
            highest_id = highest_id.max(level.id.value());

            for wall in &level.walls {
                if !seen_walls.insert(wall.id) {
                    return Err(CoreError::InvalidProject(format!(
                        "{} appears twice",
                        wall.id
                    )));
                }
                highest_id = highest_id.max(wall.id.value());
                if !wall.is_valid() {
                    return Err(CoreError::InvalidProject(format!(
                        "{} has impossible dimensions",
                        wall.id
                    )));
                }
            }

            for opening in &level.openings {
                if !seen_openings.insert(opening.id) {
                    return Err(CoreError::InvalidProject(format!(
                        "{} appears twice",
                        opening.id
                    )));
                }
                highest_id = highest_id.max(opening.id.value());
                let host = level.wall(opening.host).ok_or_else(|| {
                    CoreError::InvalidProject(format!(
                        "{} is hosted on {}, which is missing",
                        opening.id, opening.host
                    ))
                })?;
                if !opening.fits_on(host) {
                    return Err(CoreError::InvalidProject(format!(
                        "{} does not fit on {}",
                        opening.id, opening.host
                    )));
                }
            }

            for dimension in &level.dimensions {
                if !seen_dimensions.insert(dimension.id) {
                    return Err(CoreError::InvalidProject(format!(
                        "{} appears twice",
                        dimension.id
                    )));
                }
                highest_id = highest_id.max(dimension.id.value());
                if !dimension.is_valid() {
                    return Err(CoreError::InvalidProject(format!(
                        "{} has a zero length or a non-positive offset",
                        dimension.id
                    )));
                }
            }

            validate_constraints(level)?;

            // Openings on one wall may sit side by side with zero gap, but may
            // never overlap each other.
            for wall in &level.walls {
                let mut spans: Vec<(f64, f64, OpeningId)> = level
                    .openings_on(wall.id)
                    .map(|o| (o.start_offset(), o.end_offset(), o.id))
                    .collect();
                spans.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                for window in spans.windows(2) {
                    if window[0].1 > window[1].0 {
                        return Err(CoreError::InvalidProject(format!(
                            "{} and {} overlap on {}",
                            window[0].2, window[1].2, wall.id
                        )));
                    }
                }
            }
        }

        if self.next_id <= highest_id {
            return Err(CoreError::InvalidProject(
                "the identity counter is behind existing elements".into(),
            ));
        }

        self.title_block
            .validate()
            .map_err(|error| CoreError::InvalidProject(error.to_string()))?;

        for viewport in &self.viewports {
            if self.level(viewport.level).is_none() {
                return Err(CoreError::InvalidProject(format!(
                    "a viewport is bound to {}, which this project does not have",
                    viewport.level
                )));
            }
            if !viewport.is_valid() {
                return Err(CoreError::InvalidProject(
                    "a viewport has a non-positive scale or an empty region".into(),
                ));
            }
            if !viewport.is_within_sheet() {
                return Err(CoreError::InvalidProject(
                    "a viewport region lies outside the sheet".into(),
                ));
            }
        }

        Ok(())
    }
}

/// Check one level's persisted constraints: every rule must name geometry that
/// level actually has, its parameters must be usable, and no two rules of the
/// same kind may pin the same dimension.
fn validate_constraints(level: &Level) -> Result<(), CoreError> {
    crate::constraint::validate_constraints(level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraint::Constraint;
    use crate::dimension::{Dimension, DimensionKind};
    use crate::geom::{Bounds, Point2};
    use crate::ids::{DimensionId, WallId};
    use crate::opening::{Opening, OpeningKind};
    use crate::wall::Wall;

    fn project_with_wall() -> (Project, WallId) {
        let mut project = Project::new(UNTITLED, 1_760_000_000);
        let wall_id = WallId(project.allocate_id());
        project
            .active_level_mut()
            .expect("active level")
            .walls
            .push(Wall::new(
                wall_id,
                Point2::new(0.0, 0.0),
                Point2::new(4000.0, 0.0),
                200.0,
                2800.0,
            ));
        (project, wall_id)
    }

    fn project_with_wall_and_dimension() -> (Project, WallId, DimensionId) {
        let (mut project, wall_id) = project_with_wall();
        let dimension_id = DimensionId(project.allocate_id());
        project
            .active_level_mut()
            .expect("active level")
            .dimensions
            .push(Dimension::new(
                dimension_id,
                DimensionKind::LinearHorizontal,
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                500.0,
            ));
        (project, wall_id, dimension_id)
    }

    fn push_constraint(project: &mut Project, constraint: Constraint) {
        project
            .active_level_mut()
            .expect("active level")
            .constraints
            .push(constraint);
    }

    #[test]
    fn validation_accepts_a_well_formed_constraint_set() {
        let (mut project, wall_id, dimension_id) = project_with_wall_and_dimension();
        push_constraint(
            &mut project,
            Constraint::PerpendicularDistance {
                dimension: dimension_id,
                wall_id,
                param: 0.5,
            },
        );
        push_constraint(
            &mut project,
            Constraint::FixedAngle {
                dimension: dimension_id,
                degrees: 90.0,
            },
        );
        assert!(project.validate().is_ok());
    }

    #[test]
    fn validation_catches_a_dangling_constraint_wall() {
        let (mut project, _, dimension_id) = project_with_wall_and_dimension();
        push_constraint(
            &mut project,
            Constraint::PerpendicularDistance {
                dimension: dimension_id,
                wall_id: WallId(999),
                param: 0.5,
            },
        );
        let error = project.validate().expect_err("dangling wall");
        assert!(error.to_string().contains("which this level does not have"));
    }

    #[test]
    fn validation_catches_a_dangling_constraint_dimension() {
        let (mut project, wall_id, _) = project_with_wall_and_dimension();
        push_constraint(
            &mut project,
            Constraint::PerpendicularDistance {
                dimension: DimensionId(999),
                wall_id,
                param: 0.5,
            },
        );
        let error = project.validate().expect_err("dangling dimension");
        assert!(error.to_string().contains("which this level does not have"));
    }

    #[test]
    fn validation_catches_a_constraint_parameter_outside_its_wall() {
        let (mut project, wall_id, dimension_id) = project_with_wall_and_dimension();
        push_constraint(
            &mut project,
            Constraint::PerpendicularDistance {
                dimension: dimension_id,
                wall_id,
                param: 1.5,
            },
        );
        let error = project.validate().expect_err("param outside 0..1");
        assert!(error.to_string().contains("outside 0..1"));
    }

    #[test]
    fn validation_catches_a_duplicate_constraint_pin() {
        let (mut project, wall_id, dimension_id) = project_with_wall_and_dimension();
        for param in [0.25, 0.75] {
            push_constraint(
                &mut project,
                Constraint::PerpendicularDistance {
                    dimension: dimension_id,
                    wall_id,
                    param,
                },
            );
        }
        let error = project.validate().expect_err("duplicate pin");
        assert!(error.to_string().contains("more than one constraint"));
    }

    #[test]
    fn validation_catches_a_self_equal_length_constraint() {
        let (mut project, _, dimension_id) = project_with_wall_and_dimension();
        push_constraint(
            &mut project,
            Constraint::EqualLength {
                a: dimension_id,
                b: dimension_id,
            },
        );
        let error = project.validate().expect_err("self equal length");
        assert!(error.to_string().contains("equal to itself"));
    }

    #[test]
    fn a_new_project_is_valid_and_has_one_level() {
        let project = Project::new("Flat 3B", 1_760_000_000);
        assert_eq!(project.levels.len(), 1);
        assert_eq!(project.active_level, LevelId(1));
        assert!(project.validate().is_ok());
    }

    #[test]
    fn blank_names_fall_back_to_untitled() {
        let project = Project::new("   ", 1);
        assert_eq!(project.name, UNTITLED);
    }

    #[test]
    fn ids_are_allocated_monotonically() {
        let mut project = Project::new(UNTITLED, 1);
        let first = project.allocate_id();
        let second = project.allocate_id();
        assert_eq!(second, first + 1);
        assert!(project.next_id > second);
    }

    #[test]
    fn validation_catches_a_dangling_opening_host() {
        let (mut project, wall_id) = project_with_wall();
        let opening_id = OpeningId(project.allocate_id());
        project
            .active_level_mut()
            .expect("level")
            .openings
            .push(Opening::new(
                opening_id,
                OpeningKind::Door,
                WallId(wall_id.value() + 100),
                2000.0,
            ));
        let error = project.validate().expect_err("dangling host");
        assert!(error.to_string().contains("which is missing"));
    }

    #[test]
    fn validation_catches_overlapping_openings() {
        let (mut project, wall_id) = project_with_wall();
        let first = OpeningId(project.allocate_id());
        let second = OpeningId(project.allocate_id());
        {
            let level = project.active_level_mut().expect("level");
            level
                .openings
                .push(Opening::new(first, OpeningKind::Door, wall_id, 1500.0));
            level
                .openings
                .push(Opening::new(second, OpeningKind::Door, wall_id, 2000.0));
        }
        let error = project.validate().expect_err("overlap");
        assert!(error.to_string().contains("overlap"));
    }

    #[test]
    fn validation_rejects_a_stale_identity_counter() {
        let (mut project, _) = project_with_wall();
        project.next_id = 2;
        assert!(project.validate().is_err());
    }

    #[test]
    fn validation_refuses_a_newer_file_format() {
        let mut project = Project::new(UNTITLED, 1);
        project.format_version = FORMAT_VERSION + 1;
        assert!(matches!(
            project.validate(),
            Err(CoreError::UnsupportedVersion { .. })
        ));
    }

    #[test]
    fn a_valid_dimension_counts_and_passes_validation() {
        let mut project = Project::new(UNTITLED, 1);
        let id = DimensionId(project.allocate_id());
        project
            .active_level_mut()
            .expect("level")
            .dimensions
            .push(Dimension::new(
                id,
                DimensionKind::LinearHorizontal,
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                500.0,
            ));
        assert_eq!(project.dimension_count(), 1);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn validation_catches_a_zero_length_dimension() {
        let mut project = Project::new(UNTITLED, 1);
        let id = DimensionId(project.allocate_id());
        project
            .active_level_mut()
            .expect("level")
            .dimensions
            .push(Dimension::new(
                id,
                DimensionKind::Aligned,
                Point2::new(100.0, 100.0),
                Point2::new(100.0, 100.0),
                500.0,
            ));
        let error = project.validate().expect_err("zero length");
        assert!(error.to_string().contains("zero length"));
    }

    fn viewport_region(x0: f64, y0: f64, x1: f64, y1: f64) -> Bounds {
        Bounds {
            min: Point2::new(x0, y0),
            max: Point2::new(x1, y1),
        }
    }

    #[test]
    fn a_new_project_has_a_default_title_block_and_no_viewports() {
        let project = Project::new("Flat 3B", 1);
        assert_eq!(project.title_block, TitleBlock::default());
        assert!(project.viewports.is_empty());
    }

    #[test]
    fn a_valid_viewport_passes_validation() {
        let mut project = Project::new("Flat 3B", 1);
        let viewport = Viewport::new(
            LevelId(1),
            viewport_region(10.0, 40.0, 150.0, 140.0),
            50,
            "Ground floor".to_string(),
        );
        project.viewports.push(viewport);
        assert!(project.validate().is_ok());
    }

    #[test]
    fn validation_catches_a_viewport_bound_to_a_missing_level() {
        let mut project = Project::new("Flat 3B", 1);
        let viewport = Viewport::new(
            LevelId(999),
            viewport_region(10.0, 40.0, 150.0, 140.0),
            50,
            String::new(),
        );
        project.viewports.push(viewport);
        let error = project.validate().expect_err("dangling level");
        assert!(
            error.to_string().contains("which this project does not have")
        );
    }

    #[test]
    fn validation_catches_a_viewport_with_a_non_positive_scale() {
        let mut project = Project::new("Flat 3B", 1);
        let viewport = Viewport::new(
            LevelId(1),
            viewport_region(10.0, 40.0, 150.0, 140.0),
            0,
            String::new(),
        );
        project.viewports.push(viewport);
        let error = project.validate().expect_err("zero scale");
        assert!(error.to_string().contains("non-positive scale"));
    }

    #[test]
    fn validation_catches_a_viewport_region_off_the_sheet() {
        let mut project = Project::new("Flat 3B", 1);
        let viewport = Viewport::new(
            LevelId(1),
            viewport_region(0.0, 0.0, 5000.0, 5000.0),
            100,
            String::new(),
        );
        project.viewports.push(viewport);
        let error = project.validate().expect_err("region off the sheet");
        assert!(error.to_string().contains("outside the sheet"));
    }

    #[test]
    fn validation_catches_a_zero_sheet_number_on_the_title_block() {
        let mut project = Project::new("Flat 3B", 1);
        project.title_block.sheet_number = Some(0);
        let error = project.validate().expect_err("zero sheet number");
        assert!(error.to_string().contains("1 or more"));
    }

    #[test]
    fn projects_saved_before_title_blocks_and_viewports_still_load() {
        let json = r#"{
            "format_version": 1,
            "name": "Old plan",
            "units": "Metric",
            "levels": [{
                "id": 1,
                "name": "Level 0",
                "elevation": 0.0,
                "walls": [],
                "openings": []
            }],
            "active_level": 1,
            "created_epoch_s": 0,
            "modified_epoch_s": 0,
            "notes": "",
            "next_id": 2
        }"#;
        let project: Project = serde_json::from_str(json).expect("old project loads");
        assert_eq!(project.title_block, TitleBlock::default());
        assert!(project.viewports.is_empty());
        assert!(project.validate().is_ok());
    }

    #[test]
    fn validation_catches_a_non_positive_dimension_offset() {
        let mut project = Project::new(UNTITLED, 1);
        let id = DimensionId(project.allocate_id());
        project
            .active_level_mut()
            .expect("level")
            .dimensions
            .push(Dimension::new(
                id,
                DimensionKind::LinearHorizontal,
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                0.0,
            ));
        let error = project.validate().expect_err("zero offset");
        assert!(error.to_string().contains("non-positive offset"));
    }
}
