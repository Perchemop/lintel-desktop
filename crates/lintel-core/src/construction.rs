//! Construction geometry: guides that help you draw and never reach the sheet.
//!
//! Construction lines, rays, circles and arcs are real model elements: they
//! snap, they can be picked and they survive save/load. They are **not**
//! printable — the PDF exporter never visits
//! [`crate::level::Level::construction`] — and they never contribute to the
//! drawing extent, so an infinite construction line cannot push the plan off
//! the page.
//!
//! Everything here is pure geometry in millimetres, with y pointing up like
//! the rest of the domain.

use serde::{Deserialize, Serialize};

use crate::geom::{Point2, Vec2, arc_points, point_segment_distance};
use crate::ids::ConstructionId;

/// Shortest construction circle or arc the editor accepts, in millimetres.
pub const MIN_CONSTRUCTION_RADIUS: f64 = 1.0;

/// How many straight segments a full circle is approximated with when a caller
/// asks for a polyline without choosing a count.
pub const DEFAULT_CIRCLE_SEGMENTS: usize = 64;

fn default_true() -> bool {
    true
}

fn point_at_angle(center: Point2, radius: f64, degrees: f64) -> Point2 {
    let radians = degrees.to_radians();
    Point2::new(
        center.x + radius * radians.cos(),
        center.y + radius * radians.sin(),
    )
}

/// Anticlockwise sweep from `start_degrees` to `end_degrees`, in `0..360`.
fn sweep_degrees(start_degrees: f64, end_degrees: f64) -> f64 {
    (end_degrees - start_degrees).rem_euclid(360.0)
}

/// Is `angle` inside the anticlockwise sweep `start..start + sweep`?
fn angle_in_sweep(angle: f64, start_degrees: f64, sweep: f64) -> bool {
    (angle - start_degrees).rem_euclid(360.0) <= sweep + 1e-9
}

/// The shape of a construction entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstructionKind {
    /// A finite segment between two points.
    Line { start: Point2, end: Point2 },
    /// A line through `point` extending forever in both directions.
    InfiniteLine { point: Point2, direction: Vec2 },
    /// A line starting at `origin` and extending forever forwards.
    Ray { origin: Point2, direction: Vec2 },
    Circle { center: Point2, radius: f64 },
    Arc {
        center: Point2,
        radius: f64,
        start_deg: f64,
        end_deg: f64,
    },
}

impl ConstructionKind {
    /// Short human-readable name for menus and status lines.
    pub fn label(&self) -> &'static str {
        match self {
            ConstructionKind::Line { .. } => "Construction line",
            ConstructionKind::InfiniteLine { .. } => "Infinite construction line",
            ConstructionKind::Ray { .. } => "Construction ray",
            ConstructionKind::Circle { .. } => "Construction circle",
            ConstructionKind::Arc { .. } => "Construction arc",
        }
    }

    /// True when every coordinate is finite and the shape can be drawn.
    pub fn is_valid(&self) -> bool {
        match self {
            ConstructionKind::Line { start, end } => {
                (*start).is_finite() && (*end).is_finite() && (*start).distance(*end) > 0.0
            }
            ConstructionKind::InfiniteLine { point, direction }
            | ConstructionKind::Ray { origin: point, direction } => {
                (*point).is_finite()
                    && direction.is_finite()
                    && direction.length_sq() > f64::EPSILON
            }
            ConstructionKind::Circle { center, radius } => {
                (*center).is_finite()
                    && radius.is_finite()
                    && *radius >= MIN_CONSTRUCTION_RADIUS
            }
            ConstructionKind::Arc {
                center,
                radius,
                start_deg,
                end_deg,
            } => {
                (*center).is_finite()
                    && radius.is_finite()
                    && *radius >= MIN_CONSTRUCTION_RADIUS
                    && start_deg.is_finite()
                    && end_deg.is_finite()
            }
        }
    }

    /// Why the shape is not usable, in plain language.
    pub fn invalid_reason(&self) -> Option<&'static str> {
        if self.is_valid() {
            return None;
        }
        Some(match self {
            ConstructionKind::Line { .. } => {
                "a construction line needs two different finite points"
            }
            ConstructionKind::InfiniteLine { .. } | ConstructionKind::Ray { .. } => {
                "a construction line needs a finite origin and direction"
            }
            ConstructionKind::Circle { .. } | ConstructionKind::Arc { .. } => {
                "a construction circle needs a positive radius"
            }
        })
    }

    /// False for infinite lines and rays, which have no finite extent.
    pub fn is_bounded(&self) -> bool {
        matches!(
            self,
            ConstructionKind::Line { .. }
                | ConstructionKind::Circle { .. }
                | ConstructionKind::Arc { .. }
        )
    }

    /// The anticlockwise sweep of an arc, in `0..360`. Zero for other shapes.
    pub fn sweep_degrees(&self) -> f64 {
        match self {
            ConstructionKind::Arc {
                start_deg, end_deg, ..
            } => sweep_degrees(*start_deg, *end_deg),
            _ => 0.0,
        }
    }

    /// Points worth snapping to: ends, origins and centres.
    pub fn anchors(&self) -> Vec<Point2> {
        match self {
            ConstructionKind::Line { start, end } => vec![*start, *end],
            ConstructionKind::InfiniteLine { point, .. } => vec![*point],
            ConstructionKind::Ray { origin, .. } => vec![*origin],
            ConstructionKind::Circle { center, .. } => vec![*center],
            ConstructionKind::Arc {
                center,
                radius,
                start_deg,
                end_deg,
            } => vec![
                *center,
                point_at_angle(*center, *radius, *start_deg),
                point_at_angle(*center, *radius, *end_deg),
            ],
        }
    }

    /// Distance from `point` to the shape, in millimetres. `None` when the
    /// shape is invalid, so a corrupt file cannot poison hit-testing.
    pub fn distance_to(&self, point: Point2) -> Option<f64> {
        if !point.is_finite() || !self.is_valid() {
            return None;
        }
        match self {
            ConstructionKind::Line { start, end } => {
                Some(point_segment_distance(point, *start, *end))
            }
            ConstructionKind::InfiniteLine { point: on, direction } => {
                let unit = direction.normalized();
                Some((point - *on).cross(unit).abs())
            }
            ConstructionKind::Ray { origin, direction } => {
                let unit = direction.normalized();
                let along = (point - *origin).dot(unit);
                if along >= 0.0 {
                    Some((point - *origin).cross(unit).abs())
                } else {
                    Some(point.distance(*origin))
                }
            }
            ConstructionKind::Circle { center, radius } => {
                Some((point.distance(*center) - radius).abs())
            }
            ConstructionKind::Arc {
                center,
                radius,
                start_deg,
                end_deg,
            } => {
                let offset = point - *center;
                let bearing = offset.angle_degrees();
                let sweep = sweep_degrees(*start_deg, *end_deg);
                if angle_in_sweep(bearing, *start_deg, sweep) {
                    Some((offset.length() - radius).abs())
                } else {
                    let start = point_at_angle(*center, *radius, *start_deg);
                    let end = point_at_angle(*center, *radius, *end_deg);
                    Some(point.distance(start).min(point.distance(end)))
                }
            }
        }
    }

    /// A drawable polyline for the UI. `None` for the unbounded shapes, which
    /// the canvas clips to the viewport instead of baking into a point list.
    pub fn polyline(&self, segments: usize) -> Option<Vec<Point2>> {
        if !self.is_valid() {
            return None;
        }
        match self {
            ConstructionKind::Line { start, end } => Some(vec![*start, *end]),
            ConstructionKind::InfiniteLine { .. } | ConstructionKind::Ray { .. } => None,
            ConstructionKind::Circle { center, radius } => Some(arc_points(
                *center,
                *radius,
                0.0,
                360.0,
                segments.max(3),
            )),
            ConstructionKind::Arc {
                center,
                radius,
                start_deg,
                ..
            } => {
                let sweep = self.sweep_degrees();
                Some(arc_points(
                    *center,
                    *radius,
                    *start_deg,
                    *start_deg + sweep,
                    segments.max(2),
                ))
            }
        }
    }
}

/// A guide in a level: its shape plus the switches that make it useful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructionEntity {
    pub id: ConstructionId,
    pub kind: ConstructionKind,
    /// Whether passing points snap to this guide.
    #[serde(default = "default_true")]
    pub snap_enabled: bool,
    /// Hidden guides are neither drawn nor picked.
    #[serde(default = "default_true")]
    pub visible: bool,
    /// Reserved: construction geometry never prints, whatever this says.
    #[serde(default)]
    pub print_enabled: bool,
}

impl ConstructionEntity {
    /// A visible, snapping guide that will never print.
    pub fn new(id: ConstructionId, kind: ConstructionKind) -> Self {
        Self {
            id,
            kind,
            snap_enabled: true,
            visible: true,
            print_enabled: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.kind.is_valid()
    }

    /// Points this guide offers to the snapper; empty when snapping is off.
    pub fn anchors(&self) -> Vec<Point2> {
        if self.snap_enabled {
            self.kind.anchors()
        } else {
            Vec::new()
        }
    }

    /// Distance for hit-testing; hidden guides are not pickable.
    pub fn distance_to(&self, point: Point2) -> Option<f64> {
        if self.visible {
            self.kind.distance_to(point)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line() -> ConstructionKind {
        ConstructionKind::Line {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(1000.0, 0.0),
        }
    }

    fn circle() -> ConstructionKind {
        ConstructionKind::Circle {
            center: Point2::new(0.0, 0.0),
            radius: 10.0,
        }
    }

    fn arc() -> ConstructionKind {
        ConstructionKind::Arc {
            center: Point2::new(0.0, 0.0),
            radius: 10.0,
            start_deg: 0.0,
            end_deg: 90.0,
        }
    }

    #[test]
    fn a_line_measures_to_its_segment() {
        let kind = line();
        assert!(kind.is_valid());
        assert!(kind.is_bounded());
        let distance = kind
            .distance_to(Point2::new(500.0, 300.0))
            .expect("distance");
        assert!((distance - 300.0).abs() < 1e-9);
        // Past the end the distance grows to the endpoint.
        let beyond = kind
            .distance_to(Point2::new(1000.0, 300.0))
            .expect("distance");
        assert!((beyond - 300.0).abs() < 1e-9);
        assert_eq!(kind.anchors().len(), 2);
    }

    #[test]
    fn an_infinite_line_ignores_the_endpoints() {
        let kind = ConstructionKind::InfiniteLine {
            point: Point2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
        };
        assert!(!kind.is_bounded());
        assert!((kind.distance_to(Point2::new(5000.0, 3.0)).expect("distance") - 3.0).abs() < 1e-9);
        // The direction need not be a unit vector.
        let scaled = ConstructionKind::InfiniteLine {
            point: Point2::new(0.0, 0.0),
            direction: Vec2::new(10.0, 0.0),
        };
        let off_axis = scaled
            .distance_to(Point2::new(-40.0, 2.5))
            .expect("distance");
        assert!((off_axis - 2.5).abs() < 1e-9);
    }

    #[test]
    fn a_ray_stops_at_its_origin() {
        let kind = ConstructionKind::Ray {
            origin: Point2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
        };
        assert!(!kind.is_bounded());
        assert!((kind.distance_to(Point2::new(5.0, 3.0)).expect("distance") - 3.0).abs() < 1e-9);
        assert!((kind.distance_to(Point2::new(-5.0, 0.0)).expect("distance") - 5.0).abs() < 1e-9);
        assert_eq!(kind.anchors(), vec![Point2::new(0.0, 0.0)]);
    }

    #[test]
    fn a_circle_measures_to_its_rim() {
        let kind = circle();
        assert!(kind.is_valid());
        assert!(kind.is_bounded());
        assert!((kind.distance_to(Point2::new(13.0, 0.0)).expect("distance") - 3.0).abs() < 1e-9);
        // The centre is a full radius away.
        assert!((kind.distance_to(Point2::ORIGIN).expect("distance") - 10.0).abs() < 1e-9);
        assert_eq!(kind.anchors(), vec![Point2::ORIGIN]);
    }

    #[test]
    fn an_arc_measures_to_its_sweep_and_its_ends() {
        let kind = arc();
        assert!(kind.is_valid());
        assert!((kind.sweep_degrees() - 90.0).abs() < 1e-9);
        // 45 degrees is on the arc, so distance is radial.
        let angle = 45.0_f64.to_radians();
        let inside = Point2::new(12.0 * angle.cos(), 12.0 * angle.sin());
        assert!((kind.distance_to(inside).expect("distance") - 2.0).abs() < 1e-9);
        // 180 degrees is off the arc, so the nearest end wins.
        let outside = kind
            .distance_to(Point2::new(-10.0, 0.0))
            .expect("distance");
        assert!((outside - 200.0_f64.sqrt()).abs() < 1e-9);
    }

    #[test]
    fn an_arc_may_sweep_across_zero_degrees() {
        let kind = ConstructionKind::Arc {
            center: Point2::new(0.0, 0.0),
            radius: 10.0,
            start_deg: 350.0,
            end_deg: 10.0,
        };
        assert!((kind.sweep_degrees() - 20.0).abs() < 1e-9);
        // A point at 0 degrees lies inside the short way round.
        let distance = kind.distance_to(Point2::new(10.0, 0.0)).expect("distance");
        assert!(distance.abs() < 1e-9);
    }

    #[test]
    fn invalid_construction_shapes_are_reported() {
        let zero_length = ConstructionKind::Line {
            start: Point2::new(5.0, 5.0),
            end: Point2::new(5.0, 5.0),
        };
        assert!(!zero_length.is_valid());
        assert!(zero_length.invalid_reason().is_some());
        assert!(zero_length.distance_to(Point2::ORIGIN).is_none());

        let no_direction = ConstructionKind::Ray {
            origin: Point2::ORIGIN,
            direction: Vec2::ZERO,
        };
        assert!(!no_direction.is_valid());

        let tiny = ConstructionKind::Circle {
            center: Point2::ORIGIN,
            radius: MIN_CONSTRUCTION_RADIUS - 0.5,
        };
        assert!(!tiny.is_valid());

        let nan = ConstructionKind::Arc {
            center: Point2::ORIGIN,
            radius: f64::NAN,
            start_deg: 0.0,
            end_deg: 90.0,
        };
        assert!(!nan.is_valid());
        assert!(line().invalid_reason().is_none());
    }

    #[test]
    fn bounded_shapes_can_be_turned_into_polylines() {
        assert_eq!(line().polyline(8).expect("points").len(), 2);
        assert!(circle().polyline(16).expect("points").len() >= 4);
        assert!(arc().polyline(8).expect("points").len() >= 3);
        assert!(
            ConstructionKind::InfiniteLine {
                point: Point2::ORIGIN,
                direction: Vec2::new(1.0, 0.0)
            }
            .polyline(8)
            .is_none()
        );
        assert!(
            ConstructionKind::Ray {
                origin: Point2::ORIGIN,
                direction: Vec2::new(1.0, 0.0)
            }
            .polyline(8)
            .is_none()
        );
    }

    #[test]
    fn a_new_construction_entity_snaps_and_never_prints() {
        let entity = ConstructionEntity::new(ConstructionId(3), line());
        assert_eq!(entity.id, ConstructionId(3));
        assert!(entity.snap_enabled);
        assert!(entity.visible);
        assert!(!entity.print_enabled);
        assert!(entity.is_valid());
        assert_eq!(entity.anchors().len(), 2);
    }

    #[test]
    fn hidden_or_snap_disabled_guides_step_aside() {
        let mut entity = ConstructionEntity::new(ConstructionId(1), line());
        entity.visible = false;
        assert!(entity.distance_to(Point2::new(500.0, 0.0)).is_none());

        entity.visible = true;
        entity.snap_enabled = false;
        assert!(entity.anchors().is_empty());
    }

    #[test]
    fn every_construction_shape_round_trips_through_json() {
        for kind in [
            line(),
            ConstructionKind::InfiniteLine {
                point: Point2::new(1.0, 2.0),
                direction: Vec2::new(3.0, 4.0),
            },
            ConstructionKind::Ray {
                origin: Point2::new(5.0, 6.0),
                direction: Vec2::new(0.0, 1.0),
            },
            circle(),
            arc(),
        ] {
            let entity = ConstructionEntity::new(ConstructionId(9), kind);
            let json = serde_json::to_string(&entity).expect("serialize");
            let back: ConstructionEntity = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, entity);
            assert_eq!(back.kind.label(), entity.kind.label());
        }
    }

    #[test]
    fn a_construction_entity_saved_before_the_switches_still_loads() {
        let json = r#"{
            "id": 4,
            "kind": { "Line": { "start": { "x": 0.0, "y": 0.0 }, "end": { "x": 100.0, "y": 0.0 } } }
        }"#;
        let entity: ConstructionEntity = serde_json::from_str(json).expect("old guide");
        assert!(entity.snap_enabled);
        assert!(entity.visible);
        assert!(!entity.print_enabled);
        assert!(entity.is_valid());
    }
}
