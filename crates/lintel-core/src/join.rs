//! Drawing geometry derived from the model.
//!
//! Nothing in here is stored: wall bands, joint extensions and door swings are
//! computed from walls and openings every time they are needed. Moving one
//! wall therefore cannot leave a stale junction behind.
//!
//! **Known v0.1.0 limitation:** a joint is filled by extending each wall end by
//! half the *joining* wall's thickness. For equal thicknesses this is exact;
//! for very unequal thicknesses the outside corner can show a small step. The
//! limitation is documented in `docs/architecture.md` and scheduled for the
//! joint-resolution milestone.

use crate::dimension::Dimension;
use crate::geom::{Bounds, Point2, Vec2, point_segment_distance};
use crate::ids::{DimensionId, OpeningId, WallId};
use crate::level::Level;
use crate::opening::{Opening, OpeningKind};
use crate::wall::Wall;

/// Two wall ends closer than this are treated as joined, in millimetres.
pub const JOIN_TOLERANCE: f64 = 1.0;

/// Tolerance used when a click should find a wall, on top of half its
/// thickness, in millimetres.
pub const HIT_TOLERANCE: f64 = 40.0;

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedWall {
    pub wall: Wall,
    /// How far the centreline was extended past each original end to fill a
    /// joint, in millimetres. Zero at a free end.
    pub start_extension: f64,
    pub end_extension: f64,
    /// The wall band, anticlockwise.
    pub corners: [Point2; 4],
}

impl ResolvedWall {
    pub fn id(&self) -> WallId {
        self.wall.id
    }

    pub fn extended_start(&self) -> Point2 {
        self.wall.start - self.wall.direction() * self.start_extension
    }

    pub fn extended_end(&self) -> Point2 {
        self.wall.end + self.wall.direction() * self.end_extension
    }
}

/// A door leaf shown open at 90°, drawn as one line and one arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swing {
    pub hinge: Point2,
    pub leaf_end: Point2,
    pub radius: f64,
    /// Degrees, anticlockwise, of the closed leaf position.
    pub from_degrees: f64,
    /// Degrees, anticlockwise, of the open leaf position.
    pub to_degrees: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedOpening {
    pub id: OpeningId,
    pub kind: OpeningKind,
    pub host: WallId,
    /// Jamb positions on the host centreline.
    pub start: Point2,
    pub end: Point2,
    pub centre: Point2,
    pub width: f64,
    pub height: f64,
    pub sill: f64,
    pub flipped: bool,
    /// Direction of the host wall, for drawing the window reveal.
    pub host_direction: Vec2,
    pub host_thickness: f64,
    /// Doors only.
    pub swing: Option<Swing>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hit {
    Wall(WallId),
    Opening(OpeningId),
    Dimension(DimensionId),
}

/// Everything needed to paint one level, in world millimetres.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlanGeometry {
    pub walls: Vec<ResolvedWall>,
    pub openings: Vec<ResolvedOpening>,
    pub dimensions: Vec<Dimension>,
}

impl PlanGeometry {
    pub fn of_level(level: &Level) -> Self {
        let walls: Vec<ResolvedWall> = level
            .walls
            .iter()
            .map(|w| resolve_wall(w, &level.walls))
            .collect();
        let openings: Vec<ResolvedOpening> = level
            .openings
            .iter()
            .filter_map(|o| resolve_opening(level, o))
            .collect();
        Self {
            walls,
            openings,
            dimensions: level.dimensions.clone(),
        }
    }

    pub fn bounds(&self) -> Option<Bounds> {
        let points: Vec<Point2> = self.walls.iter().flat_map(|w| w.corners).collect();
        Bounds::from_points(&points)
    }

    pub fn wall(&self, id: WallId) -> Option<&ResolvedWall> {
        self.walls.iter().find(|w| w.wall.id == id)
    }

    pub fn thickness_of(&self, id: WallId) -> f64 {
        self.wall(id).map_or(200.0, |w| w.wall.thickness)
    }

    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    /// What is under the pointer. Doors and windows win over their host wall,
    /// because they are drawn on top of it and are harder to hit.
    pub fn hit_test(&self, point: Point2, tolerance: f64) -> Option<Hit> {
        if let Some(opening) = self.openings.iter().find(|o| {
            point_segment_distance(point, o.start, o.end) <= o.host_thickness * 0.5 + tolerance
        }) {
            return Some(Hit::Opening(opening.id));
        }
        let mut best: Option<(f64, WallId)> = None;
        for wall in &self.walls {
            let distance = point_segment_distance(point, wall.wall.start, wall.wall.end);
            if distance <= wall.wall.thickness * 0.5 + tolerance {
                let better = best.is_none_or(|(best_distance, _)| distance < best_distance);
                if better {
                    best = Some((distance, wall.wall.id));
                }
            }
        }
        // Dimensions: hit test against their dimension line.
        for dim in &self.dimensions {
            if let Some((a, b)) = dim.dimension_line() {
                if point_segment_distance(point, a, b) <= tolerance {
                    return Some(Hit::Dimension(dim.id));
                }
            }
        }
        best.map(|(_, id)| Hit::Wall(id))
    }
}

/// Resolve one wall's band, extending joined ends.
pub fn resolve_wall(wall: &Wall, others: &[Wall]) -> ResolvedWall {
    let start_extension = joining_thickness(wall, wall.start, others);
    let end_extension = joining_thickness(wall, wall.end, others);
    let direction = wall.direction();
    let start = wall.start - direction * start_extension;
    let end = wall.end + direction * end_extension;
    let normal = direction.perpendicular() * (wall.thickness * 0.5);
    ResolvedWall {
        wall: wall.clone(),
        start_extension,
        end_extension,
        corners: [start + normal, end + normal, end - normal, start - normal],
    }
}

/// Half the thickest wall that meets `point` — either because its own end is
/// there, or because its centreline passes through it (a T junction).
fn joining_thickness(wall: &Wall, point: Point2, others: &[Wall]) -> f64 {
    others
        .iter()
        .filter(|other| other.id != wall.id)
        .filter(|other| {
            other.start.distance(point) <= JOIN_TOLERANCE
                || other.end.distance(point) <= JOIN_TOLERANCE
                || point_segment_distance(point, other.start, other.end) <= JOIN_TOLERANCE
        })
        .map(|other| other.thickness * 0.5)
        .fold(0.0, f64::max)
}

fn resolve_opening(level: &Level, opening: &Opening) -> Option<ResolvedOpening> {
    let host = level.wall(opening.host)?;
    let direction = host.direction();
    if direction == Vec2::ZERO {
        return None;
    }
    let start = host.point_at(opening.start_offset());
    let end = host.point_at(opening.end_offset());
    let centre = host.point_at(opening.offset);

    let swing = if opening.kind == OpeningKind::Door {
        let (hinge, closed_jamb) = if opening.flipped {
            (end, start)
        } else {
            (start, end)
        };
        let leaf = (closed_jamb - hinge).normalized();
        let mut open = leaf.perpendicular();
        if opening.flipped {
            open = open.scaled(-1.0);
        }
        let radius = opening.width;
        Some(Swing {
            hinge,
            leaf_end: hinge + open * radius,
            radius,
            from_degrees: leaf.angle_degrees(),
            to_degrees: open.angle_degrees(),
        })
    } else {
        None
    };

    Some(ResolvedOpening {
        id: opening.id,
        kind: opening.kind,
        host: opening.host,
        start,
        end,
        centre,
        width: opening.width,
        height: opening.height,
        sill: opening.sill,
        flipped: opening.flipped,
        host_direction: direction,
        host_thickness: host.thickness,
        swing,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::Level;
    use crate::wall::DEFAULT_WALL_THICKNESS;

    fn wall(id: u64, start: (f64, f64), end: (f64, f64)) -> Wall {
        Wall::new(
            WallId(id),
            Point2::new(start.0, start.1),
            Point2::new(end.0, end.1),
            DEFAULT_WALL_THICKNESS,
            2800.0,
        )
    }

    #[test]
    fn a_free_standing_wall_keeps_its_exact_ends() {
        let w = wall(1, (0.0, 0.0), (4000.0, 0.0));
        let resolved = resolve_wall(&w, &[]);
        assert_eq!(resolved.start_extension, 0.0);
        assert_eq!(resolved.end_extension, 0.0);
        assert_eq!(resolved.corners[0], Point2::new(0.0, 100.0));
        assert_eq!(resolved.corners[1], Point2::new(4000.0, 100.0));
    }

    #[test]
    fn two_walls_at_a_corner_extend_into_the_joint() {
        let a = wall(1, (0.0, 0.0), (4000.0, 0.0));
        let b = wall(2, (4000.0, 0.0), (4000.0, 4000.0));
        let walls = vec![a.clone(), b.clone()];
        let resolved_a = resolve_wall(&a, &walls);
        let resolved_b = resolve_wall(&b, &walls);
        assert_eq!(resolved_a.end_extension, 100.0);
        assert_eq!(resolved_b.start_extension, 100.0);
        assert_eq!(resolved_a.start_extension, 0.0);
        assert_eq!(resolved_b.end_extension, 0.0);
    }

    #[test]
    fn a_t_junction_extends_the_meeting_wall() {
        // The long wall passes through the short wall's end.
        let wall_along = wall(1, (0.0, 0.0), (6000.0, 0.0));
        let wall_down = wall(2, (3000.0, 4000.0), (3000.0, 0.0));
        let walls = vec![wall_along.clone(), wall_down.clone()];
        let resolved_down = resolve_wall(&wall_down, &walls);
        assert_eq!(resolved_down.end_extension, 100.0);
        // The long wall keeps its ends.
        let resolved_along = resolve_wall(&wall_along, &walls);
        assert_eq!(resolved_along.start_extension, 0.0);
        assert_eq!(resolved_along.end_extension, 0.0);
    }

    #[test]
    fn extent_of_joined_walls_covers_the_corner() {
        let a = wall(1, (0.0, 0.0), (4000.0, 0.0));
        let b = wall(2, (4000.0, 0.0), (4000.0, 4000.0));
        let walls = vec![a.clone(), b.clone()];
        let resolved: Vec<ResolvedWall> = walls.iter().map(|w| resolve_wall(w, &walls)).collect();
        let geometry = PlanGeometry {
            walls: resolved,
            openings: Vec::new(),
        };
        let bounds = geometry.bounds().expect("bounds");
        // The corner is filled: the horizontal wall reaches x = 4100 and the
        // vertical wall reaches y = -100, so the union spans the joint.
        assert_eq!(bounds.min.x, 0.0);
        assert_eq!(bounds.min.y, -100.0);
        assert_eq!(bounds.max.x, 4100.0);
        assert_eq!(bounds.max.y, 4000.0);
    }

    #[test]
    fn hit_test_prefers_an_opening_over_its_wall() {
        let mut level = Level::new(crate::ids::LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let opening_id = OpeningId(2);
        level.openings.push(Opening::new(
            opening_id,
            OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        let geometry = PlanGeometry::of_level(&level);
        assert_eq!(
            geometry.hit_test(Point2::new(2000.0, 0.0), HIT_TOLERANCE),
            Some(Hit::Opening(opening_id))
        );
        assert_eq!(
            geometry.hit_test(Point2::new(300.0, 0.0), HIT_TOLERANCE),
            Some(Hit::Wall(WallId(1)))
        );
        assert_eq!(
            geometry.hit_test(Point2::new(300.0, 5000.0), HIT_TOLERANCE),
            None
        );
    }

    #[test]
    fn door_swing_starts_closed_and_opens_ninety_degrees() {
        let mut level = Level::new(crate::ids::LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.openings.push(Opening::new(
            OpeningId(1),
            OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        let geometry = PlanGeometry::of_level(&level);
        let door = &geometry.openings[0];
        let swing = door.swing.expect("door swing");
        assert_eq!(swing.radius, 900.0);
        assert!(
            (swing.from_degrees - 0.0).abs() < 1e-9 || (swing.from_degrees - 360.0).abs() < 1e-9
        );
        assert!((swing.to_degrees - 90.0).abs() < 1e-9);
        assert_eq!(swing.leaf_end, Point2::new(1550.0, 900.0));
    }

    #[test]
    fn a_flipped_door_hinges_at_the_other_jamb() {
        let mut level = Level::new(crate::ids::LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let mut door = Opening::new(OpeningId(1), OpeningKind::Door, WallId(1), 2000.0);
        door.flipped = true;
        level.openings.push(door);
        let geometry = PlanGeometry::of_level(&level);
        let swing = geometry.openings[0].swing.expect("swing");
        assert_eq!(swing.hinge, Point2::new(2450.0, 0.0));
        assert_eq!(swing.leaf_end, Point2::new(2450.0, 900.0));
    }

    #[test]
    fn windows_have_no_swing() {
        let mut level = Level::new(crate::ids::LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.openings.push(Opening::new(
            OpeningId(1),
            OpeningKind::Window,
            WallId(1),
            2000.0,
        ));
        let geometry = PlanGeometry::of_level(&level);
        assert!(geometry.openings[0].swing.is_none());
    }
}
