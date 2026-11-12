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
//!
//! Either measured point may be *anchored* to a wall. An anchor stores a wall
//! id and a fraction along that wall rather than a coordinate, so the measured
//! point follows the wall when it moves; the stored point stays behind as a
//! fallback for a level that no longer holds the wall. Witness lines join a
//! measured point to the dimension line, leaving a small gap at the wall and a
//! small overrun past the line, the way a drafted dimension reads.

use serde::{Deserialize, Serialize};

use crate::geom::{Point2, Vec2};
use crate::ids::{DimensionId, LayerId, WallId};
use crate::layer::EntityStyle;
use crate::level::Level;
use crate::wall::Wall;

/// Offset the UI should start with when the user has not dragged a dimension
/// line anywhere yet, in millimetres.
pub const DEFAULT_DIMENSION_OFFSET: f64 = 500.0;

/// Gap between a measured point and the start of its witness line, in
/// millimetres.
pub const DEFAULT_WITNESS_GAP: f64 = 200.0;

/// How far a witness line runs past the dimension line, in millimetres.
pub const DEFAULT_WITNESS_OVERRUN: f64 = 100.0;

fn default_witness_gap() -> f64 {
    DEFAULT_WITNESS_GAP
}

fn default_witness_overrun() -> f64 {
    DEFAULT_WITNESS_OVERRUN
}

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

/// A dimension's attachment of one measured point to a wall.
///
/// The anchor stores *where* on the wall the point sits rather than a
/// coordinate, so the measured point follows the wall when it moves or
/// stretches. The position is a fraction of the wall's length: `0.0` is its
/// start, `1.0` its end.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DimensionAnchor {
    /// Wall the measured point sits on.
    pub wall_id: WallId,
    /// Position along the wall as a fraction of its length.
    pub param: f64,
    /// Whether the dimension measures across the wall. The constraint solver
    /// maintains this flag; it never moves the resolved point.
    #[serde(default)]
    pub perpendicular: bool,
}

impl DimensionAnchor {
    /// An anchor on `wall_id`, `param` of the way along it.
    pub const fn new(wall_id: WallId, param: f64) -> Self {
        Self {
            wall_id,
            param,
            perpendicular: false,
        }
    }

    /// True when `param` is a finite fraction of the wall.
    pub fn is_valid(&self) -> bool {
        self.param.is_finite() && (0.0..=1.0).contains(&self.param)
    }

    /// The point this anchor pins on `wall`: `param` of the way along its
    /// centreline from its start to its end.
    ///
    /// `None` when the parameter is not a fraction of the wall or the wall has
    /// no usable geometry. The point stays on the centreline whatever
    /// [`DimensionAnchor::perpendicular`] says.
    pub fn resolve_on(&self, wall: &Wall) -> Option<Point2> {
        if !self.is_valid() || !wall.start.is_finite() || !wall.end.is_finite() {
            return None;
        }
        Some(wall.start + (wall.end - wall.start) * self.param)
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
    /// Wall attachment for the first measured point, if it has one.
    #[serde(default)]
    pub start_anchor: Option<DimensionAnchor>,
    /// Wall attachment for the second measured point, if it has one.
    #[serde(default)]
    pub end_anchor: Option<DimensionAnchor>,
    /// Gap between a measured point and the start of its witness line, in
    /// millimetres.
    #[serde(default = "default_witness_gap")]
    pub witness_gap: f64,
    /// How far a witness line runs past the dimension line, in millimetres.
    #[serde(default = "default_witness_overrun")]
    pub witness_overrun: f64,
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
            start_anchor: None,
            end_anchor: None,
            witness_gap: DEFAULT_WITNESS_GAP,
            witness_overrun: DEFAULT_WITNESS_OVERRUN,
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
        Some(self.offset_line(self.start, self.end))
    }

    /// The first measured point, resolved through its wall anchor when it has
    /// one and `level` still holds that wall.
    ///
    /// Falls back to the stored point otherwise, so a dimension whose wall was
    /// deleted still reads the geometry it was last measured against.
    pub fn effective_start(&self, level: Option<&Level>) -> Point2 {
        self.start_anchor
            .as_ref()
            .zip(level)
            .and_then(|(anchor, level)| self.resolve_anchor(anchor, level))
            .unwrap_or(self.start)
    }

    /// The second measured point, resolved the same way as
    /// [`Dimension::effective_start`].
    pub fn effective_end(&self, level: Option<&Level>) -> Point2 {
        self.end_anchor
            .as_ref()
            .zip(level)
            .and_then(|(anchor, level)| self.resolve_anchor(anchor, level))
            .unwrap_or(self.end)
    }

    /// The point `anchor` pins on its wall in `level`.
    ///
    /// `None` when the wall is not on `level`, when the parameter is not a
    /// finite fraction of the wall, or when the wall has no usable geometry.
    /// [`DimensionAnchor::perpendicular`] is constraint metadata and never
    /// moves the point.
    pub fn resolve_anchor(&self, anchor: &DimensionAnchor, level: &Level) -> Option<Point2> {
        let wall = level.wall(anchor.wall_id)?;
        anchor.resolve_on(wall)
    }

    /// The measured value for this kind, using the resolved measured points.
    pub fn length_at(&self, level: Option<&Level>) -> f64 {
        let start = self.effective_start(level);
        let end = self.effective_end(level);
        match self.kind {
            DimensionKind::LinearHorizontal => (end.x - start.x).abs(),
            DimensionKind::LinearVertical => (end.y - start.y).abs(),
            DimensionKind::Aligned => start.distance(end),
        }
    }

    /// [`Dimension::is_valid`] applied to the resolved measured points.
    pub fn is_valid_at(&self, level: Option<&Level>) -> bool {
        let start = self.effective_start(level);
        let end = self.effective_end(level);
        if !start.is_finite() || !end.is_finite() {
            return false;
        }
        let length = self.length_at(level);
        if !length.is_finite() || length <= 0.0 {
            return false;
        }
        self.offset.is_finite() && self.offset > 0.0
    }

    /// [`Dimension::dimension_line`] drawn through the resolved measured
    /// points. `None` when the dimension is not drawable there.
    pub fn dimension_line_at(&self, level: Option<&Level>) -> Option<(Point2, Point2)> {
        if !self.is_valid_at(level) {
            return None;
        }
        let start = self.effective_start(level);
        let end = self.effective_end(level);
        Some(self.offset_line(start, end))
    }

    /// The witness line joining the first measured point to the dimension line:
    /// it starts `witness_gap` past the measured point and runs
    /// `witness_overrun` beyond the dimension line.
    ///
    /// `None` when extension lines are off, when the dimension cannot be drawn,
    /// or when either margin is not a non-negative distance.
    pub fn start_witness_line(&self, level: Option<&Level>) -> Option<(Point2, Point2)> {
        self.witness_line(level, true)
    }

    /// The witness line joining the second measured point to the dimension
    /// line, drawn like [`Dimension::start_witness_line`].
    pub fn end_witness_line(&self, level: Option<&Level>) -> Option<(Point2, Point2)> {
        self.witness_line(level, false)
    }

    /// The dimension line through `start..end`, moved by `offset`.
    fn offset_line(&self, start: Point2, end: Point2) -> (Point2, Point2) {
        match self.kind {
            DimensionKind::LinearHorizontal => {
                let y = (start.y + end.y) * 0.5 + self.offset;
                (Point2::new(start.x, y), Point2::new(end.x, y))
            }
            DimensionKind::LinearVertical => {
                let x = (start.x + end.x) * 0.5 + self.offset;
                (Point2::new(x, start.y), Point2::new(x, end.y))
            }
            DimensionKind::Aligned => {
                let normal = (end - start).normalized().perpendicular();
                let shift = normal * self.offset;
                (start + shift, end + shift)
            }
        }
    }

    fn witness_line(&self, level: Option<&Level>, from_start: bool) -> Option<(Point2, Point2)> {
        if !self.extension_lines || !self.has_usable_margins() {
            return None;
        }
        let (line_start, line_end) = self.dimension_line_at(level)?;
        let start = self.effective_start(level);
        let end = self.effective_end(level);
        let direction = self.witness_direction(start, end);
        let (measured, on_line) = if from_start {
            (start, line_start)
        } else {
            (end, line_end)
        };
        let from = measured + direction * self.witness_gap;
        let to = on_line + direction * self.witness_overrun;
        Some((from, to))
    }

    /// Unit direction a witness line runs in for this kind: up for a horizontal
    /// dimension, right for a vertical one, and along the left-hand normal of
    /// the span for an aligned one.
    fn witness_direction(&self, start: Point2, end: Point2) -> Vec2 {
        match self.kind {
            DimensionKind::LinearHorizontal => Vec2::new(0.0, 1.0),
            DimensionKind::LinearVertical => Vec2::new(1.0, 0.0),
            DimensionKind::Aligned => (end - start).normalized().perpendicular(),
        }
    }

    fn has_usable_margins(&self) -> bool {
        self.witness_gap.is_finite()
            && self.witness_gap >= 0.0
            && self.witness_overrun.is_finite()
            && self.witness_overrun >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::LevelId;
    use crate::project::Project;

    /// A 4 m wall along the x-axis, on a level of its own.
    fn level_with_wall(wall_id: WallId) -> Level {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(Wall::new(
            wall_id,
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 0.0),
            200.0,
            2800.0,
        ));
        level
    }

    /// The standard 0..3000 horizontal dimension, anchored a quarter and three
    /// quarters of the way along `wall_id`.
    fn anchored(kind: DimensionKind, wall_id: WallId) -> Dimension {
        let mut dimension = dimension(kind);
        dimension.start_anchor = Some(DimensionAnchor::new(wall_id, 0.25));
        dimension.end_anchor = Some(DimensionAnchor::new(wall_id, 0.75));
        dimension
    }

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

    #[test]
    fn a_new_dimension_has_no_anchors_and_default_witness_margins() {
        let dimension = dimension(DimensionKind::LinearHorizontal);
        assert!(dimension.start_anchor.is_none());
        assert!(dimension.end_anchor.is_none());
        assert_eq!(dimension.witness_gap, DEFAULT_WITNESS_GAP);
        assert_eq!(dimension.witness_overrun, DEFAULT_WITNESS_OVERRUN);
    }

    #[test]
    fn dimension_anchors_round_trip_and_validate() {
        let anchor = DimensionAnchor::new(WallId(3), 0.5);
        let json = serde_json::to_string(&anchor).expect("serialize");
        let back: DimensionAnchor = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, anchor);
        assert!(!back.perpendicular);

        assert!(DimensionAnchor::new(WallId(3), 0.0).is_valid());
        assert!(DimensionAnchor::new(WallId(3), 1.0).is_valid());
        assert!(!DimensionAnchor::new(WallId(3), -0.01).is_valid());
        assert!(!DimensionAnchor::new(WallId(3), f64::NAN).is_valid());
    }

    #[test]
    fn anchored_dimensions_round_trip_through_json() {
        let original = anchored(DimensionKind::LinearHorizontal, WallId(2));
        let json = serde_json::to_string(&original).expect("serialize");
        let back: Dimension = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
        let start = back.start_anchor.expect("start anchor");
        let end = back.end_anchor.expect("end anchor");
        assert_eq!(start, DimensionAnchor::new(WallId(2), 0.25));
        assert_eq!(end, DimensionAnchor::new(WallId(2), 0.75));
    }

    #[test]
    fn a_dimension_saved_before_constraints_still_loads() {
        let json = r#"{
            "id": 1,
            "kind": "Aligned",
            "start": { "x": 0.0, "y": 0.0 },
            "end": { "x": 3000.0, "y": 0.0 },
            "offset": 500.0,
            "extension_lines": true
        }"#;
        let dimension: Dimension = serde_json::from_str(json).expect("old dimension");
        assert!(dimension.start_anchor.is_none());
        assert!(dimension.end_anchor.is_none());
        assert_eq!(dimension.witness_gap, DEFAULT_WITNESS_GAP);
        assert_eq!(dimension.witness_overrun, DEFAULT_WITNESS_OVERRUN);
        assert!(dimension.is_valid_at(None));
    }

    #[test]
    fn effective_points_fall_back_to_the_stored_points_without_a_level() {
        let dimension = anchored(DimensionKind::LinearHorizontal, WallId(2));
        assert_eq!(dimension.effective_start(None), dimension.start);
        assert_eq!(dimension.effective_end(None), dimension.end);
        assert!((dimension.length_at(None) - dimension.length()).abs() < 1e-9);
    }

    #[test]
    fn effective_points_follow_the_anchored_wall() {
        let level = level_with_wall(WallId(2));
        let dimension = anchored(DimensionKind::LinearHorizontal, WallId(2));
        assert_eq!(
            dimension.effective_start(Some(&level)),
            Point2::new(1000.0, 0.0)
        );
        assert_eq!(
            dimension.effective_end(Some(&level)),
            Point2::new(3000.0, 0.0)
        );
        assert!((dimension.length_at(Some(&level)) - 2000.0).abs() < 1e-9);
        assert!(dimension.is_valid_at(Some(&level)));
        let (from, to) = dimension
            .dimension_line_at(Some(&level))
            .expect("dimension line");
        assert_eq!(from, Point2::new(1000.0, DEFAULT_DIMENSION_OFFSET));
        assert_eq!(to, Point2::new(3000.0, DEFAULT_DIMENSION_OFFSET));
    }

    #[test]
    fn a_wall_moved_after_anchoring_moves_the_measured_points() {
        let mut level = level_with_wall(WallId(2));
        let dimension = anchored(DimensionKind::LinearHorizontal, WallId(2));
        level.walls[0].end = Point2::new(8000.0, 0.0);
        assert_eq!(
            dimension.effective_start(Some(&level)),
            Point2::new(2000.0, 0.0)
        );
        assert_eq!(
            dimension.effective_end(Some(&level)),
            Point2::new(6000.0, 0.0)
        );
        assert!((dimension.length_at(Some(&level)) - 4000.0).abs() < 1e-9);
    }

    #[test]
    fn the_perpendicular_flag_never_moves_the_resolved_point() {
        let level = level_with_wall(WallId(2));
        let mut dimension = anchored(DimensionKind::LinearHorizontal, WallId(2));
        let anchor = dimension.start_anchor.as_mut().expect("start anchor");
        anchor.perpendicular = true;
        assert_eq!(
            dimension.effective_start(Some(&level)),
            Point2::new(1000.0, 0.0)
        );
    }

    #[test]
    fn an_anchor_off_the_wall_does_not_resolve() {
        let level = level_with_wall(WallId(2));
        let mut dimension = anchored(DimensionKind::LinearHorizontal, WallId(2));
        let anchor = dimension.start_anchor.expect("start anchor");
        assert!(dimension.resolve_anchor(&anchor, &level).is_some());

        let mut beyond = anchor;
        beyond.param = 1.5;
        assert!(!beyond.is_valid());
        assert!(dimension.resolve_anchor(&beyond, &level).is_none());

        beyond.param = f64::NAN;
        assert!(dimension.resolve_anchor(&beyond, &level).is_none());

        // A wall that is not on the level cannot resolve either, and the
        // dimension falls back to the point it stored.
        let missing = DimensionAnchor::new(WallId(99), 0.5);
        assert!(dimension.resolve_anchor(&missing, &level).is_none());
        dimension.start_anchor = Some(missing);
        assert_eq!(
            dimension.effective_start(Some(&level)),
            dimension.start,
            "a dangling anchor falls back instead of vanishing"
        );
    }

    #[test]
    fn an_anchored_dimension_that_collapses_is_invalid_at() {
        let level = level_with_wall(WallId(2));
        let mut dimension = dimension(DimensionKind::LinearHorizontal);
        dimension.start_anchor = Some(DimensionAnchor::new(WallId(2), 0.5));
        dimension.end_anchor = Some(DimensionAnchor::new(WallId(2), 0.5));
        assert!(dimension.is_valid(), "the stored points still measure");
        assert!(!dimension.is_valid_at(Some(&level)));
        assert!(dimension.dimension_line_at(Some(&level)).is_none());
        assert!(dimension.start_witness_line(Some(&level)).is_none());
    }

    #[test]
    fn witness_lines_leave_a_gap_and_an_overrun() {
        let dimension = dimension(DimensionKind::LinearHorizontal);
        let (from, to) = dimension.start_witness_line(None).expect("start witness");
        assert_eq!(from, Point2::new(0.0, 200.0));
        assert_eq!(to, Point2::new(0.0, 600.0));
        let (from, to) = dimension.end_witness_line(None).expect("end witness");
        assert_eq!(from, Point2::new(3000.0, 200.0));
        assert_eq!(to, Point2::new(3000.0, 600.0));

        let mut tighter = dimension.clone();
        tighter.witness_gap = 50.0;
        tighter.witness_overrun = 25.0;
        let (from, to) = tighter.start_witness_line(None).expect("start witness");
        assert_eq!(from, Point2::new(0.0, 50.0));
        assert_eq!(to, Point2::new(0.0, 525.0));
    }

    #[test]
    fn witness_lines_are_not_drawn_without_extension_lines() {
        let mut dimension = dimension(DimensionKind::LinearHorizontal);
        dimension.extension_lines = false;
        assert!(dimension.start_witness_line(None).is_none());
        assert!(dimension.end_witness_line(None).is_none());

        dimension.extension_lines = true;
        dimension.witness_gap = -1.0;
        assert!(dimension.start_witness_line(None).is_none());
    }

    #[test]
    fn vertical_and_aligned_witness_lines_run_in_their_own_direction() {
        let vertical = Dimension::new(
            DimensionId(5),
            DimensionKind::LinearVertical,
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 2000.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        let (from, to) = vertical.start_witness_line(None).expect("start witness");
        assert_eq!(from, Point2::new(200.0, 0.0));
        assert_eq!(to, Point2::new(600.0, 0.0));
        let (from, to) = vertical.end_witness_line(None).expect("end witness");
        assert_eq!(from, Point2::new(200.0, 2000.0));
        assert_eq!(to, Point2::new(600.0, 2000.0));

        // An aligned dimension runs along the left-hand normal of its span:
        // (-0.8, 0.6) for the 3-4-5 span below.
        let aligned = Dimension::new(
            DimensionId(6),
            DimensionKind::Aligned,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 4000.0),
            DEFAULT_DIMENSION_OFFSET,
        );
        let (from, to) = aligned.start_witness_line(None).expect("start witness");
        assert!((from.x + 160.0).abs() < 1e-9);
        assert!((from.y - 120.0).abs() < 1e-9);
        assert!((to.x + 480.0).abs() < 1e-9);
        assert!((to.y - 360.0).abs() < 1e-9);
    }
}
