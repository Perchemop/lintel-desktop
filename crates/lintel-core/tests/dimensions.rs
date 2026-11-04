//! Dimension-line workflow tests (guide §6.1, addendum K.11.4).
//!
//! These exercise the *same* API the desktop shell uses — commands in, saved
//! file out — with no window, no GPU and no display. They gate v0.2.0's
//! dimension editing: place, move, re-offset, hit-test and delete.

use lintel_core::join::{HIT_TOLERANCE, Hit, PlanGeometry};
use lintel_core::{Command, DimensionId, DimensionKind, Document, Point2, WallId};

const T0: i64 = 1_760_000_000;

/// A document with a single 6 m wall along the x-axis to dimension.
fn plan_with_wall() -> (Document, WallId) {
    let mut document = Document::new("Dimension flat", T0);
    let wall = document
        .apply(
            Command::AddWall {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(6000.0, 0.0),
                thickness: 200.0,
                height: 2800.0,
            },
            T0 + 1,
        )
        .expect("add wall")
        .wall
        .expect("wall id");
    (document, wall)
}

/// Place a horizontal dimension measuring `start..end` at `offset` and
/// return its id.
fn place_horizontal(
    document: &mut Document,
    start: Point2,
    end: Point2,
    offset: f64,
    now: i64,
) -> DimensionId {
    document
        .apply(
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start,
                end,
                offset,
            },
            now,
        )
        .expect("place dimension")
        .dimension
        .expect("dimension id")
}

/// The measured span of a dimension, copied out so the caller keeps editing.
fn span_of(document: &Document, id: DimensionId) -> (Point2, Point2) {
    let dimension = document
        .project()
        .active_level()
        .expect("level")
        .dimension(id)
        .expect("dimension");
    (dimension.start, dimension.end)
}

/// The current offset of a dimension.
fn offset_of(document: &Document, id: DimensionId) -> f64 {
    document
        .project()
        .active_level()
        .expect("level")
        .dimension(id)
        .expect("dimension")
        .offset
}

#[test]
fn workflow_place_dimension_and_round_trip() {
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("dimensions.lintel.json");

    let (mut document, _wall) = plan_with_wall();
    let id = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        500.0,
        T0 + 10,
    );
    assert_eq!(document.project().dimension_count(), 1);

    // Save, then reopen from disk.
    let saved_at = document.save_as(&path).expect("save");
    assert_eq!(saved_at, path);
    assert!(!document.dirty());

    let reopened = Document::open(&path).expect("reopen");
    assert_eq!(reopened.project(), document.project());
    assert_eq!(reopened.project().dimension_count(), 1);

    // The measured span survives the file untouched.
    let dimension = reopened
        .project()
        .active_level()
        .expect("level")
        .dimension(id)
        .expect("dimension");
    assert_eq!(dimension.kind, DimensionKind::LinearHorizontal);
    assert_eq!(dimension.start, Point2::new(0.0, 0.0));
    assert_eq!(dimension.end, Point2::new(6000.0, 0.0));
    assert_eq!(dimension.offset, 500.0);
    assert!(dimension.extension_lines);
}

#[test]
fn workflow_dimension_move_and_redo() {
    let (mut document, _wall) = plan_with_wall();
    let id = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        500.0,
        T0 + 10,
    );

    // Drag the dimension onto a shorter span.
    document
        .apply(
            Command::MoveDimension {
                id,
                start: Point2::new(1000.0, 0.0),
                end: Point2::new(5000.0, 0.0),
            },
            T0 + 11,
        )
        .expect("move dimension");
    let (start, end) = span_of(&document, id);
    assert_eq!(start, Point2::new(1000.0, 0.0));
    assert_eq!(end, Point2::new(5000.0, 0.0));

    // Undo returns to the original measured span…
    assert!(document.undo(T0 + 12));
    let (start, end) = span_of(&document, id);
    assert_eq!(start, Point2::new(0.0, 0.0));
    assert_eq!(end, Point2::new(6000.0, 0.0));

    // …and redo moves it again.
    assert!(document.redo(T0 + 13));
    let (start, end) = span_of(&document, id);
    assert_eq!(start, Point2::new(1000.0, 0.0));
    assert_eq!(end, Point2::new(5000.0, 0.0));
}

#[test]
fn workflow_dimension_delete() {
    let (mut document, _wall) = plan_with_wall();
    let id = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        500.0,
        T0 + 10,
    );
    assert_eq!(document.project().dimension_count(), 1);

    document
        .apply(Command::DeleteDimension { id }, T0 + 11)
        .expect("delete dimension");
    assert_eq!(document.project().dimension_count(), 0);

    // Deleting it again finds nothing to remove.
    let error = document
        .apply(Command::DeleteDimension { id }, T0 + 12)
        .expect_err("already gone");
    assert!(error.to_string().contains("does not exist"));

    // Undo brings it back.
    assert!(document.undo(T0 + 13));
    assert_eq!(document.project().dimension_count(), 1);
    assert!(document.project().validate().is_ok());
}

#[test]
fn workflow_dimension_offset_change() {
    let (mut document, _wall) = plan_with_wall();
    let id = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        500.0,
        T0 + 10,
    );

    // Push the line further from the measured span.
    document
        .apply(Command::SetDimensionOffset { id, offset: 800.0 }, T0 + 11)
        .expect("set offset");
    assert_eq!(offset_of(&document, id), 800.0);

    // Zero and negative offsets are refused, leaving the offset alone: a
    // dimension cannot sit on what it measures.
    let error = document
        .apply(Command::SetDimensionOffset { id, offset: 0.0 }, T0 + 12)
        .expect_err("zero offset");
    assert!(error.to_string().contains("greater than zero"));
    let error = document
        .apply(Command::SetDimensionOffset { id, offset: -250.0 }, T0 + 13)
        .expect_err("negative offset");
    assert!(error.to_string().contains("greater than zero"));
    assert_eq!(
        offset_of(&document, id),
        800.0,
        "a rejected change is ignored"
    );
    assert_eq!(
        document.undo_depth(),
        3,
        "wall, placement and offset change"
    );
}

#[test]
fn workflow_dimension_hit_test() {
    let (mut document, wall) = plan_with_wall();
    let id = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        500.0,
        T0 + 10,
    );
    // A second line hugs the span so it lies inside the wall's band.
    let over_wall = place_horizontal(
        &mut document,
        Point2::new(0.0, 0.0),
        Point2::new(6000.0, 0.0),
        100.0,
        T0 + 11,
    );

    let level = document.project().active_level().expect("level");
    let geometry = PlanGeometry::of_level(level);
    assert_eq!(geometry.dimensions().len(), 2);

    // Points on the dimension line (500 mm above the span) find it…
    assert_eq!(
        geometry.hit_test(Point2::new(3000.0, 500.0), HIT_TOLERANCE),
        Some(Hit::Dimension(id))
    );
    // …and so do points within the click tolerance of the line.
    assert_eq!(
        geometry.hit_test(Point2::new(1500.0, 530.0), HIT_TOLERANCE),
        Some(Hit::Dimension(id))
    );

    // The second line lies inside the wall band: the annotation drawn on
    // top wins over the wall beneath it.
    assert_eq!(
        geometry.hit_test(Point2::new(3000.0, 100.0), HIT_TOLERANCE),
        Some(Hit::Dimension(over_wall))
    );

    // On the wall, away from both lines, the wall itself is found.
    assert_eq!(
        geometry.hit_test(Point2::new(3000.0, 0.0), HIT_TOLERANCE),
        Some(Hit::Wall(wall))
    );

    // Past the end of the lines, and far above them, there is nothing.
    assert_eq!(
        geometry.hit_test(Point2::new(7000.0, 500.0), HIT_TOLERANCE),
        None
    );
    assert_eq!(
        geometry.hit_test(Point2::new(3000.0, 3000.0), HIT_TOLERANCE),
        None
    );
}

#[test]
fn workflow_dimension_validation() {
    let (mut document, _wall) = plan_with_wall();

    // A dimension that measures nothing is refused.
    let error = document
        .apply(
            Command::PlaceDimension {
                kind: DimensionKind::Aligned,
                start: Point2::new(2000.0, 0.0),
                end: Point2::new(2000.0, 0.0),
                offset: 500.0,
            },
            T0 + 10,
        )
        .expect_err("zero length");
    assert!(error.to_string().contains("different measured points"));

    // A negative offset would put the line on the wrong side of the span.
    let error = document
        .apply(
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(6000.0, 0.0),
                offset: -500.0,
            },
            T0 + 11,
        )
        .expect_err("negative offset");
    assert!(error.to_string().contains("greater than zero"));

    // A zero offset leaves the line on top of what it measures.
    let error = document
        .apply(
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(6000.0, 0.0),
                offset: 0.0,
            },
            T0 + 12,
        )
        .expect_err("zero offset");
    assert!(error.to_string().contains("greater than zero"));

    // None of the rejections changed the document.
    assert_eq!(document.project().dimension_count(), 0);
    assert_eq!(document.undo_depth(), 1, "only the wall was committed");
    assert!(document.project().validate().is_ok());
}
