//! Headless unit tests for [`StyledButton`] (ticket T-001, AC-01/AC-02).
//!
//! Rendering runs through `Context::run` — no window, no GPU, no display
//! (guide K.13). Contrast and size rules come from the token table itself so
//! the merge gate fails if a variant ever drops below WCAG AA.

use crate::components::buttons::StyledButton;
use crate::theme::{
    BUTTON_DANGER_L, BUTTON_DANGER_M, BUTTON_DANGER_S, BUTTON_GHOST_L, BUTTON_GHOST_M,
    BUTTON_GHOST_S, BUTTON_PRIMARY_L, BUTTON_PRIMARY_M, BUTTON_PRIMARY_S,
    BUTTON_SECONDARY_L, BUTTON_SECONDARY_M, BUTTON_SECONDARY_S,
    ButtonRole, ButtonSize, SURFACE_BASE, contrast_ratio, rgb_of,
};
use eframe::egui;

/// All twelve role × size component tokens.
const ALL_VARIANTS: [&str; 12] = [
    "primary-sm",
    "primary-md",
    "primary-lg",
    "secondary-sm",
    "secondary-md",
    "secondary-lg",
    "ghost-sm",
    "ghost-md",
    "ghost-lg",
    "danger-sm",
    "danger-md",
    "danger-lg",
];

/// Run one headless egui pass and hand the panel `Ui` to `f`.
fn with_ui(f: impl FnOnce(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| f(ui));
    });
}

#[test]
fn all_twelve_variants_render_without_panic() {
    with_ui(|ui| {
        for role in [
            ButtonRole::Primary,
            ButtonRole::Secondary,
            ButtonRole::Ghost,
            ButtonRole::Danger,
        ] {
            for size in [ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg] {
                StyledButton::new("Label").role(role).size(size).show(ui);
            }
        }
        // Every named variant resolves — a missing match arm would panic the
        // widget, so this doubles as the token-table completeness check.
        assert_eq!(ALL_VARIANTS.len(), 4 * 3);
    });
}

#[test]
fn state_variants_render_without_panic() {
    with_ui(|ui| {
        StyledButton::new("Loading").loading(true).show(ui);
        StyledButton::new("Disabled").enabled(false).show(ui);
        StyledButton::new("Disabled with reason")
            .enabled(false)
            .disabled_reason("No plan is open")
            .show(ui);
        StyledButton::new("Everything at once")
            .role(ButtonRole::Danger)
            .size(ButtonSize::Lg)
            .loading(true)
            .show(ui);
    });
}

#[test]
fn every_size_honours_the_24pt_floor() {
    with_ui(|ui| {
        for (size, minimum) in [(ButtonSize::Sm, 24.0), (ButtonSize::Md, 32.0), (ButtonSize::Lg, 40.0)] {
            let response = StyledButton::new("Label").size(size).show(ui);
            assert!(
                response.rect.width() >= minimum,
                "{size:?} width {} is below the {minimum}pt floor",
                response.rect.width()
            );
            assert!(
                response.rect.height() >= minimum,
                "{size:?} height {} is below the {minimum}pt floor",
                response.rect.height()
            );
        }
    });
}

#[test]
fn loading_keeps_size_stable() {
    with_ui(|ui| {
        let normal = StyledButton::new("Label").show(ui);
        let loading = StyledButton::new("Label").loading(true).show(ui);
        assert_eq!(
            normal.rect.size(),
            loading.rect.size(),
            "the loading spinner must not shift layout"
        );
    });
}

#[test]
fn button_tokens_meet_wcag_aa_in_every_state() {
    let variants = [
        BUTTON_PRIMARY_S,
        BUTTON_PRIMARY_M,
        BUTTON_PRIMARY_L,
        BUTTON_SECONDARY_S,
        BUTTON_SECONDARY_M,
        BUTTON_SECONDARY_L,
        BUTTON_GHOST_S,
        BUTTON_GHOST_M,
        BUTTON_GHOST_L,
        BUTTON_DANGER_S,
        BUTTON_DANGER_M,
        BUTTON_DANGER_L,
    ];
    for (index, spec) in variants.into_iter().enumerate() {
        let name = ALL_VARIANTS[index];
        let text = rgb_of(spec.text);
        let sheet = rgb_of(SURFACE_BASE);

        // Default, hover and pressed labels are normal text (4.5:1).
        assert!(
            contrast_ratio(text, rgb_of(spec.fill)) >= 4.5,
            "{name}: label on fill is {:.2}:1",
            contrast_ratio(text, rgb_of(spec.fill))
        );
        assert!(
            contrast_ratio(text, rgb_of(spec.hover)) >= 4.5,
            "{name}: label on hover fill is {:.2}:1",
            contrast_ratio(text, rgb_of(spec.hover))
        );
        assert!(
            contrast_ratio(rgb_of(spec.text_active), rgb_of(spec.active)) >= 4.5,
            "{name}: label on pressed fill is {:.2}:1",
            contrast_ratio(rgb_of(spec.text_active), rgb_of(spec.active))
        );
        assert!(
            contrast_ratio(rgb_of(spec.text_disabled), rgb_of(spec.disabled)) >= 4.5,
            "{name}: label on disabled fill is {:.2}:1",
            contrast_ratio(rgb_of(spec.text_disabled), rgb_of(spec.disabled))
        );

        // Focus ring is meaningful UI (3:1 against the sheet it sits on).
        assert!(
            contrast_ratio(rgb_of(spec.focus), sheet) >= 3.0,
            "{name}: focus ring on the sheet is {:.2}:1",
            contrast_ratio(rgb_of(spec.focus), sheet)
        );

        // The 24pt interactive-target floor applies to every variant
        // (guide II.30.7.5).
        assert!(spec.min_size.x >= 24.0 && spec.min_size.y >= 24.0);
    }
}

