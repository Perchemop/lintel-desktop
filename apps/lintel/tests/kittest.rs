//! Environment smoke test for the `kittest` CI check.
//!
//! Lintel is a binary crate, so integration tests cannot import its internals
//! (a `lib` target split is tracked for a later slice). The AccessKit
//! assertions for the app's own widgets live in
//! `apps/lintel/src/tests/kittest_buttons.rs` and run under
//! `cargo test --workspace`. This target keeps the `kittest` required check
//! meaningful on its own: it proves the egui_kittest harness itself works
//! headlessly on CI — real egui widgets, a real AccessKit tree, no display.

use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn harness_exposes_an_egui_button_with_role_and_label() {
    let mut harness = egui_kittest::Harness::new_ui(|ui| {
        ui.button("Harness smoke");
    });
    harness.run();

    let node = harness.get_by_label("Harness smoke");
    assert_eq!(
        node.accesskit_node().role(),
        egui::accesskit::Role::Button,
        "the widget must expose role=Button to AccessKit"
    );
    assert_eq!(
        node.accesskit_node().label().as_deref(),
        Some("Harness smoke"),
        "the button must expose its label as accessible name"
    );

    // A click through the harness must not panic the headless pass.
    node.click();
    harness.run();
}
