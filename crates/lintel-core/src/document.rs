//! The editing session: a project, its undo history and its file identity.
//!
//! Commands are applied to a clone first and committed only when they succeed,
//! so a rejected edit can never leave the project half-changed.

use std::path::{Path, PathBuf};

use crate::command::{apply_command, Applied, Command};
use crate::error::CoreError;
use crate::project::Project;

/// How many edits the undo stack keeps. Snapshots are small (a plan is
/// kilobytes of JSON), so a generous depth costs little.
pub const MAX_UNDO: usize = 128;

#[derive(Debug, Clone)]
pub struct Document {
    project: Project,
    undo: Vec<Project>,
    redo: Vec<Project>,
    path: Option<PathBuf>,
    dirty: bool,
}

impl Document {
    pub fn new(name: impl Into<String>, now_epoch_s: i64) -> Self {
        Self::from_project(Project::new(name, now_epoch_s))
    }

    pub fn from_project(project: Project) -> Self {
        Self {
            project,
            undo: Vec::new(),
            redo: Vec::new(),
            path: None,
            dirty: false,
        }
    }

    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn apply(&mut self, command: Command, now_epoch_s: i64) -> Result<Applied, CoreError> {
        let mut candidate = self.project.clone();
        let applied = apply_command(&mut candidate, command)?;
        if candidate == self.project {
            return Ok(applied);
        }
        candidate.modified_epoch_s = now_epoch_s;
        let previous = std::mem::replace(&mut self.project, candidate);
        self.undo.push(previous);
        if self.undo.len() > MAX_UNDO {
            self.undo.remove(0);
        }
        self.redo.clear();
        self.dirty = true;
        Ok(applied)
    }

    pub fn undo(&mut self, now_epoch_s: i64) -> bool {
        let Some(previous) = self.undo.pop() else {
            return false;
        };
        let current = std::mem::replace(&mut self.project, previous);
        self.redo.push(current);
        self.project.modified_epoch_s = now_epoch_s;
        self.dirty = true;
        true
    }

    pub fn redo(&mut self, now_epoch_s: i64) -> bool {
        let Some(next) = self.redo.pop() else {
            return false;
        };
        let current = std::mem::replace(&mut self.project, next);
        self.undo.push(current);
        self.project.modified_epoch_s = now_epoch_s;
        self.dirty = true;
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Write to the document's current path. `None` means "this document has
    /// never been saved" — the UI must ask for a location first.
    pub fn save(&mut self) -> Result<PathBuf, CoreError> {
        let path = self
            .path
            .clone()
            .ok_or_else(|| CoreError::invalid("this plan has not been saved yet"))?;
        crate::persistence::save_atomic(&self.project, &path)?;
        self.dirty = false;
        Ok(path)
    }

    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<PathBuf, CoreError> {
        let path = path.into();
        crate::persistence::save_atomic(&self.project, &path)?;
        self.path = Some(path.clone());
        self.dirty = false;
        Ok(path)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref();
        let project = crate::persistence::load(path)?;
        let mut document = Self::from_project(project);
        document.path = Some(path.to_path_buf());
        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point2;

    fn wall(start: Point2, end: Point2) -> Command {
        Command::AddWall {
            start,
            end,
            thickness: 200.0,
            height: 2800.0,
        }
    }

    #[test]
    fn a_fresh_document_is_clean_and_has_no_history() {
        let document = Document::new("Plan", 1_760_000_000);
        assert!(!document.dirty());
        assert!(!document.can_undo());
        assert!(!document.can_redo());
    }

    #[test]
    fn applying_a_command_dirties_the_document_and_fills_the_undo_stack() {
        let mut document = Document::new("Plan", 1_760_000_000);
        document
            .apply(
                wall(Point2::new(0.0, 0.0), Point2::new(4000.0, 0.0)),
                1_760_000_100,
            )
            .expect("add wall");
        assert!(document.dirty());
        assert!(document.can_undo());
        assert_eq!(document.project().wall_count(), 1);
        assert_eq!(document.project().modified_epoch_s, 1_760_000_100);
    }

    #[test]
    fn undo_and_redo_restore_the_project() {
        let mut document = Document::new("Plan", 1_760_000_000);
        document
            .apply(
                wall(Point2::new(0.0, 0.0), Point2::new(4000.0, 0.0)),
                1_760_000_100,
            )
            .expect("add wall");
        assert!(document.undo(1_760_000_200));
        assert_eq!(document.project().wall_count(), 0);
        assert!(document.can_redo());
        assert!(document.redo(1_760_000_300));
        assert_eq!(document.project().wall_count(), 1);
    }

    #[test]
    fn a_failed_command_changes_nothing() {
        let mut document = Document::new("Plan", 1_760_000_000);
        let before = document.project().clone();
        let error = document
            .apply(
                wall(Point2::new(0.0, 0.0), Point2::new(1.0, 0.0)),
                1_760_000_100,
            )
            .expect_err("too short");
        assert!(error.to_string().contains("at least 10 mm"));
        assert_eq!(document.project(), &before);
        assert!(!document.can_undo());
        assert!(!document.dirty());
    }

    #[test]
    fn a_new_edit_clears_the_redo_stack() {
        let mut document = Document::new("Plan", 1_760_000_000);
        document
            .apply(
                wall(Point2::new(0.0, 0.0), Point2::new(4000.0, 0.0)),
                1,
            )
            .expect("add wall");
        document.undo(2);
        assert!(document.can_redo());
        document
            .apply(
                wall(Point2::new(0.0, 0.0), Point2::new(0.0, 4000.0)),
                3,
            )
            .expect("add wall");
        assert!(!document.can_redo());
    }

    #[test]
    fn saving_without_a_path_is_a_plain_error() {
        let mut document = Document::new("Plan", 1);
        let error = document.save().expect_err("no path");
        assert!(error.to_string().contains("not been saved"));
    }
}
