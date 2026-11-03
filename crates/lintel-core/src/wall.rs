//! A wall: a straight centreline with a thickness and a height.
//!
//! Walls are the only thing the drawing is built from at v0.1.0. Openings are
//! hosted on them (see [`crate::opening`]); joints between walls are resolved
//! at draw time (see [`crate::join`]) rather than stored, so moving a wall
//! keeps every junction consistent.

use serde::{Deserialize, Serialize};

use crate::geom::{Point2, Vec2};
use crate::ids::WallId;

/// Shortest wall the editor will create. Below this the two halves of the
/// thickness overlap and the result is not a useful drawing element.
pub const MIN_WALL_LENGTH: f64 = 10.0;
pub const MIN_WALL_THICKNESS: f64 = 20.0;
pub const MAX_WALL_THICKNESS: f64 = 2000.0;
pub const MIN_WALL_HEIGHT: f64 = 300.0;
pub const MAX_WALL_HEIGHT: f64 = 20000.0;

/// Defaults chosen to match common European practice: a 200 mm internal wall
/// at 2.8 m storey height.
pub const DEFAULT_WALL_THICKNESS: f64 = 200.0;
pub const DEFAULT_WALL_HEIGHT: f64 = 2800.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wall {
    pub id: WallId,
    pub start: Point2,
    pub end: Point2,
    /// Full thickness in millimetres, measured across the centreline.
    pub thickness: f64,
    /// Storey height in millimetres; used by the 3D model in later milestones.
    pub height: f64,
}

impl Wall {
    pub fn new(id: WallId, start: Point2, end: Point2, thickness: f64, height: f64) -> Self {
        Self {
            id,
            start,
            end,
            thickness,
            height,
        }
    }

    pub fn length(&self) -> f64 {
        self.start.distance(self.end)
    }

    pub fn midpoint(&self) -> Point2 {
        Point2::new(
            (self.start.x + self.end.x) * 0.5,
            (self.start.y + self.end.y) * 0.5,
        )
    }

    /// Unit direction from start to end. `ZERO` for a degenerate wall.
    pub fn direction(&self) -> Vec2 {
        (self.end - self.start).normalized()
    }

    /// Point at `offset` millimetres from the start, along the centreline.
    pub fn point_at(&self, offset: f64) -> Point2 {
        self.start + self.direction() * offset
    }

    /// The four corners of the wall band, anticlockwise from the start face.
    /// `None` for a degenerate wall (no meaningful face).
    pub fn footprint(&self) -> Option<[Point2; 4]> {
        let dir = self.direction();
        if dir == Vec2::ZERO || !self.thickness.is_finite() || self.thickness <= 0.0 {
            return None;
        }
        let offset = dir.perpendicular() * (self.thickness * 0.5);
        Some([
            self.start + offset,
            self.end + offset,
            self.end - offset,
            self.start - offset,
        ])
    }

    /// Distance from a point to the centreline (not the extent).
    pub fn distance_to_centreline(&self, p: Point2) -> f64 {
        crate::geom::point_segment_distance(p, self.start, self.end)
    }

    /// True when the geometry is finite and long enough to draw.
    pub fn is_valid(&self) -> bool {
        self.start.is_finite()
            && self.end.is_finite()
            && self.length() >= MIN_WALL_LENGTH
            && self.thickness.is_finite()
            && (MIN_WALL_THICKNESS..=MAX_WALL_THICKNESS).contains(&self.thickness)
            && self.height.is_finite()
            && (MIN_WALL_HEIGHT..=MAX_WALL_HEIGHT).contains(&self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wall(start: (f64, f64), end: (f64, f64)) -> Wall {
        Wall::new(
            WallId(1),
            Point2::new(start.0, start.1),
            Point2::new(end.0, end.1),
            DEFAULT_WALL_THICKNESS,
            DEFAULT_WALL_HEIGHT,
        )
    }

    #[test]
    fn length_uses_full_precision() {
        let w = wall((0.0, 0.0), (3000.0, 4000.0));
        assert!((w.length() - 5000.0).abs() < 1e-9);
    }

    #[test]
    fn point_at_walks_the_centreline() {
        let w = wall((0.0, 0.0), (1000.0, 0.0));
        assert_eq!(w.point_at(0.0), Point2::new(0.0, 0.0));
        assert_eq!(w.point_at(500.0), Point2::new(500.0, 0.0));
        assert_eq!(w.point_at(1000.0), Point2::new(1000.0, 0.0));
    }

    #[test]
    fn footprint_is_a_band_with_the_requested_thickness() {
        let w = wall((0.0, 0.0), (2000.0, 0.0));
        let corners = w.footprint().expect("footprint");
        // width across the band equals thickness
        let across = corners[0].distance(corners[3]);
        assert!((across - DEFAULT_WALL_THICKNESS).abs() < 1e-9);
        // band length equals the wall length
        let along = corners[0].distance(corners[1]);
        assert!((along - 2000.0).abs() < 1e-9);
    }

    #[test]
    fn degenerate_walls_have_no_footprint() {
        let w = wall((100.0, 100.0), (100.0, 100.0));
        assert!(w.footprint().is_none());
        assert!(!w.is_valid());
    }

    #[test]
    fn validity_rejects_absurd_dimensions() {
        let mut w = wall((0.0, 0.0), (2000.0, 0.0));
        w.thickness = 5000.0;
        assert!(!w.is_valid());
        w.thickness = DEFAULT_WALL_THICKNESS;
        w.height = 0.0;
        assert!(!w.is_valid());
    }

    #[test]
    fn distance_to_centreline_is_perpendicular() {
        let w = wall((0.0, 0.0), (3000.0, 0.0));
        assert!((w.distance_to_centreline(Point2::new(1500.0, 400.0)) - 400.0).abs() < 1e-9);
    }
}
