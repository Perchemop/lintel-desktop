//! AccessKit assertions for [`StyledButton`] via the `egui_kittest` harness
//! (ticket T-001, AC-02): role = button, accessible name present, size ≥ 24pt,
//! and every variant reachable with Tab. Headless — no display, no GPU.

use crate::components::buttons::{ButtonRole, ButtonSize, StyledButton};
use eframe::egui;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};

/// Unique label per variant, in creation (tab) order.
const LABELS: [&str; 12] = [
    "B01", "B02", "B03", "B04", "B05", "B06", "B07", "B08", "B09", "B10", "B11", "B12",
];

fn all_variants_ui(ui: &mut egui::Ui) {
    let roles = [
        ButtonRole::Primary,
        ButtonRole::Secondary,
        ButtonRole::Ghost,
        ButtonRole::Danger,
    ];
    let sizes = [ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg];
    for (index, label) in LABELS.into_iter().enumerate() {
        let role = roles[index / 3];
        let size = sizes[index % 3];
        StyledButton::new(label).role(role).size(size).show(ui);
    }
}

#[test]
fn every_variant_is_a_named_button_of_at_least_24pt() {
    let mut harness = egui_kittest::Harness::new_ui(all_variants_ui);
    harness.run();

    let buttons: Vec<_> = harness.get_all_by_role(Role::Button).collect();
    assert_eq!(buttons.len(), 12, "all 12 variants must reach AccessKit");

    for node in &buttons {
        let name = node.accesskit_node().label().unwrap_or_default();
        assert_eq!(
            node.accesskit_node().role(),
            Role::Button,
            "{name}: role must be Button"
        );
        assert!(
            LABELS.contains(&name.as_str()),
            "accessible name {name:?} must be the human label"
        );
        let rect = node.rect();
        assert!(
            rect.width() >= 24.0 && rect.height() >= 24.0,
            "{name}: {rect:?} is below the 24pt target floor (II.30.7.5)"
        );
    }

    // Each of the twelve labels is queryable by name.
    for label in LABELS {
        let node = harness.get_by_label(label);
        assert_eq!(
            node.accesskit_node().label().as_deref(),
            Some(label),
            "{label} must be findable by its accessible name"
        );
    }
}

#[test]
fn tab_reaches_every_button_and_reports_focus() {
    let mut harness = egui_kittest::Harness::new_ui(all_variants_ui);
    harness.run();

    for (step, label) in LABELS.into_iter().enumerate() {
        harness.key_press(egui::Key::Tab);
        harness.run();

        let node = harness.get_by_label(label);
        assert!(
            node.is_focused(),
            "Tab press {} must focus {label} (focus ring is painted on has_focus)",
            step + 1
        );
    }
}
