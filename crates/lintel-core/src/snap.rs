//! Point snapping while drawing.
//!
//! Snapping is pure: give it the raw pointer position, the candidate anchors
//! (wall ends), the previous picked point and the settings, and it returns
//! where the point should actually land. The canvas draws the marker from the
//! returned `SnapKind`, so what the user sees is exactly what the model gets.
//!
//! Beyond wall ends the snapper understands a richer vocabulary of
//! [`SnapCandidate`] anchors — midpoints, wall intersections, circle centres
//! and quadrants, and the pre-computed [`generate_snap_candidates`] list is
//! what a caller feeds back through [`snap_candidate`]. The older
//! [`snap_point`] entry point is kept for callers that only have bare points;
//! it now builds endpoint candidates and defers to the same priority order.

use serde::{Deserialize, Serialize};

use crate::construction::{ConstructionEntity, ConstructionKind};
use crate::geom::Point2;
use crate::ids::{ConstructionId, WallId};
use crate::level::Level;
use crate::wall::Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SnapResult {
    pub point: Point2,
    pub kind: SnapKind,
}

/// The kind of geometric feature a [`SnapCandidate`] locks onto.
///
/// The order of the variants is also the snapping priority, highest first, as
/// returned by [`SnapCandidateKind::priority`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapCandidateKind {
    /// A wall or guide end.
    Endpoint,
    /// Where two walls' centrelines cross.
    Intersection,
    /// The foot of a perpendicular from the previous point.
    Perpendicular,
    /// A point of tangency from the previous point.
    Tangent,
    /// The middle of a wall or guide.
    Midpoint,
    /// The centre of a circle or arc.
    Center,
    /// A circle or arc quadrant (0°, 90°, 180°, 270°).
    Quadrant,
    /// A point on the drawing grid.
    Grid,
    /// A point constrained to horizontal or vertical from the previous point.
    Ortho,
    /// No constraint: the pointer's own position.
    Free,
}

impl SnapCandidateKind {
    /// The snapping priority: a higher number wins over a lower one when both
    /// are within tolerance. Distances only break ties inside one priority.
    pub fn priority(self) -> u8 {
        match self {
            SnapCandidateKind::Endpoint => 100,
            SnapCandidateKind::Intersection => 90,
            SnapCandidateKind::Perpendicular => 80,
            SnapCandidateKind::Tangent => 70,
            SnapCandidateKind::Midpoint => 60,
            SnapCandidateKind::Center => 50,
            SnapCandidateKind::Quadrant => 40,
            SnapCandidateKind::Grid => 20,
            SnapCandidateKind::Ortho => 10,
            SnapCandidateKind::Free => 0,
        }
    }

    /// Short human-readable name for the snap marker and the status line.
    pub fn label(self) -> &'static str {
        match self {
            SnapCandidateKind::Endpoint => "End",
            SnapCandidateKind::Intersection => "Intersection",
            SnapCandidateKind::Perpendicular => "Perpendicular",
            SnapCandidateKind::Tangent => "Tangent",
            SnapCandidateKind::Midpoint => "Mid",
            SnapCandidateKind::Center => "Centre",
            SnapCandidateKind::Quadrant => "Quadrant",
            SnapCandidateKind::Grid => "Grid",
            SnapCandidateKind::Ortho => "Ortho",
            SnapCandidateKind::Free => "Free",
        }
    }
}

/// What a candidate was derived from, so the UI can highlight the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapSource {
    /// One end of a wall.
    WallEndpoint(WallId),
    /// The midpoint of a wall.
    WallMidpoint(WallId),
    /// The centre of a wall. Reserved for callers that offer one.
    WallCenter(WallId),
    /// The crossing of two walls.
    Intersection(WallId, WallId),
    /// A point offered by a construction guide.
    Construction(ConstructionId),
    /// A snap computed at pick time (grid, ortho or free) with no source entity.
    Computed,
}

/// One point the snapper may lock onto, with its kind, priority and source.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SnapCandidate {
    pub point: Point2,
    pub kind: SnapCandidateKind,
    /// Cached [`SnapCandidateKind::priority`]; the two never disagree.
    pub priority: u8,
    pub source: SnapSource,
}

impl SnapCandidate {
    /// A candidate whose priority is taken from its kind.
    pub fn new(point: Point2, kind: SnapCandidateKind, source: SnapSource) -> Self {
        Self {
            point,
            kind,
            priority: kind.priority(),
            source,
        }
    }
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

/// Snap against bare anchor points.
///
/// This is the original entry point, kept for callers that only hold a list of
/// points. It offers each anchor as an [`SnapCandidateKind::Endpoint`] and
/// defers to [`snap_candidate`], so the priority order is the same as the rich
/// candidate path.
pub fn snap_point(
    raw: Point2,
    anchors: &[Point2],
    origin: Option<Point2>,
    settings: &SnapSettings,
) -> SnapResult {
    let mut candidates = Vec::with_capacity(anchors.len());
    for &point in anchors {
        candidates.push(SnapCandidate::new(
            point,
            SnapCandidateKind::Endpoint,
            SnapSource::Computed,
        ));
    }
    let chosen = snap_candidate(raw, &candidates, origin, settings);
    SnapResult {
        point: chosen.point,
        kind: legacy_kind(chosen.kind),
    }
}

/// Snap against a pre-computed candidate list.
///
/// The candidate with the highest [`SnapCandidateKind::priority`] within
/// [`SnapSettings::anchor_tolerance`] wins; distance only breaks ties inside a
/// priority. With no candidate in range the point is straightened from `origin`
/// (when ortho is on) and then rounded to the grid, exactly like the legacy
/// path. The returned candidate always carries a [`SnapSource`], so the caller
/// can highlight what was picked.
pub fn snap_candidate(
    raw: Point2,
    candidates: &[SnapCandidate],
    origin: Option<Point2>,
    settings: &SnapSettings,
) -> SnapCandidate {
    if !raw.is_finite() {
        return SnapCandidate::new(
            Point2::ORIGIN,
            SnapCandidateKind::Free,
            SnapSource::Computed,
        );
    }

    // 1. The highest-priority candidate in range, nearest to break ties.
    let mut best: Option<&SnapCandidate> = None;
    for candidate in candidates {
        if candidate.point.distance(raw) > settings.anchor_tolerance {
            continue;
        }
        let wins = match best {
            None => true,
            Some(current) => ranks_above(candidate, current, raw),
        };
        if wins {
            best = Some(candidate);
        }
    }
    if let Some(candidate) = best {
        return *candidate;
    }

    // 2. Straight from the last point, if asked for.
    let mut point = raw;
    let mut kind = SnapCandidateKind::Free;
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
            kind = SnapCandidateKind::Ortho;
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
        if kind == SnapCandidateKind::Free {
            kind = SnapCandidateKind::Grid;
        }
    }

    SnapCandidate::new(point, kind, SnapSource::Computed)
}

/// Pre-compute every candidate a level offers: wall ends, wall midpoints, wall
/// intersections and construction-guide points (ends, centres, quadrants).
///
/// Perpendicular and tangent snaps depend on the previous picked point, so they
/// are not pre-computable; a caller builds those at pick time. A zero (or
/// non-finite) anchor tolerance means the snapper can never lock onto a
/// candidate, so the list is empty.
pub fn generate_snap_candidates(level: &Level, settings: &SnapSettings) -> Vec<SnapCandidate> {
    if !settings.anchor_tolerance.is_finite() || settings.anchor_tolerance <= 0.0 {
        return Vec::new();
    }

    let mut candidates = Vec::new();

    for wall in &level.walls {
        candidates.push(SnapCandidate::new(
            wall.start,
            SnapCandidateKind::Endpoint,
            SnapSource::WallEndpoint(wall.id),
        ));
        candidates.push(SnapCandidate::new(
            wall.end,
            SnapCandidateKind::Endpoint,
            SnapSource::WallEndpoint(wall.id),
        ));
        candidates.push(SnapCandidate::new(
            wall.midpoint(),
            SnapCandidateKind::Midpoint,
            SnapSource::WallMidpoint(wall.id),
        ));
    }

    for (index, first) in level.walls.iter().enumerate() {
        for second in level.walls.iter().skip(index + 1) {
            let crossing = segment_intersection(first.start, first.end, second.start, second.end);
            if let Some(point) = crossing {
                candidates.push(SnapCandidate::new(
                    point,
                    SnapCandidateKind::Intersection,
                    SnapSource::Intersection(first.id, second.id),
                ));
            }
        }
    }

    for entity in &level.construction {
        if !entity.visible || !entity.snap_enabled {
            continue;
        }
        let source = SnapSource::Construction(entity.id);
        for (point, kind) in construction_candidates(entity) {
            candidates.push(SnapCandidate::new(point, kind, source));
        }
    }

    candidates
}

/// Reduce the four-state legacy [`SnapKind`] to the richer candidate kind.
fn legacy_kind(kind: SnapCandidateKind) -> SnapKind {
    match kind {
        SnapCandidateKind::Grid => SnapKind::Grid,
        SnapCandidateKind::Ortho => SnapKind::Ortho,
        SnapCandidateKind::Free => SnapKind::Free,
        _ => SnapKind::Endpoint,
    }
}

/// Does `candidate` rank above `current`? Priority first, distance as the tie.
fn ranks_above(candidate: &SnapCandidate, current: &SnapCandidate, raw: Point2) -> bool {
    if candidate.priority != current.priority {
        return candidate.priority > current.priority;
    }
    candidate.point.distance(raw) < current.point.distance(raw)
}

/// The geometric points a construction guide contributes, as candidate kinds.
fn construction_candidates(entity: &ConstructionEntity) -> Vec<(Point2, SnapCandidateKind)> {
    match &entity.kind {
        ConstructionKind::Line { start, end } => vec![
            (*start, SnapCandidateKind::Endpoint),
            (*end, SnapCandidateKind::Endpoint),
            (midpoint(*start, *end), SnapCandidateKind::Midpoint),
        ],
        ConstructionKind::InfiniteLine { point, .. } => {
            vec![(*point, SnapCandidateKind::Endpoint)]
        }
        ConstructionKind::Ray { origin, .. } => {
            vec![(*origin, SnapCandidateKind::Endpoint)]
        }
        ConstructionKind::Circle { center, radius } => {
            let mut points = vec![(*center, SnapCandidateKind::Center)];
            for quadrant in quadrant_points(*center, *radius) {
                points.push((quadrant, SnapCandidateKind::Quadrant));
            }
            points
        }
        ConstructionKind::Arc {
            center,
            radius,
            start_deg,
            end_deg,
        } => {
            let start_arc = point_on_circle(*center, *radius, *start_deg);
            let end_arc = point_on_circle(*center, *radius, *end_deg);
            vec![
                (*center, SnapCandidateKind::Center),
                (start_arc, SnapCandidateKind::Endpoint),
                (end_arc, SnapCandidateKind::Endpoint),
            ]
        }
    }
}

/// Point where two segments cross strictly inside both, if any.
fn segment_intersection(a1: Point2, a2: Point2, b1: Point2, b2: Point2) -> Option<Point2> {
    let r = a2 - a1;
    let s = b2 - b1;
    let denominator = r.cross(s);
    if !denominator.is_finite() || denominator.abs() <= f64::EPSILON {
        return None;
    }
    let delta = b1 - a1;
    let t = delta.cross(s) / denominator;
    let u = delta.cross(r) / denominator;
    if !(0.0..1.0).contains(&t) || !(0.0..1.0).contains(&u) {
        return None;
    }
    let point = a1 + r * t;
    point.is_finite().then_some(point)
}

/// The four quadrant points of a circle, at 0°, 90°, 180° and 270°.
fn quadrant_points(centre: Point2, radius: f64) -> [Point2; 4] {
    [
        point_on_circle(centre, radius, 0.0),
        point_on_circle(centre, radius, 90.0),
        point_on_circle(centre, radius, 180.0),
        point_on_circle(centre, radius, 270.0),
    ]
}

/// A point at `degrees` anticlockwise from +x on a circle.
fn point_on_circle(centre: Point2, radius: f64, degrees: f64) -> Point2 {
    let radians = degrees.to_radians();
    Point2::new(
        centre.x + radius * radians.cos(),
        centre.y + radius * radians.sin(),
    )
}

/// Midpoint of a segment.
fn midpoint(a: Point2, b: Point2) -> Point2 {
    Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::LevelId;
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
        let walls = vec![wall(1, (0.0, 0.0), (4000.0, 0.0))];
        let anchors = wall_anchors(&walls);
        assert_eq!(anchors.len(), 2);
        assert!(anchors.contains(&Point2::new(0.0, 0.0)));
        assert!(anchors.contains(&Point2::new(4000.0, 0.0)));
    }

    #[test]
    fn priorities_match_the_spec() {
        assert_eq!(SnapCandidateKind::Endpoint.priority(), 100);
        assert_eq!(SnapCandidateKind::Intersection.priority(), 90);
        assert_eq!(SnapCandidateKind::Perpendicular.priority(), 80);
        assert_eq!(SnapCandidateKind::Tangent.priority(), 70);
        assert_eq!(SnapCandidateKind::Midpoint.priority(), 60);
        assert_eq!(SnapCandidateKind::Center.priority(), 50);
        assert_eq!(SnapCandidateKind::Quadrant.priority(), 40);
        assert_eq!(SnapCandidateKind::Grid.priority(), 20);
        assert_eq!(SnapCandidateKind::Ortho.priority(), 10);
        assert_eq!(SnapCandidateKind::Free.priority(), 0);
    }

    #[test]
    fn a_new_candidate_caches_its_priority_and_has_a_label() {
        let candidate = SnapCandidate::new(
            Point2::new(1.0, 2.0),
            SnapCandidateKind::Midpoint,
            SnapSource::Computed,
        );
        assert_eq!(candidate.priority, SnapCandidateKind::Midpoint.priority());
        assert_eq!(candidate.point, Point2::new(1.0, 2.0));
        assert!(!candidate.kind.label().is_empty());
    }

    #[test]
    fn candidates_include_wall_ends_midpoints_and_intersections() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.walls.push(wall(2, (1000.0, -2000.0), (1000.0, 2000.0)));
        let candidates = generate_snap_candidates(&level, &settings());
        let points: Vec<(Point2, SnapCandidateKind)> =
            candidates.iter().map(|c| (c.point, c.kind)).collect();
        assert!(points.contains(&(Point2::new(0.0, 0.0), SnapCandidateKind::Endpoint)));
        assert!(points.contains(&(Point2::new(2000.0, 0.0), SnapCandidateKind::Midpoint)));
        assert!(points.contains(&(Point2::new(1000.0, 0.0), SnapCandidateKind::Intersection)));

        let start = candidates
            .iter()
            .find(|c| c.point == Point2::new(0.0, 0.0))
            .expect("endpoint candidate");
        assert_eq!(start.source, SnapSource::WallEndpoint(WallId(1)));

        let crossing = candidates
            .iter()
            .find(|c| c.kind == SnapCandidateKind::Intersection)
            .expect("intersection candidate");
        assert_eq!(
            crossing.source,
            SnapSource::Intersection(WallId(1), WallId(2))
        );
    }

    #[test]
    fn construction_circles_offer_centres_and_quadrants() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.construction.push(ConstructionEntity::new(
            ConstructionId(7),
            ConstructionKind::Circle {
                center: Point2::new(0.0, 0.0),
                radius: 1000.0,
            },
        ));
        let candidates = generate_snap_candidates(&level, &settings());
        let points: Vec<(Point2, SnapCandidateKind)> =
            candidates.iter().map(|c| (c.point, c.kind)).collect();
        assert!(points.contains(&(Point2::new(0.0, 0.0), SnapCandidateKind::Center)));
        assert!(points.contains(&(Point2::new(1000.0, 0.0), SnapCandidateKind::Quadrant)));
    }

    #[test]
    fn hidden_construction_guides_offer_no_candidates() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        let mut guide = ConstructionEntity::new(
            ConstructionId(1),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(1000.0, 0.0),
            },
        );
        guide.visible = false;
        level.construction.push(guide);
        assert!(generate_snap_candidates(&level, &settings()).is_empty());
    }

    #[test]
    fn zero_tolerance_generates_no_candidates() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let mut options = settings();
        options.anchor_tolerance = 0.0;
        assert!(generate_snap_candidates(&level, &options).is_empty());
    }

    #[test]
    fn a_higher_priority_kind_beats_a_nearer_lower_one() {
        let midpoint = SnapCandidate::new(
            Point2::new(10.0, 0.0),
            SnapCandidateKind::Midpoint,
            SnapSource::Computed,
        );
        let endpoint = SnapCandidate::new(
            Point2::new(60.0, 0.0),
            SnapCandidateKind::Endpoint,
            SnapSource::Computed,
        );
        let mut options = settings();
        options.grid_enabled = false;
        options.anchor_tolerance = 100.0;
        let candidates = [midpoint, endpoint];
        let chosen = snap_candidate(Point2::new(0.0, 0.0), &candidates, None, &options);
        assert_eq!(chosen.kind, SnapCandidateKind::Endpoint);
        assert_eq!(chosen.point, Point2::new(60.0, 0.0));
    }

    #[test]
    fn snap_candidate_falls_back_to_grid_and_ortho() {
        let options = settings();
        let grid = snap_candidate(Point2::new(1240.0, 380.0), &[], None, &options);
        assert_eq!(grid.kind, SnapCandidateKind::Grid);
        assert_eq!(grid.point, Point2::new(1000.0, 500.0));

        let mut ortho_options = settings();
        ortho_options.ortho = true;
        ortho_options.grid_enabled = false;
        let ortho = snap_candidate(
            Point2::new(3000.0, 200.0),
            &[],
            Some(Point2::ORIGIN),
            &ortho_options,
        );
        assert_eq!(ortho.kind, SnapCandidateKind::Ortho);
        assert_eq!(ortho.point, Point2::new(3000.0, 0.0));
    }

    #[test]
    fn every_candidate_kind_has_a_label() {
        let kinds = [
            SnapCandidateKind::Endpoint,
            SnapCandidateKind::Intersection,
            SnapCandidateKind::Perpendicular,
            SnapCandidateKind::Tangent,
            SnapCandidateKind::Midpoint,
            SnapCandidateKind::Center,
            SnapCandidateKind::Quadrant,
            SnapCandidateKind::Grid,
            SnapCandidateKind::Ortho,
            SnapCandidateKind::Free,
        ];
        for kind in kinds {
            assert!(!kind.label().is_empty());
        }
    }

    #[test]
    fn candidates_and_sources_round_trip_through_json() {
        let candidate = SnapCandidate::new(
            Point2::new(1.5, -2.5),
            SnapCandidateKind::Intersection,
            SnapSource::Intersection(WallId(1), WallId(2)),
        );
        let json = serde_json::to_string(&candidate).expect("serialize");
        let back: SnapCandidate = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, candidate);

        for source in [
            SnapSource::WallEndpoint(WallId(1)),
            SnapSource::WallMidpoint(WallId(1)),
            SnapSource::WallCenter(WallId(1)),
            SnapSource::Intersection(WallId(1), WallId(2)),
            SnapSource::Construction(ConstructionId(3)),
            SnapSource::Computed,
        ] {
            let json = serde_json::to_string(&source).expect("serialize");
            let back: SnapSource = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, source);
        }
    }

    #[test]
    fn snap_settings_round_trip_and_default_from_a_partial_object() {
        let options = SnapSettings::default();
        let json = serde_json::to_string(&options).expect("serialize");
        let back: SnapSettings = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, options);

        let partial: SnapSettings = serde_json::from_str("{}").expect("empty object");
        assert_eq!(partial, SnapSettings::default());
    }
}
