//! Measurement queries over the project.
//!
//! All geometry is stored in millimetres; these queries return raw values in
//! the model coordinate system. Formatting to Metric/Imperial is a UI concern.

use serde::{Deserialize, Serialize};

use crate::ids::LevelId;
use crate::project::Project;
use crate::room::detect_rooms;

/// Totals for a single level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LevelTotals {
    /// Total length of all walls on the level, in millimetres.
    pub wall_length_mm: f64,
    /// Total width of all openings on the level, in millimetres.
    pub opening_length_mm: f64,
    /// Number of enclosed rooms detected on the level.
    pub room_count: usize,
    /// Sum of all room areas on the level, in square millimetres.
    pub total_room_area_mm2: f64,
    /// Sum of all room perimeters on the level, in millimetres.
    pub total_room_perimeter_mm: f64,
}

impl LevelTotals {
    pub fn new() -> Self {
        Self {
            wall_length_mm: 0.0,
            opening_length_mm: 0.0,
            room_count: 0,
            total_room_area_mm2: 0.0,
            total_room_perimeter_mm: 0.0,
        }
    }
}

/// Totals aggregated over the entire project.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProjectTotals {
    /// Total length of all walls across all levels, in millimetres.
    pub wall_length_mm: f64,
    /// Total width of all openings across all levels, in millimetres.
    pub opening_length_mm: f64,
    /// Total number of enclosed rooms across all levels.
    pub room_count: usize,
    /// Sum of all room areas across all levels, in square millimetres.
    pub total_room_area_mm2: f64,
    /// Sum of all room perimeters across all levels, in millimetres.
    pub total_room_perimeter_mm: f64,
}

impl ProjectTotals {
    pub fn new() -> Self {
        Self {
            wall_length_mm: 0.0,
            opening_length_mm: 0.0,
            room_count: 0,
            total_room_area_mm2: 0.0,
            total_room_perimeter_mm: 0.0,
        }
    }
}

/// Headless queries for measurement data.
///
/// The model is always millimetres. Formatting for display is handled by the
/// [`crate::units`] module in the UI shell.
pub struct Measurements<'a> {
    project: &'a Project,
}

impl<'a> Measurements<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    /// Area of a detected room on a level, in square millimetres.
    ///
    /// Rooms are detected dynamically via [`detect_rooms`]; their ordering is
    /// deterministic for a given set of walls. `room_index` is zero-based within
    /// the detected rooms for the specified level.
    pub fn room_area_mm2(&self, level_id: LevelId, room_index: usize) -> Option<f64> {
        let level = self.project.level(level_id)?;
        let rooms = detect_rooms(&level.walls);
        rooms.get(room_index).map(|room| room.area_mm2)
    }

    /// Perimeter of a detected room on a level, in millimetres.
    ///
    /// `room_index` is zero-based within the detected rooms for the specified
    /// level.
    pub fn room_perimeter_mm(&self, level_id: LevelId, room_index: usize) -> Option<f64> {
        let level = self.project.level(level_id)?;
        let rooms = detect_rooms(&level.walls);
        rooms.get(room_index).map(|room| room.perimeter_mm)
    }

    /// Sum of the centreline lengths of all walls on a level, in millimetres.
    pub fn level_wall_length_mm(&self, level_id: LevelId) -> f64 {
        let level = match self.project.level(level_id) {
            Some(l) => l,
            None => return 0.0,
        };
        level.walls.iter().map(|w| w.length()).sum()
    }

    /// Sum of the widths of all openings on a level, in millimetres.
    pub fn level_opening_length_mm(&self, level_id: LevelId) -> f64 {
        let level = match self.project.level(level_id) {
            Some(l) => l,
            None => return 0.0,
        };
        level.openings.iter().map(|o| o.width).sum()
    }

    /// Compute aggregate statistics for a single level.
    pub fn level_totals(&self, level_id: LevelId) -> LevelTotals {
        let Some(level) = self.project.level(level_id) else {
            return LevelTotals::new();
        };

        let wall_length_mm: f64 = level.walls.iter().map(|w| w.length()).sum();
        let opening_length_mm: f64 = level.openings.iter().map(|o| o.width).sum();

        let rooms = detect_rooms(&level.walls);
        let room_count = rooms.len();
        let total_room_area_mm2: f64 = rooms.iter().map(|r| r.area_mm2).sum();
        let total_room_perimeter_mm: f64 = rooms.iter().map(|r| r.perimeter_mm).sum();

        LevelTotals {
            wall_length_mm,
            opening_length_mm,
            room_count,
            total_room_area_mm2,
            total_room_perimeter_mm,
        }
    }

    /// Compute aggregate statistics for the entire project.
    pub fn project_totals(&self) -> ProjectTotals {
        let mut totals = ProjectTotals::new();
        for level in &self.project.levels {
            let level_totals = self.level_totals(level.id);
            totals.wall_length_mm += level_totals.wall_length_mm;
            totals.opening_length_mm += level_totals.opening_length_mm;
            totals.room_count += level_totals.room_count;
            totals.total_room_area_mm2 += level_totals.total_room_area_mm2;
            totals.total_room_perimeter_mm += level_totals.total_room_perimeter_mm;
        }
        totals
    }

    /// Convenience query for the active level's wall length.
    pub fn active_level_wall_length_mm(&self) -> f64 {
        self.level_wall_length_mm(self.project.active_level)
    }

    /// Convenience query for the active level's opening length.
    pub fn active_level_opening_length_mm(&self) -> f64 {
        self.level_opening_length_mm(self.project.active_level)
    }

    /// Convenience query for the active level's room count.
    pub fn active_level_room_count(&self) -> usize {
        let Some(level) = self.project.level(self.project.active_level) else {
            return 0;
        };
        detect_rooms(&level.walls).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point2;
    use crate::ids::LevelId;
    use crate::project::Project;

    fn project_with_rectangle() -> Project {
        let mut project = Project::new("Test", 1_760_000_000);
        let level_id = LevelId(1);
        let level = project.level_mut(level_id).expect("level");

        // 4000 mm x 4000 mm square
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 0.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(2),
            Point2::new(4000.0, 0.0),
            Point2::new(4000.0, 4000.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(3),
            Point2::new(4000.0, 4000.0),
            Point2::new(0.0, 4000.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(4),
            Point2::new(0.0, 4000.0),
            Point2::new(0.0, 0.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));

        project
    }

    #[test]
    fn room_area_is_detected_correctly() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let area = measurements
            .room_area_mm2(LevelId(1), 0)
            .expect("room exists");
        assert!((area - 16_000_000.0).abs() < 1.0);
    }

    #[test]
    fn room_perimeter_is_detected_correctly() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let perimeter = measurements
            .room_perimeter_mm(LevelId(1), 0)
            .expect("room exists");
        assert!((perimeter - 16_000.0).abs() < 1.0);
    }

    #[test]
    fn level_wall_length_sums_all_walls() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let length = measurements.level_wall_length_mm(LevelId(1));
        assert!((length - 16_000.0).abs() < 1.0);
    }

    #[test]
    fn level_opening_length_is_zero_without_openings() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let length = measurements.level_opening_length_mm(LevelId(1));
        assert_eq!(length, 0.0);
    }

    #[test]
    fn level_totals_aggregate_correctly() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let totals = measurements.level_totals(LevelId(1));
        assert!((totals.wall_length_mm - 16_000.0).abs() < 1.0);
        assert_eq!(totals.opening_length_mm, 0.0);
        assert_eq!(totals.room_count, 1);
        assert!((totals.total_room_area_mm2 - 16_000_000.0).abs() < 1.0);
        assert!((totals.total_room_perimeter_mm - 16_000.0).abs() < 1.0);
    }

    #[test]
    fn project_totals_aggregate_across_levels() {
        let mut project = project_with_rectangle();
        // Add a second level with the same walls
        let new_level_id = LevelId(2);
        project.levels.push(crate::level::Level::new(new_level_id, "Level 1", 3000.0));
        let level = project.level_mut(new_level_id).expect("new level");
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(5),
            Point2::new(0.0, 0.0),
            Point2::new(2000.0, 0.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(6),
            Point2::new(2000.0, 0.0),
            Point2::new(2000.0, 2000.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(7),
            Point2::new(2000.0, 2000.0),
            Point2::new(0.0, 2000.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));
        level.walls.push(crate::wall::Wall::new(
            crate::ids::WallId(8),
            Point2::new(0.0, 2000.0),
            Point2::new(0.0, 0.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        ));

        let measurements = Measurements::new(&project);
        let totals = measurements.project_totals();
        // Level 1: 16,000 mm walls + 16,000,000 mm² area
        // Level 2: 8,000 mm walls + 4,000,000 mm² area
        assert!((totals.wall_length_mm - 24_000.0).abs() < 1.0);
        assert_eq!(totals.room_count, 2);
        assert!((totals.total_room_area_mm2 - 20_000_000.0).abs() < 1.0);
    }

    #[test]
    fn missing_level_returns_none_or_zero() {
        let project = Project::new("Empty", 1);
        let measurements = Measurements::new(&project);
        assert_eq!(measurements.room_area_mm2(LevelId(999), 0), None);
        assert_eq!(measurements.level_wall_length_mm(LevelId(999)), 0.0);
        assert_eq!(measurements.level_totals(LevelId(999)), LevelTotals::new());
    }

    #[test]
    fn room_index_out_of_range_returns_none() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        assert_eq!(measurements.room_area_mm2(LevelId(1), 99), None);
        assert_eq!(measurements.room_perimeter_mm(LevelId(1), 99), None);
    }

    #[test]
    fn empty_project_has_zero_totals() {
        let project = Project::new("Empty", 1);
        let measurements = Measurements::new(&project);
        let totals = measurements.project_totals();
        assert_eq!(totals.wall_length_mm, 0.0);
        assert_eq!(totals.opening_length_mm, 0.0);
        assert_eq!(totals.room_count, 0);
        assert_eq!(totals.total_room_area_mm2, 0.0);
        assert_eq!(totals.total_room_perimeter_mm, 0.0);
    }

    #[test]
    fn level_totals_round_trips_through_json() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let original = measurements.level_totals(LevelId(1));
        let json = serde_json::to_string(&original).expect("serialize");
        let back: LevelTotals = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn project_totals_round_trips_through_json() {
        let project = project_with_rectangle();
        let measurements = Measurements::new(&project);
        let original = measurements.project_totals();
        let json = serde_json::to_string(&original).expect("serialize");
        let back: ProjectTotals = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }
}
