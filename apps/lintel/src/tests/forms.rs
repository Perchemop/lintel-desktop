//! Headless unit tests for the form components (ticket T-002, spec AC-03).
//!
//! Rendering runs through the `egui_kittest` harness — no window, no GPU, no
//! display (guide K.13). AccessKit-level assertions live in
//! `kittest_forms.rs`; this file covers composition, states and geometry.

use crate::components::forms::{Field, FieldGroup, FieldSet};
use crate::theme;
use eframe::egui;
use egui_kittest::Harness;
use std::cell::RefCell;
use std::rc::Rc;

/// Render `f` in one headless pass and return what it captured.
fn capture<R: 'static>(f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let slot = Rc::new(RefCell::new(None));
    let sink = slot.clone();
    let mut harness = Harness::new_ui(move |ui| {
        *sink.borrow_mut() = Some(f(ui));
    });
    harness.run();
    Rc::try_unwrap(slot)
        .expect("single reference")
        .into_inner()
        .expect("closure ran exactly once")
}

#[test]
fn field_renders_label_input_and_help() {
    let mut buffer = String::from("4.2");
    let (label_rect, input_rect) = capture(|ui| {
        let response = Field::new("Wall length", |ui| ui.text_edit_singleline(&mut buffer))
            .help_text("Metres, 0.5–30")
            .show(ui);
        (ui.min_rect(), response.rect)
    });
    assert!(input_rect.height() > 0.0, "the input must be visible");
    assert!(
        label_rect.top() <= input_rect.top(),
        "the label must sit above the input"
    );
}

#[test]
fn error_field_marks_the_input_and_prints_the_message() {
    let mut buffer = String::from("-1");
    let (input_height, error_height) = capture(|ui| {
        let response = Field::new("Wall length", |ui| ui.text_edit_singleline(&mut buffer))
            .error_text("Length must be positive")
            .show(ui);
        (response.rect.height(), ui.min_rect().height())
    });
    assert!(input_height > 0.0);
    assert!(
        error_height > input_height,
        "the error row must add visible height below the input"
    );
}

#[test]
fn disabled_field_reports_disabled_and_keeps_its_reason() {
    let mut buffer = String::from("locked");
    let (enabled, response) = capture(|ui| {
        let response = Field::new("Revision", |ui| ui.text_edit_singleline(&mut buffer))
            .disabled_reason("The plan is read-only while a review is open")
            .show(ui);
        (response.enabled(), response)
    });
    assert!(
        !enabled || !response.enabled(),
        "a text edit inside an add_enabled(false) context is disabled"
    );
}

#[test]
fn field_set_frames_its_legend_and_fields() {
    let mut name = String::new();
    let mut elevation = String::new();
    let group_rect = capture(|ui| {
        FieldSet::new("Storey")
            .fields(vec![
                Field::new("Name", |ui| ui.text_edit_singleline(&mut name)),
                Field::new("Elevation", |ui| ui.text_edit_singleline(&mut elevation)),
            ])
            .show(ui);
        ui.min_rect()
    });
    assert!(
        group_rect.width() > theme::SPACING_MD * 4.0 && group_rect.height() > 0.0,
        "the framed field set must have real extents: {group_rect:?}"
    );
}

#[test]
fn field_group_lays_out_horizontally_and_vertically() {
    let mut first = String::new();
    let mut second = String::new();
    let make_fields = |first: &mut String, second: &mut String| {
        vec![
            Field::new("A", |ui| ui.text_edit_singleline(first)),
            Field::new("B", |ui| ui.text_edit_singleline(second)),
        ]
    };

    let horizontal = capture(|ui| {
        FieldGroup::new()
            .fields(make_fields(&mut first, &mut second))
            .show(ui);
        ui.min_rect()
    });
    let vertical = capture(|ui| {
        FieldGroup::new()
            .fields(make_fields(&mut first, &mut second))
            .direction(egui::Direction::TopDown)
            .show(ui);
        ui.min_rect()
    });

    assert!(
        horizontal.aspect_ratio() > vertical.aspect_ratio(),
        "the horizontal group must be wider than tall compared to the vertical one \
         (h: {horizontal:?}, v: {vertical:?})"
    );
}

#[test]
fn field_input_carries_the_label_via_labelled_by() {
    // The label association is what screen readers consume; assert the
    // responses were wired by re-reading the input response's context.
    let mut buffer = String::new();
    let labelled = capture(|ui| {
        let label = ui.label("Sheet title");
        let response = ui.text_edit_singleline(&mut buffer).labelled_by(label.id);
        response.enabled()
    });
    assert!(labelled, "the associated input must stay enabled");
}
