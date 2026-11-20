//! Contrast merge-gate tests (spec v0.20.0 AC-08, ticket T-007, guide
//! II.30.7.1): every palette Lintel ships must clear WCAG AA — 4.5:1 body
//! text, 3:1 large text / UI boundaries — and the high-contrast reference
//! palette must clear AAA (7:1) text. No large-text role ships: every text
//! role clears 4.5:1, which subsumes the 3:1 large-text floor.
//!
//! Headless: pure token maths, no display (K.13). CI runs these through the
//! required `contrast-test` check (`cargo test -p lintel --bin lintel
//! contrast_test`).

use crate::theme::{
    ACCENT_DEFAULT, ACCENT_HOVER, BORDER_STRONG, BUTTON_DANGER_M, BUTTON_GHOST_M,
    BUTTON_PRIMARY_M, BUTTON_SECONDARY_M, ContrastFailure, SURFACE_BASE, SURFACE_PANEL,
    SURFACE_STATUS, TEXT_DANGER, TEXT_PRIMARY, TEXT_SECONDARY, TEXT_WARNING, Theme,
    contrast::composited_over, contrast_ratio, contrast_test, palette, rgb_of,
};

/// The merge gate itself: all three palettes must pass. Failure output lists
/// role, measured ratio and required ratio per failure (ticket T-007).
#[test]
fn contrast_test_all_palettes_pass() {
    if let Err(failures) = contrast_test() {
        let listed: String = failures.iter().map(|f| format!("\n  - {f}")).collect();
        panic!("{} contrast failure(s):{listed}", failures.len());
    }
}

/// Body text on the shipped light palette: `TEXT_PRIMARY` and
/// `TEXT_SECONDARY` clear 4.5:1 on every surface they are drawn on.
#[test]
fn contrast_test_light_body_text_meets_aa() {
    for (role, fg) in [("TEXT_PRIMARY", TEXT_PRIMARY), ("TEXT_SECONDARY", TEXT_SECONDARY)] {
        for (surface, bg) in [
            ("SURFACE_BASE", SURFACE_BASE),
            ("SURFACE_PANEL", SURFACE_PANEL),
            ("SURFACE_STATUS", SURFACE_STATUS),
        ] {
            let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
            assert!(r >= 4.5, "{role} on {surface} is {r:.2}:1 (needs 4.5:1)");
        }
    }
}

/// Dark reference palette body text: the same 4.5:1 AA floor.
#[test]
fn contrast_test_dark_body_text_meets_aa() {
    let p = palette(Theme::Dark);
    for (role, fg) in [("text.primary", p.text_primary), ("text.secondary", p.text_secondary)] {
        for (surface, bg) in [
            ("surface.base", p.surface_base),
            ("surface.panel", p.surface_panel),
            ("surface.status", p.surface_status),
        ] {
            let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
            assert!(r >= 4.5, "dark {role} on {surface} is {r:.2}:1 (needs 4.5:1)");
        }
    }
}

/// High-contrast reference palette: every text role clears the 7:1 AAA floor
/// on every surface (ticket T-007 threshold, guide II.30.7.1).
#[test]
fn contrast_test_high_contrast_text_meets_aaa() {
    let p = palette(Theme::HighContrast);
    let roles = [
        ("text.primary", p.text_primary),
        ("text.secondary", p.text_secondary),
        ("text.muted", p.text_muted),
        ("text.hint", p.text_hint),
        ("text.disabled", p.text_disabled),
        ("text.strong", p.text_strong),
        ("text.success", p.text_success),
        ("text.danger", p.text_danger),
        ("text.warning", p.text_warning),
    ];
    for (role, fg) in roles {
        for (surface, bg) in [
            ("surface.base", p.surface_base),
            ("surface.panel", p.surface_panel),
            ("surface.status", p.surface_status),
        ] {
            let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
            assert!(
                r >= 7.0,
                "high-contrast {role} on {surface} is {r:.2}:1 (needs 7:1)"
            );
        }
    }
}

/// Accent, focus-ring and border tokens against the neutral surfaces they
/// are drawn over: the 3:1 UI floor (WCAG 2.2 §1.4.11) on all three
/// palettes. `BORDER_DEFAULT` is a decorative divider, which §1.4.11 exempts.
#[test]
fn contrast_test_accent_and_border_tokens_meet_ui_floor() {
    for theme in Theme::ALL {
        let p = palette(theme);
        for (role, fg) in [
            ("accent", p.accent),
            ("accent.hover", p.accent_hover),
            ("focus.ring", p.focus_ring),
            ("border.strong", p.border_strong),
        ] {
            for (surface, bg) in
                [("surface.base", p.surface_base), ("surface.panel", p.surface_panel)]
            {
                let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
                assert!(
                    r >= 3.0,
                    "{} {role} on {surface} is {r:.2}:1 (needs 3:1)",
                    theme.name()
                );
            }
        }
    }
    // The shipped light tokens, by name.
    for (role, fg) in [
        ("ACCENT_DEFAULT", ACCENT_DEFAULT),
        ("ACCENT_HOVER", ACCENT_HOVER),
        ("BORDER_STRONG", BORDER_STRONG),
    ] {
        let r = contrast_ratio(rgb_of(fg), rgb_of(SURFACE_BASE));
        assert!(r >= 3.0, "{role} on SURFACE_BASE is {r:.2}:1 (needs 3:1)");
    }
}

/// Button colour states: the label clears its palette's text floor on the
/// rest, hover and pressed fill of every role, on all three palettes.
/// Disabled labels clear 4.5:1 in every theme — WCAG 2.2 §1.4.3 exempts
/// disabled controls, and the palettes hold the floor anyway.
#[test]
fn contrast_test_button_states_all_roles_all_themes() {
    for theme in Theme::ALL {
        let floor = theme.text_floor();
        for (role, b) in palette(theme).buttons {
            for (state, fg, bg, state_floor) in [
                ("fill", b.text, b.fill, floor),
                ("hover", b.text, b.hover, floor),
                ("active", b.text_active, b.active, floor),
                ("disabled", b.text_disabled, b.disabled, 4.5),
            ] {
                if bg.a() == 0 {
                    // Ghost: no fill of its own; the label reads against the
                    // light surfaces the button is drawn over.
                    for (surface, wash) in
                        [("surface.base", SURFACE_BASE), ("surface.panel", SURFACE_PANEL)]
                    {
                        let r = contrast_ratio(rgb_of(fg), rgb_of(wash));
                        assert!(
                            r >= state_floor,
                            "{} button.{role} label on {state} over {surface} is {r:.2}:1 \
                             (needs {state_floor}:1)",
                            theme.name()
                        );
                    }
                } else {
                    let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
                    assert!(
                        r >= state_floor,
                        "{} button.{role} label on {state} is {r:.2}:1 (needs {state_floor}:1)",
                        theme.name()
                    );
                }
            }
        }
    }
}

/// Input fields: value text and placeholder clear the palette's text floor
/// on the field fill, and the border clears the 3:1 component-boundary
/// floor, on all three palettes. This pins the reference pairing the T-002
/// `Field` components must implement (D-035).
#[test]
fn contrast_test_input_fields_all_themes() {
    for theme in Theme::ALL {
        let p = palette(theme);
        let floor = theme.text_floor();
        let text = contrast_ratio(rgb_of(p.input_text), rgb_of(p.input_fill));
        assert!(
            text >= floor,
            "{} input text on fill is {text:.2}:1 (needs {floor}:1)",
            theme.name()
        );
        let ph = contrast_ratio(rgb_of(p.input_placeholder), rgb_of(p.input_fill));
        assert!(
            ph >= floor,
            "{} input placeholder on fill is {ph:.2}:1 (needs {floor}:1)",
            theme.name()
        );
        let border = contrast_ratio(rgb_of(p.input_border), rgb_of(p.input_fill));
        assert!(
            border >= 3.0,
            "{} input border on fill is {border:.2}:1 (needs 3:1)",
            theme.name()
        );
    }
}

/// Warning and error message text clears its palette's floor on every
/// surface messages are drawn on.
#[test]
fn contrast_test_warning_and_error_text_all_themes() {
    for theme in Theme::ALL {
        let p = palette(theme);
        let floor = theme.text_floor();
        for (role, fg) in [("text.warning", p.text_warning), ("text.danger", p.text_danger)] {
            for (surface, bg) in [
                ("surface.base", p.surface_base),
                ("surface.panel", p.surface_panel),
                ("surface.status", p.surface_status),
            ] {
                let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
                assert!(
                    r >= floor,
                    "{} {role} on {surface} is {r:.2}:1 (needs {floor}:1)",
                    theme.name()
                );
            }
        }
    }
    // The shipped light tokens, by name.
    for (role, fg) in [("TEXT_WARNING", TEXT_WARNING), ("TEXT_DANGER", TEXT_DANGER)] {
        for (surface, bg) in [
            ("SURFACE_BASE", SURFACE_BASE),
            ("SURFACE_PANEL", SURFACE_PANEL),
            ("SURFACE_STATUS", SURFACE_STATUS),
        ] {
            let r = contrast_ratio(rgb_of(fg), rgb_of(bg));
            assert!(r >= 4.5, "{role} on {surface} is {r:.2}:1 (needs 4.5:1)");
        }
    }
}

/// Selection indicators: the composited edge clears the 3:1 state-indicator
/// floor on both surfaces, the wash is not a no-op, and the selected row's
/// label reads over the composited wash. (D-035 records the interpretation:
/// the edge is the §1.4.11 marker; the 18-alpha wash alone is a tint, so it
/// is checked as non-no-op plus label readability.)
#[test]
fn contrast_test_selection_indicators_all_themes() {
    for theme in Theme::ALL {
        let p = palette(theme);
        let edge_base = composited_over(p.surface_base, p.border_selection);
        let r_base = contrast_ratio(rgb_of(edge_base), rgb_of(p.surface_base));
        assert!(
            r_base >= 3.0,
            "{} selection edge on surface.base is {r_base:.2}:1 (needs 3:1)",
            theme.name()
        );
        let edge_panel = composited_over(p.surface_panel, p.border_selection);
        let r_panel = contrast_ratio(rgb_of(edge_panel), rgb_of(p.surface_panel));
        assert!(
            r_panel >= 3.0,
            "{} selection edge on surface.panel is {r_panel:.2}:1 (needs 3:1)",
            theme.name()
        );
        let fill = composited_over(p.surface_base, p.selection_fill);
        assert_ne!(
            fill, p.surface_base,
            "{} selection wash must change the row it covers",
            theme.name()
        );
        let label = contrast_ratio(rgb_of(p.text_strong), rgb_of(fill));
        assert!(
            label >= theme.text_floor(),
            "{} selected-row label on the wash is {label:.2}:1 (needs {})",
            theme.name(),
            theme.text_floor()
        );
    }
}

/// The light palette's button states are the shipped `BUTTON_*_M` component
/// tokens — the palette table must never drift from the tokens (one token
/// source, guide II.30.6).
#[test]
fn contrast_test_light_button_states_match_component_tokens() {
    let buttons = palette(Theme::Light).buttons;
    let shipped = [
        ("primary", BUTTON_PRIMARY_M),
        ("secondary", BUTTON_SECONDARY_M),
        ("ghost", BUTTON_GHOST_M),
        ("danger", BUTTON_DANGER_M),
    ];
    for (i, (name, style)) in shipped.into_iter().enumerate() {
        assert_eq!(buttons[i].0, name, "palette button order changed");
        assert_eq!(
            buttons[i].1.fill, style.fill,
            "{name} fill drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.text, style.text,
            "{name} text drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.hover, style.hover,
            "{name} hover drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.active, style.active,
            "{name} active drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.text_active, style.text_active,
            "{name} active label drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.disabled, style.disabled,
            "{name} disabled fill drifted from the component token"
        );
        assert_eq!(
            buttons[i].1.text_disabled, style.text_disabled,
            "{name} disabled label drifted from the component token"
        );
    }
}

/// Failure reporting: `Display` names the role, the measured ratio and the
/// required ratio, so a red CI log is actionable (ticket T-007).
#[test]
fn contrast_test_failure_display_names_role_ratio_required() {
    let f = ContrastFailure::Text {
        role: "dark: text.muted on surface.panel".to_string(),
        ratio: 4.2,
        required: 4.5,
    };
    let shown = f.to_string();
    assert!(
        shown.contains("dark: text.muted on surface.panel"),
        "message names the role: {shown}"
    );
    assert!(shown.contains("4.20"), "message names the measured ratio: {shown}");
    assert!(shown.contains("4.5"), "message names the required ratio: {shown}");

    let u = ContrastFailure::Ui {
        role: "light: border.strong on surface.base".to_string(),
        ratio: 2.8,
        required: 3.0,
    };
    let shown = u.to_string();
    assert!(shown.contains("ui contrast failure"), "message states the kind: {shown}");
    assert!(shown.contains("2.80:1"), "message states the measured ratio: {shown}");
}
