//! The title block every exported sheet carries.
//!
//! Issuing a drawing means the sheet says who drew it, when, and at which
//! revision. That information belongs to the project, not to a single sheet, so
//! it is stored once here. A field the project does not set is left blank on
//! the sheet rather than padded with placeholder text: the project name and the
//! sheet title fall back to the project's own name and the sheet's own title,
//! and the date falls back to the export date, so even a default block prints a
//! complete title block.

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// Longest revision mark the title block will carry, in characters.
pub const MAX_REVISION_CHARS: usize = 8;

/// The title block printed on every exported sheet.
///
/// Every field is optional. An empty string is omitted from the sheet; where a
/// field has a natural fallback (project name, sheet title, date) the exporter
/// uses it, so a default block still produces a complete sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TitleBlock {
    /// Practice or client name, printed above the project name. Blank = omitted.
    #[serde(default)]
    pub company: String,
    /// Project name. Blank falls back to the project's own name.
    #[serde(default)]
    pub project_name: String,
    /// Drawing title for the sheet. Blank falls back to the sheet's own title.
    #[serde(default)]
    pub sheet_title: String,
    /// Revision mark, e.g. `A`. Kept short; blank = omitted.
    #[serde(default)]
    pub revision: String,
    /// Issue date written `YYYY-MM-DD`. Blank uses the export date.
    #[serde(default)]
    pub date: String,
    /// Whoever drew the sheet, e.g. initials. Blank = omitted.
    #[serde(default)]
    pub drawn_by: String,
    /// Number of the first sheet of a set. `None` keeps each sheet's own
    /// number; `Some(n)` makes the set's numbers run `n`, `n + 1`, … and must
    /// be at least one.
    #[serde(default)]
    pub sheet_number: Option<u32>,
}

impl TitleBlock {
    /// A block whose project name is already filled in. Used when a caller
    /// wants the printed name pinned at creation rather than following renames.
    pub fn for_project(name: impl Into<String>) -> Self {
        Self {
            project_name: name.into(),
            ..Self::default()
        }
    }

    /// The project name to print: the block's own, or `fallback` when blank.
    pub fn project_name_or<'a>(&'a self, fallback: &'a str) -> &'a str {
        non_empty(&self.project_name).unwrap_or(fallback)
    }

    /// The sheet title to print: the block's own, or `fallback` when blank.
    pub fn sheet_title_or<'a>(&'a self, fallback: &'a str) -> &'a str {
        non_empty(&self.sheet_title).unwrap_or(fallback)
    }

    /// The date to print: the block's own, or `fallback` when blank.
    pub fn date_or<'a>(&'a self, fallback: &'a str) -> &'a str {
        non_empty(&self.date).unwrap_or(fallback)
    }

    /// The practice or client name, if one is set.
    pub fn company(&self) -> Option<&str> {
        non_empty(&self.company)
    }

    /// The revision mark, if one is set.
    pub fn revision(&self) -> Option<&str> {
        non_empty(&self.revision)
    }

    /// Whoever drew the sheet, if named.
    pub fn drawn_by(&self) -> Option<&str> {
        non_empty(&self.drawn_by)
    }

    /// Reject a title block the sheet cannot print honestly.
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.sheet_number == Some(0) {
            return Err(CoreError::invalid("a sheet number must be 1 or more"));
        }
        if self.revision.chars().count() > MAX_REVISION_CHARS {
            return Err(CoreError::invalid(format!(
                "a revision must be at most {MAX_REVISION_CHARS} characters"
            )));
        }
        let date = self.date.trim();
        if !date.is_empty() && !is_iso_date(date) {
            return Err(CoreError::invalid(
                "a title-block date must be written as YYYY-MM-DD",
            ));
        }
        Ok(())
    }
}

/// A trimmed view of a field, or `None` when it holds only whitespace.
fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// True for a plain `YYYY-MM-DD` date with a real month and day. ASCII only, so
/// the byte slices below cannot split a character.
fn is_iso_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits_ok = bytes
        .iter()
        .enumerate()
        .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit());
    if !digits_ok {
        return false;
    }
    let month = text[5..7].parse::<u32>().ok();
    let day = text[8..10].parse::<u32>().ok();
    matches!(month, Some(1..=12)) && matches!(day, Some(1..=31))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_default_block_is_empty_and_valid() {
        let block = TitleBlock::default();
        assert!(block.company().is_none());
        assert!(block.revision().is_none());
        assert!(block.drawn_by().is_none());
        assert_eq!(block.sheet_number, None);
        assert!(block.validate().is_ok());
    }

    #[test]
    fn blank_fields_fall_back_to_their_source() {
        let block = TitleBlock::default();
        assert_eq!(block.project_name_or("Flat 3B"), "Flat 3B");
        assert_eq!(block.sheet_title_or("Ground floor"), "Ground floor");
        assert_eq!(block.date_or("2026-11-14"), "2026-11-14");
    }

    #[test]
    fn set_fields_win_over_their_fallback() {
        let block = TitleBlock {
            project_name: "  Riverside House  ".into(),
            sheet_title: "Roof plan".into(),
            date: "2026-01-02".into(),
            ..TitleBlock::default()
        };
        assert_eq!(block.project_name_or("ignored"), "Riverside House");
        assert_eq!(block.sheet_title_or("ignored"), "Roof plan");
        assert_eq!(block.date_or("ignored"), "2026-01-02");
    }

    #[test]
    fn whitespace_only_fields_are_omitted() {
        let block = TitleBlock {
            company: "   ".into(),
            revision: " \t".into(),
            drawn_by: String::new(),
            ..TitleBlock::default()
        };
        assert!(block.company().is_none());
        assert!(block.revision().is_none());
        assert!(block.drawn_by().is_none());
    }

    #[test]
    fn a_for_project_block_fills_the_name() {
        let block = TitleBlock::for_project("Flat 3B");
        assert_eq!(block.project_name_or("other"), "Flat 3B");
    }

    #[test]
    fn a_zero_sheet_number_is_rejected() {
        let block = TitleBlock {
            sheet_number: Some(0),
            ..TitleBlock::default()
        };
        let error = block.validate().expect_err("zero sheet number");
        assert!(error.to_string().contains("1 or more"));
    }

    #[test]
    fn a_first_sheet_number_is_accepted() {
        let block = TitleBlock {
            sheet_number: Some(1),
            ..TitleBlock::default()
        };
        assert!(block.validate().is_ok());
    }

    #[test]
    fn a_long_revision_is_rejected() {
        let block = TitleBlock {
            revision: "REVISION-TOO-LONG".into(),
            ..TitleBlock::default()
        };
        let error = block.validate().expect_err("long revision");
        assert!(error.to_string().contains("at most 8 characters"));
    }

    #[test]
    fn a_malformed_date_is_rejected() {
        for date in [
            "14/11/2026",
            "2026-13-01",
            "2026-00-10",
            "not a date",
            "2026-1-1",
        ] {
            let block = TitleBlock {
                date: date.into(),
                ..TitleBlock::default()
            };
            assert!(block.validate().is_err(), "{date} should be rejected");
        }
    }

    #[test]
    fn a_plain_iso_date_is_accepted() {
        for date in ["2026-11-14", "1970-01-01", "2000-02-29"] {
            let block = TitleBlock {
                date: date.into(),
                ..TitleBlock::default()
            };
            assert!(block.validate().is_ok(), "{date} should be accepted");
        }
    }

    #[test]
    fn a_title_block_round_trips_through_json() {
        let block = TitleBlock {
            company: "Acme Drafting".into(),
            project_name: "Flat 3B".into(),
            sheet_title: "Ground floor".into(),
            revision: "B".into(),
            date: "2026-11-14".into(),
            drawn_by: "JD".into(),
            sheet_number: Some(101),
        };
        let json = serde_json::to_string(&block).expect("serialize");
        let back: TitleBlock = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, block);
    }

    #[test]
    fn a_title_block_with_no_fields_loads_as_the_default() {
        let back: TitleBlock = serde_json::from_str("{}").expect("empty block");
        assert_eq!(back, TitleBlock::default());
    }
}
