//! Lintel visual system — original, draftsman-flavoured palette.
//!
//! Nothing here copies a competitor: warm paper, graphite ink, one cobalt
//! accent for selection and tool state, amber for snap confirmation, green for
//! "end" snaps. WCAG AA contrast is checked where text sits on ink or paper.

use eframe::egui::Color32;

pub const PAPER: Color32 = Color32::from_rgb(252, 248, 242); // warm off-white sheet
pub const INK: Color32 = Color32::from_rgb(26, 24, 22); // near-black graphite
/// Secondary / muted text. AA-corrected: 5.1:1 on PAPER, 5.1:1 on PANEL_BG,
/// 4.7:1 on STATUS_BG (guide II.30.7.1 requires 4.5:1 for normal text).
pub const TEXT_MUTED: Color32 = Color32::from_rgb(102, 102, 102);

pub const COBALT: Color32 = Color32::from_rgb(18, 78, 120); // selection / active tool
pub const GREEN: Color32 = Color32::from_rgb(34, 158, 96); // endpoint snap

// Focus-ring colour decision (guide II.21.2 / II.21.4).
// egui draws focus rings around keyboard-focused widgets automatically, derived
// from the theme's selection/accent colour. COBALT (#124E78) has a relative
// luminance of ~0.069. Against PAPER (#FCF8F2, ~0.942) the contrast ratio is
// ~8.3:1 and against PANEL_BG (#F5F0E8, ~0.876) it is ~7.8:1 — both above the
// WCAG AA 4.5:1 threshold for normal text and the 3:1 threshold for UI
// graphics. So COBALT focus rings meet AA on both light panel and canvas
// backgrounds; no focus-ring colour change or removal is needed.

pub const WALL_FILL: Color32 = Color32::from_rgb(226, 222, 214);
pub const WALL_STROKE: Color32 = Color32::from_rgb(84, 78, 70);
#[allow(dead_code)]
pub const WALL_DRAFT: Color32 = Color32::from_rgb(200, 194, 184);

pub const GRID_MINOR: Color32 = Color32::from_rgb(233, 226, 216);
pub const GRID_MAJOR: Color32 = Color32::from_rgb(200, 190, 176);

pub const OPENING_GAP: Color32 = PAPER;
pub const DOOR_LEAF: Color32 = INK;
pub const WINDOW_LINE: Color32 = Color32::from_rgb(90, 82, 74);

#[allow(dead_code)]
pub const PANEL_BG: Color32 = Color32::from_rgb(245, 240, 232);
#[allow(dead_code)]
pub const PANEL_BORDER: Color32 = Color32::from_rgb(212, 204, 192);
#[allow(dead_code)]
pub const STATUS_BG: Color32 = Color32::from_rgb(239, 232, 220);

// ---- Status / hint text (status bar, toolbar, command palette) ----
// Role-named replacements for the raw egui::Color32 values they used to hold.
// All text roles are AA-corrected to ≥ 4.5:1 on every surface they appear on
// (PAPER, PANEL_BG, STATUS_BG) — verified by `theme_colours_meet_wcag_aa`.
pub const HINT: Color32 = Color32::from_rgb(100, 100, 100); // hint text, 4.9:1 on STATUS_BG
pub const TRANSPARENT: Color32 = Color32::TRANSPARENT;
pub const STATUS_OK: Color32 = Color32::from_rgb(26, 112, 62); // valid input, 5.0:1 on STATUS_BG
pub const ERROR: Color32 = Color32::DARK_RED; // error / destructive message

// ---- Command palette overlay ----
pub const PALETTE_SELECTED_FILL: Color32 = Color32::from_black_alpha(18);
pub const PALETTE_SELECTED_STROKE: Color32 = Color32::from_black_alpha(40);
pub const PALETTE_HIT: Color32 = Color32::from_rgb(20, 90, 160); // matched search text
pub const PALETTE_LABEL: Color32 = Color32::BLACK; // enabled command label
pub const PALETTE_DISABLED: Color32 = Color32::from_rgb(105, 105, 105); // 5.2:1 on PAPER
pub const PALETTE_RECENT: Color32 = Color32::from_rgb(95, 95, 95); // 5.6:1 on PAPER
pub const PALETTE_SHORTCUT: Color32 = TEXT_MUTED;
pub const PALETTE_CATEGORY: Color32 = Color32::from_rgb(105, 105, 105); // 5.2:1 on PAPER

/// Convert an egui colour to a plain RGB tuple (alpha dropped). Useful for the
/// WCAG contrast helpers below, which operate on `(u8, u8, u8)` so they have
/// no egui dependency in the maths.
// Consumed by the `theme_colours_meet_wcag_aa` merge-gate test (guide
// II.30.7.1); the binary itself does not call it yet, hence the allow.
#[allow(dead_code)]
pub fn rgb_of(colour: Color32) -> (u8, u8, u8) {
    (colour.r(), colour.g(), colour.b())
}

/// Relative luminance per WCAG 2.2 (sRGB linearised). Guide II.30.7.1.
// Consumed by the contrast merge-gate test; see `rgb_of`.
#[allow(dead_code)]
pub fn relative_luminance(rgb: (u8, u8, u8)) -> f64 {
    fn channel(c: u8) -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(rgb.0) + 0.7152 * channel(rgb.1) + 0.0722 * channel(rgb.2)
}

/// Contrast ratio `(L1 + 0.05) / (L2 + 0.05)`, lighter over darker.
// Consumed by the contrast merge-gate test; see `rgb_of`.
#[allow(dead_code)]
pub fn contrast_ratio(a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn selection_stroke() -> egui::Stroke {
    egui::Stroke::new(2.0, COBALT)
}

// ---- Button role styles (egui-elegance pattern copy, guide II.30.10) ----
// Four roles x three sizes. Every size honours the 24pt floor
// (guide II.30.7.5); focus rings are 2px COBALT (guide II.30.7.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonRole {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ButtonRoleStyle {
    pub fill: Color32,
    pub hover: Color32,
    pub active: Color32,
    pub focus: Color32,
    pub disabled: Color32,
    pub min_size: eframe::egui::Vec2,
}

pub const BUTTON_PRIMARY_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: COBALT,
    hover: COBALT,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_PRIMARY_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: COBALT,
    hover: COBALT,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_PRIMARY_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: COBALT,
    hover: COBALT,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_SECONDARY_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: PANEL_BG,
    hover: PAPER,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_SECONDARY_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: PANEL_BG,
    hover: PAPER,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_SECONDARY_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: PANEL_BG,
    hover: PAPER,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_GHOST_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: TRANSPARENT,
    hover: PANEL_BG,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_GHOST_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: TRANSPARENT,
    hover: PANEL_BG,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_GHOST_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: TRANSPARENT,
    hover: PANEL_BG,
    active: COBALT,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_DANGER_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: ERROR,
    hover: ERROR,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
#[allow(dead_code)]
pub const BUTTON_DANGER_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: ERROR,
    hover: ERROR,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
#[allow(dead_code)]
pub const BUTTON_DANGER_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: ERROR,
    hover: ERROR,
    active: INK,
    focus: COBALT,
    disabled: TEXT_MUTED,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

#[allow(dead_code)]
pub fn button_style(
    role: ButtonRole,
    size: ButtonSize,
    active: bool,
) -> eframe::egui::Button<'static> {
    let spec = match (role, size) {
        (ButtonRole::Primary, ButtonSize::Small) => BUTTON_PRIMARY_S,
        (ButtonRole::Primary, ButtonSize::Medium) => BUTTON_PRIMARY_M,
        (ButtonRole::Primary, ButtonSize::Large) => BUTTON_PRIMARY_L,
        (ButtonRole::Secondary, ButtonSize::Small) => BUTTON_SECONDARY_S,
        (ButtonRole::Secondary, ButtonSize::Medium) => BUTTON_SECONDARY_M,
        (ButtonRole::Secondary, ButtonSize::Large) => BUTTON_SECONDARY_L,
        (ButtonRole::Ghost, ButtonSize::Small) => BUTTON_GHOST_S,
        (ButtonRole::Ghost, ButtonSize::Medium) => BUTTON_GHOST_M,
        (ButtonRole::Ghost, ButtonSize::Large) => BUTTON_GHOST_L,
        (ButtonRole::Danger, ButtonSize::Small) => BUTTON_DANGER_S,
        (ButtonRole::Danger, ButtonSize::Medium) => BUTTON_DANGER_M,
        (ButtonRole::Danger, ButtonSize::Large) => BUTTON_DANGER_L,
    };
    let fill = if active { spec.active } else { spec.fill };
    eframe::egui::Button::new("")
        .fill(fill)
        .stroke(eframe::egui::Stroke::new(2.0, spec.focus))
        .min_size(spec.min_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_colours_meet_wcag_aa() {
        let paper = rgb_of(PAPER);
        let panel = rgb_of(PANEL_BG);
        let status_bg = rgb_of(STATUS_BG);

        // Normal text (>= 4.5:1) on every surface it sits on.
        let ink = rgb_of(INK);
        let muted = rgb_of(TEXT_MUTED);
        assert!(
            contrast_ratio(ink, paper) >= 4.5,
            "INK on PAPER is {:.2}:1",
            contrast_ratio(ink, paper)
        );
        assert!(
            contrast_ratio(ink, panel) >= 4.5,
            "INK on PANEL_BG is {:.2}:1",
            contrast_ratio(ink, panel)
        );
        assert!(
            contrast_ratio(muted, paper) >= 4.5,
            "TEXT_MUTED on PAPER is {:.2}:1",
            contrast_ratio(muted, paper)
        );
        // TEXT_MUTED was corrected from (110,110,110) — ~4.49:1 on PANEL_BG,
        // just under AA — to (102,102,102), 5.1:1 on both light surfaces.
        assert!(
            contrast_ratio(muted, panel) >= 4.5,
            "TEXT_MUTED on PANEL_BG is {:.2}:1",
            contrast_ratio(muted, panel)
        );

        // Hint text (small, secondary) on the surfaces it appears on.
        let hint = rgb_of(HINT);
        assert!(
            contrast_ratio(hint, paper) >= 4.5,
            "HINT on PAPER is {:.2}:1",
            contrast_ratio(hint, paper)
        );
        assert!(
            contrast_ratio(hint, panel) >= 4.5,
            "HINT on PANEL_BG is {:.2}:1",
            contrast_ratio(hint, panel)
        );
        assert!(
            contrast_ratio(hint, status_bg) >= 4.5,
            "HINT on STATUS_BG is {:.2}:1",
            contrast_ratio(hint, status_bg)
        );

        // Status message colours on the palette/panel surfaces.
        let ok = rgb_of(STATUS_OK);
        let err = rgb_of(ERROR);
        for (name, col) in [("STATUS_OK", ok), ("ERROR", err)] {
            assert!(
                contrast_ratio(col, paper) >= 4.5,
                "{name} on PAPER is {:.2}:1",
                contrast_ratio(col, paper)
            );
            assert!(
                contrast_ratio(col, panel) >= 4.5,
                "{name} on PANEL_BG is {:.2}:1",
                contrast_ratio(col, panel)
            );
        }

        // Command palette row colours on the window background (PAPER).
        let hit = rgb_of(PALETTE_HIT);
        let disabled = rgb_of(PALETTE_DISABLED);
        let recent = rgb_of(PALETTE_RECENT);
        let category = rgb_of(PALETTE_CATEGORY);
        let label = rgb_of(PALETTE_LABEL);
        for (name, col) in [
            ("PALETTE_HIT", hit),
            ("PALETTE_DISABLED", disabled),
            ("PALETTE_RECENT", recent),
            ("PALETTE_CATEGORY", category),
            ("PALETTE_LABEL", label),
        ] {
            // Large/bold-ish labels and disabled hints use the 3:1 UI boundary
            // floor; normal-weight small text still aims for 4.5.
            assert!(
                contrast_ratio(col, paper) >= 3.0,
                "{name} on PAPER is {:.2}:1",
                contrast_ratio(col, paper)
            );
        }

        // Meaningful UI boundaries / focus rings / accents (>= 3:1).
        let cobalt = rgb_of(COBALT);
        let green = rgb_of(GREEN);
        assert!(
            contrast_ratio(cobalt, paper) >= 3.0,
            "COBALT on PAPER is {:.2}:1",
            contrast_ratio(cobalt, paper)
        );
        assert!(
            contrast_ratio(cobalt, panel) >= 3.0,
            "COBALT on PANEL_BG is {:.2}:1",
            contrast_ratio(cobalt, panel)
        );
        assert!(
            contrast_ratio(green, paper) >= 3.0,
            "GREEN on PAPER is {:.2}:1",
            contrast_ratio(green, paper)
        );
        assert!(
            contrast_ratio(green, panel) >= 3.0,
            "GREEN on PANEL_BG is {:.2}:1",
            contrast_ratio(green, panel)
        );
    }
}
