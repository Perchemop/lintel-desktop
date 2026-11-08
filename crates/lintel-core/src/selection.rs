//! Rectangle and fence selection over a level's derived geometry.
//!
//! Selection is pure: hand it the geometry the canvas is already holding, a
//! mode and a filter, and it answers which entities the pick touches. It never
//! mutates a level and holds no state of its own — the shell owns the
//! [`Selection`] and rebuilds it after every edit, exactly like undo. A single
//! pick is a [`select`] call; the stored outcome is a [`Selection`].
//!
//! Coordinates are millimetres, matching the rest of the domain.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::construction::{ConstructionKind, DEFAULT_CIRCLE_SEGMENTS};
use crate::geom::{Bounds, Point2, point_segment_distance};
use crate::ids::{EntityId, LayerId};
use crate::join::PlanGeometry;
use crate::label::PlanLabel;

/// How far a construction guide is extended when a window or fence tests it,
/// so an "infinite" line becomes a long finite segment, in millimetres.
const CONSTRUCTION_EXTENT_MM: f64 = 1.0e9;

/// A point closer than this to a fence segment counts as touching it.
const FENCE_TOLERANCE_MM: f64 = 1.0;

/// What kind of element an [`EntityId`] names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Wall,
    Opening,
    Dimension,
    Label,
    Construction,
}

impl EntityType {
    /// Short human-readable name for menus and status lines.
    pub fn label(self) -> &'static str {
        match self {
            EntityType::Wall => "wall",
            EntityType::Opening => "opening",
            EntityType::Dimension => "dimension",
            EntityType::Label => "label",
            EntityType::Construction => "construction",
        }
    }

    /// The type of a tagged id.
    pub fn of(id: EntityId) -> Self {
        match id {
            EntityId::Wall(_) => EntityType::Wall,
            EntityId::Opening(_) => EntityType::Opening,
            EntityId::Dimension(_) => EntityType::Dimension,
            EntityId::Label(_) => EntityType::Label,
            EntityId::Construction(_) => EntityType::Construction,
        }
    }
}

/// The shape of a pick.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SelectionMode {
    /// A rectangular window. With `crossing` false only entities whose centre
    /// falls inside are picked; with `crossing` true anything the window
    /// touches is picked as well.
    Window {
        bounds: Bounds,
        crossing: bool,
    },
    /// A free polyline; anything it crosses is picked.
    Fence {
        points: Vec<Point2>,
    },
    /// Everything the filter allows.
    #[default]
    All,
}

/// Which entities a pick is allowed to touch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SelectionFilter {
    /// No restriction.
    #[default]
    All,
    /// Only this kind of entity.
    EntityType(EntityType),
    /// Only entities sitting on this layer.
    Layer(LayerId),
}

impl SelectionFilter {
    /// True when an entity of `entity_type` on `layer` passes this filter.
    /// Construction guides have no layer, so a layer filter excludes them.
    pub fn accepts(self, entity_type: EntityType, layer: Option<LayerId>) -> bool {
        match self {
            SelectionFilter::All => true,
            SelectionFilter::EntityType(wanted) => wanted == entity_type,
            SelectionFilter::Layer(wanted) => layer == Some(wanted),
        }
    }
}

/// The result of a pick: the entities it resolved to and how they were found.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Selection {
    /// The entities the pick selected.
    #[serde(default)]
    pub entity_ids: HashSet<EntityId>,
    /// How the pick was made, kept so a redraw can re-run it.
    #[serde(default)]
    pub mode: SelectionMode,
}

impl Selection {
    /// Run a pick and store the outcome.
    pub fn pick(geometry: &PlanGeometry, mode: SelectionMode, filter: SelectionFilter) -> Self {
        let entity_ids = select(geometry, mode.clone(), filter);
        Self { entity_ids, mode }
    }

    pub fn len(&self) -> usize {
        self.entity_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.entity_ids.contains(&id)
    }

    /// Forget everything and go back to the neutral mode.
    pub fn clear(&mut self) {
        self.entity_ids.clear();
        self.mode = SelectionMode::All;
    }
}

/// The entities a pick touches.
///
/// Pure: the geometry is borrowed, nothing is mutated, and the same inputs
/// always give the same set. An inverted window (dragged up and to the left) is
/// normalised before it is tested.
pub fn select(
    geometry: &PlanGeometry,
    mode: SelectionMode,
    filter: SelectionFilter,
) -> HashSet<EntityId> {
    let mode = normalised_mode(mode);
    let mut selected = HashSet::new();

    for wall in &geometry.walls {
        if !filter.accepts(EntityType::Wall, Some(wall.wall.layer_id)) {
            continue;
        }
        let footprint = Footprint {
            centre: wall.wall.midpoint(),
            segments: vec![(wall.wall.start, wall.wall.end)],
            box_bounds: Bounds::from_points(&wall.corners),
        };
        if mode_picks(&mode, &footprint) {
            selected.insert(EntityId::Wall(wall.wall.id));
        }
    }

    for opening in &geometry.openings {
        if !filter.accepts(EntityType::Opening, Some(opening.layer_id)) {
            continue;
        }
        let footprint = Footprint {
            centre: opening.centre,
            segments: vec![(opening.start, opening.end)],
            box_bounds: None,
        };
        if mode_picks(&mode, &footprint) {
            selected.insert(EntityId::Opening(opening.id));
        }
    }

    for dimension in &geometry.dimensions {
        if !filter.accepts(EntityType::Dimension, Some(dimension.layer_id)) {
            continue;
        }
        let (start, end) = dimension
            .dimension_line()
            .unwrap_or((dimension.start, dimension.end));
        let footprint = Footprint {
            centre: midpoint(start, end),
            segments: vec![(start, end)],
            box_bounds: None,
        };
        if mode_picks(&mode, &footprint) {
            selected.insert(EntityId::Dimension(dimension.id));
        }
    }

    for label in &geometry.labels {
        if !filter.accepts(EntityType::Label, Some(label.layer_id)) {
            continue;
        }
        let footprint = label_footprint(label);
        if mode_picks(&mode, &footprint) {
            selected.insert(EntityId::Label(label.id));
        }
    }

    for entity in &geometry.construction {
        // Hidden guides are neither drawn nor picked.
        if !entity.visible {
            continue;
        }
        if !filter.accepts(EntityType::Construction, None) {
            continue;
        }
        let footprint = construction_footprint(&entity.kind);
        if mode_picks(&mode, &footprint) {
            selected.insert(EntityId::Construction(entity.id));
        }
    }

    selected
}

/// One entity's geometry, reduced to what a pick needs: a centre to test a
/// containing window against, a polyline for a crossing window or a fence, and
/// an optional box for shapes that enclose an area (walls, labels).
struct Footprint {
    centre: Point2,
    segments: Vec<(Point2, Point2)>,
    box_bounds: Option<Bounds>,
}

/// Does this mode pick an entity with `footprint`?
fn mode_picks(mode: &SelectionMode, footprint: &Footprint) -> bool {
    match mode {
        SelectionMode::All => true,
        SelectionMode::Window { bounds, crossing } => {
            if *crossing {
                crossing_hits_bounds(footprint, *bounds)
            } else {
                bounds.contains(footprint.centre)
            }
        }
        SelectionMode::Fence { points } => fence_hits(footprint, points),
    }
}

/// A crossing window picks anything that overlaps the rectangle at all.
fn crossing_hits_bounds(footprint: &Footprint, bounds: Bounds) -> bool {
    let overlaps_box = footprint
        .box_bounds
        .is_some_and(|box_bounds| bounds_overlap(box_bounds, bounds));
    if overlaps_box {
        return true;
    }
    if footprint.segments.is_empty() {
        return bounds.contains(footprint.centre);
    }
    footprint
        .segments
        .iter()
        .any(|&(a, b)| segment_intersects_bounds(a, b, bounds))
}

/// A fence picks anything it crosses, plus a point entity it runs through.
fn fence_hits(footprint: &Footprint, fence: &[Point2]) -> bool {
    if fence.len() < 2 {
        return false;
    }
    let crosses = footprint
        .segments
        .iter()
        .any(|&(a, b)| segment_crosses_polyline(a, b, fence));
    if crosses {
        return true;
    }
    footprint.segments.is_empty() && point_near_polyline(footprint.centre, fence)
}

/// The footprint of a label: its drawn box, estimated from the text.
fn label_footprint(label: &PlanLabel) -> Footprint {
    let corner = Point2::new(
        label.anchor.x + label.width_mm(),
        label.anchor.y + label.height_mm,
    );
    Footprint {
        centre: midpoint(label.anchor, corner),
        segments: box_segments(label.anchor, corner),
        box_bounds: Bounds::from_points(&[label.anchor, corner]),
    }
}

/// The footprint of a construction guide. Bounded shapes use the same polyline
/// the canvas draws; an infinite line or ray is approximated by one very long
/// segment so a crossing window and a fence can test it.
fn construction_footprint(kind: &ConstructionKind) -> Footprint {
    match kind {
        ConstructionKind::InfiniteLine { point, direction } => {
            let unit = direction.normalized();
            let start = *point - unit * CONSTRUCTION_EXTENT_MM;
            let end = *point + unit * CONSTRUCTION_EXTENT_MM;
            Footprint {
                centre: *point,
                segments: vec![(start, end)],
                box_bounds: None,
            }
        }
        ConstructionKind::Ray { origin, direction } => {
            let unit = direction.normalized();
            let end = *origin + unit * CONSTRUCTION_EXTENT_MM;
            Footprint {
                centre: *origin,
                segments: vec![(*origin, end)],
                box_bounds: None,
            }
        }
        _ => {
            let polygon = kind.polyline(DEFAULT_CIRCLE_SEGMENTS).unwrap_or_default();
            let centre = Bounds::from_points(&polygon)
                .map_or_else(|| anchor_centre(kind), |bounds| bounds.centre());
            Footprint {
                centre,
                segments: polyline_segments(&polygon),
                box_bounds: None,
            }
        }
    }
}

/// The centre of a guide's anchor points, or the origin when it has none.
fn anchor_centre(kind: &ConstructionKind) -> Point2 {
    let anchors = kind.anchors();
    Bounds::from_points(&anchors).map_or(Point2::ORIGIN, |bounds| bounds.centre())
}

/// Normalise an inverted window so `min` is at or below `max`.
fn normalised_mode(mode: SelectionMode) -> SelectionMode {
    match mode {
        SelectionMode::Window { bounds, crossing } => SelectionMode::Window {
            bounds: bounds.normalised(),
            crossing,
        },
        other => other,
    }
}

/// Do two axis-aligned boxes touch?
fn bounds_overlap(a: Bounds, b: Bounds) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x && a.min.y <= b.max.y && a.max.y >= b.min.y
}

/// Does a segment touch an axis-aligned box? Liang–Barsky clipping.
fn segment_intersects_bounds(a: Point2, b: Point2, bounds: Bounds) -> bool {
    let delta = b - a;
    let mut t0 = 0.0_f64;
    let mut t1 = 1.0_f64;
    let mut clip = |p: f64, q: f64| -> bool {
        if p.abs() <= f64::EPSILON {
            q >= 0.0
        } else {
            let r = q / p;
            if p < 0.0 {
                if r > t1 {
                    return false;
                }
                if r > t0 {
                    t0 = r;
                }
            } else {
                if r < t0 {
                    return false;
                }
                if r < t1 {
                    t1 = r;
                }
            }
            true
        }
    };
    clip(-delta.x, a.x - bounds.min.x)
        && clip(delta.x, bounds.max.x - a.x)
        && clip(-delta.y, a.y - bounds.min.y)
        && clip(delta.y, bounds.max.y - a.y)
}

/// Does a segment cross any edge of a polyline?
fn segment_crosses_polyline(a: Point2, b: Point2, polyline: &[Point2]) -> bool {
    polyline
        .windows(2)
        .any(|edge| segments_intersect(a, b, edge[0], edge[1]))
}

/// Is a point within [`FENCE_TOLERANCE_MM`] of any edge of a polyline?
fn point_near_polyline(point: Point2, polyline: &[Point2]) -> bool {
    polyline
        .windows(2)
        .any(|edge| point_segment_distance(point, edge[0], edge[1]) <= FENCE_TOLERANCE_MM)
}

/// The four edges of a box from one corner to the opposite one.
fn box_segments(a: Point2, b: Point2) -> Vec<(Point2, Point2)> {
    let top_left = Point2::new(a.x, b.y);
    let bottom_right = Point2::new(b.x, a.y);
    vec![
        (a, top_left),
        (top_left, b),
        (b, bottom_right),
        (bottom_right, a),
    ]
}

/// Consecutive segments of a polyline.
fn polyline_segments(polygon: &[Point2]) -> Vec<(Point2, Point2)> {
    polygon.windows(2).map(|pair| (pair[0], pair[1])).collect()
}

/// Midpoint of a segment.
fn midpoint(a: Point2, b: Point2) -> Point2 {
    Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)
}

/// Do two segments cross or touch?
fn segments_intersect(a: Point2, b: Point2, c: Point2, d: Point2) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);
    if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
        return true;
    }
    collinear_within(a, b, c, o1)
        || collinear_within(a, b, d, o2)
        || collinear_within(c, d, a, o3)
        || collinear_within(c, d, b, o4)
}

/// Signed area of the triangle `p q r`; zero when the three are collinear.
fn orientation(p: Point2, q: Point2, r: Point2) -> f64 {
    (q - p).cross(r - p)
}

/// When `p` is collinear with `a..b` (cross product `cross`), is it inside the
/// segment's bounding box?
fn collinear_within(a: Point2, b: Point2, p: Point2, cross: f64) -> bool {
    cross.abs() <= f64::EPSILON && on_segment(a, b, p)
}

/// Is `p` inside the bounding box of `a..b`? Only meaningful when collinear.
fn on_segment(a: Point2, b: Point2, p: Point2) -> bool {
    p.x >= a.x.min(b.x) - FENCE_TOLERANCE_MM
        && p.x <= a.x.max(b.x) + FENCE_TOLERANCE_MM
        && p.y >= a.y.min(b.y) - FENCE_TOLERANCE_MM
        && p.y <= a.y.max(b.y) + FENCE_TOLERANCE_MM
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::construction::ConstructionEntity;
    use crate::ids::{ConstructionId, LabelId, LevelId, OpeningId, WallId};
    use crate::layer::Layer;
    use crate::level::Level;
    use crate::wall::Wall;

    fn wall(id: u64, start: (f64, f64), end: (f64, f64)) -> Wall {
        Wall::new(
            WallId(id),
            Point2::new(start.0, start.1),
            Point2::new(end.0, end.1),
            200.0,
            2800.0,
        )
    }

    fn window(min: (f64, f64), max: (f64, f64), crossing: bool) -> SelectionMode {
        SelectionMode::Window {
            bounds: Bounds {
                min: Point2::new(min.0, min.1),
                max: Point2::new(max.0, max.1),
            },
            crossing,
        }
    }

    #[test]
    fn a_window_picks_entities_whose_centre_is_inside() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.walls.push(wall(2, (0.0, 5000.0), (4000.0, 5000.0)));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            window((1000.0, -500.0), (3000.0, 500.0), false),
            SelectionFilter::All,
        );
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&EntityId::Wall(WallId(1))));
    }

    #[test]
    fn a_crossing_window_picks_a_wall_it_merely_crosses() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let geometry = PlanGeometry::of_level(&level);

        // The wall's centre (2000, 0) is outside the window, so a containing
        // window misses it but a crossing one still catches the wall band.
        let containing = select(
            &geometry,
            window((500.0, -500.0), (1500.0, 500.0), false),
            SelectionFilter::All,
        );
        assert!(!containing.contains(&EntityId::Wall(WallId(1))));

        let crossing = select(
            &geometry,
            window((500.0, -500.0), (1500.0, 500.0), true),
            SelectionFilter::All,
        );
        assert!(crossing.contains(&EntityId::Wall(WallId(1))));
    }

    #[test]
    fn a_window_far_away_picks_nothing() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            window((10_000.0, 10_000.0), (11_000.0, 11_000.0), true),
            SelectionFilter::All,
        );
        assert!(selected.is_empty());
    }

    #[test]
    fn an_inverted_window_still_selects() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let geometry = PlanGeometry::of_level(&level);
        let inverted = SelectionMode::Window {
            bounds: Bounds {
                min: Point2::new(3000.0, 500.0),
                max: Point2::new(1000.0, -500.0),
            },
            crossing: false,
        };
        let selected = select(&geometry, inverted, SelectionFilter::All);
        assert!(selected.contains(&EntityId::Wall(WallId(1))));
    }

    #[test]
    fn the_filter_restricts_by_entity_type() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.labels.push(PlanLabel::new(
            LabelId(2),
            Point2::new(1000.0, 1000.0),
            "Hall",
        ));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            SelectionMode::All,
            SelectionFilter::EntityType(EntityType::Label),
        );
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&EntityId::Label(LabelId(2))));
    }

    #[test]
    fn the_filter_restricts_by_layer() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.layers.push(Layer::new(LayerId(5), "Annotations"));
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.labels.push(
            PlanLabel::new(LabelId(2), Point2::new(1000.0, 1000.0), "Hall").on_layer(LayerId(5)),
        );
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            SelectionMode::All,
            SelectionFilter::Layer(LayerId(5)),
        );
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&EntityId::Label(LabelId(2))));
    }

    #[test]
    fn a_fence_picks_what_it_crosses() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.walls.push(wall(2, (0.0, 5000.0), (4000.0, 5000.0)));
        let geometry = PlanGeometry::of_level(&level);
        let fence = SelectionMode::Fence {
            points: vec![Point2::new(1000.0, -500.0), Point2::new(1000.0, 500.0)],
        };
        let selected = select(&geometry, fence, SelectionFilter::All);
        assert!(selected.contains(&EntityId::Wall(WallId(1))));
        assert!(!selected.contains(&EntityId::Wall(WallId(2))));
    }

    #[test]
    fn a_label_is_picked_through_its_box() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.labels.push(PlanLabel::new(
            LabelId(2),
            Point2::new(1000.0, 1000.0),
            "Hall",
        ));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            window((1000.0, 1000.0), (1100.0, 1100.0), true),
            SelectionFilter::EntityType(EntityType::Label),
        );
        assert!(selected.contains(&EntityId::Label(LabelId(2))));
    }

    #[test]
    fn construction_guides_are_selectable_by_type() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.construction.push(ConstructionEntity::new(
            ConstructionId(3),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
            },
        ));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            window((1000.0, -500.0), (3000.0, 500.0), true),
            SelectionFilter::EntityType(EntityType::Construction),
        );
        assert!(selected.contains(&EntityId::Construction(ConstructionId(3))));
    }

    #[test]
    fn hidden_construction_guides_are_not_selected() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        let mut guide = ConstructionEntity::new(
            ConstructionId(3),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
            },
        );
        guide.visible = false;
        level.construction.push(guide);
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(&geometry, SelectionMode::All, SelectionFilter::All);
        assert!(selected.is_empty());
    }

    #[test]
    fn a_layer_filter_excludes_construction() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.construction.push(ConstructionEntity::new(
            ConstructionId(3),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
            },
        ));
        let geometry = PlanGeometry::of_level(&level);
        let selected = select(
            &geometry,
            SelectionMode::All,
            SelectionFilter::Layer(LayerId::DEFAULT),
        );
        assert!(selected.is_empty());
    }

    #[test]
    fn pick_stores_the_ids_and_the_mode() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.openings.push(crate::opening::Opening::new(
            OpeningId(2),
            crate::opening::OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        let geometry = PlanGeometry::of_level(&level);

        let mut selection = Selection::pick(&geometry, SelectionMode::All, SelectionFilter::All);
        assert!(selection.contains(EntityId::Wall(WallId(1))));
        assert!(selection.contains(EntityId::Opening(OpeningId(2))));
        assert_eq!(selection.len(), 2);
        assert_eq!(selection.mode, SelectionMode::All);
        assert!(!selection.is_empty());

        selection.clear();
        assert!(selection.is_empty());
    }

    #[test]
    fn filters_accept_the_right_entities() {
        assert!(SelectionFilter::All.accepts(EntityType::Wall, Some(LayerId(2))));
        assert!(SelectionFilter::All.accepts(EntityType::Construction, None));

        let walls_only = SelectionFilter::EntityType(EntityType::Wall);
        assert!(walls_only.accepts(EntityType::Wall, None));
        assert!(!walls_only.accepts(EntityType::Opening, None));

        let on_layer_two = SelectionFilter::Layer(LayerId(2));
        assert!(on_layer_two.accepts(EntityType::Label, Some(LayerId(2))));
        assert!(!on_layer_two.accepts(EntityType::Label, Some(LayerId(3))));
        assert!(!on_layer_two.accepts(EntityType::Construction, None));
    }

    #[test]
    fn entity_type_of_maps_every_id_kind() {
        assert_eq!(EntityType::of(EntityId::Wall(WallId(1))), EntityType::Wall);
        assert_eq!(
            EntityType::of(EntityId::Opening(OpeningId(1))),
            EntityType::Opening
        );
        assert_eq!(
            EntityType::of(EntityId::Dimension(crate::ids::DimensionId(1))),
            EntityType::Dimension
        );
        assert_eq!(
            EntityType::of(EntityId::Label(LabelId(1))),
            EntityType::Label
        );
        assert_eq!(
            EntityType::of(EntityId::Construction(ConstructionId(1))),
            EntityType::Construction
        );
        assert!(!EntityType::Wall.label().is_empty());
    }

    #[test]
    fn selection_round_trips_through_json() {
        let mut selection = Selection::default();
        selection.entity_ids.insert(EntityId::Wall(WallId(1)));
        selection.entity_ids.insert(EntityId::Opening(OpeningId(2)));
        selection.mode = SelectionMode::Window {
            bounds: Bounds {
                min: Point2::new(0.0, 0.0),
                max: Point2::new(10.0, 10.0),
            },
            crossing: true,
        };
        let json = serde_json::to_string(&selection).expect("serialize");
        let back: Selection = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, selection);
    }

    #[test]
    fn selection_modes_and_filters_round_trip_through_json() {
        for mode in [
            SelectionMode::All,
            SelectionMode::Window {
                bounds: Bounds {
                    min: Point2::new(0.0, 0.0),
                    max: Point2::new(10.0, 10.0),
                },
                crossing: false,
            },
            SelectionMode::Fence {
                points: vec![Point2::new(0.0, 0.0), Point2::new(10.0, 10.0)],
            },
        ] {
            let json = serde_json::to_string(&mode).expect("serialize");
            let back: SelectionMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, mode);
        }

        for filter in [
            SelectionFilter::All,
            SelectionFilter::EntityType(EntityType::Dimension),
            SelectionFilter::Layer(LayerId(4)),
        ] {
            let json = serde_json::to_string(&filter).expect("serialize");
            let back: SelectionFilter = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, filter);
        }
    }
}
