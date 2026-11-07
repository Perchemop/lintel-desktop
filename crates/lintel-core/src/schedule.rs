//! The opening schedule: a table of distinct opening types and their counts.
//!
//! Like rooms, the schedule is derived — nothing in the project stores a row.
//! Rows are grouped from the openings of one level each time the table is
//! drawn, so adding or resizing an opening can never leave a stale count.

use serde::{Deserialize, Serialize};

use crate::level::Level;
use crate::opening::OpeningKind;

/// One row of the schedule: a distinct opening type and how many exist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleRow {
    /// Type mark shown on the plan and in the table: `D1`, `W1`, …
    pub mark: String,
    pub kind: OpeningKind,
    pub width: f64,
    pub height: f64,
    /// How many openings share this type and size.
    pub count: usize,
}

impl ScheduleRow {
    /// `"900 x 2100"` — ASCII only, so the PDF can print it.
    pub fn size_label(&self) -> String {
        format!("{} x {}", size_number(self.width), size_number(self.height))
    }
}

/// Group `level`'s openings by kind and size into schedule rows.
///
/// Doors first, then windows; within a kind, descending width then descending
/// height. Marks run `D1..Dn` and `W1..Wn` in that order.
pub fn opening_schedule(level: &Level) -> Vec<ScheduleRow> {
    let mut rows: Vec<ScheduleRow> = Vec::new();
    for opening in &level.openings {
        let existing = rows.iter_mut().find(|row| {
            row.kind == opening.kind && row.width == opening.width && row.height == opening.height
        });
        match existing {
            Some(row) => row.count += 1,
            None => rows.push(ScheduleRow {
                mark: String::new(),
                kind: opening.kind,
                width: opening.width,
                height: opening.height,
                count: 1,
            }),
        }
    }

    rows.sort_by(|a, b| {
        kind_rank(a.kind)
            .cmp(&kind_rank(b.kind))
            .then_with(|| b.width.total_cmp(&a.width))
            .then_with(|| b.height.total_cmp(&a.height))
    });

    let mut doors = 0;
    let mut windows = 0;
    for row in &mut rows {
        match row.kind {
            OpeningKind::Door => {
                doors += 1;
                row.mark = format!("D{doors}");
            }
            OpeningKind::Window => {
                windows += 1;
                row.mark = format!("W{windows}");
            }
        }
    }
    rows
}

/// Total number of openings counted by `rows`.
pub fn opening_total(rows: &[ScheduleRow]) -> usize {
    rows.iter().map(|row| row.count).sum()
}

/// Doors sort before windows.
fn kind_rank(kind: OpeningKind) -> u8 {
    match kind {
        OpeningKind::Door => 0,
        OpeningKind::Window => 1,
    }
}

/// A millimetre value as a plain ASCII number, with no trailing `.0`.
fn size_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    // Tenths of a millimetre are the finest resolution the sheet can print.
    let rounded = (value * 10.0).round() / 10.0;
    format!("{rounded}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{LevelId, OpeningId, WallId};
    use crate::opening::Opening;

    fn level_with(openings: Vec<Opening>) -> Level {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.openings = openings;
        level
    }

    fn opening(id: u64, kind: OpeningKind, width: f64, height: f64) -> Opening {
        let mut opening = Opening::new(OpeningId(id), kind, WallId(1), 1000.0);
        opening.width = width;
        opening.height = height;
        opening
    }

    #[test]
    fn empty_level_has_no_rows() {
        let level = level_with(Vec::new());
        assert!(opening_schedule(&level).is_empty());
        assert_eq!(opening_total(&opening_schedule(&level)), 0);
    }

    #[test]
    fn identical_openings_share_one_row() {
        let level = level_with(vec![
            opening(1, OpeningKind::Door, 900.0, 2100.0),
            opening(2, OpeningKind::Door, 900.0, 2100.0),
        ]);
        let rows = opening_schedule(&level);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].count, 2);
        assert_eq!(rows[0].mark, "D1");
    }

    #[test]
    fn doors_and_windows_get_separate_marks() {
        let level = level_with(vec![
            opening(1, OpeningKind::Door, 900.0, 2100.0),
            opening(2, OpeningKind::Window, 1200.0, 1200.0),
        ]);
        let rows = opening_schedule(&level);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].mark, "D1");
        assert_eq!(rows[0].kind, OpeningKind::Door);
        assert_eq!(rows[1].mark, "W1");
        assert_eq!(rows[1].kind, OpeningKind::Window);
    }

    #[test]
    fn a_wider_door_sorts_before_a_narrower_one() {
        let level = level_with(vec![
            opening(1, OpeningKind::Door, 800.0, 2100.0),
            opening(2, OpeningKind::Door, 1000.0, 2100.0),
        ]);
        let rows = opening_schedule(&level);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].width, 1000.0);
        assert_eq!(rows[0].mark, "D1");
        assert_eq!(rows[1].width, 800.0);
        assert_eq!(rows[1].mark, "D2");
    }

    #[test]
    fn a_taller_opening_sorts_before_a_shorter_one_of_the_same_width() {
        let level = level_with(vec![
            opening(1, OpeningKind::Window, 1200.0, 1000.0),
            opening(2, OpeningKind::Window, 1200.0, 1400.0),
        ]);
        let rows = opening_schedule(&level);
        assert_eq!(rows[0].height, 1400.0);
        assert_eq!(rows[1].height, 1000.0);
    }

    #[test]
    fn size_label_is_ascii_with_an_x() {
        let row = ScheduleRow {
            mark: "D1".to_string(),
            kind: OpeningKind::Door,
            width: 900.0,
            height: 2100.0,
            count: 1,
        };
        let label = row.size_label();
        assert_eq!(label, "900 x 2100");
        assert!(label.chars().all(|character| character.is_ascii()));
    }

    #[test]
    fn opening_total_sums_the_counts() {
        let rows = vec![
            ScheduleRow {
                mark: "D1".to_string(),
                kind: OpeningKind::Door,
                width: 900.0,
                height: 2100.0,
                count: 2,
            },
            ScheduleRow {
                mark: "W1".to_string(),
                kind: OpeningKind::Window,
                width: 1200.0,
                height: 1200.0,
                count: 3,
            },
        ];
        assert_eq!(opening_total(&rows), 5);
    }

    #[test]
    fn schedule_row_round_trips_through_json() {
        let row = ScheduleRow {
            mark: "W2".to_string(),
            kind: OpeningKind::Window,
            width: 1200.0,
            height: 1400.0,
            count: 4,
        };
        let json = serde_json::to_string(&row).expect("serialize");
        let back: ScheduleRow = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, row);
    }
}
