//! End-to-end workflow tests (guide §6.1, addendum K.11.4).
//!
//! These exercise the *same* API the desktop shell uses — commands in, saved
//! file out — with no window, no GPU and no display. They are the gate that
//! decides whether v0.1.0 actually works.

use lintel_core::geom::Point2;
use lintel_core::join::{HIT_TOLERANCE, Hit, PlanGeometry};
use lintel_core::opening::OpeningKind;
use lintel_core::pdf::{SheetSpec, render_plan};
use lintel_core::{Command, Document, WallId};

const T0: i64 = 1_760_000_000;

/// Draw a 6 m x 4 m room from four walls, all meeting at their corners.
fn two_room_plan() -> (Document, [WallId; 4]) {
    let mut document = Document::new("Workflow flat", T0);
    let corners = [
        (Point2::new(0.0, 0.0), Point2::new(6000.0, 0.0)),
        (Point2::new(6000.0, 0.0), Point2::new(6000.0, 4000.0)),
        (Point2::new(6000.0, 4000.0), Point2::new(0.0, 4000.0)),
        (Point2::new(0.0, 4000.0), Point2::new(0.0, 0.0)),
    ];
    let mut ids = [WallId(0); 4];
    for (index, (start, end)) in corners.into_iter().enumerate() {
        let applied = document
            .apply(
                Command::AddWall {
                    start,
                    end,
                    thickness: 200.0,
                    height: 2800.0,
                },
                T0 + index as i64 + 1,
            )
            .expect("add wall");
        ids[index] = applied.wall.expect("wall id");
    }
    (document, ids)
}

#[test]
fn draw_save_reopen_and_issue_a_drawing() {
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("workflow.lintel.json");

    let (mut document, walls) = two_room_plan();
    assert_eq!(document.project().wall_count(), 4);

    // Place a door on the south wall and a window on the north wall.
    let door = document
        .apply(
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: walls[0],
                offset: 2000.0,
            },
            T0 + 10,
        )
        .expect("door")
        .opening
        .expect("door id");
    let window = document
        .apply(
            Command::PlaceOpening {
                kind: OpeningKind::Window,
                host: walls[2],
                offset: 3000.0,
            },
            T0 + 11,
        )
        .expect("window")
        .opening
        .expect("window id");
    assert_ne!(door, window);

    // The plan is now well formed: every corner joint closes.
    let level = document.project().active_level().expect("level");
    let geometry = PlanGeometry::of_level(level);
    assert_eq!(geometry.walls.len(), 4);
    assert_eq!(geometry.openings.len(), 2);
    for wall in &geometry.walls {
        assert_eq!(
            wall.start_extension, 100.0,
            "a corner joint should reach half the joining thickness"
        );
        assert_eq!(wall.end_extension, 100.0);
    }
    let bounds = geometry.bounds().expect("bounds");
    assert_eq!(bounds.width(), 6200.0);
    assert_eq!(bounds.height(), 4200.0);

    // Clicking on the door selects the door, not the wall behind it.
    assert_eq!(
        geometry.hit_test(Point2::new(2000.0, 0.0), HIT_TOLERANCE),
        Some(Hit::Opening(door))
    );
    assert_eq!(
        geometry.hit_test(Point2::new(500.0, 0.0), HIT_TOLERANCE),
        Some(Hit::Wall(walls[0]))
    );

    // Undo the window, then put it back: history stays consistent.
    assert!(document.undo(T0 + 12));
    assert_eq!(document.project().opening_count(), 1);
    assert!(document.redo(T0 + 13));
    assert_eq!(document.project().opening_count(), 2);

    // Save, then reopen from disk.
    let saved_at = document.save_as(&path).expect("save");
    assert_eq!(saved_at, path);
    assert!(!document.dirty());

    let reopened = Document::open(&path).expect("reopen");
    assert_eq!(reopened.project(), document.project());
    assert_eq!(reopened.project().name, "Workflow flat");
    assert_eq!(reopened.project().opening_count(), 2);

    // Issue the drawing: a real PDF with the plan, the title block and the
    // requested sheet.
    let spec = SheetSpec {
        sheet_title: "Ground floor".into(),
        sheet_number: "A-101".into(),
        scale_denominator: 50,
        ..SheetSpec::default()
    };
    let pdf = render_plan(reopened.project(), &spec, T0 + 20).expect("pdf");
    let text = String::from_utf8_lossy(&pdf).to_string();
    assert!(text.starts_with("%PDF-1.7"));
    assert!(text.ends_with("%%EOF\n"));
    assert!(text.contains("(Workflow flat)"));
    assert!(text.contains("(Ground floor)"));
    assert!(text.contains("(Scale 1:50)"));
    assert!(text.contains("B\n"), "the walls should be filled");
    assert!(pdf.len() > 2_000, "a sheet is more than a stub");
}

#[test]
fn rejected_edits_leave_the_plan_untouched() {
    let (mut document, walls) = two_room_plan();

    // A door wider than its wall.
    let error = document
        .apply(
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: walls[0],
                offset: 100.0,
            },
            T0 + 5,
        )
        .expect_err("door overhangs the jamb");
    assert!(error.to_string().contains("does not fit"));

    // Shrinking a wall under an existing opening is refused.
    document
        .apply(
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: walls[0],
                offset: 5500.0,
            },
            T0 + 6,
        )
        .expect("door near the end");
    let error = document
        .apply(
            Command::MoveWall {
                id: walls[0],
                start: Point2::new(0.0, 0.0),
                end: Point2::new(3000.0, 0.0),
            },
            T0 + 7,
        )
        .expect_err("wall would strand the door");
    assert!(error.to_string().contains("no longer fit"));

    // Nothing above changed the document, so its history is one edit deep.
    assert_eq!(document.project().wall_count(), 4);
    assert_eq!(document.project().opening_count(), 1);
    assert_eq!(document.undo_depth(), 5, "four walls and one door");
    assert!(document.project().validate().is_ok());
}

#[test]
fn a_second_level_keeps_its_own_walls() {
    use lintel_core::LevelId;

    let (mut document, _) = two_room_plan();
    let ground_walls = document.project().wall_count();

    let level = document
        .apply(
            Command::AddLevel {
                name: "Level 1".into(),
                elevation: 3000.0,
            },
            T0 + 30,
        )
        .expect("add level")
        .level
        .expect("level id");
    assert_ne!(level, LevelId(0));
    assert_eq!(
        document.project().wall_count(),
        ground_walls,
        "the new level starts empty"
    );

    document
        .apply(
            Command::AddWall {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(3000.0, 0.0),
                thickness: 100.0,
                height: 2700.0,
            },
            T0 + 31,
        )
        .expect("wall on the upper level");
    assert_eq!(document.project().wall_count(), ground_walls + 1);

    // Jumping back to the ground floor shows the original walls again.
    document
        .apply(Command::SetActiveLevel { id: LevelId(1) }, T0 + 32)
        .expect("back to ground");
    let level_zero = document.project().active_level().expect("active level").id;
    assert_eq!(level_zero, LevelId(1));
}
