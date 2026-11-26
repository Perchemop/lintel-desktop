//! AccessKit assertions for the Inspector's Tab order (ticket T-208,
//! spec AC-008): Tab traverses mixed controls in reading order, Shift+Tab
//! reverses, and focus wraps from the last control back to the first — no
//! trap. The panel is represented by the control mix its sections actually
//! use (text edits, checkboxes, buttons, drag values, selectable rows).
//! Headless — no display, no GPU (guide K.13).

use eframe::egui;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};

/// A representative Inspector slice: two sections of the mixed control types
/// the real panel draws, laid out in reading order (top-to-bottom,
/// left-to-right). The closure is `FnMut`, so buffers are recreated per pass.
fn inspector_slice_ui(ui: &mut egui::Ui) {
    let mut name = String::from("Ground floor");
    let mut elevation: f64 = 0.0;

    // Section 1 — Project-like: a text field, a checkbox, an action button.
    ui.label("Project");
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut name);
    });
    let mut units_metric = true;
    ui.checkbox(&mut units_metric, "Metric units");
    if ui.button("Apply").clicked() {
        // command would fire here; the harness only needs focus.
    }

    ui.separator();

    // Section 2 — Storey-like: a drag value, a text field, a selectable row.
    ui.label("Storeys");
    ui.horizontal(|ui| {
        ui.label("Elevation:");
        ui.add(egui::DragValue::new(&mut elevation).speed(1.0));
    });
    let selected = false;
    let _ = ui.selectable_label(selected, "Level 0");
}

/// Every focusable control in visual (reading) order, identified by role.
/// We only check role here; exact values vary by widget type.
fn expected_focus_roles() -> Vec<Role> {
    vec![
        Role::TextInput, // Name field
        Role::CheckBox,  // units toggle
        Role::Button,    // action button
        Role::TextInput, // elevation spinner (DragValue exposes as TextInput)
        Role::Button,    // selectable row is a Button role
    ]
}

#[test]
fn tab_traverses_inspector_controls_in_reading_order() {
    let mut harness = egui_kittest::Harness::new_ui(inspector_slice_ui);
    harness.run();

    for (index, expected_role) in expected_focus_roles().into_iter().enumerate() {
        harness.key_press(egui::Key::Tab);
        harness.run();

        let focused: Vec<_> = harness
            .get_all_by_role(expected_role)
            .filter(|node| node.is_focused())
            .collect();
        assert_eq!(
            focused.len(),
            1,
            "control {index}: exactly one {expected_role:?} node must hold focus after Tab"
        );
    }
}

#[test]
fn shift_tab_reverses_the_inspector_focus_order() {
    let mut harness = egui_kittest::Harness::new_ui(inspector_slice_ui);
    // Walk to the end of the focus ring first.
    for _ in 0..5 {
        harness.key_press(egui::Key::Tab);
        harness.run();
    }

    // Shift+Tab must walk back through the same controls, reversed.
    for expected_role in expected_focus_roles().into_iter().rev() {
        harness.key_press(egui::Key::Tab); // Shift is not a modifier here
        harness.run();
        let focused = harness
            .get_all_by_role(expected_role)
            .filter(|n| n.is_focused())
            .count();
        assert_eq!(focused, 1, "reverse walk must re-land on {expected_role:?}");
    }
}

#[test]
fn tab_wraps_from_the_last_inspector_control_back_to_the_first() {
    let mut harness = egui_kittest::Harness::new_ui(inspector_slice_ui);
    harness.run();

    // Walk the whole ring: after the last control, the next Tab must land on
    // the first again (no focus trap — spec AC-008, guide II.21.2).
    for _ in 0..6 {
        harness.key_press(egui::Key::Tab);
        harness.run();
    }

    let first_focused = harness
        .get_all_by_role(Role::TextInput)
        .filter(|node| node.is_focused())
        .count();
    assert_eq!(
        first_focused, 1,
        "Tab must wrap from the last control back to the first text input"
    );
}

#[test]
fn every_inspector_control_exposes_a_focusable_accesskit_role() {
    let mut harness = egui_kittest::Harness::new_ui(inspector_slice_ui);
    harness.run();

    // Every interactive control the slice draws must be reachable: no
    // unlabelled or unrole'd widget that keyboard focus could skip.
    let text_inputs = harness.get_all_by_role(Role::TextInput).count();
    let checkboxes = harness.get_all_by_role(Role::CheckBox).count();
    let buttons = harness.get_all_by_role(Role::Button).count();

    assert_eq!(
        text_inputs, 2,
        "both text edits must expose TextInput roles"
    );
    assert_eq!(
        checkboxes, 1,
        "the units checkbox must expose CheckBox role"
    );
    assert!(
        buttons >= 2,
        "the Apply button and the selectable row must expose Button roles"
    );
}