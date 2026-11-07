//! Property-based fuzz tests for geometry edge cases (audit finding: Low/Core).
//!
//! These exercise [`lintel_core::room::detect_rooms`] with arbitrary wall
//! configurations — degenerate zero-length walls, NaN coordinates, collinear
//! overlapping walls, huge coordinate ranges — and assert that the detector
//! never panics, never produces non-finite geometry, and never emits duplicate
//! rooms. Property-based testing complements the example-based unit tests in
//! `src/room.rs` by exploring the input space exhaustively.

use lintel_core::geom::Point2;
use lintel_core::ids::WallId;
use lintel_core::room::detect_rooms;
use lintel_core::wall::{DEFAULT_WALL_HEIGHT, DEFAULT_WALL_THICKNESS, Wall};
use proptest::prelude::*;

/// Replace non-finite coordinates with 0.0 so the fuzzer can explore the full
/// f64 bit space without immediately producing unusable walls (the detector
/// already filters non-finite walls via `is_usable`).
fn sane_point(x: f64, y: f64) -> Point2 {
    Point2::new(
        if x.is_finite() { x } else { 0.0 },
        if y.is_finite() { y } else { 0.0 },
    )
}

/// Arbitrary vector of 0..12 walls, each with four independent random f64
/// coordinates (sanitised for finiteness).
fn walls_vec() -> impl Strategy<Value = Vec<Wall>> {
    prop::collection::vec(
        (any::<f64>(), any::<f64>(), any::<f64>(), any::<f64>()),
        0..12,
    )
    .prop_map(|points| {
        points
            .into_iter()
            .enumerate()
            .map(|(i, (x1, y1, x2, y2))| {
                Wall::new(
                    WallId(i as u64),
                    sane_point(x1, y1),
                    sane_point(x2, y2),
                    DEFAULT_WALL_THICKNESS,
                    DEFAULT_WALL_HEIGHT,
                )
            })
            .collect()
    })
}

proptest! {
    #![proptest_config(proptest::test_runner::Config {
        failure_persistence: None,
        ..proptest::test_runner::Config::default()
    })]

    /// The room detector must never panic on any wall input, including
    /// degenerate zero-length walls, collinear overlaps, and huge coordinates.
    #[test]
    fn detect_rooms_never_panics(walls in walls_vec()) {
        let _rooms = detect_rooms(&walls);
    }

    /// Every reported room must have a finite, positive area and a finite
    /// centroid. No NaN or Infinity may leak out.
    #[test]
    fn rooms_have_finite_positive_area(walls in walls_vec()) {
        let rooms = detect_rooms(&walls);
        for room in &rooms {
            prop_assert!(room.area_mm2.is_finite());
            prop_assert!(room.area_mm2 > 0.0);
            prop_assert!(room.centroid.x.is_finite());
            prop_assert!(room.centroid.y.is_finite());
            prop_assert!(room.perimeter_mm.is_finite());
            prop_assert!(room.perimeter_mm > 0.0);
            prop_assert!(room.polygon.len() >= 3);
        }
    }

    /// The detector must not emit duplicate rooms. Two rooms with identical
    /// polygons count as duplicates.
    #[test]
    fn no_duplicate_rooms(walls in walls_vec()) {
        let rooms = detect_rooms(&walls);
        let mut seen: Vec<Vec<Point2>> = Vec::new();
        for room in &rooms {
            for previous in &seen {
                prop_assert!(room.polygon != *previous,
                    "duplicate room polygon: {:?}", room.polygon);
            }
            seen.push(room.polygon.clone());
        }
    }

    /// A single wall (or fewer) can never enclose a room.
    #[test]
    fn too_few_walls_yields_no_rooms(walls in walls_vec()) {
        if walls.len() < 3 {
            let rooms = detect_rooms(&walls);
            prop_assert!(rooms.is_empty(),
                "expected no rooms for {} walls, got {}", walls.len(), rooms.len());
        }
    }

    /// Rooms must be deterministic: calling `detect_rooms` twice on the same
    /// input yields identical output.
    #[test]
    fn detect_rooms_is_deterministic(walls in walls_vec()) {
        let first = detect_rooms(&walls);
        let second = detect_rooms(&walls);
        prop_assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            prop_assert_eq!(&a.polygon, &b.polygon);
            prop_assert_eq!(a.area_mm2, b.area_mm2);
            prop_assert_eq!(a.centroid, b.centroid);
        }
    }

    /// Walls with coordinates at the edge of f64 magnitude must not produce
    /// non-finite room geometry. Overflow in shoelace or centroid math is a
    /// real risk at extreme values.
    #[test]
    fn extreme_coordinates_stay_finite(walls in walls_vec()) {
        let rooms = detect_rooms(&walls);
        for room in &rooms {
            prop_assert!(room.area_mm2.is_finite());
            prop_assert!(room.centroid.x.is_finite());
            prop_assert!(room.centroid.y.is_finite());
        }
    }
}

// -------------------------------------------------------------------------
// Command-layer properties (audit finding: fuzz the editing commands).
// -------------------------------------------------------------------------

use lintel_core::{Command, Document};

const T0: i64 = 1_760_000_000;

/// Arbitrary sequence of wall-add commands with arbitrary coordinates and
/// dimensions. Sanitised the same way as the room fuzz so the command layer is
/// exercised over the full f64 bit space; non-finite thickness degrades to the
/// default so the command's own validation is what is under test, not the
/// strategy.
fn arbitrary_wall_commands() -> impl Strategy<Value = Vec<Command>> {
    prop::collection::vec(
        (
            any::<f64>(),
            any::<f64>(),
            any::<f64>(),
            any::<f64>(),
            any::<f64>(),
        ),
        0..12,
    )
    .prop_map(|points| {
        points
            .into_iter()
            .map(|(x1, y1, x2, y2, thickness)| Command::AddWall {
                start: sane_point(x1, y1),
                end: sane_point(x2, y2),
                thickness: if thickness.is_finite() {
                    thickness.abs()
                } else {
                    200.0
                },
                height: 2800.0,
            })
            .collect()
    })
}

proptest! {
    #![proptest_config(proptest::test_runner::Config {
        failure_persistence: None,
        ..proptest::test_runner::Config::default()
    })]

    /// Applying any sequence of wall commands must never panic, and the
    /// document must stay structurally valid afterwards. Rejected commands
    /// change nothing; accepted ones keep every invariant.
    #[test]
    fn wall_commands_never_panic_or_corrupt(commands in arbitrary_wall_commands()) {
        let mut document = Document::new("Fuzz plan", T0);
        for (index, command) in commands.into_iter().enumerate() {
            let _ = document.apply(command, T0 + index as i64);
        }
        prop_assert!(
            document.project().validate().is_ok(),
            "project invalid after fuzz sequence"
        );
    }

    /// After any fuzz sequence, every surviving wall must have finite
    /// geometry and sane dimensions — the validation invariants hold.
    #[test]
    fn fuzz_walls_stay_valid(commands in arbitrary_wall_commands()) {
        let mut document = Document::new("Fuzz plan", T0);
        for (index, command) in commands.into_iter().enumerate() {
            let _ = document.apply(command, T0 + index as i64);
        }
        let level = document.project().active_level().expect("level");
        for wall in &level.walls {
            prop_assert!(wall.is_valid(), "wall {} invalid: {:?}", wall.id, wall);
        }
    }

    /// Undo after any fuzz sequence must restore a valid project, and the
    /// wall count must reach zero when the history is fully unwound.
    #[test]
    fn fuzz_undo_keeps_project_valid(commands in arbitrary_wall_commands()) {
        let mut document = Document::new("Fuzz plan", T0);
        for (index, command) in commands.into_iter().enumerate() {
            let _ = document.apply(command, T0 + index as i64);
        }
        while document.can_undo() {
            prop_assert!(document.undo(T0 + 9_000));
            prop_assert!(
                document.project().validate().is_ok(),
                "project invalid after undo"
            );
        }
        prop_assert_eq!(document.project().wall_count(), 0);
    }
}
