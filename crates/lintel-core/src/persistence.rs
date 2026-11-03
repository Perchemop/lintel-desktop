//! Reading and writing the project file.
//!
//! The format is pretty-printed, versioned JSON with a `.lintel.json` suffix.
//! Saving is atomic: the new bytes go to a sibling temporary file and are
//! renamed into place, so a crash mid-save cannot truncate the user's plan.
//! Loading validates structure before the file is accepted, and refuses a file
//! written by a newer build rather than half-reading it.

use std::path::{Path, PathBuf};

use crate::error::CoreError;
use crate::project::Project;

/// Suffix of a Lintel project file. Kept as a full suffix (not a bare `json`)
/// so the file dialog can find plans without sweeping the user's JSON files.
pub const PROJECT_SUFFIX: &str = "lintel.json";

pub fn to_json(project: &Project) -> Result<String, CoreError> {
    Ok(serde_json::to_string_pretty(project)?)
}

/// Parse and validate. The only entry point that trusts a byte string.
pub fn from_json(text: &str) -> Result<Project, CoreError> {
    let project: Project = serde_json::from_str(text)?;
    project.validate()?;
    Ok(project)
}

/// Write the project to `path`, replacing any existing file atomically.
pub fn save_atomic(project: &Project, path: &Path) -> Result<(), CoreError> {
    // Never write a file that would not load again.
    project.validate()?;
    let text = to_json(project)?;

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(CoreError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("the folder {} does not exist", parent.display()),
            )));
        }
    }

    let temporary = temporary_path(path);
    std::fs::write(&temporary, text)?;

    // Windows will not rename onto an existing file.
    #[cfg(windows)]
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }

    if let Err(error) = std::fs::rename(&temporary, path) {
        let _ = std::fs::remove_file(&temporary);
        return Err(CoreError::Io(error));
    }
    Ok(())
}

pub fn load(path: &Path) -> Result<Project, CoreError> {
    let text = std::fs::read_to_string(path)?;
    from_json(&text)
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(std::ffi::OsStr::to_os_string)
        .unwrap_or_default();
    name.push(".tmp");
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{Applied, Command};
    use crate::document::Document;
    use crate::geom::Point2;
    use crate::project::UNTITLED;

    fn documented_plan() -> Document {
        let mut document = Document::new("Flat 3B", 1_760_000_000);
        let applied: Applied = document
            .apply(
                Command::AddWall {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(4000.0, 0.0),
                    thickness: 200.0,
                    height: 2800.0,
                },
                1_760_000_100,
            )
            .expect("add wall");
        document
            .apply(
                Command::PlaceOpening {
                    kind: crate::opening::OpeningKind::Door,
                    host: applied.wall.expect("wall"),
                    offset: 2000.0,
                },
                1_760_000_200,
            )
            .expect("place door");
        document
    }

    #[test]
    fn a_project_round_trips_through_the_file() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("plan.lintel.json");
        let original = documented_plan();
        crate::persistence::save_atomic(original.project(), &path).expect("save");
        let loaded = load(&path).expect("load");
        assert_eq!(&loaded, original.project());
        assert_eq!(loaded.wall_count(), 1);
        assert_eq!(loaded.opening_count(), 1);
    }

    #[test]
    fn saving_replaces_an_existing_file_and_leaves_no_temporary() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("plan.lintel.json");
        let mut document = documented_plan();
        document.save_as(&path).expect("first save");
        document
            .apply(
                Command::RenameProject {
                    name: "Flat 3C".into(),
                },
                1_760_000_300,
            )
            .expect("rename");
        document.save().expect("second save");
        let loaded = load(&path).expect("load");
        assert_eq!(loaded.name, "Flat 3C");

        let leftovers: Vec<_> = std::fs::read_dir(directory.path())
            .expect("read dir")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .filter(|name| name.ends_with(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "temporary files left behind: {leftovers:?}");
    }

    #[test]
    fn the_file_is_readable_json_and_names_the_format() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join(UNTITLED);
        let document = documented_plan();
        crate::persistence::save_atomic(document.project(), &path).expect("save");
        let text = std::fs::read_to_string(&path).expect("read");
        assert!(text.contains("\"format_version\": 1"));
        assert!(text.contains("\"name\": \"Flat 3B\""));
        assert!(text.contains('\n'), "pretty printed");
    }

    #[test]
    fn broken_json_is_reported_not_panicked() {
        let error = from_json("{ this is not json }").expect_err("bad json");
        assert!(matches!(error, CoreError::Json(_)));
    }

    #[test]
    fn a_newer_file_format_is_refused() {
        let text = r#"{
            "format_version": 99,
            "name": "From the future",
            "units": "Metric",
            "levels": [],
            "active_level": 1,
            "created_epoch_s": 0,
            "modified_epoch_s": 0,
            "notes": "",
            "next_id": 1
        }"#;
        let error = from_json(text).expect_err("too new");
        assert!(matches!(
            error,
            CoreError::UnsupportedVersion { found: 99, .. }
        ));
    }

    #[test]
    fn structurally_broken_projects_are_refused() {
        // A wall with two ends in the same place cannot be drawn.
        let text = r#"{
            "format_version": 1,
            "name": "Broken",
            "units": "Metric",
            "levels": [{
                "id": 1,
                "name": "Level 0",
                "elevation": 0.0,
                "walls": [{
                    "id": 2,
                    "start": {"x": 0.0, "y": 0.0},
                    "end": {"x": 0.0, "y": 0.0},
                    "thickness": 200.0,
                    "height": 2800.0
                }],
                "openings": []
            }],
            "active_level": 1,
            "created_epoch_s": 0,
            "modified_epoch_s": 0,
            "notes": "",
            "next_id": 3
        }"#;
        let error = from_json(text).expect_err("degenerate wall");
        assert!(error.to_string().contains("impossible dimensions"));
    }

    #[test]
    fn saving_into_a_missing_folder_is_a_clear_error() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("nope").join("plan.lintel.json");
        let document = documented_plan();
        let error =
            crate::persistence::save_atomic(document.project(), &path).expect_err("missing folder");
        assert!(error.to_string().contains("does not exist"));
    }

    #[test]
    fn a_document_remembers_where_it_was_saved() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("plan.lintel.json");
        let mut document = documented_plan();
        assert!(document.dirty());
        document.save_as(&path).expect("save");
        assert!(!document.dirty());
        assert_eq!(document.path(), Some(path.as_path()));
        let reopened = Document::open(&path).expect("open");
        assert_eq!(reopened.project(), document.project());
        assert!(!reopened.dirty());
    }
}
