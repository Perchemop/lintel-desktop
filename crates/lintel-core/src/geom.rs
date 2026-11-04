//! Two-dimensional geometry in millimetres.
//!
//! World coordinates are millimetres with **y pointing up** (the drafting
//! convention). The UI is the only place that flips y for screen space.

use serde::{Deserialize, Serialize};

/// A point in the drawing plane, in millimetres.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub const ORIGIN: Point2 = Point2 { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Point2) -> f64 {
        (self - other).length()
    }

    pub fn distance_sq(self, other: Point2) -> f64 {
        (self - other).length_sq()
    }

    /// Translate by a vector.
    pub fn translated(self, v: Vec2) -> Point2 {
        Point2::new(self.x + v.x, self.y + v.y)
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Scale this point by a factor (multiplies both coordinates).
    pub fn scaled(self, factor: f64) -> Point2 {
        Point2::new(self.x * factor, self.y * factor)
    }
}

impl core::ops::Sub for Point2 {
    type Output = Vec2;

    fn sub(self, rhs: Point2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl core::ops::Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Vec2) -> Point2 {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl core::ops::Sub<Vec2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Vec2) -> Point2 {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// A displacement in the drawing plane, in millimetres.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length_sq(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Left-hand normal: rotate +90°.
    pub fn perpendicular(self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }

    /// Unit vector, or `ZERO` for a degenerate vector.
    pub fn normalized(self) -> Vec2 {
        let len = self.length();
        if len > 0.0 && len.is_finite() {
            Vec2::new(self.x / len, self.y / len)
        } else {
            Vec2::ZERO
        }
    }

    pub fn scaled(self, factor: f64) -> Vec2 {
        Vec2::new(self.x * factor, self.y * factor)
    }

    /// Angle in degrees, measured anticlockwise from +x, in `0..360`.
    pub fn angle_degrees(self) -> f64 {
        let deg = self.y.atan2(self.x).to_degrees();
        if deg < 0.0 { deg + 360.0 } else { deg }
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl core::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl core::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl core::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        self.scaled(rhs)
    }
}

/// Axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Point2,
    pub max: Point2,
}

impl Bounds {
    pub fn from_points(points: &[Point2]) -> Option<Self> {
        let finite: Vec<Point2> = points.iter().copied().filter(|p| p.is_finite()).collect();
        let first = *finite.first()?;
        let mut bounds = Bounds {
            min: first,
            max: first,
        };
        for &p in &finite[1..] {
            bounds.expand(p);
        }
        Some(bounds)
    }

    pub fn expand(&mut self, p: Point2) {
        if !p.is_finite() {
            return;
        }
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
    }

    pub fn include(&mut self, other: Bounds) {
        self.expand(other.min);
        self.expand(other.max);
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn centre(&self) -> Point2 {
        Point2::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
        )
    }

    pub fn contains(&self, p: Point2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Grow the box by `margin` on every side.
    pub fn padded(&self, margin: f64) -> Bounds {
        Bounds {
            min: Point2::new(self.min.x - margin, self.min.y - margin),
            max: Point2::new(self.max.x + margin, self.max.y + margin),
        }
    }
}

/// Closest point on the segment `a..b` to `p`, and the parameter `t` in
/// `0..=1` locating it.
pub fn closest_point_on_segment(p: Point2, a: Point2, b: Point2) -> (f64, Point2) {
    let ab = b - a;
    let len_sq = ab.length_sq();
    if len_sq <= f64::EPSILON {
        return (0.0, a);
    }
    let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    (t, a + ab * t)
}

/// Shortest distance from a point to a segment, in millimetres.
pub fn point_segment_distance(p: Point2, a: Point2, b: Point2) -> f64 {
    let (_, closest) = closest_point_on_segment(p, a, b);
    p.distance(closest)
}

/// Approximate a circular arc as a polyline. Angles are in degrees.
pub fn arc_points(
    centre: Point2,
    radius: f64,
    start_degrees: f64,
    end_degrees: f64,
    segments: usize,
) -> Vec<Point2> {
    let segments = segments.max(2);
    let start = start_degrees.to_radians();
    let end = end_degrees.to_radians();
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let t = i as f64 / segments as f64;
        let angle = start + (end - start) * t;
        points.push(Point2::new(
            centre.x + radius * angle.cos(),
            centre.y + radius * angle.sin(),
        ));
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_is_symmetric_and_zero_for_identical_points() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(3000.0, 4000.0);
        assert!((a.distance(b) - 5000.0).abs() < 1e-9);
        assert!(a.distance(a) == 0.0);
    }

    #[test]
    fn closest_point_clamps_to_segment_ends() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1000.0, 0.0);
        let (t, p) = closest_point_on_segment(Point2::new(-500.0, 250.0), a, b);
        assert_eq!(t, 0.0);
        assert_eq!(p, a);
    }

    #[test]
    fn point_segment_distance_uses_perpendicular_projection() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1000.0, 0.0);
        assert!((point_segment_distance(Point2::new(500.0, 300.0), a, b) - 300.0).abs() < 1e-9);
    }

    #[test]
    fn bounds_cover_all_finite_points() {
        let b = Bounds::from_points(&[
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 3000.0),
            Point2::new(-1000.0, 500.0),
        ])
        .expect("bounds");
        assert_eq!(b.width(), 5000.0);
        assert_eq!(b.height(), 3000.0);
        assert!(b.contains(Point2::new(2000.0, 1000.0)));
        assert!(!b.contains(Point2::new(6000.0, 1000.0)));
    }

    #[test]
    fn normalized_handles_degenerate_vectors() {
        assert_eq!(Vec2::ZERO.normalized(), Vec2::ZERO);
        let v = Vec2::new(3.0, 4.0).normalized();
        assert!((v.length() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn arc_points_span_the_requested_sweep() {
        let points = arc_points(Point2::new(0.0, 0.0), 1000.0, 0.0, 90.0, 4);
        assert_eq!(points.len(), 5);
        assert!((points[0].distance(Point2::new(1000.0, 0.0))).abs() < 1e-9);
        assert!((points[4].distance(Point2::new(0.0, 1000.0))).abs() < 1e-9);
    }

    impl From<Point2> for Vec2 {
        fn from(p: Point2) -> Vec2 {
            Vec2::new(p.x, p.y)
        }
    }
}
