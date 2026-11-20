//! AccessKit assertions for the form components via the `egui_kittest`
//! harness (ticket T-002, spec AC-03; ticket T-008): labels are associated
//! with their inputs, errors are announced as text, and Tab follows visual
//! order. Headless — no display, no GPU (guide K.13).

use crate::components::buttons::{ButtonRole, StyledButton};
use crate::components::forms::{Field, FieldSet};
use eframe::egui;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};

/// A representative form: a framed field set plus the primary submit.
fn storey_form_ui(ui: &mut egui::Ui) {
    let mut name = String::from("Ground floor");
    let mut elevation = String::from("0");
    FieldSet::new("Storey")
        .fields(vec![
            Field::new("Name").input(|ui| ui.text_edit_singleline(&mut name)),
            Field::new("Elevation").input(|ui| ui.text_edit_singleline(&mut elevation)),
        ])
        .show(ui);
    StyledButton::new("Save storey")
        .role(ButtonRole::Primary)
        .show(ui);
}

#[test]
fn inputs_carry_their_labels_and_tab_follows_visual_order() {
    let mut harness = egui_kittest::Harness::new_ui(storey_form_ui);
    harness.run();

    // The field labels are real AccessKit nodes…
    assert!(
        harness.query_by_label("Name").is_some(),
        "the 'Name' label must reach AccessKit"
    );
    assert!(
        harness.query_by_label("Elevation").is_some(),
        "the 'Elevation' label must reach AccessKit"
    );

    // …and each text input points back at a label through the labelled-by
    // relation, so a screen reader announces the field name with the input.
    let inputs: Vec<_> = harness.get_all_by_role(Role::TextInput).collect();
    assert_eq!(inputs.len(), 2, "both fields must expose TextInput roles");
    for node in &inputs {
        let labelled_by = node.accesskit_node().labelled_by();
        assert!(
            !labelled_by.is_empty(),
            "every form input must be associated with its label (II.30.7.4)"
        );
    }

    // Tab order follows visual order: Name ("Ground floor"), Elevation
    // ("0"), then the submit. The input's value identifies which field is
    // focused without relying on paint order.
    for expected_value in ["Ground floor", "0"] {
        harness.key_press(egui::Key::Tab);
        harness.run();
        let focused: Vec<_> = harness
            .get_all_by_role(Role::TextInput)
            .filter(|node| node.is_focused())
            .collect();
        assert_eq!(
            focused.len(),
            1,
            "exactly one text input must hold focus after Tab"
        );
        let value = focused[0]
            .accesskit_node()
            .value()
            .unwrap_or_default();
        assert_eq!(
            value.as_str(),
            expected_value,
            "Tab must land on the next field in visual order"
        );
    }
    harness.key_press(egui::Key::Tab);
    harness.run();
    let submit = harness.get_by_label("Save storey");
    assert!(
        submit.is_focused(),
        "the third Tab must reach the submit button"
    );
}

#[test]
fn the_error_state_is_announced_as_text() {
    let mut buffer = String::from("-1");
    let mut harness = egui_kittest::Harness::new_ui(move |ui| {
        // Recreate the buffer each pass: the harness closure is FnMut, and a
        // static error keeps the tree stable across passes.
        let mut buffer = buffer.clone();
        Field::new("Wall length")
            .input(|ui| ui.text_edit_singleline(&mut buffer))
            .error_text("Length must be positive")
            .show(ui);
    });
    harness.run();

    // Colour alone is never the signal (II.30.7.2): the message itself must
    // be a labelled AccessKit node. egui exposes labels either as Label or
    // StaticText depending on context — both are announced.
    let error = harness.get_by_label("Length must be positive");
    let role = error.accesskit_node().role();
    assert!(
        matches!(role, Role::Label | Role::StaticText),
        "the error message must be announced as a label, got {role:?}"
    );
}

#[test]
fn help_text_is_announced_as_text() {
    let mut buffer = String::from("4.2");
    let mut harness = egui_kittest::Harness::new_ui(move |ui| {
        let mut buffer = buffer.clone();
        Field::new("Wall length")
            .input(|ui| ui.text_edit_singleline(&mut buffer))
            .help_text("Metres, 0.5–30")
            .show(ui);
    });
    harness.run();

    assert!(
        harness.query_by_label("Metres, 0.5–30").is_some(),
        "the help hint must reach AccessKit"
    );
}
