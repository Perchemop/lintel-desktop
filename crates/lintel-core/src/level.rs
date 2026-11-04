//! A storey: a named elevation holding its own walls, openings and dimensions.
//!
//! v0.1.0 edits one storey at a time, but the file format already holds a
//! list, so multi-storey work is a UI milestone rather than a format change.

use serde::{Deserialize, Serialize};

use crate::dimension::Dimension;
use crate::geom::Bounds;
use crate::ids::{DimensionId, LevelId, OpeningId, WallId};
use crate::opening::Opening;
use crate::wall::Wall;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub id: LevelId,
    pub name: String,
    /// Floor level in millimetres above the project datum.
    pub elevation: f64,
    pub walls: Vec<Wall>,
    pub openings: Vec<Opening>,
    /// Annotation drawn over the plan. `serde(default)` keeps files written
    /// before dimensions existed readable.
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
}

impl Level {
    pub fn new(id: LevelId, name: impl Into<String>, elevation: f64) -> Self {
        Self {
            id,
            name: name.into(),
            elevation,
            walls: Vec::new(),
            openings: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn wall(&self, id: WallId) -> Option<&Wall> {
        self.walls.iter().find(|w| w.id == id)
    }

    pub fn wall_mut(&mut self, id: WallId) -> Option<&mut Wall> {
        self.walls.iter_mut().find(|w| w.id == id)
    }

    pub fn opening(&self, id: OpeningId) -> Option<&Opening> {
        self.openings.iter().find(|o| o.id == id)
    }

    pub fn opening_mut(&mut self, id: OpeningId) -> Option<&mut Opening> {
        self.openings.iter_mut().find(|o| o.id == id)
    }

    pub fn dimension(&self, id: DimensionId) -> Option<&Dimension> {
        self.dimensions.iter().find(|d| d.id == id)
    }

    pub fn dimension_mut(&mut self, id: DimensionId) -> Option<&mut Dimension> {
        self.dimensions.iter_mut().find(|d| d.id == id)
    }

    pub fn openings_on(&self, wall: WallId) -> impl Iterator<Item = &Opening> {
        self.openings.iter().filter(move |o| o.host == wall)
    }

    pub fn wall_count(&self) -> usize {
        self.walls.len()
    }

    pub fn opening_count(&self) -> usize {
        self.openings.len()
    }

    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }

    /// Bounds of every wall band on this level, for zoom-to-fit and the sheet
    /// layout. `None` when the level is empty.
    pub fn extent(&self) -> Option<Bounds> {
        let mut bounds: Option<Bounds> = None;
        for wall in &self.walls {
            if let Some(corners) = wall.footprint() {
                if let Some(wall_bounds) = Bounds::from_points(&corners) {
                    match &mut bounds {
                        Some(existing) => existing.include(wall_bounds),
                        None => bounds = Some(wall_bounds),
                    }
                }
            }
        }
        bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::{Dimension, DimensionKind};
    use crate::geom::Point2;

    #[test]
    fn empty_level_has_no_extent() {
        let level = Level::new(LevelId(1), "Level 0", 0.0);
        assert!(level.extent().is_none());
        assert_eq!(level.wall_count(), 0);
        assert_eq!(level.dimension_count(), 0);
    }

    #[test]
    fn extent_covers_the_wall_bands_not_just_centrelines() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(2000.0, 0.0),
            200.0,
            2800.0,
        ));
        let bounds = level.extent().expect("extent");
        assert_eq!(bounds.width(), 2000.0);
        assert_eq!(bounds.height(), 200.0);
        assert_eq!(bounds.min.y, -100.0);
        assert_eq!(bounds.max.y, 100.0);
    }

    #[test]
    fn openings_are_found_by_host() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.openings.push(Opening::new(
            OpeningId(1),
            crate::opening::OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        level.openings.push(Opening::new(
            OpeningId(2),
            crate::opening::OpeningKind::Window,
            WallId(2),
            2000.0,
        ));
        let on_first: Vec<_> = level.openings_on(WallId(1)).collect();
        assert_eq!(on_first.len(), 1);
        assert_eq!(on_first[0].id, OpeningId(1));
    }

    #[test]
    fn dimensions_are_found_by_id() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.dimensions.push(Dimension::new(
            DimensionId(7),
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        assert_eq!(level.dimension_count(), 1);
        let dimension = level.dimension(DimensionId(7)).expect("dimension");
        assert_eq!(dimension.kind, DimensionKind::LinearHorizontal);
        level
            .dimension_mut(DimensionId(7))
            .expect("dimension")
            .offset = 700.0;
        assert_eq!(
            level.dimension(DimensionId(7)).expect("dimension").offset,
            700.0
        );
        assert!(level.dimension(DimensionId(8)).is_none());
    }

    #[test]
    fn levels_saved_before_dimensions_still_load() {
        let json = r#"{
            "id": 1,
            "name": "Level 0",
            "elevation": 0.0,
            "walls": [],
            "openings": []
        }"#;
        let level: Level = serde_json::from_str(json).expect("old level");
        assert!(level.dimensions.is_empty());
    }
}
