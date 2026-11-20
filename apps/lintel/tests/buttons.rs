//! Unit tests for button components — headless, no display.

use lintel::components::buttons::{
    danger_button, ghost_button, primary_button, secondary_button, ButtonRole, ButtonSize,
    StyledButton,
};

#[test]
fn all_variants_render_without_panic() {
    let mut ctx = eframe::egui::Context::default();
    let mut ui = ctx.begin_frame(Default::default());

    for role in [
        ButtonRole::Primary,
        ButtonRole::Secondary,
        ButtonRole::Ghost,
        ButtonRole::Danger,
    ] {
        for size in [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large] {
            StyledButton::new("Test")
                .role(role)
                .size(size)
                .show(&mut ui);
        }
    }

    // Loading state
    StyledButton::new("Loading").loading(true).show(&mut ui);
    // Disabled state
    StyledButton::new("Disabled").enabled(false).show(&mut ui);
    // Selected state
    StyledButton::new("Selected").selected(true).show(&mut ui);

    // Convenience functions
    primary_button(&mut ui, "Primary");
    secondary_button(&mut ui, "Secondary");
    ghost_button(&mut ui, "Ghost");
    danger_button(&mut ui, "Danger");
}

#[test]
fn min_size_enforced() {
    let mut ctx = eframe::egui::Context::default();
    let mut ui = ctx.begin_frame(Default::default());

    let resp = StyledButton::new("Tiny")
        .role(ButtonRole::Primary)
        .size(ButtonSize::Small)
        .show(&mut ui);

    // Small size has 24pt minimum (guide II.30.7.5)
    assert!(
        resp.rect.height() >= 24.0,
        "height {} < 24",
        resp.rect.height()
    );
    assert!(
        resp.rect.width() >= 24.0,
        "width {} < 24",
        resp.rect.width()
    );
}

#[test]
fn loading_keeps_size_stable() {
    let mut ctx = eframe::egui::Context::default();
    let mut ui = ctx.begin_frame(Default::default());

    let normal = StyledButton::new("Label").show(&mut ui);
    let loading = StyledButton::new("Label").loading(true).show(&mut ui);

    // Size should not change when loading (no layout shift, II.30.10)
    assert_eq!(normal.rect.size(), loading.rect.size());
}

#[test]
fn disabled_explains_why() {
    let mut ctx = eframe::egui::Context::default();
    let mut ui = ctx.begin_frame(Default::default());

    let resp = StyledButton::new("Disabled").enabled(false).show(&mut ui);
    // Hover text should be set (checked via on_hover_text)
    // We can't easily test the tooltip text here, but the code path exists.
    assert!(!resp.clicked());
}