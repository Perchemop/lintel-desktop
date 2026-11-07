//! Rooms derived from the wall centrelines, never stored.
//!
//! A room is found every time [`detect_rooms`] runs: the walls form a planar
//! graph, its faces are traversed, and every face that encloses a real area
//! becomes a [`Room`]. Because nothing is cached, moving a wall can never
//! leave a stale room behind — the next call simply sees the new geometry.

use serde::{Deserialize, Serialize};

use crate::geom::{Point2, closest_point_on_segment};
use crate::wall::Wall;

/// Enclosed areas smaller than this are not reported, in square millimetres (0.5 m²).
pub const MIN_ROOM_AREA_MM2: f64 = 500_000.0;

/// Wall points closer than this are treated as the same node, in millimetres.
pub const NODE_TOLERANCE: f64 = 0.5;

/// A closed area bounded by the wall centrelines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    /// Boundary polygon, anticlockwise; the first point is NOT repeated.
    pub polygon: Vec<Point2>,
    /// Enclosed area, square millimetres.
    pub area_mm2: f64,
    /// Boundary length, millimetres.
    pub perimeter_mm: f64,
    /// Centroid of the area (not the average of the vertices).
    pub centroid: Point2,
}

impl Room {
    /// Area in square metres.
    pub fn area_m2(&self) -> f64 {
        self.area_mm2 / 1_000_000.0
    }

    /// Short caption for drawing, e.g. `"12.40 m²"` (uses the Unicode superscript).
    pub fn area_label(&self) -> String {
        format!("{:.2} m²", self.area_m2())
    }

    /// Ray-casting point-in-polygon test. Points exactly on an edge may go either way.
    pub fn contains(&self, point: Point2) -> bool {
        if self.polygon.len() < 3 || !point.is_finite() {
            return false;
        }
        let mut inside = false;
        let mut previous = self.polygon.len() - 1;
        for (index, current) in self.polygon.iter().enumerate() {
            let other = self.polygon[previous];
            let straddles = (current.y > point.y) != (other.y > point.y);
            if straddles {
                let t = (point.y - current.y) / (other.y - current.y);
                let crossing_x = current.x + t * (other.x - current.x);
                if point.x < crossing_x {
                    inside = !inside;
                }
            }
            previous = index;
        }
        inside
    }
}

/// A directed edge between two canonical nodes.
///
/// Half-edges are always pushed in pairs, so the reverse of edge `h` is
/// `h ^ 1`.
#[derive(Debug, Clone, Copy)]
struct HalfEdge {
    from: usize,
    to: usize,
}

/// Detect every enclosed area bounded by the walls' centrelines.
pub fn detect_rooms(walls: &[Wall]) -> Vec<Room> {
    let usable: Vec<&Wall> = walls.iter().filter(|wall| is_usable(wall)).collect();
    if usable.is_empty() {
        return Vec::new();
    }

    let nodes = canonical_nodes(&usable);
    if nodes.len() < 3 {
        return Vec::new();
    }

    let edges = split_walls(&usable, &nodes);
    // A closed polygon needs at least a triangle, so six half-edges.
    if edges.len() < 6 {
        return Vec::new();
    }

    let outgoing = sorted_outgoing(&nodes, &edges);
    let walks = trace_faces(&edges, &outgoing);
    faces_to_rooms(walks, &edges, &nodes)
}

/// True when a wall has finite geometry and a centreline long enough to split.
fn is_usable(wall: &Wall) -> bool {
    wall.start.is_finite() && wall.end.is_finite() && wall.start.distance(wall.end) > NODE_TOLERANCE
}

/// Every point where walls meet or cross, with duplicates merged.
fn canonical_nodes(walls: &[&Wall]) -> Vec<Point2> {
    let mut candidates: Vec<Point2> = Vec::with_capacity(walls.len() * 2);
    for wall in walls {
        candidates.push(wall.start);
        candidates.push(wall.end);
    }

    // A wall endpoint landing on another wall's interior is a T junction.
    for (index, wall) in walls.iter().enumerate() {
        for (other_index, other) in walls.iter().enumerate() {
            if index == other_index {
                continue;
            }
            for endpoint in [wall.start, wall.end] {
                if let Some(node) = t_junction(endpoint, other) {
                    candidates.push(node);
                }
            }
        }
    }

    // A proper crossing splits both walls at the intersection point.
    for (index, first) in walls.iter().enumerate() {
        for second in walls.iter().skip(index + 1) {
            if let Some(node) = proper_crossing(first, second) {
                candidates.push(node);
            }
        }
    }

    merge_nodes(&candidates)
}

/// The projection of `endpoint` onto `wall`, when it lands strictly inside it
/// and is closer than [`NODE_TOLERANCE`].
fn t_junction(endpoint: Point2, wall: &Wall) -> Option<Point2> {
    let length = wall.start.distance(wall.end);
    if !length.is_finite() || length <= NODE_TOLERANCE {
        return None;
    }
    let (t, closest) = closest_point_on_segment(endpoint, wall.start, wall.end);
    let along = t * length;
    if along <= NODE_TOLERANCE || length - along <= NODE_TOLERANCE {
        return None;
    }
    let touches = closest.is_finite() && closest.distance(endpoint) <= NODE_TOLERANCE;
    touches.then_some(closest)
}

/// The point where two segments cross strictly inside both, if any.
fn proper_crossing(first: &Wall, second: &Wall) -> Option<Point2> {
    let r = first.end - first.start;
    let s = second.end - second.start;
    let denominator = r.x * s.y - r.y * s.x;
    let scale = r.length() * s.length();
    if !denominator.is_finite() || denominator.abs() <= f64::EPSILON * scale.max(1.0) {
        return None;
    }
    let delta = second.start - first.start;
    let t = (delta.x * s.y - delta.y * s.x) / denominator;
    let u = (delta.x * r.y - delta.y * r.x) / denominator;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
        return None;
    }
    let first_length = r.length();
    let second_length = s.length();
    let first_interior =
        t * first_length > NODE_TOLERANCE && (1.0 - t) * first_length > NODE_TOLERANCE;
    let second_interior =
        u * second_length > NODE_TOLERANCE && (1.0 - u) * second_length > NODE_TOLERANCE;
    if !first_interior || !second_interior {
        return None;
    }
    let point = first.start + r * t;
    point.is_finite().then_some(point)
}

/// Collapse candidates that sit within [`NODE_TOLERANCE`] of each other.
fn merge_nodes(candidates: &[Point2]) -> Vec<Point2> {
    let mut merged: Vec<Point2> = Vec::new();
    for &candidate in candidates {
        if !candidate.is_finite() {
            continue;
        }
        let known = merged
            .iter()
            .any(|node| node.distance(candidate) <= NODE_TOLERANCE);
        if !known {
            merged.push(candidate);
        }
    }
    merged
}

/// Split every wall at the nodes lying on it and add both directions of each
/// resulting segment.
fn split_walls(walls: &[&Wall], nodes: &[Point2]) -> Vec<HalfEdge> {
    let mut edges: Vec<HalfEdge> = Vec::new();
    for wall in walls {
        let mut hits: Vec<(f64, usize)> = Vec::new();
        for (index, node) in nodes.iter().enumerate() {
            let (t, closest) = closest_point_on_segment(*node, wall.start, wall.end);
            if closest.distance(*node) <= NODE_TOLERANCE {
                hits.push((t, index));
            }
        }
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        hits.dedup_by_key(|hit| hit.1);
        for pair in hits.windows(2) {
            let from = pair[0].1;
            let to = pair[1].1;
            if nodes[from].distance(nodes[to]) <= NODE_TOLERANCE {
                continue;
            }
            edges.push(HalfEdge { from, to });
            edges.push(HalfEdge { from: to, to: from });
        }
    }
    edges
}

/// The outgoing half-edges of every node, sorted by the angle of the edge.
fn sorted_outgoing(nodes: &[Point2], edges: &[HalfEdge]) -> Vec<Vec<usize>> {
    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); nodes.len()];
    for (index, edge) in edges.iter().enumerate() {
        outgoing[edge.from].push(index);
    }
    for list in &mut outgoing {
        list.sort_by(|&a, &b| edge_angle(nodes, edges, a).total_cmp(&edge_angle(nodes, edges, b)));
    }
    outgoing
}

/// Direction of a half-edge in radians, from `atan2`.
fn edge_angle(nodes: &[Point2], edges: &[HalfEdge], edge: usize) -> f64 {
    let direction = nodes[edges[edge].to] - nodes[edges[edge].from];
    direction.y.atan2(direction.x)
}

/// Collect the closed walks of half-edges. Each half-edge belongs to exactly
/// one walk because the next-edge rule is a permutation of the half-edges.
fn trace_faces(edges: &[HalfEdge], outgoing: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut used = vec![false; edges.len()];
    let mut walks = Vec::new();
    let mut start = 0;
    while start < edges.len() {
        if used[start] {
            start += 1;
            continue;
        }
        let mut walk = Vec::new();
        let mut current = start;
        let mut closed = false;
        loop {
            used[current] = true;
            walk.push(current);
            current = next_half_edge(current, edges, outgoing);
            if current == start {
                closed = true;
                break;
            }
            if used[current] || walk.len() > edges.len() {
                break;
            }
        }
        if closed && walk.len() >= 3 {
            walks.push(walk);
        }
        start += 1;
    }
    walks
}

/// The next half-edge clockwise from the reverse of `current` around its
/// destination node, wrapping around when no smaller angle exists.
fn next_half_edge(current: usize, edges: &[HalfEdge], outgoing: &[Vec<usize>]) -> usize {
    let reverse = current ^ 1;
    let node = edges[reverse].from;
    let list = &outgoing[node];
    let Some(position) = list.iter().position(|&edge| edge == reverse) else {
        return reverse;
    };
    let previous = (position + list.len() - 1) % list.len();
    list[previous]
}

/// Turn the traced walks into rooms, dropping the outermost face and any
/// enclosed area below [`MIN_ROOM_AREA_MM2`].
fn faces_to_rooms(walks: Vec<Vec<usize>>, edges: &[HalfEdge], nodes: &[Point2]) -> Vec<Room> {
    let mut faces: Vec<(Vec<Point2>, f64)> = Vec::new();
    for walk in walks {
        let polygon: Vec<Point2> = walk.iter().map(|&edge| nodes[edges[edge].from]).collect();
        let area = signed_area(&polygon);
        if polygon.len() >= 3 && area.is_finite() {
            faces.push((polygon, area));
        }
    }
    if faces.is_empty() {
        return Vec::new();
    }

    // With the "next edge clockwise" rule used by `next_half_edge`, every
    // bounded face is traced anticlockwise (positive shoelace area) and the
    // unbounded face of each connected group of walls is traced clockwise
    // (negative). Dropping *all* negative walks is therefore required: a plan
    // with two separate rooms has two of them, not one.
    let mut rooms: Vec<Room> = Vec::new();
    for (polygon, area) in faces {
        if area <= 0.0 || area < MIN_ROOM_AREA_MM2 {
            continue;
        }
        let perimeter_mm = polygon_perimeter(&polygon);
        let centroid = polygon_centroid(&polygon, area);
        if !area.is_finite() || !perimeter_mm.is_finite() || !centroid.is_finite() {
            continue;
        }
        rooms.push(Room {
            polygon,
            area_mm2: area,
            perimeter_mm,
            centroid,
        });
    }
    rooms.sort_by(|a, b| b.area_mm2.total_cmp(&a.area_mm2));
    rooms
}

/// Shoelace area of a polygon; positive when the vertices run anticlockwise.
fn signed_area(polygon: &[Point2]) -> f64 {
    if polygon.len() < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    for (index, a) in polygon.iter().enumerate() {
        let b = polygon[(index + 1) % polygon.len()];
        sum += a.x * b.y - b.x * a.y;
    }
    sum * 0.5
}

/// Closed boundary length, including the edge from the last point to the first.
fn polygon_perimeter(polygon: &[Point2]) -> f64 {
    if polygon.len() < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    for (index, a) in polygon.iter().enumerate() {
        let b = polygon[(index + 1) % polygon.len()];
        sum += a.distance(b);
    }
    sum
}

/// Area centroid of a polygon with signed area `signed`.
fn polygon_centroid(polygon: &[Point2], signed: f64) -> Point2 {
    if polygon.len() < 3 || !signed.is_finite() || signed.abs() < f64::EPSILON {
        return vertex_mean(polygon);
    }
    let mut x = 0.0;
    let mut y = 0.0;
    for (index, a) in polygon.iter().enumerate() {
        let b = polygon[(index + 1) % polygon.len()];
        let cross = a.x * b.y - b.x * a.y;
        x += (a.x + b.x) * cross;
        y += (a.y + b.y) * cross;
    }
    let factor = 1.0 / (6.0 * signed);
    Point2::new(x * factor, y * factor)
}

/// Fallback centroid when the area is too small to divide by.
fn vertex_mean(polygon: &[Point2]) -> Point2 {
    if polygon.is_empty() {
        return Point2::ORIGIN;
    }
    let mut x = 0.0;
    let mut y = 0.0;
    for point in polygon {
        x += point.x;
        y += point.y;
    }
    let count = polygon.len() as f64;
    Point2::new(x / count, y / count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::WallId;
    use crate::wall::{DEFAULT_WALL_HEIGHT, DEFAULT_WALL_THICKNESS};

    fn wall(index: u64, start: (f64, f64), end: (f64, f64)) -> Wall {
        Wall::new(
            WallId(index),
            Point2::new(start.0, start.1),
            Point2::new(end.0, end.1),
            DEFAULT_WALL_THICKNESS,
            DEFAULT_WALL_HEIGHT,
        )
    }

    /// Four walls enclosing an axis-aligned rectangle, anticlockwise.
    fn rectangle_walls(min: (f64, f64), max: (f64, f64)) -> Vec<Wall> {
        vec![
            wall(1, (min.0, min.1), (max.0, min.1)),
            wall(2, (max.0, min.1), (max.0, max.1)),
            wall(3, (max.0, max.1), (min.0, max.1)),
            wall(4, (min.0, max.1), (min.0, min.1)),
        ]
    }

    fn approximately(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= expected.abs().max(1.0) * 1e-6
    }

    /// Two rooms that do not touch are two separate groups of walls. Each
    /// group has its own unbounded face, so a detector that drops only the
    /// single largest outside walk reports a duplicate room.
    #[test]
    fn two_disconnected_rooms_are_both_reported_exactly_once() {
        let mut walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        walls.extend(rectangle_walls((10_000.0, 0.0), (15_000.0, 2000.0)));
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 2, "rooms: {rooms:?}");
        assert!(approximately(rooms[0].area_mm2, 16_000_000.0));
        assert!(approximately(rooms[1].area_mm2, 10_000_000.0));
    }

    #[test]
    fn empty_input_has_no_rooms() {
        assert!(detect_rooms(&[]).is_empty());
    }

    #[test]
    fn a_square_box_is_one_room() {
        let walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 1);
        let room = &rooms[0];
        assert!(approximately(room.area_mm2, 16_000_000.0));
        assert!(approximately(room.perimeter_mm, 16_000.0));
        assert!(approximately(room.centroid.x, 2000.0));
        assert!(approximately(room.centroid.y, 2000.0));
        assert_eq!(room.polygon.len(), 4);
        assert!(signed_area(&room.polygon) > 0.0, "polygon is anticlockwise");
    }

    #[test]
    fn an_l_shaped_room_has_the_notched_area() {
        // A 6000 x 6000 square with a 3000 x 3000 notch removed.
        let walls = vec![
            wall(1, (0.0, 0.0), (6000.0, 0.0)),
            wall(2, (6000.0, 0.0), (6000.0, 6000.0)),
            wall(3, (6000.0, 6000.0), (3000.0, 6000.0)),
            wall(4, (3000.0, 6000.0), (3000.0, 3000.0)),
            wall(5, (3000.0, 3000.0), (0.0, 3000.0)),
            wall(6, (0.0, 3000.0), (0.0, 0.0)),
        ];
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 1);
        assert!(approximately(rooms[0].area_mm2, 27_000_000.0));
        assert_eq!(rooms[0].polygon.len(), 6);
    }

    #[test]
    fn a_dividing_wall_makes_two_rooms() {
        let mut walls = rectangle_walls((0.0, 0.0), (6000.0, 4000.0));
        walls.push(wall(5, (3000.0, 0.0), (3000.0, 4000.0)));
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 2);
        for room in &rooms {
            assert!(approximately(room.area_mm2, 12_000_000.0));
        }
        let mut centres: Vec<f64> = rooms.iter().map(|room| room.centroid.x).collect();
        centres.sort_by(f64::total_cmp);
        assert!(approximately(centres[0], 1500.0));
        assert!(approximately(centres[1], 4500.0));
    }

    #[test]
    fn an_open_shape_encloses_nothing() {
        let walls = vec![
            wall(1, (0.0, 0.0), (4000.0, 0.0)),
            wall(2, (4000.0, 0.0), (4000.0, 4000.0)),
            wall(3, (4000.0, 4000.0), (0.0, 4000.0)),
            // The fourth side is missing.
        ];
        assert!(detect_rooms(&walls).is_empty());
    }

    #[test]
    fn a_dangling_stub_keeps_the_room() {
        let mut walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        walls.push(wall(5, (2000.0, 0.0), (2000.0, 1000.0)));
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 1);
        assert!(approximately(rooms[0].area_mm2, 16_000_000.0));
    }

    #[test]
    fn crossing_walls_enclose_nothing() {
        let walls = vec![
            wall(1, (0.0, 0.0), (4000.0, 4000.0)),
            wall(2, (0.0, 4000.0), (4000.0, 0.0)),
        ];
        assert!(detect_rooms(&walls).is_empty());
    }

    #[test]
    fn contains_matches_the_polygon() {
        let walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        let rooms = detect_rooms(&walls);
        let room = rooms.first().expect("room");
        assert!(room.contains(room.centroid));
        assert!(room.contains(Point2::new(100.0, 100.0)));
        assert!(!room.contains(Point2::new(9000.0, 9000.0)));
        assert!(!room.contains(Point2::new(-500.0, 2000.0)));
    }

    #[test]
    fn area_label_uses_two_decimals_and_square_metres() {
        let walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        let rooms = detect_rooms(&walls);
        let label = rooms[0].area_label();
        assert_eq!(label, "16.00 m²");
        assert!(label.ends_with("m²"));
    }

    #[test]
    fn non_finite_walls_are_ignored() {
        let mut walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        walls.push(wall(5, (f64::NAN, 0.0), (f64::INFINITY, 4000.0)));
        let rooms = detect_rooms(&walls);
        assert_eq!(rooms.len(), 1);
    }

    #[test]
    fn room_round_trips_through_json() {
        let walls = rectangle_walls((0.0, 0.0), (4000.0, 4000.0));
        let original = detect_rooms(&walls).remove(0);
        let json = serde_json::to_string(&original).expect("serialize");
        let back: Room = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }
}
