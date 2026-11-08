//! A dimension line: the measured distance between two points on a plan.
//!
//! A dimension stores the two measured points exactly as the user drew them
//! and derives its value from its kind every time it is asked. Nothing is
//! cached, so moving a wall or an opening and then re-basing the dimension on
//! the new points cannot leave a stale number behind.
//!
//! Coordinates are millimetres, matching the rest of the domain. A positive
//! `offset` moves the dimension line **up** for a horizontal dimension,
//! **right** for a vertical one, and along the left-hand normal of
//! `start..end` for an aligned one.

use serde::{Deserialize, Serialize};

use crate::geom::Point2;
use crate::ids::{DimensionId, LayerId};
use crate::layer::EntityStyle;

/// Offset the UI should start with when the user has not dragged a dimension
/// line anywhere yet, in millimetres.
pub const DEFAULT_DIMENSION_OFFSET: f64 = 500.0;

/// How a dimension measures the span between its two points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionKind {
    /// Horizontal distance: the absolute difference in `x`.
    LinearHorizontal,
    /// Vertical distance: the absolute difference in `y`.
    LinearVertical,
    /// True distance along the line joining the two points.
    Aligned,
}

impl DimensionKind {
    /// Short human-readable name for menus and the inspector.
    pub fn label(self) -> &'static str {
        match self {
            DimensionKind::LinearHorizontal => "Horizontal",
            DimensionKind::LinearVertical => "Vertical",
            DimensionKind::Aligned => "Aligned",
        }
    }
}

/// A measured span annotated on the plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub id: DimensionId,
    /// Layer this dimension is drawn on. Kept in step with `style.layer_id`.
    #[serde(default)]
    pub layer_id: LayerId,
    /// Per-dimension overrides of the layer's appearance.
    #[serde(default)]
    pub style: EntityStyle,
    pub kind: DimensionKind,
    /// First measured point.
    pub start: Point2,
    /// Second measured point.
    pub end: Point2,
    /// Distance from the measured span to the dimension line, in millimetres.
    /// Always greater than zero; a dimension cannot sit on what it measures.
    pub offset: f64,
    /// Whether the two short witness lines from the measured points down to
    /// the dimension line are drawn.
    pub extension_lines: bool,
}

impl Dimension {
    /// Create a dimension with extension lines enabled.
    pub fn new(
        id: DimensionId,
        kind: DimensionKind,
        start: Point2,
        end: Point2,
        offset: f64,
    ) -> Self {
        Self {
            id,
            layer_id: LayerId::DEFAULT,
            style: EntityStyle::default(),
            kind,
            start,
            end,
            offset,
            extension_lines: true,
        }
    }

    /// Move this dimension onto `layer_id`, keeping its style mirror in step.
    pub fn on_layer(mut self, layer_id: LayerId) -> Self {
        self.layer_id = layer_id;
        self.style.layer_id = layer_id;
        self
    }

    /// The measured value for this kind, in millimetres.
    pub fn length(&self) -> f64 {
        match self.kind {
            DimensionKind::LinearHorizontal => (self.end.x - self.start.x).abs(),
            DimensionKind::LinearVertical => (self.end.y - self.start.y).abs(),
            DimensionKind::Aligned => self.start.distance(self.end),
        }
    }

    /// True when the geometry is finite, the measured span is non-zero for
    /// this kind, and the offset is a positive distance.
    pub fn is_valid(&self) -> bool {
        if !self.start.is_finite() || !self.end.is_finite() {
            return false;
        }
        let length = self.length();
        if !length.is_finite() || length <= 0.0 {
            return false;
        }
        self.offset.is_finite() && self.offset > 0.0
    }

    /// The two endpoints of the dimension line itself, already moved by
    /// `offset`. `None` when the dimension is not drawable.
    ///
    /// For a horizontal dimension the line sits above both points (at their
    /// mean `y` plus the offset); for a vertical one it sits to their right.
    /// An aligned dimension is shifted along the left-hand normal of
    /// `start..end`.
    pub fn dimension_line(&self) -> Option<(Point2, Point2)> {
        if !self.is_valid() {
            return None;
        }
        match self.kind {
            DimensionKind::LinearHorizontal => {
                let y = (self.start.y + self.end.y) * 0.5 + self.offset;
                Some((Point2::new(self.start.x, y), Point2::new(self.end.x, y)))
            }
            DimensionKind::LinearVertical => {
                let x = (self.start.x + self.end.x) * 0.5 + self.offset;
                Some((Point2::new(x, self.start.y), Point2::new(x, self.end.y)))
            }
            DimensionKind::Aligned => {
                let normal = (self.end - self.start).normalized().perpendicular();
                let shift = normal * self.offset;
                Some((self.start + shift, self.end + shift))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::Project;

    fn dimension(kind: DimensionKind) -> Dimension {
        Dimension::new(
            DimensionId(1),
            kind,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            DEFAULT_DIMENSION_OFFSET,
        )
    }

    #[test]
    fn each_kind_measures_its_own_span() {
        let horizontal = dimension(DimensionKind::LinearHorizontal);
        assert!((horizontal.length() - 3000.0).abs() < 1e-9);
        assert!(horizontal.is_valid());

        let vertical = Dimension::new(
            DimensionId(2),
            DimensionKind::LinearVertical,
            Point2::new(100.0, 500.0),
            Point2::new(100.0, 2500.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        assert!((vertical.length() - 2000.0).abs() < 1e-9);
        assert!(vertical.is_valid());

        let aligned = Dimension::new(
            DimensionId(3),
            DimensionKind::Aligned,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 4000.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        assert!((aligned.length() - 5000.0).abs() < 1e-9);
        assert!(aligned.is_valid());
    }

    #[test]
    fn zero_length_dimensions_are_invalid() {
        let mut aligned = dimension(DimensionKind::Aligned);
        aligned.end = aligned.start;
        assert!(!aligned.is_valid());

        // Two points that differ only vertically still measure zero when the
        // dimension is horizontal.
        let horizontal = Dimension::new(
            DimensionId(4),
            DimensionKind::LinearHorizontal,
            Point2::new(100.0, 100.0),
            Point2::new(100.0, 1000.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        assert!(!horizontal.is_valid());
        assert!(horizontal.dimension_line().is_none());
    }

    #[test]
    fn a_zero_or_negative_offset_is_invalid() {
        let mut dimension = dimension(DimensionKind::LinearHorizontal);
        dimension.offset = 0.0;
        assert!(!dimension.is_valid());
        dimension.offset = -10.0;
        assert!(!dimension.is_valid());
        dimension.offset = f64::NAN;
        assert!(!dimension.is_valid());
    }

    #[test]
    fn extension_lines_are_on_by_default() {
        assert!(dimension(DimensionKind::LinearHorizontal).extension_lines);
    }

    #[test]
    fn the_dimension_line_sits_at_the_requested_offset() {
        let horizontal = dimension(DimensionKind::LinearHorizontal);
        let (a, b) = horizontal.dimension_line().expect("dimension line");
        assert_eq!(a, Point2::new(0.0, DEFAULT_DIMENSION_OFFSET));
        assert_eq!(b, Point2::new(3000.0, DEFAULT_DIMENSION_OFFSET));

        let vertical = Dimension::new(
            DimensionId(2),
            DimensionKind::LinearVertical,
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 2000.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        let (a, b) = vertical.dimension_line().expect("dimension line");
        assert_eq!(a, Point2::new(DEFAULT_DIMENSION_OFFSET, 0.0));
        assert_eq!(b, Point2::new(DEFAULT_DIMENSION_OFFSET, 2000.0));
    }

    #[test]
    fn dimension_ids_are_allocated_monotonically() {
        let mut project = Project::new("Test plan", 1_760_000_000);
        let first = DimensionId(project.allocate_id());
        let second = DimensionId(project.allocate_id());
        assert_eq!(second.value(), first.value() + 1);
        assert!(project.next_id > second.value());
    }

    #[test]
    fn dimensions_round_trip_through_json() {
        let original = dimension(DimensionKind::Aligned);
        let json = serde_json::to_string(&original).expect("serialize");
        let back: Dimension = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn a_new_dimension_starts_on_the_default_layer() {
        let dimension = dimension(DimensionKind::LinearHorizontal);
        assert_eq!(dimension.layer_id, LayerId::DEFAULT);
        assert_eq!(dimension.style.layer_id, LayerId::DEFAULT);
    }

    #[test]
    fn a_dimension_can_be_placed_on_another_layer() {
        let dimension = dimension(DimensionKind::Aligned).on_layer(LayerId(8));
        assert_eq!(dimension.layer_id, LayerId(8));
        assert_eq!(dimension.style.layer_id, LayerId(8));
        let json = serde_json::to_string(&dimension).expect("serialize");
        let back: Dimension = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, dimension);
    }

    #[test]
    fn a_dimension_saved_before_layers_still_loads() {
        let json = r#"{
            "id": 1,
            "kind": "Aligned",
            "start": { "x": 0.0, "y": 0.0 },
            "end": { "x": 3000.0, "y": 0.0 },
            "offset": 500.0,
            "extension_lines": true
        }"#;
        let dimension: Dimension = serde_json::from_str(json).expect("old dimension");
        assert_eq!(dimension.layer_id, LayerId::DEFAULT);
        assert_eq!(dimension.style, EntityStyle::default());
        assert!(dimension.is_valid());
    }

    #[test]
    fn dimension_kinds_round_trip_and_have_labels() {
        for kind in [
            DimensionKind::LinearHorizontal,
            DimensionKind::LinearVertical,
            DimensionKind::Aligned,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let back: DimensionKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, kind);
            assert!(!kind.label().is_empty());
        }
    }
}
