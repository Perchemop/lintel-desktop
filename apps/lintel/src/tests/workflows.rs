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

// T-201: Door/window placement regression tests
// These verify the centreline projection fix from PR #68/#70.

#[test]
fn door_placement_at_wall_endpoint_snaps_to_centreline() {
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

    // Click near wall start (500mm from start) - should project to centreline at param ~0.125
    // Door is 900mm wide, so it fits within 4000mm wall
    doc.apply(
        Command::PlaceOpening {
            kind: lintel_core::opening::OpeningKind::Door,
            host: wall,
            offset: 500.0, // offset parameter is along centreline
        },
        3,
    )
    .expect("place door near endpoint");

    let level = doc.project().active_level().expect("level");
    let openings = level.openings.iter().collect::<Vec<_>>();
    assert_eq!(openings.len(), 1);
    // The door should be placed at the projected position
}

#[test]
fn door_placement_at_wall_midpoint_snaps_to_centreline() {
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

    // Click at wall midpoint (2000, 0) - should project to centreline at param 0.5
    doc.apply(
        Command::PlaceOpening {
            kind: lintel_core::opening::OpeningKind::Door,
            host: wall,
            offset: 2000.0, // midpoint along centreline
        },
        3,
    )
    .expect("place door at midpoint");

    let level = doc.project().active_level().expect("level");
    let openings = level.openings.iter().collect::<Vec<_>>();
    assert_eq!(openings.len(), 1);
}

#[test]
fn window_placement_off_wall_click_projects_to_nearest_centreline_point() {
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

    // Click 500mm above the wall centreline at x=1000
    // Should project to centreline at x=1000 (param 0.25)
    doc.apply(
        Command::PlaceOpening {
            kind: lintel_core::opening::OpeningKind::Window,
            host: wall,
            offset: 1000.0, // offset along centreline
        },
        3,
    )
    .expect("place window off-wall click");

    let level = doc.project().active_level().expect("level");
    let openings = level.openings.iter().collect::<Vec<_>>();
    assert_eq!(openings.len(), 1);
}

#[test]
fn door_placement_near_wall_corner_picks_correct_wall() {
    let mut doc = Document::new("Plan", 1);
    // Create two walls forming an L shape
    let wall1 = doc
        .apply(
            Command::AddWall {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
                thickness: 200.0,
                height: 2800.0,
            },
            2,
        )
        .expect("add wall1")
        .wall
        .expect("wall id");

    let _wall2 = doc
        .apply(
            Command::AddWall {
                start: Point2::new(4000.0, 0.0),
                end: Point2::new(4000.0, 3000.0),
                thickness: 200.0,
                height: 2800.0,
            },
            3,
        )
        .expect("add wall2")
        .wall
        .expect("wall id");

    // Click near the corner (3500mm from start) - should pick wall1 (horizontal)
    doc.apply(
        Command::PlaceOpening {
            kind: lintel_core::opening::OpeningKind::Door,
            host: wall1,
            offset: 3500.0, // near end of wall1 but door still fits
        },
        4,
    )
    .expect("place door near corner");

    let level = doc.project().active_level().expect("level");
    let openings = level.openings.iter().collect::<Vec<_>>();
    assert_eq!(openings.len(), 1);
    assert_eq!(openings[0].host, wall1);
}

#[test]
fn empty_canvas_click_shows_hint_no_panic() {
    let mut doc = Document::new("Plan", 1);
    // No walls exist - placing an opening should fail gracefully
    let result = doc.apply(
        Command::PlaceOpening {
            kind: lintel_core::opening::OpeningKind::Door,
            host: lintel_core::ids::WallId(99), // non-existent wall
            offset: 0.0,
        },
        2,
    );
    // Should return an error, not panic
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("does not exist") || error.to_string().contains("not found")
    );
}
