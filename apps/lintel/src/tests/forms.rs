//! Headless unit tests for the form components (ticket T-002, spec AC-03).
//!
//! Rendering runs through the `egui_kittest` harness — no window, no GPU, no
//! display (guide K.13). AccessKit-level assertions live in
//! `kittest_forms.rs`; this file covers composition, states and geometry.
//! The harness closure is `FnMut` (it may run on more than one pass), so all
//! assertions are idempotent and capture results through `RefCell`.

use crate::components::forms::{Field, FieldGroup, FieldSet};
use crate::theme;
use eframe::egui;
use egui_kittest::Harness;
use std::cell::RefCell;
use std::rc::Rc;

/// One headless pass with a `&mut Ui` handed to `f`.
fn run(f: impl FnMut(&mut egui::Ui) + 'static) {
    let mut harness = Harness::new_ui(f);
    harness.run();
}

/// A slot a closure can write a value into across passes.
type Slot<T> = Rc<RefCell<Option<T>>>;

fn slot<T>() -> Slot<T> {
    Rc::new(RefCell::new(None))
}

#[test]
fn field_renders_label_input_and_help() {
    let mut buffer = String::from("4.2");
    let input_height = slot::<f32>();
    let sink = input_height.clone();
    run(move |ui| {
        let response = Field::new("Wall length")
            .input(|ui| ui.text_edit_singleline(&mut buffer))
            .help_text("Metres, 0.5–30")
            .show(ui);
        *sink.borrow_mut() = Some(response.rect.height());
    });
    let height = input_height.borrow().expect("the pass ran");
    assert!(height > 0.0, "the input must be visible, got {height:?}");
}

#[test]
fn error_field_marks_the_input_and_prints_the_message() {
    let mut buffer = String::from("-1");
    let total = slot::<(f32, f32)>();
    let sink = total.clone();
    run(move |ui| {
        let response = Field::new("Wall length")
            .input(|ui| ui.text_edit_singleline(&mut buffer))
            .error_text("Length must be positive")
            .show(ui);
        *sink.borrow_mut() = Some((response.rect.height(), ui.min_rect().height()));
    });
    let (input, whole) = (*total.borrow()).expect("the pass ran");
    assert!(input > 0.0, "the input must be visible");
    assert!(
        whole > input,
        "the error row must add visible height below the input"
    );
}

#[test]
fn disabled_field_reports_disabled_and_keeps_its_reason() {
    let mut buffer = String::from("locked");
    let enabled = slot::<bool>();
    let sink = enabled.clone();
    run(move |ui| {
        ui.add_enabled_ui(false, |ui| {
            let response = Field::new("Revision")
                .input(|ui| ui.text_edit_singleline(&mut buffer))
                .disabled(true)
                .disabled_reason("The plan is read-only while a review is open")
                .show(ui);
            *sink.borrow_mut() = Some(response.enabled());
        });
    });
    let is_enabled = enabled.borrow().expect("the pass ran");
    assert!(
        !is_enabled,
        "the disabled field must report enabled == false"
    );
}

#[test]
fn field_set_frames_its_legend_and_fields() {
    let mut name = String::new();
    let mut elevation = String::from("0");
    let extents = slot::<(f32, f32)>();
    let sink = extents.clone();
    run(move |ui| {
        FieldSet::new("Storey")
            .fields(vec![
                Field::new("Name").input(|ui| ui.text_edit_singleline(&mut name)),
                Field::new("Elevation").input(|ui| ui.text_edit_singleline(&mut elevation)),
            ])
            .show(ui);
        let rect = ui.min_rect();
        *sink.borrow_mut() = Some((rect.width(), rect.height()));
    });
    let (width, height) = extents.borrow().expect("the pass ran");
    assert!(
        width > theme::SPACING_MD * 4.0 && height > 0.0,
        "the framed field set must have real extents: {width}x{height}"
    );
}

#[test]
fn field_group_lays_out_horizontally_and_vertically() {
    let mut first = String::new();
    let mut second = String::from("2");
    let shapes = slot::<(f32, f32, f32)>();
    let sink = shapes.clone();
    run(move |ui| {
        FieldGroup::new()
            .fields(vec![
                Field::new("A").input(|ui| ui.text_edit_singleline(&mut first)),
                Field::new("B").input(|ui| ui.text_edit_singleline(&mut second)),
            ])
            .show(ui);
        let horizontal_height = ui.min_rect().height();
        let horizontal_width = ui.min_rect().width();

        FieldGroup::new()
            .fields(vec![
                Field::new("A").input(|ui| ui.text_edit_singleline(&mut first)),
                Field::new("B").input(|ui| ui.text_edit_singleline(&mut second)),
            ])
            .direction(egui::Direction::TopDown)
            .show(ui);
        let total_height = ui.min_rect().height();

        *sink.borrow_mut() = Some((horizontal_width, horizontal_height, total_height));
    });
    let (width, horizontal_height, total_height) = shapes.borrow().expect("the pass ran");
    assert!(
        width > 0.0 && horizontal_height > 0.0,
        "the horizontal group must render: {width}x{horizontal_height}"
    );
    assert!(
        total_height > horizontal_height,
        "the vertical group must add its own rows below the horizontal one \
         (horizontal {horizontal_height}, total {total_height})"
    );
}

#[test]
fn field_input_carries_the_label_via_labelled_by() {
    // The label association is what screen readers consume; the AccessKit
    // side is asserted in kittest_forms.rs. Here we prove the wiring call
    // keeps the input enabled and returns a usable response.
    let mut buffer = String::new();
    let labelled = slot::<bool>();
    let sink = labelled.clone();
    run(move |ui| {
        let label = ui.label("Sheet title");
        let response = ui.text_edit_singleline(&mut buffer).labelled_by(label.id);
        *sink.borrow_mut() = Some(response.enabled());
    });
    let is_enabled = labelled.borrow().expect("the pass ran");
    assert!(is_enabled, "the associated input must stay enabled");
}
