//! `plan_summary` workflow tests — moved verbatim from the inline test module
//! in `main.rs` when the shared `src/tests` root was introduced (the file
//! module and the inline module would otherwise collide on the name `tests`).

use crate::plan_summary;
use crate::wall_offset_along_centerline;
use lintel_core::command::Command;
use lintel_core::dimension::DimensionKind;
use lintel_core::document::Document;
use lintel_core::geom::Point2;
use lintel_core::ids::WallId;
use lintel_core::opening::OpeningKind;
use lintel_core::wall::Wall;

#[test]
fn an_empty_level_summarises_to_zeroes() {
    let doc = Document::new("Plan", 1);
    assert_eq!(
        plan_summary(doc.project()),
        "Level 0: 0 walls, 0 doors, 0 windows, 0 dimensions"
    );
}

#[test]
fn the_summary_uses_singular_words_for_single_items() {
    let mut doc = Document::new("Plan", 1);
    let wall = doc
        .apply(
            Command::AddWall {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
                thickness: 200.0,
                height: 2800.0,
            },
            2,
        )
        .expect("add wall")
        .wall
        .expect("wall id");
    doc.apply(
        Command::PlaceOpening {
            kind: OpeningKind::Door,
            host: wall,
            offset: 2000.0,
        },
        3,
    )
    .expect("place door");
    doc.apply(
        Command::PlaceDimension {
            kind: DimensionKind::LinearHorizontal,
            start: Point2::new(0.0, 0.0),
            end: Point2::new(4000.0, 0.0),
            offset: 500.0,
        },
        4,
    )
    .expect("place dimension");

    assert_eq!(
        plan_summary(doc.project()),
        "Level 0: 1 wall, 1 door, 0 windows, 1 dimension"
    );
}

#[test]
fn the_summary_follows_the_active_level() {
    let mut doc = Document::new("Plan", 1);
    let applied = doc
        .apply(
            Command::AddLevel {
                name: "First floor".to_string(),
                elevation: 3000.0,
            },
            2,
        )
        .expect("add level");
    assert!(
        plan_summary(doc.project()).starts_with("First floor:"),
        "a new storey becomes active: {}",
        plan_summary(doc.project())
    );

    // Switching back to the ground storey is reflected immediately.
    let ground = lintel_core::ids::LevelId(1);
    assert_ne!(ground, applied.level.expect("level id"));
    doc.apply(Command::SetActiveLevel { id: ground }, 3)
        .expect("set active level");
    assert!(plan_summary(doc.project()).starts_with("Level 0:"));
}

#[test]
fn the_summary_appends_room_count_when_rooms_exist() {
    let mut doc = Document::new("Plan", 1);
    // Four walls enclosing a 4000 x 4000 square.
    let ids = [
        (Point2::new(0.0, 0.0), Point2::new(4000.0, 0.0)),
        (Point2::new(4000.0, 0.0), Point2::new(4000.0, 4000.0)),
        (Point2::new(4000.0, 4000.0), Point2::new(0.0, 4000.0)),
        (Point2::new(0.0, 4000.0), Point2::new(0.0, 0.0)),
    ];
    for (s, e) in ids {
        doc.apply(
            Command::AddWall {
                start: s,
                end: e,
                thickness: 200.0,
                height: 2800.0,
            },
            2,
        )
        .expect("add wall");
    }
    assert_eq!(
        plan_summary(doc.project()),
        "Level 0: 4 walls, 0 doors, 0 windows, 0 dimensions, 1 room"
    );
}

#[test]
fn opening_offset_projects_onto_the_wall_centreline() {
    let wall = Wall::new(
        WallId(1),
        Point2::new(0.0, 0.0),
        Point2::new(4000.0, 0.0),
        200.0,
        2800.0,
    );
    // A click 200 mm off the centreline must project to 2000 mm along the wall,
    // not to the ~2010 mm raw Euclidean distance that used to break fitting.
    let offset = wall_offset_along_centerline(&wall, Point2::new(2000.0, -200.0));
    assert!(
        (offset - 2000.0).abs() < 1e-6,
        "offset should project onto the centreline, got {offset}"
    );
}
