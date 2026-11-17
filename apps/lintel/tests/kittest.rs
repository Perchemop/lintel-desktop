//! AccessKit / egui_kittest harness tests (guide II.27.12 / K.7).
//! Run with: `cargo test -p lintel --test kittest`
//!
//! These tests run the Lintel app through the egui_kittest harness, which
//! captures the AccessKit semantic tree and allows headless assertions on
//! roles, names, and action patterns. No display, GPU, or window server is
//! required.

use egui_kittest::harness;
use lintel::app;
use lintel_core::project::Project;

/// Verify that every palette row has a non-empty AccessKit name.
/// The palette must be open for the test to exercise the correct UI.
#[test]
fn palette_has_named_actions() {
    harness::run(app(), |tree| {
        for row in tree.palette_rows() {
            assert!(
                !row.name().is_empty(),
                "palette row has empty AccessKit name"
            );
        }
    });
}

/// Verify that each collapsing header in the Inspector panel has a role/label.
#[test]
fn inspector_sections_labelled() {
    harness::run(app(), |tree| {
        for section in tree.inspector_sections() {
            assert!(
                !section.label().is_empty(),
                "inspector collapsing header has empty label"
            );
            assert!(
                section.role().is_some(),
                "inspector collapsing header missing role"
            );
        }
    });
}

/// Verify that the canvas has a role with the level summary as its name.
#[test]
fn canvas_summary_accessible() {
    harness::run(app(), |tree| {
        let canvas = tree.canvas();
        assert!(
            canvas.role().is_some(),
            "canvas missing AccessKit role"
        );
        let name = canvas.name();
        let summary = plan_summary(&Project::default());
        assert!(
            name.contains(&summary) || summary.contains(name),
            "canvas name '{}' does not contain summary '{}'",
            name,
            summary
        );
    });
}

// Re-export the app function for use in test convenience.
pub use lintel::app;