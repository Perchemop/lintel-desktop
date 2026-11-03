//! Point snapping while drawing.
//!
//! Snapping is pure: give it the raw pointer position, the candidate anchors
//! (wall ends), the previous picked point and the settings, and it returns
//! where the point should actually land. The canvas draws the marker from the
//! returned `SnapKind`, so what the user sees is exactly what the model gets.

use crate::geom::Point2;
use crate::wall::Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapKind {
    /// Locked onto an existing wall end.
    Endpoint,
    /// Constrained to horizontal or vertical from the previous point.
    Ortho,
    /// Rounded to the drawing grid.
    Grid,
    /// Left exactly where the pointer is.
    Free,
}

impl SnapKind {
    pub fn label(self) -> &'static str {
        match self {
            SnapKind::Endpoint => "End",
            SnapKind::Ortho => "Ortho",
            SnapKind::Grid => "Grid",
            SnapKind::Free => "Free",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapSettings {
    /// Grid pitch in millimetres. Ignored when `grid_enabled` is false.
    pub grid: f64,
    pub grid_enabled: bool,
    /// Straight-line drawing on or off.
    pub ortho: bool,
    /// How close, in millimetres, a point must be to an anchor to lock on.
    /// The canvas scales this with zoom so it feels constant on screen.
    pub anchor_tolerance: f64,
}

impl Default for SnapSettings {
    fn default() -> Self {
        // 500 mm grid: the smallest bay a domestic plan is ever laid out on,
        // fine enough to be useful and coarse enough to stay legible.
        Self {
            grid: 500.0,
            grid_enabled: true,
            ortho: true,
            anchor_tolerance: 150.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapResult {
    pub point: Point2,
    pub kind: SnapKind,
}

/// Anchor points of a set of walls: both ends of each.
pub fn wall_anchors(walls: &[Wall]) -> Vec<Point2> {
    let mut anchors = Vec::with_capacity(walls.len() * 2);
    for wall in walls {
        anchors.push(wall.start);
        anchors.push(wall.end);
    }
    anchors
}

pub fn snap_point(
    raw: Point2,
    anchors: &[Point2],
    origin: Option<Point2>,
    settings: &SnapSettings,
) -> SnapResult {
    if !raw.is_finite() {
        return SnapResult {
            point: Point2::ORIGIN,
            kind: SnapKind::Free,
        };
    }

    // 1. An existing end always wins: reusing a corner is what keeps a plan
    //    connected instead of nearly connected.
    let mut nearest: Option<(f64, Point2)> = None;
    for anchor in anchors {
        let distance = raw.distance(*anchor);
        if distance <= settings.anchor_tolerance
            && nearest.is_none_or(|(best, _)| distance < best)
        {
            nearest = Some((distance, *anchor));
        }
    }
    if let Some((_, anchor)) = nearest {
        return SnapResult {
            point: anchor,
            kind: SnapKind::Endpoint,
        };
    }

    // 2. Straight from the last point, if asked for.
    let mut point = raw;
    let mut kind = SnapKind::Free;
    let mut horizontal = None;
    if settings.ortho {
        if let Some(origin) = origin {
            let horizontal_run = (raw.x - origin.x).abs() >= (raw.y - origin.y).abs();
            horizontal = Some(horizontal_run);
            if horizontal_run {
                point.y = origin.y;
            } else {
                point.x = origin.x;
            }
            kind = SnapKind::Ortho;
        }
    }

    // 3. Grid, without breaking the ortho constraint.
    if settings.grid_enabled && settings.grid.is_finite() && settings.grid > 0.0 {
        let grid = settings.grid;
        let rounded = Point2::new(
            (point.x / grid).round() * grid,
            (point.y / grid).round() * grid,
        );
        point = match (horizontal, origin) {
            (Some(true), Some(origin)) => Point2::new(rounded.x, origin.y),
            (Some(false), Some(origin)) => Point2::new(origin.x, rounded.y),
            _ => rounded,
        };
        if kind == SnapKind::Free {
            kind = SnapKind::Grid;
        }
    }

    SnapResult { point, kind }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wall::DEFAULT_WALL_THICKNESS;

    fn settings() -> SnapSettings {
        SnapSettings {
            grid: 500.0,
            grid_enabled: true,
            ortho: false,
            anchor_tolerance: 150.0,
        }
    }

    #[test]
    fn an_endpoint_within_tolerance_wins() {
        let anchors = vec![Point2::new(4000.0, 0.0)];
        let result = snap_point(Point2::new(4100.0, 50.0), &anchors, None, &settings());
        assert_eq!(result.kind, SnapKind::Endpoint);
        assert_eq!(result.point, Point2::new(4000.0, 0.0));
    }

    #[test]
    fn a_distant_endpoint_is_ignored() {
        let anchors = vec![Point2::new(4000.0, 0.0)];
        let mut s = settings();
        s.grid_enabled = false;
        let result = snap_point(Point2::new(2000.0, 0.0), &anchors, None, &s);
        assert_eq!(result.kind, SnapKind::Free);
        assert_eq!(result.point, Point2::new(2000.0, 0.0));
    }

    #[test]
    fn ortho_straightens_from_the_origin() {
        let mut s = settings();
        s.ortho = true;
        s.grid_enabled = false;
        let origin = Point2::new(0.0, 0.0);
        let result = snap_point(Point2::new(3000.0, 200.0), &[], Some(origin), &s);
        assert_eq!(result.kind, SnapKind::Ortho);
        assert_eq!(result.point, Point2::new(3000.0, 0.0));
        let result = snap_point(Point2::new(200.0, 3000.0), &[], Some(origin), &s);
        assert_eq!(result.point, Point2::new(0.0, 3000.0));
    }

    #[test]
    fn grid_rounds_when_nothing_else_applies() {
        let result = snap_point(Point2::new(1240.0, 380.0), &[], None, &settings());
        assert_eq!(result.kind, SnapKind::Grid);
        assert_eq!(result.point, Point2::new(1000.0, 500.0));
    }

    #[test]
    fn grid_never_pulls_a_point_off_the_ortho_line() {
        let mut s = settings();
        s.ortho = true;
        let origin = Point2::new(0.0, 0.0);
        let result = snap_point(Point2::new(1260.0, 120.0), &[], Some(origin), &s);
        assert_eq!(result.kind, SnapKind::Ortho);
        assert_eq!(result.point, Point2::new(1500.0, 0.0));
    }

    #[test]
    fn everything_off_leaves_the_pointer_alone() {
        let s = SnapSettings {
            grid: 0.0,
            grid_enabled: false,
            ortho: false,
            anchor_tolerance: 0.0,
        };
        let raw = Point2::new(1234.5, -678.9);
        let result = snap_point(raw, &[], None, &s);
        assert_eq!(result.kind, SnapKind::Free);
        assert_eq!(result.point, raw);
    }

    #[test]
    fn non_finite_input_degrades_to_the_origin() {
        let result = snap_point(Point2::new(f64::NAN, 0.0), &[], None, &settings());
        assert_eq!(result.kind, SnapKind::Free);
        assert_eq!(result.point, Point2::ORIGIN);
    }

    #[test]
    fn anchors_come_from_wall_ends() {
        let walls = vec![Wall::new(
            crate::ids::WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 0.0),
            DEFAULT_WALL_THICKNESS,
            2800.0,
        )];
        let anchors = wall_anchors(&walls);
        assert_eq!(anchors.len(), 2);
        assert!(anchors.contains(&Point2::new(0.0, 0.0)));
        assert!(anchors.contains(&Point2::new(4000.0, 0.0)));
    }
}
