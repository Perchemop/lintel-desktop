//! Headless unit tests for [`StyledButton`] (ticket T-001, AC-01/AC-02).
//!
//! Rendering runs through `Context::run_ui` — no window, no GPU, no display
//! (guide K.13). The six-state state machine is pure and tested directly;
//! contrast and size rules come from the token table itself so the merge gate
//! fails if a variant ever drops below WCAG AA.

use crate::components::buttons::{ButtonVisualState, StyledButton};
use crate::theme::{
    BUTTON_DANGER_L, BUTTON_DANGER_M, BUTTON_DANGER_S, BUTTON_GHOST_L, BUTTON_GHOST_M,
    BUTTON_GHOST_S, BUTTON_PRIMARY_L, BUTTON_PRIMARY_M, BUTTON_PRIMARY_S, BUTTON_SECONDARY_L,
    BUTTON_SECONDARY_M, BUTTON_SECONDARY_S, ButtonRole, ButtonSize, SURFACE_BASE, contrast_ratio,
    rgb_of,
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

/// All four roles, in token order.
const ALL_ROLES: [ButtonRole; 4] = [
    ButtonRole::Primary,
    ButtonRole::Secondary,
    ButtonRole::Ghost,
    ButtonRole::Danger,
];

/// Run one headless egui pass and hand the panel `Ui` to `f`.
// `Context::run_ui` re-runs its closure, so the body must be `FnMut`; the
// test bodies are pure draws and satisfy that trivially.
fn with_ui(mut f: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    let mut out = ctx.run_ui(egui::RawInput::default(), |root| {
        egui::CentralPanel::default().show(root, |ui| f(ui));
    });
    // epaint panics if a TexturesDelta drops with unapplied deltas; a bare
    // run_ui never feeds a renderer, so mark the delta handled.
    out.textures_delta.clear();
}

/// Run one pass on `ctx` and return the rect of the widget `f` adds.
fn button_rect_with_input(
    ctx: &egui::Context,
    input: egui::RawInput,
    mut f: impl FnMut(&mut egui::Ui) -> egui::Response,
) -> egui::Rect {
    let slot = std::cell::RefCell::new(None);
    let mut out = ctx.run_ui(input, |root| {
        egui::CentralPanel::default().show(root, |ui| {
            *slot.borrow_mut() = Some(f(ui).rect);
        });
    });
    out.textures_delta.clear(); // no renderer consumes the pass output
    slot.into_inner().expect("ui body ran")
}

/// Run one pass on `ctx` with default input and return the widget's rect.
fn button_rect(ctx: &egui::Context, f: impl FnMut(&mut egui::Ui) -> egui::Response) -> egui::Rect {
    button_rect_with_input(ctx, egui::RawInput::default(), f)
}

#[test]
fn all_twelve_variants_render_without_panic() {
    with_ui(|ui| {
        for role in ALL_ROLES {
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
        StyledButton::new("Selected").selected(true).show(ui);
        StyledButton::new("Everything at once")
            .role(ButtonRole::Danger)
            .size(ButtonSize::Lg)
            .loading(true)
            .show(ui);
        StyledButton::new("Selected and loading")
            .selected(true)
            .loading(true)
            .show(ui);
        StyledButton::new("Selected and disabled")
            .selected(true)
            .enabled(false)
            .show(ui);
    });
}

#[test]
fn every_size_honours_the_24pt_floor() {
    with_ui(|ui| {
        for (size, minimum) in [
            (ButtonSize::Sm, 24.0),
            (ButtonSize::Md, 32.0),
            (ButtonSize::Lg, 40.0),
        ] {
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
fn narrow_label_pins_the_24pt_floor_for_every_role() {
    // A one-glyph label is narrower than the floor, so the token minimum —
    // not the text — decides the size (guide II.30.7.5).
    with_ui(|ui| {
        for role in ALL_ROLES {
            let response = StyledButton::new("·")
                .role(role)
                .size(ButtonSize::Sm)
                .show(ui);
            assert!(
                response.rect.width() >= 24.0,
                "{role:?}: width {} is below the 24pt floor",
                response.rect.width()
            );
            assert!(
                response.rect.height() >= 24.0,
                "{role:?}: height {} is below the 24pt floor",
                response.rect.height()
            );
        }
    });
}

#[test]
fn long_labels_grow_the_button_beyond_the_floor() {
    // `min_size` is a floor, never a cap: a long label must widen the button
    // instead of clipping (guide II.30.7.6).
    with_ui(|ui| {
        let narrow = StyledButton::new("·")
            .size(ButtonSize::Sm)
            .show(ui)
            .rect
            .width();
        let wide = StyledButton::new("Construction circle")
            .size(ButtonSize::Sm)
            .show(ui)
            .rect
            .width();
        assert!(
            wide > narrow,
            "a long label must grow the button (narrow {narrow}, wide {wide}), not clip"
        );
        assert!(wide >= 24.0);
    });
}

#[test]
fn loading_keeps_size_stable() {
    with_ui(|ui| {
        for role in ALL_ROLES {
            for size in [ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg] {
                let normal = StyledButton::new("Label")
                    .role(role)
                    .size(size)
                    .show(ui)
                    .rect
                    .size();
                let loading = StyledButton::new("Label")
                    .role(role)
                    .size(size)
                    .loading(true)
                    .show(ui)
                    .rect
                    .size();
                assert_eq!(
                    normal, loading,
                    "{role:?}/{size:?}: the loading spinner must not shift layout"
                );
            }
        }
        // A label wider than the token floor: the label's reserved width must
        // survive loading, because the spinner is drawn inside that space.
        let normal = StyledButton::new("Construction circle")
            .show(ui)
            .rect
            .size();
        let loading = StyledButton::new("Construction circle")
            .loading(true)
            .show(ui)
            .rect
            .size();
        assert_eq!(normal, loading, "a wide label's size must survive loading");
        assert!(
            normal.x > 40.0,
            "the label must exceed the largest floor for this check to bind (got {normal:?})"
        );
    });
}

#[test]
fn selected_state_renders_and_keeps_size() {
    with_ui(|ui| {
        let normal = StyledButton::new("Wall").show(ui).rect.size();
        let selected = StyledButton::new("Wall")
            .selected(true)
            .show(ui)
            .rect
            .size();
        assert_eq!(normal, selected, "selection must not shift layout");
    });
}

#[test]
fn hover_does_not_move_or_resize_a_button() {
    // Two passes on one context: the second moves the pointer onto the
    // button, exercising the hover fill path (guide II.30.10).
    let ctx = egui::Context::default();
    let at_rest = button_rect(&ctx, |ui| StyledButton::new("Hover me").show(ui));

    let mut hovered_input = egui::RawInput::default();
    hovered_input
        .events
        .push(egui::Event::PointerMoved(at_rest.center()));
    let hovered = button_rect_with_input(&ctx, hovered_input, |ui| {
        StyledButton::new("Hover me").show(ui)
    });

    assert_eq!(at_rest, hovered, "hover must not move or resize the button");
}

#[test]
fn hovering_a_disabled_button_is_stable_and_panic_free() {
    // The second pass hovers a disabled button, running the
    // "explain why" tooltip path end to end (guide II.30.10).
    let ctx = egui::Context::default();
    let at_rest = button_rect(&ctx, |ui| {
        StyledButton::new("Delete").enabled(false).show(ui)
    });

    let mut hovered_input = egui::RawInput::default();
    hovered_input
        .events
        .push(egui::Event::PointerMoved(at_rest.center()));
    let hovered = button_rect_with_input(&ctx, hovered_input, |ui| {
        StyledButton::new("Delete").enabled(false).show(ui)
    });
    assert_eq!(
        at_rest, hovered,
        "hover on a disabled button must not move or resize it"
    );
}

#[test]
fn visual_state_precedence() {
    // Disabled > Loading > Selected > Active > Hover > Default.
    assert_eq!(
        ButtonVisualState::resolve(false, true, true, true, true),
        ButtonVisualState::Disabled,
        "disabled beats every other flag"
    );
    assert_eq!(
        ButtonVisualState::resolve(true, true, true, true, true),
        ButtonVisualState::Loading,
        "loading beats selected/pressed/hover"
    );
    assert_eq!(
        ButtonVisualState::resolve(true, false, true, true, true),
        ButtonVisualState::Selected,
        "selected beats pressed/hover"
    );
    assert_eq!(
        ButtonVisualState::resolve(true, false, false, true, true),
        ButtonVisualState::Active,
        "pressed beats hover"
    );
    assert_eq!(
        ButtonVisualState::resolve(true, false, false, true, false),
        ButtonVisualState::Hover
    );
    assert_eq!(
        ButtonVisualState::resolve(true, false, false, false, false),
        ButtonVisualState::Default
    );
}

#[test]
fn visual_state_colours_resolve_through_the_role_tokens() {
    let spec = BUTTON_PRIMARY_M;
    // Loading keeps the rest fill and the normal label colour — the theme
    // contract in `theme.rs` (spinner in the label colour, no size change).
    assert_eq!(ButtonVisualState::Loading.fill(&spec), spec.fill);
    assert_eq!(ButtonVisualState::Loading.text(&spec), spec.text);
    assert_eq!(ButtonVisualState::Default.fill(&spec), spec.fill);
    assert_eq!(ButtonVisualState::Default.text(&spec), spec.text);
    assert_eq!(ButtonVisualState::Hover.fill(&spec), spec.hover);
    assert_eq!(ButtonVisualState::Hover.text(&spec), spec.text);
    assert_eq!(ButtonVisualState::Active.fill(&spec), spec.active);
    assert_eq!(ButtonVisualState::Active.text(&spec), spec.text_active);
    assert_eq!(ButtonVisualState::Selected.fill(&spec), spec.active);
    assert_eq!(ButtonVisualState::Selected.text(&spec), spec.text_active);
    assert_eq!(ButtonVisualState::Disabled.fill(&spec), spec.disabled);
    assert_eq!(ButtonVisualState::Disabled.text(&spec), spec.text_disabled);
}

#[test]
fn non_interactive_states_refuse_interaction() {
    for state in [
        ButtonVisualState::Default,
        ButtonVisualState::Hover,
        ButtonVisualState::Active,
        ButtonVisualState::Selected,
    ] {
        assert!(state.interactive(), "{state:?} must accept interaction");
    }
    for state in [ButtonVisualState::Disabled, ButtonVisualState::Loading] {
        assert!(
            !state.interactive(),
            "{state:?} must not accept interaction"
        );
    }
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

        // Ghost buttons have no fill of their own; their label reads against
        // the sheet it sits on (the palette-level gate applies the same rule).
        let fill_surface = if spec.fill.a() == 0 {
            SURFACE_BASE
        } else {
            spec.fill
        };

        // Default, hover and pressed labels are normal text (4.5:1).
        assert!(
            contrast_ratio(text, rgb_of(fill_surface)) >= 4.5,
            "{name}: label on fill is {:.2}:1",
            contrast_ratio(text, rgb_of(fill_surface))
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
