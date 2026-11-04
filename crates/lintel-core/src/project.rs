//! The project document: metadata plus a list of storeys.
//!
//! Everything the user creates lives here. The file is a plain, versioned
//! JSON document (`<name>.lintel.json`) so a user can read it, diff it, keep
//! it under version control, and reopen it in five years without this app.

use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::geom::Bounds;
use crate::ids::{LevelId, OpeningId};
use crate::level::Level;
use crate::units::UnitSystem;

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
            next_id: 2,
        }
    }

    /// Allocate a new identity. Ids are never reused within a project.
    pub fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point2;
    use crate::ids::WallId;
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
}
