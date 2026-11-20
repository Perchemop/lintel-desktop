//! Lintel visual system — original, draftsman-flavoured palette.
//!
//! Nothing here copies a competitor: warm paper, graphite ink, one cobalt
//! accent for selection and tool state, amber for snap confirmation, green for
//! "end" snaps. WCAG AA contrast is checked where text sits on ink or paper.
//!
//! # Three tiers (egui-pop `ThemeSpec` pattern, copied not adopted)
//!
//! Colours resolve through three layers, and each layer points only at the
//! layer beneath it — one token source, one design path (guide II.30.4,
//! II.30.6):
//!
//! 1. **Base palette** (`BASE_*`) — raw swatches. This is the only place in the
//!    crate where a literal colour is constructed.
//! 2. **Semantic roles** (`SURFACE_*`, `TEXT_*`, `ACCENT_*`, `DRAFT_*`, …) — what
//!    a colour *means*, never what it looks like.
//! 3. **Component tokens** (`WALL_FILL`, `PALETTE_*`, `BUTTON_*`) — where a role
//!    is used. Widget code reads this layer only.
//!
//! [`ThemeSpec`] groups the three tiers into one value and [`SPEC`] is the
//! single handle widget code reads (`SPEC.component.<group>.<token>`): the
//! flat constants are the named registry, tier 1 and 2 are never widget input.
//!
//! Re-hueing the product is a tier-1 edit; re-styling one component is a tier-3
//! edit; nothing in between needs to move.

use eframe::egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions};

// === TIER 1: BASE PALETTE ==================================================
// Raw swatches, no meaning attached — the only `Color32` constructors here.
// ===========================================================================

pub const BASE_PAPER: Color32 = Color32::from_rgb(252, 248, 242); // warm off-white sheet
pub const BASE_INK: Color32 = Color32::from_rgb(26, 24, 22); // near-black graphite
pub const BASE_GRAPHITE_MUTED: Color32 = Color32::from_rgb(102, 102, 102);
pub const BASE_GRAPHITE_HINT: Color32 = Color32::from_rgb(100, 100, 100);
pub const BASE_GRAPHITE_DISABLED: Color32 = Color32::from_rgb(105, 105, 105);
pub const BASE_GRAPHITE_SECONDARY: Color32 = Color32::from_rgb(95, 95, 95);
pub const BASE_BLACK: Color32 = Color32::BLACK;
pub const BASE_MATCH_BLUE: Color32 = Color32::from_rgb(20, 90, 160);

pub const BASE_COBALT: Color32 = Color32::from_rgb(18, 78, 120); // selection / active tool
/// Pointer hover on accent fills: one step lighter than BASE_COBALT so a
/// filled button visibly responds to hover without relying on its border
/// (guide II.30.10). Text contrast: 6.6:1 with paper labels.
pub const BASE_COBALT_HOVER: Color32 = Color32::from_rgb(24, 94, 140);
pub const BASE_GREEN: Color32 = Color32::from_rgb(34, 158, 96); // endpoint snap
pub const BASE_VALID: Color32 = Color32::from_rgb(26, 112, 62); // valid-input message
pub const BASE_ERROR: Color32 = Color32::DARK_RED;
/// Pointer hover on destructive fills: one step darker than BASE_ERROR
/// (guide II.30.10). Text contrast: 11.0:1 with paper labels.
pub const BASE_DANGER_HOVER: Color32 = Color32::from_rgb(120, 0, 0);
/// Base danger red for error states and destructive actions.
pub const BASE_DANGER: Color32 = Color32::from_rgb(180, 0, 0);
/// Tier-1 swatch for warning text. Darkened from the snap amber
/// (176,106,22) — 4.0:1 on PAPER, under the 4.5:1 text floor — so a warning
/// reads as text, not only as a snap marker (guide II.30.7.1).
#[allow(dead_code)] // consumed by TEXT_WARNING and the contrast merge-gate tests
pub const BASE_WARNING: Color32 = Color32::from_rgb(140, 84, 14);

pub const BASE_PANEL_BG: Color32 = Color32::from_rgb(245, 240, 232);
pub const BASE_PANEL_BORDER: Color32 = Color32::from_rgb(212, 204, 192);
pub const BASE_STATUS_BG: Color32 = Color32::from_rgb(239, 232, 220);

pub const BASE_WALL_FILL: Color32 = Color32::from_rgb(226, 222, 214);
pub const BASE_WALL_STROKE: Color32 = Color32::from_rgb(84, 78, 70);
pub const BASE_WALL_PENDING: Color32 = Color32::from_rgb(200, 194, 184);
pub const BASE_GRID_MINOR: Color32 = Color32::from_rgb(233, 226, 216);
pub const BASE_GRID_MAJOR: Color32 = Color32::from_rgb(200, 190, 176);
pub const BASE_WINDOW_LINE: Color32 = Color32::from_rgb(90, 82, 74);

pub const BASE_SELECTION_WASH: Color32 = Color32::from_black_alpha(18);
/// Selection-state edge (command-palette rows). Raised from alpha 40 — a
/// 1.2:1 whisper that failed the WCAG 2.2 §1.4.11 state-indicator floor — to
/// alpha 140: 4.7:1 composited on PAPER and 4.6:1 on the panel
/// (guide II.30.7.1, ticket T-007, D-035).
pub const BASE_SELECTION_EDGE: Color32 = Color32::from_black_alpha(140);
pub const BASE_TRANSPARENT: Color32 = Color32::TRANSPARENT;

pub const BASE_SNAP_MIDPOINT: Color32 = Color32::from_rgb(176, 106, 22);
pub const BASE_SNAP_PERPENDICULAR: Color32 = Color32::from_rgb(120, 74, 160);
pub const BASE_SNAP_TANGENT: Color32 = Color32::from_rgb(24, 128, 140);
pub const BASE_SNAP_ORTHO: Color32 = Color32::from_rgb(90, 82, 74);
pub const BASE_SNAP_GRID: Color32 = Color32::from_rgb(130, 130, 130);
/// Construction-geometry stroke. Darkened from (168,148,116) — 2.77:1 on
/// PAPER, below the 3:1 UI-graphics floor — to 4.0:1 (guide II.30.7.1).
pub const BASE_CONSTRUCTION: Color32 = Color32::from_rgb(140, 120, 90);
/// Outline drawn around the selected block instance (main shell). Pinned at
/// its pre-token value: 4.2:1 on PAPER, above the 3:1 UI-graphics floor.
pub const BASE_BLOCK_HIGHLIGHT: Color32 = Color32::from_rgb(220, 60, 60);
// === TIER 2: SEMANTIC ROLES ================================================
// What a colour means, never what it looks like: every role is one hop from
// a `BASE_*` swatch (or a sibling role), never a new value.
// ===========================================================================

// Surfaces.
/// Canvas sheet and app background.
pub const SURFACE_BASE: Color32 = BASE_PAPER;
/// Inspector / toolbar panel fill.
pub const SURFACE_PANEL: Color32 = BASE_PANEL_BG;
/// Status-bar strip fill.
pub const SURFACE_STATUS: Color32 = BASE_STATUS_BG;
/// "Paint nothing" — the fill of a widget that has no background.
pub const SURFACE_TRANSPARENT: Color32 = BASE_TRANSPARENT;
/// Selected-row wash in the command palette.
pub const SURFACE_SELECTION: Color32 = BASE_SELECTION_WASH;

// Borders.
/// Panel divider.
pub const BORDER_DEFAULT: Color32 = BASE_PANEL_BORDER;
/// Outline of a selected row.
pub const BORDER_SELECTION: Color32 = BASE_SELECTION_EDGE;
/// Strong outline: input-field borders and any boundary that must identify a
/// component on its own (WCAG 2.2 §1.4.11): 7.8:1 on the sheet, 7.2:1 on the
/// panel and 6.8:1 on the status strip. `BORDER_DEFAULT` stays the decorative
/// divider, which §1.4.11 exempts. Consumed by the `FIELD_*` registry and the
/// contrast merge-gate tests.
pub const BORDER_STRONG: Color32 = BASE_WALL_STROKE;

// Text.
/// Body text on every light surface.
pub const TEXT_PRIMARY: Color32 = BASE_INK;
/// Secondary / muted text. AA-corrected: 5.1:1 on the sheet, 5.1:1 on the panel,
/// 4.7:1 on the status strip (guide II.30.7.1 requires 4.5:1 for normal text).
pub const TEXT_MUTED: Color32 = BASE_GRAPHITE_MUTED;
/// Small hint text: 4.9:1 on the status strip.
pub const TEXT_HINT: Color32 = BASE_GRAPHITE_HINT;
/// Strongest text (command-palette row label).
pub const TEXT_STRONG: Color32 = BASE_BLACK;
/// Disabled text: 5.2:1 on the sheet.
pub const TEXT_DISABLED: Color32 = BASE_GRAPHITE_DISABLED;
/// Recent-item text: 5.6:1 on the sheet.
pub const TEXT_SECONDARY: Color32 = BASE_GRAPHITE_SECONDARY;
/// Matched search text in the command palette.
pub const TEXT_MATCH: Color32 = BASE_MATCH_BLUE;
/// Valid-input message: 5.0:1 on the status strip.
pub const TEXT_SUCCESS: Color32 = BASE_VALID;
/// Error / destructive message.
pub const TEXT_ERROR: Color32 = BASE_ERROR;
/// Warning message: 5.9:1 on the sheet, 5.5:1 on the panel, 5.1:1 on the
/// status strip (guide II.30.7.1).
pub const TEXT_WARNING: Color32 = BASE_WARNING;
/// Destructive/danger message text: 6.5:1 on the sheet, 6.0:1 on the panel.
pub const TEXT_DANGER: Color32 = BASE_DANGER;

// Accent: selection, active tool, focus.
pub const ACCENT_DEFAULT: Color32 = BASE_COBALT;
/// Hover on accent fills (buttons): a step lighter than the rest fill, so
/// hover is a visible state change in its own right (guide II.30.10). Canvas
/// tool hover keeps its own stroke-weight signal and does not read this role.
pub const ACCENT_HOVER: Color32 = BASE_COBALT_HOVER;
/// Pressed states darken to graphite.
pub const ACCENT_ACTIVE: Color32 = BASE_INK;
/// Keyboard focus ring, 2 px (guide II.21.2, II.30.7.4).
pub const FOCUS_RING: Color32 = ACCENT_DEFAULT;
/// Confirmation green: endpoint snap, crossing selection.
pub const SUCCESS_DEFAULT: Color32 = BASE_GREEN;
/// Destructive fill.
pub const DANGER_DEFAULT: Color32 = BASE_ERROR;
/// Hover on destructive fills (buttons): a step darker than DANGER_DEFAULT.
pub const DANGER_HOVER: Color32 = BASE_DANGER_HOVER;
/// Light label text on dark fills. Reuses the paper swatch: 8.3:1 on
/// ACCENT_DEFAULT, 6.6:1 on ACCENT_HOVER, 16.7:1 on ACCENT_ACTIVE, 9.5:1 on
/// DANGER_DEFAULT, 11.0:1 on DANGER_HOVER and 5.4:1 on TEXT_MUTED — every
/// pairing clears the 4.5:1 AA floor for normal text (guide II.30.7.1).
pub const TEXT_ON_ACCENT: Color32 = BASE_PAPER;

// Structural strokes.
/// Dark stroke for dimension lines, drag rectangles and the free-snap marker.
pub const STROKE_STRONG: Color32 = BASE_INK;
/// Highlight around the selected block instance (shell-level selection).
pub const BLOCK_HIGHLIGHT: Color32 = BASE_BLOCK_HIGHLIGHT;

// Snap markers: one colour per kind so the label is never the only signal.
pub const SNAP_MIDPOINT: Color32 = BASE_SNAP_MIDPOINT;
pub const SNAP_PERPENDICULAR: Color32 = BASE_SNAP_PERPENDICULAR;
pub const SNAP_TANGENT: Color32 = BASE_SNAP_TANGENT;
pub const SNAP_ORTHO: Color32 = BASE_SNAP_ORTHO;
pub const SNAP_GRID: Color32 = BASE_SNAP_GRID;
/// Warm-grey construction guide stroke.
pub const CONSTRUCTION_STROKE: Color32 = BASE_CONSTRUCTION;
// Drafting surface: the plan itself.
pub const DRAFT_FILL: Color32 = BASE_WALL_FILL;
pub const DRAFT_STROKE: Color32 = BASE_WALL_STROKE;
/// Geometry that is not committed to the model yet.
pub const DRAFT_PENDING: Color32 = BASE_WALL_PENDING;
pub const GRID_LINE_MINOR: Color32 = BASE_GRID_MINOR;
pub const GRID_LINE_MAJOR: Color32 = BASE_GRID_MAJOR;
/// The gap an opening cuts through its host wall.
pub const OPENING_CUT: Color32 = SURFACE_BASE;
/// Door leaf and swing arc.
pub const OPENING_LEAF: Color32 = TEXT_PRIMARY;
/// Window glazing line.
pub const OPENING_GLAZING: Color32 = BASE_WINDOW_LINE;

// Focus-ring colour decision (guide II.21.2 / II.21.4).
// egui draws focus rings around keyboard-focused widgets automatically, derived
// from the theme's selection/accent colour. `FOCUS_RING` is `ACCENT_DEFAULT`
// (#124E78), relative luminance ~0.069. Against `SURFACE_BASE` (#FCF8F2, ~0.942)
// the contrast ratio is ~8.3:1 and against `SURFACE_PANEL` (#F5F0E8, ~0.876) it
// is ~7.8:1 — both above the WCAG AA 4.5:1 threshold for normal text and the
// 3:1 threshold for UI graphics. No separate focus colour is needed.

// === TIER 3: COMPONENT TOKENS ==============================================
// Where a role is used: widget code reads this layer only, through `SPEC`.
// ===========================================================================

// Canvas geometry: walls, grid and openings.
pub const WALL_FILL: Color32 = DRAFT_FILL;
pub const WALL_STROKE: Color32 = DRAFT_STROKE;
/// Geometry that is not committed to the model yet (wall-tool preview).
pub const WALL_PENDING: Color32 = DRAFT_PENDING;
pub const GRID_MINOR: Color32 = GRID_LINE_MINOR;
pub const GRID_MAJOR: Color32 = GRID_LINE_MAJOR;
pub const OPENING_GAP: Color32 = OPENING_CUT;
pub const DOOR_LEAF: Color32 = OPENING_LEAF;
pub const WINDOW_LINE: Color32 = OPENING_GLAZING;
/// The sheet the canvas paints before anything is drawn on it.
pub const CANVAS_SHEET: Color32 = SURFACE_BASE;
/// Entity labels, the empty-canvas hint and the draft readout.
pub const CANVAS_TEXT: Color32 = TEXT_PRIMARY;
/// Construction-guide stroke on the canvas (warm grey).
pub const CANVAS_CONSTRUCTION: Color32 = CONSTRUCTION_STROKE;
/// A zero-width stroke ("paint nothing").
pub const STROKE_NONE: Color32 = SURFACE_TRANSPARENT;
/// The accent anything selected turns: labels, dimension lines, wall wash.
pub const SELECTION_HIGHLIGHT: Color32 = ACCENT_DEFAULT;
/// Rubber-band window selection (solid mode).
pub const SELECTION_WINDOW: Color32 = ACCENT_DEFAULT;
/// Rubber-band crossing selection (dashed mode).
pub const SELECTION_CROSSING: Color32 = SUCCESS_DEFAULT;
/// Committed dimension lines, witness lines and ticks.
pub const DIMENSION_STROKE: Color32 = STROKE_STRONG;
/// Outline drawn around the selected block instance.
pub const BLOCK_INSTANCE_STROKE: Color32 = BLOCK_HIGHLIGHT;

// Command palette: a command row and its parts.
pub const PALETTE_SELECTED_FILL: Color32 = SURFACE_SELECTION;
pub const PALETTE_SELECTED_STROKE: Color32 = BORDER_SELECTION;
pub const PALETTE_HIT: Color32 = TEXT_MATCH; // matched search text
pub const PALETTE_LABEL: Color32 = TEXT_STRONG; // enabled command label
pub const PALETTE_DISABLED: Color32 = TEXT_DISABLED; // 5.2:1 on the sheet
pub const PALETTE_RECENT: Color32 = TEXT_SECONDARY; // 5.6:1 on the sheet
pub const PALETTE_SHORTCUT: Color32 = TEXT_MUTED;
pub const PALETTE_CATEGORY: Color32 = TEXT_DISABLED; // 5.2:1 on the sheet
/// Unselected command rows paint nothing.
pub const PALETTE_UNSELECTED_FILL: Color32 = SURFACE_TRANSPARENT;

// Panels: shell surfaces (inspector window, status strip, dividers).
pub const PANEL_SURFACE: Color32 = SURFACE_PANEL;
pub const PANEL_BORDER: Color32 = BORDER_DEFAULT;
pub const STATUS_SURFACE: Color32 = SURFACE_STATUS;

// Widget text roles: tier-3 handles over the tier-2 text roles widgets use.
pub const BODY_TEXT: Color32 = TEXT_PRIMARY;
pub const HINT_TEXT: Color32 = TEXT_HINT;
pub const MUTED_TEXT: Color32 = TEXT_MUTED;
pub const SUCCESS_TEXT: Color32 = TEXT_SUCCESS;
pub const ERROR_TEXT: Color32 = TEXT_ERROR;
pub const DANGER_TEXT: Color32 = TEXT_DANGER;

// Form fields (shadcn-egui Field pattern copy, spec v0.20.0 AC-03): every
// colour is one hop from a tier-2 role; the widget reads them through
// `SPEC.component.field`. Consumed by `crate::components::forms`.
/// Field label text.
pub const FIELD_LABEL: Color32 = TEXT_PRIMARY;
/// FieldSet legend text.
pub const FIELD_LEGEND: Color32 = TEXT_STRONG;
/// Helper text under an input: 5.1:1 on the sheet and on the panel.
pub const FIELD_HELP: Color32 = TEXT_MUTED;
/// Error message text: 9.5:1 on the sheet, 8.8:1 on the panel (guide II.30.7.1).
pub const FIELD_ERROR_TEXT: Color32 = TEXT_ERROR;
/// Error icon fill — the same danger swatch as the error border. Colour is
/// never the only signal: the message text beside it and the light
/// exclamation mark (`TEXT_ON_ACCENT`, 9.5:1 on this fill) carry the state
/// too (guide II.30.7.2).
pub const FIELD_ERROR_ICON: Color32 = DANGER_DEFAULT;
/// Input background fill (a subtle well on the sheet).
pub const FIELD_INPUT_FILL: Color32 = SURFACE_BASE;
/// Input text colour.
pub const FIELD_INPUT_TEXT: Color32 = TEXT_PRIMARY;
/// Rest-state input border — `BORDER_STRONG`, so the input identifies itself
/// on its own (WCAG 2.2 §1.4.11): 7.8:1 on the sheet, 7.2:1 on the panel.
pub const FIELD_BORDER: Color32 = BORDER_STRONG;
/// Error-state border: 9.5:1 on the sheet, 8.8:1 on the panel (≥ 3:1 UI floor).
pub const FIELD_BORDER_ERROR: Color32 = DANGER_DEFAULT;
/// Focus-state border: 8.3:1 on the sheet, 7.8:1 on the panel (guide II.30.7.4).
pub const FIELD_BORDER_FOCUSED: Color32 = FOCUS_RING;
/// Disabled input text: 5.2:1 on the sheet, 4.8:1 on the panel.
pub const FIELD_DISABLED_TEXT: Color32 = TEXT_DISABLED;
/// FieldSet group fill — the raised surface (guide II.30.6 `surface.raised`).
pub const FIELD_GROUP_FILL: Color32 = SURFACE_PANEL;
/// FieldSet group border — the group boundary identifies itself: 7.8:1 on the
/// sheet, 7.2:1 on the panel.
pub const FIELD_GROUP_BORDER: Color32 = BORDER_STRONG;

/// The 2 px accent stroke drawn around selected entities.
pub fn selection_stroke() -> egui::Stroke {
    egui::Stroke::new(2.0, ACCENT_DEFAULT)
}

/// The snap-marker colour for `kind` (tier 3 component token).
pub fn snap_marker_colour(kind: &crate::canvas::SnapModeKind) -> Color32 {
    match kind {
        crate::canvas::SnapModeKind::Midpoint => SNAP_MIDPOINT,
        crate::canvas::SnapModeKind::Perpendicular => SNAP_PERPENDICULAR,
        crate::canvas::SnapModeKind::Tangent => SNAP_TANGENT,
        crate::canvas::SnapModeKind::Ortho => SNAP_ORTHO,
        crate::canvas::SnapModeKind::Grid => SNAP_GRID,
        crate::canvas::SnapModeKind::Endpoint => SUCCESS_DEFAULT,
        crate::canvas::SnapModeKind::Intersection => ACCENT_DEFAULT,
        crate::canvas::SnapModeKind::Free => STROKE_STRONG,
    }
}
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

// ---- Button role styles (shadcn-egui / modern-egui pattern copy, guide II.30.10) ----
// Four roles x three sizes, every size honouring the 24pt floor
// (guide II.30.7.5); focus rings are 2px ACCENT_DEFAULT (guide II.30.7.4).
// Component tokens: every field points at a tier-2 role. The widget that
// consumes this table is `crate::components::buttons::StyledButton`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonRole {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

/// Per-variant button style: six states, every field a tier-2 role.
///
/// States (guide II.30.10): default (`fill` + `text`), hover (`hover`),
/// pressed (`active` + `text_active` — pressed fills are dark, so labels flip
/// to the light role), focus (`focus` ring, painted only while focused),
/// disabled (`disabled` + `text_disabled`) and loading (rest fill, interaction
/// disabled, spinner drawn in `text` — control size never changes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonRoleStyle {
    /// Rest fill.
    pub fill: Color32,
    /// Label colour on `fill` (also the loading spinner colour).
    pub text: Color32,
    /// Pointer-hover fill.
    pub hover: Color32,
    /// Pressed fill.
    pub active: Color32,
    /// Label colour on `active`.
    pub text_active: Color32,
    /// Focus-ring stroke colour, 2px, painted only while focused.
    pub focus: Color32,
    /// Disabled fill.
    pub disabled: Color32,
    /// Label colour on `disabled`.
    pub text_disabled: Color32,
    /// Minimum control size — the 24pt floor per size (guide II.30.7.5).
    pub min_size: eframe::egui::Vec2,
}

pub const BUTTON_PRIMARY_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: ACCENT_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: ACCENT_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_PRIMARY_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: ACCENT_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: ACCENT_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_PRIMARY_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: ACCENT_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: ACCENT_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_SECONDARY_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_PANEL,
    text: TEXT_PRIMARY,
    hover: SURFACE_BASE,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_SECONDARY_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_PANEL,
    text: TEXT_PRIMARY,
    hover: SURFACE_BASE,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_SECONDARY_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_PANEL,
    text: TEXT_PRIMARY,
    hover: SURFACE_BASE,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_GHOST_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_TRANSPARENT,
    text: TEXT_PRIMARY,
    hover: SURFACE_PANEL,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_GHOST_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_TRANSPARENT,
    text: TEXT_PRIMARY,
    hover: SURFACE_PANEL,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_GHOST_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: SURFACE_TRANSPARENT,
    text: TEXT_PRIMARY,
    hover: SURFACE_PANEL,
    active: ACCENT_DEFAULT,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

pub const BUTTON_DANGER_S: ButtonRoleStyle = ButtonRoleStyle {
    fill: DANGER_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: DANGER_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(24.0, 24.0),
};
pub const BUTTON_DANGER_M: ButtonRoleStyle = ButtonRoleStyle {
    fill: DANGER_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: DANGER_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(32.0, 32.0),
};
pub const BUTTON_DANGER_L: ButtonRoleStyle = ButtonRoleStyle {
    fill: DANGER_DEFAULT,
    text: TEXT_ON_ACCENT,
    hover: DANGER_HOVER,
    active: ACCENT_ACTIVE,
    text_active: TEXT_ON_ACCENT,
    focus: FOCUS_RING,
    disabled: TEXT_MUTED,
    text_disabled: TEXT_ON_ACCENT,
    min_size: eframe::egui::vec2(40.0, 40.0),
};

// ---------------------------------------------------------------------------
// Contrast methodology (spec v0.20.0 AC-08, ticket T-007, guide II.30.7.1)
// ---------------------------------------------------------------------------

/// WCAG 2.2 contrast gate for every palette Lintel ships, plus the dark and
/// high-contrast reference palettes for the not-yet-wired theme switcher.
///
/// The maths (`rgb_of`, `relative_luminance`, `contrast_ratio`) lives at the
/// theme root and is shared with the light-only test below; this module adds
/// the *methodology*: a [`Palette`] per [`Theme`], a per-palette
/// [`contrast_check`] that walks every text/UI pairing the palettes actually
/// draw, and the [`contrast_test`] merge gate that fails CI listing every
/// failure with role, measured ratio and required ratio (ticket T-007).
///
/// Everything in here is consumed by `theme_tests.rs` (headless, K.13); the
/// runtime Light/Dark/HighContrast switch is a later slice — until it lands
/// the dark and high-contrast palettes are the asserted reference for that
/// work (D-035).
pub use contrast::{contrast_check, contrast_test, ContrastFailure, Theme};
pub mod contrast {
    #![allow(dead_code)] // merge-gate API: consumed by theme_tests.rs, not yet by the binary (D-035)

    use std::fmt;

    use super::*;

    // Re-export the colour maths functions for the merge-gate tests.
    pub use super::{rgb_of, contrast_ratio, composited_over, palette, contrast_test, Theme, ContrastFailure};

    /// The palettes the contrast gate asserts (spec AC-08).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Theme {
        /// The shipped warm paper-and-ink palette.
        Light,
        /// Dark reference palette: asserted AA here; runtime switching is a
        /// later slice (D-035).
        Dark,
        /// High-contrast reference palette: WCAG AAA (7:1) text.
        HighContrast,
    }

    impl Theme {
        /// Every palette the gate walks, in display order.
        pub const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::HighContrast];

        /// Lowercase name used in failure messages.
        pub fn name(self) -> &'static str {
            match self {
                Theme::Light => "light",
                Theme::Dark => "dark",
                Theme::HighContrast => "high-contrast",
            }
        }

        /// WCAG body-text floor for this palette: AA 4.5:1, or AAA 7:1 for
        /// the high-contrast variant (ticket T-007 thresholds).
        pub fn text_floor(self) -> f64 {
            match self {
                Theme::HighContrast => 7.0,
                _ => 4.5,
            }
        }
    }

    /// One button role's colour states for a palette. Mirrors the colour
    /// fields of the light theme's `ButtonRoleStyle` (sizes are
    /// palette-independent and stay there).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ButtonStates {
        /// Rest fill. Alpha 0 marks a ghost button: its label is checked
        /// against both light surfaces it is drawn over.
        pub fill: Color32,
        /// Label colour on `fill`.
        pub text: Color32,
        /// Hover fill.
        pub hover: Color32,
        /// Pressed fill.
        pub active: Color32,
        /// Label colour on `active`.
        pub text_active: Color32,
        /// Disabled fill.
        pub disabled: Color32,
        /// Label colour on `disabled`.
        pub text_disabled: Color32,
    }

    /// One palette resolved to the colours the gate checks. Light resolves
    /// from the shipped tier-1/2/3 tokens; dark and high-contrast resolve
    /// from their reference swatches below.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Palette {
        pub surface_base: Color32,
        pub surface_panel: Color32,
        pub surface_status: Color32,
        pub text_primary: Color32,
        pub text_secondary: Color32,
        pub text_muted: Color32,
        pub text_hint: Color32,
        pub text_disabled: Color32,
        pub text_strong: Color32,
        pub text_match: Color32,
        pub text_success: Color32,
        pub text_danger: Color32,
        pub text_error: Color32,
        pub text_warning: Color32,
        pub text_on_accent: Color32,
        pub accent: Color32,
        pub accent_hover: Color32,
        pub accent_active: Color32,
        pub success: Color32,
        pub danger: Color32,
        pub danger_hover: Color32,
        pub border_strong: Color32,
        /// Selection edge; may be translucent — resolve with
        /// [`composited_over`] before comparing.
        pub border_selection: Color32,
        /// Selected-row wash; may be translucent.
        pub selection_fill: Color32,
        pub focus_ring: Color32,
        /// Button colour states in role order: primary, secondary, ghost,
        /// danger.
        pub buttons: [(&'static str, ButtonStates); 4],
        pub input_fill: Color32,
        pub input_text: Color32,
        pub input_placeholder: Color32,
        pub input_border: Color32,
    }

    /// One contrast failure: which pairing, what was measured, what is
    /// required. `Display` names all three so a red CI log is actionable
    /// (ticket T-007).
    #[derive(Debug)]
    pub enum ContrastFailure {
        /// A text pairing fell under its floor (4.5:1, or 7:1 on the
        /// high-contrast palette).
        Text {
            role: String,
            ratio: f64,
            required: f64,
        },
        /// A UI boundary (border, focus ring, accent graphic) fell under
        /// 3:1 (WCAG 2.2 §1.4.11).
        Ui {
            role: String,
            ratio: f64,
            required: f64,
        },
    }

    impl fmt::Display for ContrastFailure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (kind, role, ratio, required) = match self {
                ContrastFailure::Text {
                    role,
                    ratio,
                    required,
                } => ("text", role, ratio, required),
                ContrastFailure::Ui { role, ratio, required } => ("ui", role, ratio, required),
            };
            write!(
                f,
                "{kind} contrast failure: {role}: {ratio:.2}:1 (required {required:.1}:1)"
            )
        }
    }

    impl std::error::Error for ContrastFailure {}

    /// Composite a translucent token over `bg`, giving the solid colour the
    /// screen actually shows — WCAG contrast is defined for solid colours
    /// (guide II.30.7.1). egui colours are alpha-premultiplied, so src-over
    /// is `overlay + bg * (1 - alpha)` per channel.
    pub fn composited_over(bg: Color32, overlay: Color32) -> Color32 {
        let a = overlay.a() as u32;
        let blend = |o: u8, b: u8| -> u8 {
            let v = o as u32 + (b as u32 * (255 - a)) / 255;
            v.min(255) as u8
        };
        Color32::from_rgb(
            blend(overlay.r(), bg.r()),
            blend(overlay.g(), bg.g()),
            blend(overlay.b(), bg.b()),
        )
    }

    /// Check one palette and return every failure (empty when it passes).
    ///
    /// The pairing lists are the surfaces each role is actually drawn on —
    /// e.g. the `success` graphic appears on the sheet and panels, while the
    /// status strip carries `text.success` (a 4.5:1 text role) instead — so
    /// the gate measures real pairings, not the cross product.
    pub fn contrast_check(theme: Theme) -> Vec<ContrastFailure> {
        let p = palette(theme);
        let floor = theme.text_floor();
        let ui_floor = 3.0;
        let mut out = Vec::new();
        let mut c = Checker {
            out: &mut out,
            theme_name: theme.name(),
        };

        let surfaces: [(&str, Color32); 3] = [
            ("surface.base", p.surface_base),
            ("surface.panel", p.surface_panel),
            ("surface.status", p.surface_status),
        ];
        let sheet_and_panel: [(&str, Color32); 2] =
            [("surface.base", p.surface_base), ("surface.panel", p.surface_panel)];

        // (a) Body and status text on every surface text is drawn on.
        let text_roles: [(&str, Color32); 11] = [
            ("text.primary", p.text_primary),
            ("text.secondary", p.text_secondary),
            ("text.muted", p.text_muted),
            ("text.hint", p.text_hint),
            ("text.disabled", p.text_disabled),
            ("text.strong", p.text_strong),
            ("text.match", p.text_match),
            ("text.success", p.text_success),
            ("text.danger", p.text_danger),
            ("text.error", p.text_error),
            ("text.warning", p.text_warning),
        ];
        for (role, fg) in text_roles {
            for (surface, bg) in surfaces {
                c.text(role, fg, bg, surface, floor);
            }
        }

        // (b) Labels on accent-family fills (buttons, active-tool fills).
        let accent_fills: [(&str, Color32); 5] = [
            ("accent", p.accent),
            ("accent.hover", p.accent_hover),
            ("accent.active", p.accent_active),
            ("danger", p.danger),
            ("danger.hover", p.danger_hover),
        ];
        for (fill, bg) in accent_fills {
            c.text("text.on_accent", p.text_on_accent, bg, fill, floor);
        }

        // (c) UI boundaries and graphics: WCAG 2.2 §1.4.11, 3:1.
        for (role, fg) in [
            ("accent", p.accent),
            ("accent.hover", p.accent_hover),
            ("success", p.success),
            ("danger", p.danger),
            ("focus.ring", p.focus_ring),
        ] {
            for (surface, bg) in sheet_and_panel {
                c.ui(role, fg, bg, surface, ui_floor);
            }
        }
        // The strong border identifies input fields, so it must read on every
        // surface a field can sit on. `border.default` stays a decorative
        // divider, which §1.4.11 exempts.
        for (surface, bg) in surfaces {
            c.ui("border.strong", p.border_strong, bg, surface, ui_floor);
        }
        // Selection: the edge is the state marker; the wash only tints the
        // row. Both are translucent, so resolve them first.
        let edge_on_base = composited_over(p.surface_base, p.border_selection);
        let edge_on_panel = composited_over(p.surface_panel, p.border_selection);
        c.ui(
            "border.selection (composited)",
            edge_on_base,
            p.surface_base,
            "surface.base",
            ui_floor,
        );
        c.ui(
            "border.selection (composited)",
            edge_on_panel,
            p.surface_panel,
            "surface.panel",
            ui_floor,
        );
        let fill_on_base = composited_over(p.surface_base, p.selection_fill);
        if fill_on_base == p.surface_base {
            c.out.push(ContrastFailure::Ui {
                role: format!("{}: selection.fill is invisible on surface.base", c.theme_name),
                ratio: 1.0,
                required: 1.0,
            });
        }
        c.text(
            "text.strong on selection.fill",
            p.text_strong,
            fill_on_base,
            "surface.base (selected row)",
            floor,
        );

        // (d) Buttons: label readable in every state (guide II.30.10).
        for (role, b) in p.buttons {
            let role = format!("button.{role}");
            if b.fill.a() == 0 {
                // Ghost: no fill of its own; the label reads against the
                // surfaces it is drawn over.
                c.text(
                    &format!("{role}.text"),
                    b.text,
                    p.surface_base,
                    "surface.base",
                    floor,
                );
                c.text(
                    &format!("{role}.text"),
                    b.text,
                    p.surface_panel,
                    "surface.panel",
                    floor,
                );
            } else {
                c.text(&format!("{role}.text"), b.text, b.fill, &role, floor);
            }
            c.text(
                &format!("{role}.text (hover)"),
                b.text,
                b.hover,
                &format!("{role}.hover"),
                floor,
            );
            c.text(
                &format!("{role}.text (active)"),
                b.text_active,
                b.active,
                &format!("{role}.active"),
                floor,
            );
            // WCAG 2.2 §1.4.3 exempts disabled controls from contrast; the
            // palettes still hold disabled labels to 4.5:1.
            c.text(
                &format!("{role}.text (disabled)"),
                b.text_disabled,
                b.disabled,
                &format!("{role}.disabled"),
                4.5,
            );
        }

        // (e) Input fields: value text, placeholder text and the border that
        // identifies the field (WCAG 2.2 §1.4.11). This is the reference
        // pairing the T-002 `Field` components must implement (D-035).
        c.text("input.text", p.input_text, p.input_fill, "input.fill", floor);
        c.text(
            "input.placeholder",
            p.input_placeholder,
            p.input_fill,
            "input.fill",
            floor,
        );
        c.ui("input.border", p.input_border, p.input_fill, "input.fill", ui_floor);

        out
    }

    /// The merge gate: walk every palette; `Ok` only when all of them clear
    /// their floors. CI runs `cargo test contrast_test` (spec AC-08).
    pub fn contrast_test() -> Result<(), Vec<ContrastFailure>> {
        let failures: Vec<ContrastFailure> =
            Theme::ALL.iter().copied().flat_map(contrast_check).collect();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }

    /// The palette behind `theme`.
    pub fn palette(theme: Theme) -> Palette {
        match theme {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
            Theme::HighContrast => HIGH_CONTRAST,
        }
    }

    /// Failure collector: records every below-floor pairing with the theme,
    /// role, measured ratio and required ratio in its message.
    struct Checker<'a> {
        out: &'a mut Vec<ContrastFailure>,
        theme_name: &'static str,
    }

    impl Checker<'_> {
        fn text(&mut self, role: &str, fg: Color32, bg: Color32, surface: &str, required: f64) {
            let ratio = super::contrast_ratio(super::rgb_of(fg), super::rgb_of(bg));
            if ratio < required {
                self.out.push(ContrastFailure::Text {
                    role: format!("{}: {role} on {surface}", self.theme_name),
                    ratio,
                    required,
                });
            }
        }

        fn ui(&mut self, role: &str, fg: Color32, bg: Color32, surface: &str, required: f64) {
            let ratio = super::contrast_ratio(super::rgb_of(fg), super::rgb_of(bg));
            if ratio < required {
                self.out.push(ContrastFailure::Ui {
                    role: format!("{}: {role} on {surface}", self.theme_name),
                    ratio,
                    required,
                });
            }
        }
    }

    // Light resolves from the shipped tokens — one token source, no drift.
    const LIGHT: Palette = Palette {
        surface_base: SURFACE_BASE,
        surface_panel: SURFACE_PANEL,
        surface_status: SURFACE_STATUS,
        text_primary: TEXT_PRIMARY,
        text_secondary: TEXT_SECONDARY,
        text_muted: TEXT_MUTED,
        text_hint: TEXT_HINT,
        text_disabled: TEXT_DISABLED,
        text_strong: TEXT_STRONG,
        text_match: TEXT_MATCH,
        text_success: TEXT_SUCCESS,
        text_danger: TEXT_DANGER,
        text_error: TEXT_ERROR,
        text_warning: TEXT_WARNING,
        text_on_accent: TEXT_ON_ACCENT,
        accent: ACCENT_DEFAULT,
        accent_hover: ACCENT_HOVER,
        accent_active: ACCENT_ACTIVE,
        success: SUCCESS_DEFAULT,
        danger: DANGER_DEFAULT,
        danger_hover: DANGER_HOVER,
        border_strong: BORDER_STRONG,
        border_selection: BORDER_SELECTION,
        selection_fill: SURFACE_SELECTION,
        focus_ring: FOCUS_RING,
        buttons: [
            (
                "primary",
                ButtonStates {
                    fill: BUTTON_PRIMARY_M.fill,
                    text: BUTTON_PRIMARY_M.text,
                    hover: BUTTON_PRIMARY_M.hover,
                    active: BUTTON_PRIMARY_M.active,
                    text_active: BUTTON_PRIMARY_M.text_active,
                    disabled: BUTTON_PRIMARY_M.disabled,
                    text_disabled: BUTTON_PRIMARY_M.text_disabled,
                },
            ),
            (
                "secondary",
                ButtonStates {
                    fill: BUTTON_SECONDARY_M.fill,
                    text: BUTTON_SECONDARY_M.text,
                    hover: BUTTON_SECONDARY_M.hover,
                    active: BUTTON_SECONDARY_M.active,
                    text_active: BUTTON_SECONDARY_M.text_active,
                    disabled: BUTTON_SECONDARY_M.disabled,
                    text_disabled: BUTTON_SECONDARY_M.text_disabled,
                },
            ),
            (
                "ghost",
                ButtonStates {
                    fill: BUTTON_GHOST_M.fill,
                    text: BUTTON_GHOST_M.text,
                    hover: BUTTON_GHOST_M.hover,
                    active: BUTTON_GHOST_M.active,
                    text_active: BUTTON_GHOST_M.text_active,
                    disabled: BUTTON_GHOST_M.disabled,
                    text_disabled: BUTTON_GHOST_M.text_disabled,
                },
            ),
            (
                "danger",
                ButtonStates {
                    fill: BUTTON_DANGER_M.fill,
                    text: BUTTON_DANGER_M.text,
                    hover: BUTTON_DANGER_M.hover,
                    active: BUTTON_DANGER_M.active,
                    text_active: BUTTON_DANGER_M.text_active,
                    disabled: BUTTON_DANGER_M.disabled,
                    text_disabled: BUTTON_DANGER_M.text_disabled,
                },
            ),
        ],
        input_fill: SURFACE_BASE,
        input_text: TEXT_PRIMARY,
        input_placeholder: TEXT_MUTED,
        input_border: BORDER_STRONG,
    };

    // Dark reference swatches (tier-1 style: raw, no meaning attached).
    const DARK_SHEET: Color32 = Color32::from_rgb(32, 30, 28);
    const DARK_PANEL: Color32 = Color32::from_rgb(44, 41, 38);
    const DARK_STATUS: Color32 = Color32::from_rgb(38, 35, 33);
    const DARK_TEXT_SECONDARY: Color32 = Color32::from_rgb(176, 170, 162);
    const DARK_TEXT_MUTED: Color32 = Color32::from_rgb(152, 146, 138);
    const DARK_TEXT_HINT: Color32 = Color32::from_rgb(168, 162, 154);
    const DARK_TEXT_DISABLED: Color32 = Color32::from_rgb(156, 150, 142);
    const DARK_TEXT_MATCH: Color32 = Color32::from_rgb(110, 175, 235);
    const DARK_TEXT_WARNING: Color32 = Color32::from_rgb(222, 150, 60);
    const DARK_COBALT: Color32 = Color32::from_rgb(86, 156, 214);
    const DARK_COBALT_HOVER: Color32 = Color32::from_rgb(108, 172, 224);
    const DARK_SUCCESS: Color32 = Color32::from_rgb(74, 200, 130);
    const DARK_DANGER: Color32 = Color32::from_rgb(255, 107, 103);
    const DARK_DANGER_HOVER: Color32 = Color32::from_rgb(255, 130, 124);
    const DARK_BORDER_STRONG: Color32 = Color32::from_rgb(140, 134, 126);
    const DARK_WIDGET_HOVER: Color32 = Color32::from_rgb(52, 49, 46);
    const DARK_INPUT_FILL: Color32 = Color32::from_rgb(26, 24, 22);

    /// Dark reference palette. Warm graphite sheet, paper ink, a lighter
    /// cobalt so the accent stays a visible boundary on dark surfaces
    /// (5.6:1 on the sheet), and dark labels on the now-light accent fills.
    const DARK: Palette = Palette {
        surface_base: DARK_SHEET,
        surface_panel: DARK_PANEL,
        surface_status: DARK_STATUS,
        text_primary: BASE_PAPER,
        text_secondary: DARK_TEXT_SECONDARY,
        text_muted: DARK_TEXT_MUTED,
        text_hint: DARK_TEXT_HINT,
        text_disabled: DARK_TEXT_DISABLED,
        text_strong: Color32::WHITE,
        text_match: DARK_TEXT_MATCH,
        text_success: DARK_SUCCESS,
        text_danger: DARK_DANGER,
        // The dark reference shares one red for danger and error messages.
        text_error: DARK_DANGER,
        text_warning: DARK_TEXT_WARNING,
        text_on_accent: BASE_INK,
        accent: DARK_COBALT,
        accent_hover: DARK_COBALT_HOVER,
        accent_active: BASE_PAPER,
        success: DARK_SUCCESS,
        danger: DARK_DANGER,
        danger_hover: DARK_DANGER_HOVER,
        border_strong: DARK_BORDER_STRONG,
        border_selection: Color32::from_rgba_premultiplied(255, 255, 255, 140),
        selection_fill: Color32::from_rgba_premultiplied(255, 255, 255, 30),
        focus_ring: DARK_COBALT,
        buttons: [
            (
                "primary",
                ButtonStates {
                    fill: DARK_COBALT,
                    text: BASE_INK,
                    hover: DARK_COBALT_HOVER,
                    active: BASE_PAPER,
                    text_active: BASE_INK,
                    disabled: DARK_TEXT_MUTED,
                    text_disabled: BASE_INK,
                },
            ),
            (
                "secondary",
                ButtonStates {
                    fill: DARK_PANEL,
                    text: BASE_PAPER,
                    hover: DARK_WIDGET_HOVER,
                    active: DARK_COBALT,
                    text_active: BASE_INK,
                    disabled: DARK_TEXT_MUTED,
                    text_disabled: BASE_INK,
                },
            ),
            (
                "ghost",
                ButtonStates {
                    fill: SURFACE_TRANSPARENT,
                    text: BASE_PAPER,
                    hover: DARK_PANEL,
                    active: DARK_COBALT,
                    text_active: BASE_INK,
                    disabled: DARK_TEXT_MUTED,
                    text_disabled: BASE_INK,
                },
            ),
            (
                "danger",
                ButtonStates {
                    fill: DARK_DANGER,
                    text: BASE_INK,
                    hover: DARK_DANGER_HOVER,
                    active: BASE_PAPER,
                    text_active: BASE_INK,
                    disabled: DARK_TEXT_MUTED,
                    text_disabled: BASE_INK,
                },
            ),
        ],
        input_fill: DARK_INPUT_FILL,
        input_text: BASE_PAPER,
        input_placeholder: DARK_TEXT_MUTED,
        input_border: DARK_BORDER_STRONG,
    };

    // High-contrast reference swatches (tier-1 style: raw, no meaning).
    const HC_SHEET: Color32 = Color32::from_rgb(12, 12, 12);
    const HC_PANEL: Color32 = Color32::from_rgb(26, 26, 26);
    const HC_STATUS: Color32 = Color32::from_rgb(20, 20, 20);
    const HC_TEXT_SECONDARY: Color32 = Color32::from_rgb(200, 200, 200);
    const HC_TEXT_MUTED: Color32 = Color32::from_rgb(170, 170, 170);
    const HC_TEXT_HINT: Color32 = Color32::from_rgb(176, 176, 176);
    const HC_TEXT_DISABLED: Color32 = Color32::from_rgb(166, 166, 166);
    const HC_TEXT_MATCH: Color32 = Color32::from_rgb(150, 205, 255);
    const HC_COBALT: Color32 = Color32::from_rgb(120, 190, 255);
    const HC_COBALT_HOVER: Color32 = Color32::from_rgb(150, 205, 255);
    const HC_SUCCESS: Color32 = Color32::from_rgb(0, 230, 118);
    const HC_DANGER: Color32 = Color32::from_rgb(255, 133, 127);
    const HC_DANGER_HOVER: Color32 = Color32::from_rgb(255, 150, 145);
    const HC_WARNING: Color32 = Color32::from_rgb(255, 190, 90);
    const HC_BORDER_STRONG: Color32 = Color32::from_rgb(200, 200, 200);
    const HC_DISABLED_FILL: Color32 = Color32::from_rgb(60, 60, 60);
    const HC_WIDGET_HOVER: Color32 = Color32::from_rgb(44, 44, 44);

    /// High-contrast reference palette: near-black sheet, white ink, every
    /// text role at or over the 7:1 AAA floor (dark labels on the light
    /// accent and status fills), solid white selection edge.
    const HIGH_CONTRAST: Palette = Palette {
        surface_base: HC_SHEET,
        surface_panel: HC_PANEL,
        surface_status: HC_STATUS,
        text_primary: Color32::WHITE,
        text_secondary: HC_TEXT_SECONDARY,
        text_muted: HC_TEXT_MUTED,
        text_hint: HC_TEXT_HINT,
        text_disabled: HC_TEXT_DISABLED,
        text_strong: Color32::WHITE,
        text_match: HC_TEXT_MATCH,
        text_success: HC_SUCCESS,
        text_danger: HC_DANGER,
        // The high-contrast reference shares one red for danger and error.
        text_error: HC_DANGER,
        text_warning: HC_WARNING,
        text_on_accent: BASE_INK,
        accent: HC_COBALT,
        accent_hover: HC_COBALT_HOVER,
        accent_active: Color32::WHITE,
        success: HC_SUCCESS,
        danger: HC_DANGER,
        danger_hover: HC_DANGER_HOVER,
        border_strong: HC_BORDER_STRONG,
        border_selection: Color32::WHITE,
        selection_fill: Color32::from_rgba_premultiplied(255, 255, 255, 60),
        focus_ring: HC_COBALT,
        buttons: [
            (
                "primary",
                ButtonStates {
                    fill: HC_COBALT,
                    text: BASE_INK,
                    hover: HC_COBALT_HOVER,
                    active: Color32::WHITE,
                    text_active: BASE_INK,
                    disabled: HC_DISABLED_FILL,
                    text_disabled: HC_TEXT_DISABLED,
                },
            ),
            (
                "secondary",
                ButtonStates {
                    fill: HC_PANEL,
                    text: Color32::WHITE,
                    hover: HC_WIDGET_HOVER,
                    active: Color32::WHITE,
                    text_active: BASE_INK,
                    disabled: HC_DISABLED_FILL,
                    text_disabled: HC_TEXT_DISABLED,
                },
            ),
            (
                "ghost",
                ButtonStates {
                    fill: SURFACE_TRANSPARENT,
                    text: Color32::WHITE,
                    hover: HC_WIDGET_HOVER,
                    active: Color32::WHITE,
                    text_active: BASE_INK,
                    disabled: HC_DISABLED_FILL,
                    text_disabled: HC_TEXT_DISABLED,
                },
            ),
            (
                "danger",
                ButtonStates {
                    fill: HC_DANGER,
                    text: BASE_INK,
                    hover: HC_DANGER_HOVER,
                    active: Color32::WHITE,
                    text_active: BASE_INK,
                    disabled: HC_DISABLED_FILL,
                    text_disabled: HC_TEXT_DISABLED,
                },
            ),
        ],
        input_fill: Color32::BLACK,
        input_text: Color32::WHITE,
        input_placeholder: HC_TEXT_MUTED,
        input_border: Color32::WHITE,
    };
}

/// Button corner radius (design-token radii 2/4/8/12 — guide II.30.6).
pub const BUTTON_CORNER_RADIUS: f32 = 4.0;

/// The 2 px accent ring every button variant paints while focused, and the
/// accent outline hand-styled buttons reuse (guide II.30.7.4).
pub const BUTTON_RING: Color32 = FOCUS_RING;

/// Minimum target size for icon-only buttons (24pt floor per guide II.30.7.5).
pub const ICON_BUTTON_MIN_TARGET: f32 = 24.0;

// ---------------------------------------------------------------------------
// Spacing, radius, and elevation tokens (component tier, spec v0.20.0 AC-04)
// ---------------------------------------------------------------------------

/// Base spacing unit: 4pt (guide II.30.6 — 4-point base scale: 2-4-8-12-16-24-32-48).
pub const SPACING_BASE: f32 = 4.0;

/// Extra-small spacing: 2pt (half base).
pub const SPACING_XS: f32 = SPACING_BASE / 2.0;
/// Small spacing: 4pt (base).
pub const SPACING_SM: f32 = SPACING_BASE;
/// Medium spacing: 8pt (2× base).
pub const SPACING_MD: f32 = SPACING_BASE * 2.0;
/// Large spacing: 12pt (3× base).
pub const SPACING_LG: f32 = SPACING_BASE * 3.0;
/// Extra-large spacing: 16pt (4× base).
pub const SPACING_XL: f32 = SPACING_BASE * 4.0;
/// 2XL spacing: 24pt (6× base).
pub const SPACING_2XL: f32 = SPACING_BASE * 6.0;

/// Corner radius scale (guide II.30.6): 2 / 4 / 8 / 12.
pub const RADIUS_XS: f32 = 2.0;
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 8.0;
pub const RADIUS_LG: f32 = 12.0;

/// Elevation shadows for raised surfaces (guide II.30.6).
/// Level 1: subtle lift (cards, panels).
pub const SHADOW_LEVEL_1: egui::Shadow = egui::Shadow {
    color: Color32::from_black_alpha(20),
    offset: [0, 2],
    blur: 4,
    spread: 0,
};
/// Level 2: medium lift (dropdowns, tooltips).
pub const SHADOW_LEVEL_2: egui::Shadow = egui::Shadow {
    color: Color32::from_black_alpha(35),
    offset: [0, 6],
    blur: 12,
    spread: 0,
};
/// Level 3: high lift (modals, overlays).
pub const SHADOW_LEVEL_3: egui::Shadow = egui::Shadow {
    color: Color32::from_black_alpha(50),
    offset: [0, 16],
    blur: 24,
    spread: 0,
};

// ---------------------------------------------------------------------------
// Form field size tokens (shadcn-egui Field pattern copy, spec v0.20.0 AC-03)
// — derived from the 4-point spacing scale and 2/4/8/12 radii above
// (guide II.30.6).
// ---------------------------------------------------------------------------

/// Vertical gap between a field's label, input and message rows.
pub const FIELD_ROW_SPACING: f32 = SPACING_SM;
/// Horizontal gap between an error icon and its text.
pub const FIELD_MESSAGE_GAP: f32 = SPACING_XS;
/// Vertical gap between fields inside a FieldSet.
pub const FIELD_SET_SPACING: f32 = SPACING_MD;
/// Gap between fields in a FieldGroup.
pub const FIELD_GROUP_SPACING: f32 = SPACING_MD;
/// Corner radius of inputs and field-set frames.
pub const FIELD_CORNER_RADIUS: f32 = RADIUS_SM;
/// Gap between the input rect and the painted state stroke.
pub const FIELD_BORDER_PAD: f32 = 2.0;
/// Minimum interactive height of an input — the 24pt floor (guide II.30.7.5).
pub const FIELD_MIN_HEIGHT: f32 = 24.0;
/// Side length of the painted error icon.
pub const FIELD_ERROR_ICON_SIZE: f32 = 12.0;
/// Body text size for painter-drawn field text (legend row).
pub const FIELD_TEXT_SIZE: f32 = 14.0;

// ---------------------------------------------------------------------------
// Icon system (spaceui pattern copy, spec v0.20.0 AC-05)
//
// Original SVGs in `apps/lintel/assets/icons/` are rasterized at the exact
// physical pixel size the UI paints them at — never scaled afterwards — so a
// 16 pt icon is 16·ppi texels and stays crisp on every display (guide
// II.30.8). No icon crate is involved: the assets are Lintel's own
// (clean-room, guide II.27.2), listed in ATTRIBUTION.md.
// ---------------------------------------------------------------------------

/// Semantic names of every icon in `apps/lintel/assets/icons/`.
///
/// Each name is also the SVG file stem; `components::icons` maps names to the
/// human-readable AccessKit labels.
// Consumed by the icon unit tests until the shell slice (T-005) wires the
// toolbar; the asset registry itself is compile-time, so a missing SVG fails
// the build instead of this list.
#[allow(dead_code)]
pub const ICON_NAMES: &[&str] = &[
    // Tools (T-004)
    "wall",
    "door",
    "window",
    "dimension",
    "label",
    "measurements",
    "select",
    "construction_line",
    "construction_ray",
    "construction_circle",
    "construction_arc",
    "block",
    // Snap / Layer / UI (pre-existing)
    "snap_endpoint",
    "snap_midpoint",
    "snap_center",
    "snap_intersection",
    "snap_grid",
    "snap_ortho",
    "layer_visible",
    "layer_lock",
    "layer_print",
    "dimension_aligned",
    "dimension_radial",
    "block_insert",
];

/// Embedded icon sources, `(name, SVG bytes)`.
///
/// Compiled in with `include_bytes!` so release binaries (AppImage, MSI, dmg)
/// never read assets from disk, and so a missing or renamed file breaks the
/// build instead of shipping a blank icon.
const ICON_SVGS: &[(&str, &[u8])] = &[
    // Tools (T-004)
    ("wall", include_bytes!("../assets/icons/wall.svg")),
    ("door", include_bytes!("../assets/icons/door.svg")),
    ("window", include_bytes!("../assets/icons/window.svg")),
    ("dimension", include_bytes!("../assets/icons/dimension.svg")),
    ("label", include_bytes!("../assets/icons/label.svg")),
    ("measurements", include_bytes!("../assets/icons/measurements.svg")),
    ("select", include_bytes!("../assets/icons/select.svg")),
    ("construction_line", include_bytes!("../assets/icons/construction_line.svg")),
    ("construction_ray", include_bytes!("../assets/icons/construction_ray.svg")),
    ("construction_circle", include_bytes!("../assets/icons/construction_circle.svg")),
    ("construction_arc", include_bytes!("../assets/icons/construction_arc.svg")),
    ("block", include_bytes!("../assets/icons/block.svg")),
    // Snap / Layer / UI (pre-existing)
    ("snap_endpoint", include_bytes!("../assets/icons/snap_endpoint.svg")),
    ("snap_midpoint", include_bytes!("../assets/icons/snap_midpoint.svg")),
    ("snap_center", include_bytes!("../assets/icons/snap_center.svg")),
    ("snap_intersection", include_bytes!("../assets/icons/snap_intersection.svg")),
    ("snap_grid", include_bytes!("../assets/icons/snap_grid.svg")),
    ("snap_ortho", include_bytes!("../assets/icons/snap_ortho.svg")),
    ("layer_visible", include_bytes!("../assets/icons/layer_visible.svg")),
    ("layer_lock", include_bytes!("../assets/icons/layer_lock.svg")),
    ("layer_print", include_bytes!("../assets/icons/layer_print.svg")),
    ("dimension_aligned", include_bytes!("../assets/icons/dimension_aligned.svg")),
    ("dimension_radial", include_bytes!("../assets/icons/dimension_radial.svg")),
    ("block_insert", include_bytes!("../assets/icons/block_insert.svg")),
];

/// Painted when a caller asks for a name that is not in the registry: a
/// dashed square with a slash, so a wiring mistake is visible on screen
/// (guide II.30.11 — an error state, never a silent blank).
const PLACEHOLDER_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"#1A1816\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><rect x=\"4\" y=\"4\" width=\"16\" height=\"16\" rx=\"1\" stroke-dasharray=\"3 3\"/><path d=\"M9 15 L15 9\"/></svg>";

/// The embedded SVG for a semantic icon name, if one is registered.
fn icon_svg(name: &str) -> Option<&'static [u8]> {
    ICON_SVGS
        .iter()
        .find(|(registered, _)| *registered == name)
        .map(|(_, bytes)| *bytes)
}

/// Physical pixel edge for a square icon shown at `size_pt`.
///
/// Hostile input is clamped: a zero or negative size still yields one pixel
/// (guide §2.6 — input-derived numbers are checked, never trusted).
fn icon_pixel_size(ctx: &Context, size_pt: f32) -> usize {
    (size_pt.max(1.0) * ctx.pixels_per_point()).round().max(1.0) as usize
}

/// Rasterize an icon's SVG at exactly `px` physical pixels (square).
///
/// Renders with resvg — the same renderer egui_extras' SVG loader wraps —
/// into a transparent tiny-skia pixmap and hands the premultiplied RGBA
/// straight to epaint. Parse or allocation failures degrade to a transparent
/// image with a log line rather than panicking (guide §2.6).
pub(crate) fn rasterize_icon(name: &str, px: usize) -> ColorImage {
    let source = match icon_svg(name) {
        Some(bytes) => bytes,
        None => {
            eprintln!("lintel: unknown icon name {name:?}, painting the placeholder glyph");
            PLACEHOLDER_SVG.as_bytes()
        }
    };
    let px = px.max(1) as u32;
    let blank = || ColorImage::filled([px as usize, px as usize], Color32::TRANSPARENT);

    let options = resvg::usvg::Options::default();
    let tree = match resvg::usvg::Tree::from_data(source, &options) {
        Ok(tree) => tree,
        Err(error) => {
            eprintln!("lintel: icon {name:?} failed to parse: {error}");
            return blank();
        }
    };
    let Some(mut pixmap) = resvg::tiny_skia::Pixmap::new(px, px) else {
        eprintln!("lintel: icon {name:?} could not allocate a {px}x{px} pixmap");
        return blank();
    };
    // Every icon is authored on a square 24x24 viewBox, so one uniform scale
    // maps SVG units onto the exact requested pixel edge.
    let scale = px as f32 / tree.size().width().max(1.0);
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    ColorImage::from_rgba_premultiplied([px as usize, px as usize], pixmap.data())
}

/// The texture for icon `name`, rasterized at exactly `size_pt` points.
///
/// The requested size is converted to physical pixels with the context's
/// `pixels_per_point`, so the texture is painted 1:1 — one texel per screen
/// pixel, which is what keeps icons crisp (spec AC-05). Results are cached
/// per (name, physical pixel size): the ticket's "(name, size_pt) HashMap in
/// Theme" lives in egui's per-context scratch map (`Context::data_mut`),
/// keyed by the physical size so a display-scale change rasterizes afresh at
/// the new resolution. Per-context storage also keeps a kittest harness's
/// texture ids separate from the app's.
pub fn icon_texture(ctx: &Context, name: &str, size_pt: f32) -> TextureHandle {
    let px = icon_pixel_size(ctx, size_pt);
    let cache_key = egui::Id::new(("lintel_icon_texture", name, px));
    if let Some(handle) = ctx.data(|data| data.get_temp::<TextureHandle>(cache_key)) {
        return handle;
    }
    let image = rasterize_icon(name, px);
    let handle = ctx.load_texture(
        format!("lintel-icon-{name}@{px}px"),
        image,
        TextureOptions::LINEAR,
    );
    ctx.data_mut(|data| data.insert_temp(cache_key, handle.clone()));
    handle
}

// ===========================================================================
// ThemeSpec — the three-tier grouping (egui-pop `ThemeSpec` pattern, copied)
//
// The flat constants above are the named token registry; [`ThemeSpec`] groups
// them into the three tiers of the guide's token discipline (II.30.6), and
// [`SPEC`] is the one handle widget code reads:
//
//   * [`BasePalette`] — tier 1: raw swatches, the only `Color32` constructors.
//   * [`SemanticRoles`] — tier 2: what a colour means; every field is a
//     `BASE_*` swatch (or a sibling role), never a new value.
//   * [`ComponentTokens`] — tier 3: where a role is used; widget code reads
//     this layer only (`SPEC.component.…`), never tiers 1–2.
// ===========================================================================

/// Tier 1 — the raw colour palette (what the swatches look like).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BasePalette {
    pub paper: Color32,
    pub ink: Color32,
    pub black: Color32,
    pub graphite_muted: Color32,
    pub graphite_hint: Color32,
    pub graphite_disabled: Color32,
    pub graphite_secondary: Color32,
    pub match_blue: Color32,
    pub cobalt: Color32,
    pub cobalt_hover: Color32,
    pub green: Color32,
    pub valid: Color32,
    pub error: Color32,
    pub danger: Color32,
    pub danger_hover: Color32,
    pub warning: Color32,
    pub panel_bg: Color32,
    pub panel_border: Color32,
    pub status_bg: Color32,
    pub wall_fill: Color32,
    pub wall_stroke: Color32,
    pub wall_pending: Color32,
    pub grid_minor: Color32,
    pub grid_major: Color32,
    pub window_line: Color32,
    pub selection_wash: Color32,
    pub selection_edge: Color32,
    pub transparent: Color32,
    pub snap_midpoint: Color32,
    pub snap_perpendicular: Color32,
    pub snap_tangent: Color32,
    pub snap_ortho: Color32,
    pub snap_grid: Color32,
    pub construction: Color32,
    pub block_highlight: Color32,
}

/// Tier 2 — meaning-based roles (what each colour is for).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRoles {
    // Surfaces.
    pub surface_base: Color32,
    pub surface_panel: Color32,
    pub surface_status: Color32,
    pub surface_selection: Color32,
    pub surface_transparent: Color32,
    // Borders.
    pub border_default: Color32,
    pub border_selection: Color32,
    // Text.
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub text_hint: Color32,
    pub text_strong: Color32,
    pub text_disabled: Color32,
    pub text_match: Color32,
    pub text_success: Color32,
    pub text_warning: Color32,
    pub text_error: Color32,
    pub text_danger: Color32,
    pub text_on_accent: Color32,
    // Accent, focus and status fills.
    pub accent_default: Color32,
    pub accent_hover: Color32,
    pub accent_active: Color32,
    pub focus_ring: Color32,
    pub success_default: Color32,
    pub danger_default: Color32,
    pub danger_hover: Color32,
    // Structural strokes.
    pub stroke_strong: Color32,
    pub block_highlight: Color32,
    // Snap markers.
    pub snap_midpoint: Color32,
    pub snap_perpendicular: Color32,
    pub snap_tangent: Color32,
    pub snap_ortho: Color32,
    pub snap_grid: Color32,
    // Construction guides.
    pub construction_stroke: Color32,
    // Drafting surface.
    pub draft_fill: Color32,
    pub draft_stroke: Color32,
    pub draft_pending: Color32,
    pub grid_line_minor: Color32,
    pub grid_line_major: Color32,
    // Openings.
    pub opening_cut: Color32,
    pub opening_leaf: Color32,
    pub opening_glazing: Color32,
}

/// Tier 3 — canvas tokens: sheet, grid, walls, openings, selection,
/// dimensions and construction guides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanvasTokens {
    /// The sheet painted before anything is drawn on it.
    pub sheet: Color32,
    /// Entity labels, the empty-canvas hint and the draft readout.
    pub text: Color32,
    /// Committed wall fill.
    pub wall_fill: Color32,
    /// Committed wall outline.
    pub wall_stroke: Color32,
    /// Wall-tool preview that is not committed yet.
    pub wall_pending: Color32,
    /// Minor grid lines.
    pub grid_minor: Color32,
    /// Major grid lines.
    pub grid_major: Color32,
    /// The gap an opening cuts through its host wall.
    pub opening_gap: Color32,
    /// Door leaf and swing arc.
    pub door_leaf: Color32,
    /// Window glazing line.
    pub window_line: Color32,
    /// Construction-guide stroke.
    pub construction_stroke: Color32,
    /// A zero-width stroke ("paint nothing").
    pub stroke_none: Color32,
    /// The accent anything selected turns.
    pub selection: Color32,
    /// Rubber-band window selection.
    pub selection_window: Color32,
    /// Rubber-band crossing selection.
    pub selection_crossing: Color32,
    /// Committed dimension lines, witness lines and ticks.
    pub dimension_stroke: Color32,
    /// Outline around the selected block instance.
    pub block_instance_stroke: Color32,
}

/// Tier 3 — shell panel tokens: inspector surface, status strip, dividers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PanelTokens {
    /// Inspector / toolbar panel fill.
    pub surface: Color32,
    /// Status-bar strip fill.
    pub status: Color32,
    /// Panel divider.
    pub border: Color32,
}

/// Tier 3 — widget text tokens: the tier-2 text roles widgets actually use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentTextTokens {
    /// Primary body text (shell, panels).
    pub primary: Color32,
    /// Small hint text (status bar, palette footer).
    pub hint: Color32,
    /// Secondary / muted body text.
    pub muted: Color32,
    /// Valid-input / success message text.
    pub success: Color32,
    /// Error and destructive message text.
    pub error: Color32,
    /// Destructive accent text (form errors, T-002).
    pub danger: Color32,
}

/// Tier 3 — command-palette row tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaletteTokens {
    /// Highlighted row wash.
    pub selected_fill: Color32,
    /// Non-highlighted rows paint nothing.
    pub unselected_fill: Color32,
    /// Highlighted row outline.
    pub selected_stroke: Color32,
    /// Matched search text.
    pub hit: Color32,
    /// Enabled command label.
    pub label: Color32,
    /// Disabled command label.
    pub disabled: Color32,
    /// Recent-command label.
    pub recent: Color32,
    /// Keyboard shortcut text.
    pub shortcut: Color32,
    /// Category caption.
    pub category: Color32,
}

/// Tier 3 — button tokens: the twelve role × size styles plus the shared
/// 2 px accent ring and the corner radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonTokens {
    pub primary_sm: ButtonRoleStyle,
    pub primary_md: ButtonRoleStyle,
    pub primary_lg: ButtonRoleStyle,
    pub secondary_sm: ButtonRoleStyle,
    pub secondary_md: ButtonRoleStyle,
    pub secondary_lg: ButtonRoleStyle,
    pub ghost_sm: ButtonRoleStyle,
    pub ghost_md: ButtonRoleStyle,
    pub ghost_lg: ButtonRoleStyle,
    pub danger_sm: ButtonRoleStyle,
    pub danger_md: ButtonRoleStyle,
    pub danger_lg: ButtonRoleStyle,
    /// Focus ring shared by every variant (2px ACCENT_DEFAULT).
    pub ring: Color32,
    /// Corner radius shared by every variant.
    pub corner_radius: f32,
}

/// Tier 3 — form field tokens (shadcn-egui Field pattern copy, spec v0.20.0
/// AC-03): colours and sizes for Field, FieldSet, FieldGroup.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldTokens {
    /// Field label text.
    pub label: Color32,
    /// FieldSet legend text.
    pub legend: Color32,
    /// Helper text under an input.
    pub help: Color32,
    /// Error message text.
    pub error_text: Color32,
    /// Error icon fill.
    pub error_icon: Color32,
    /// Input background fill.
    pub input_fill: Color32,
    /// Input text colour.
    pub input_text: Color32,
    /// Rest-state input border.
    pub border: Color32,
    /// Error-state input border.
    pub border_error: Color32,
    /// Focus-state input border.
    pub border_focused: Color32,
    /// Disabled input text.
    pub disabled_text: Color32,
    /// FieldSet group fill.
    pub group_fill: Color32,
    /// FieldSet group border.
    pub group_border: Color32,
    /// Vertical gap between label, input, message rows.
    pub row_spacing: f32,
    /// Horizontal gap between error icon and text.
    pub message_gap: f32,
    /// Vertical gap between fields in a FieldSet.
    pub set_spacing: f32,
    /// Gap between fields in a FieldGroup.
    pub group_spacing: f32,
    /// Corner radius of inputs and field-set frames.
    pub corner_radius: f32,
    /// Gap between input rect and painted state stroke.
    pub border_pad: f32,
    /// Minimum interactive height of an input.
    pub min_height: f32,
    /// Side length of the painted error icon.
    pub error_icon_size: f32,
    /// Body text size for painter-drawn text.
    pub text_size: f32,
}

/// Tier 3 — widget-specific tokens (where each role is used).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComponentTokens {
    pub canvas: CanvasTokens,
    pub panel: PanelTokens,
    pub text: ComponentTextTokens,
    pub palette: PaletteTokens,
    pub button: ButtonTokens,
    pub field: FieldTokens,
}

/// The whole token system grouped by tier (egui-pop `ThemeSpec` pattern).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeSpec {
    /// Tier 1 — raw colour palette.
    pub base: BasePalette,
    /// Tier 2 — meaning-based roles.
    pub semantic: SemanticRoles,
    /// Tier 3 — widget-specific tokens.
    pub component: ComponentTokens,
}

impl ThemeSpec {
    /// The light drafting-room spec — the only palette Lintel ships today.
    /// Every field initialises from the flat registry above; nothing here
    /// constructs a colour (tier 1 stays the only constructor site).
    pub const LIGHT: ThemeSpec = ThemeSpec {
        base: BasePalette {
            paper: BASE_PAPER,
            ink: BASE_INK,
            black: BASE_BLACK,
            graphite_muted: BASE_GRAPHITE_MUTED,
            graphite_hint: BASE_GRAPHITE_HINT,
            graphite_disabled: BASE_GRAPHITE_DISABLED,
            graphite_secondary: BASE_GRAPHITE_SECONDARY,
            match_blue: BASE_MATCH_BLUE,
            cobalt: BASE_COBALT,
            cobalt_hover: BASE_COBALT_HOVER,
            green: BASE_GREEN,
            valid: BASE_VALID,
            error: BASE_ERROR,
            danger: BASE_DANGER,
            danger_hover: BASE_DANGER_HOVER,
            warning: BASE_WARNING,
            panel_bg: BASE_PANEL_BG,
            panel_border: BASE_PANEL_BORDER,
            status_bg: BASE_STATUS_BG,
            wall_fill: BASE_WALL_FILL,
            wall_stroke: BASE_WALL_STROKE,
            wall_pending: BASE_WALL_PENDING,
            grid_minor: BASE_GRID_MINOR,
            grid_major: BASE_GRID_MAJOR,
            window_line: BASE_WINDOW_LINE,
            selection_wash: BASE_SELECTION_WASH,
            selection_edge: BASE_SELECTION_EDGE,
            transparent: BASE_TRANSPARENT,
            snap_midpoint: BASE_SNAP_MIDPOINT,
            snap_perpendicular: BASE_SNAP_PERPENDICULAR,
            snap_tangent: BASE_SNAP_TANGENT,
            snap_ortho: BASE_SNAP_ORTHO,
            snap_grid: BASE_SNAP_GRID,
            construction: BASE_CONSTRUCTION,
            block_highlight: BASE_BLOCK_HIGHLIGHT,
        },
        semantic: SemanticRoles {
            surface_base: SURFACE_BASE,
            surface_panel: SURFACE_PANEL,
            surface_status: SURFACE_STATUS,
            surface_selection: SURFACE_SELECTION,
            surface_transparent: SURFACE_TRANSPARENT,
            border_default: BORDER_DEFAULT,
            border_selection: BORDER_SELECTION,
            text_primary: TEXT_PRIMARY,
            text_secondary: TEXT_SECONDARY,
            text_muted: TEXT_MUTED,
            text_hint: TEXT_HINT,
            text_strong: TEXT_STRONG,
            text_disabled: TEXT_DISABLED,
            text_match: TEXT_MATCH,
            text_success: TEXT_SUCCESS,
            text_warning: TEXT_WARNING,
            text_error: TEXT_ERROR,
            text_danger: TEXT_DANGER,
            text_on_accent: TEXT_ON_ACCENT,
            accent_default: ACCENT_DEFAULT,
            accent_hover: ACCENT_HOVER,
            accent_active: ACCENT_ACTIVE,
            focus_ring: FOCUS_RING,
            success_default: SUCCESS_DEFAULT,
            danger_default: DANGER_DEFAULT,
            danger_hover: DANGER_HOVER,
            stroke_strong: STROKE_STRONG,
            block_highlight: BLOCK_HIGHLIGHT,
            snap_midpoint: SNAP_MIDPOINT,
            snap_perpendicular: SNAP_PERPENDICULAR,
            snap_tangent: SNAP_TANGENT,
            snap_ortho: SNAP_ORTHO,
            snap_grid: SNAP_GRID,
            construction_stroke: CONSTRUCTION_STROKE,
            draft_fill: DRAFT_FILL,
            draft_stroke: DRAFT_STROKE,
            draft_pending: DRAFT_PENDING,
            grid_line_minor: GRID_LINE_MINOR,
            grid_line_major: GRID_LINE_MAJOR,
            opening_cut: OPENING_CUT,
            opening_leaf: OPENING_LEAF,
            opening_glazing: OPENING_GLAZING,
        },
        component: ComponentTokens {
            canvas: CanvasTokens {
                sheet: CANVAS_SHEET,
                text: CANVAS_TEXT,
                wall_fill: WALL_FILL,
                wall_stroke: WALL_STROKE,
                wall_pending: WALL_PENDING,
                grid_minor: GRID_MINOR,
                grid_major: GRID_MAJOR,
                opening_gap: OPENING_GAP,
                door_leaf: DOOR_LEAF,
                window_line: WINDOW_LINE,
                construction_stroke: CANVAS_CONSTRUCTION,
                stroke_none: STROKE_NONE,
                selection: SELECTION_HIGHLIGHT,
                selection_window: SELECTION_WINDOW,
                selection_crossing: SELECTION_CROSSING,
                dimension_stroke: DIMENSION_STROKE,
                block_instance_stroke: BLOCK_INSTANCE_STROKE,
            },
            panel: PanelTokens {
                surface: PANEL_SURFACE,
                status: STATUS_SURFACE,
                border: PANEL_BORDER,
            },
            text: ComponentTextTokens {
                primary: BODY_TEXT,
                hint: HINT_TEXT,
                muted: MUTED_TEXT,
                success: SUCCESS_TEXT,
                error: ERROR_TEXT,
                danger: DANGER_TEXT,
            },
            palette: PaletteTokens {
                selected_fill: PALETTE_SELECTED_FILL,
                unselected_fill: PALETTE_UNSELECTED_FILL,
                selected_stroke: PALETTE_SELECTED_STROKE,
                hit: PALETTE_HIT,
                label: PALETTE_LABEL,
                disabled: PALETTE_DISABLED,
                recent: PALETTE_RECENT,
                shortcut: PALETTE_SHORTCUT,
                category: PALETTE_CATEGORY,
            },
            button: ButtonTokens {
                primary_sm: BUTTON_PRIMARY_S,
                primary_md: BUTTON_PRIMARY_M,
                primary_lg: BUTTON_PRIMARY_L,
                secondary_sm: BUTTON_SECONDARY_S,
                secondary_md: BUTTON_SECONDARY_M,
                secondary_lg: BUTTON_SECONDARY_L,
                ghost_sm: BUTTON_GHOST_S,
                ghost_md: BUTTON_GHOST_M,
                ghost_lg: BUTTON_GHOST_L,
                danger_sm: BUTTON_DANGER_S,
                danger_md: BUTTON_DANGER_M,
                danger_lg: BUTTON_DANGER_L,
                ring: FOCUS_RING,
                corner_radius: BUTTON_CORNER_RADIUS,
            },
            field: FieldTokens {
                label: FIELD_LABEL,
                legend: FIELD_LEGEND,
                help: FIELD_HELP,
                error_text: FIELD_ERROR_TEXT,
                error_icon: FIELD_ERROR_ICON,
                input_fill: FIELD_INPUT_FILL,
                input_text: FIELD_INPUT_TEXT,
                border: FIELD_BORDER,
                border_error: FIELD_BORDER_ERROR,
                border_focused: FIELD_BORDER_FOCUSED,
                disabled_text: FIELD_DISABLED_TEXT,
                group_fill: FIELD_GROUP_FILL,
                group_border: FIELD_GROUP_BORDER,
                row_spacing: FIELD_ROW_SPACING,
                message_gap: FIELD_MESSAGE_GAP,
                set_spacing: FIELD_SET_SPACING,
                group_spacing: FIELD_GROUP_SPACING,
                corner_radius: FIELD_CORNER_RADIUS,
                border_pad: FIELD_BORDER_PAD,
                min_height: FIELD_MIN_HEIGHT,
                error_icon_size: FIELD_ERROR_ICON_SIZE,
                text_size: FIELD_TEXT_SIZE,
            },
        },
    };
}

/// The spec every widget reads: `SPEC.component.<group>.<token>`.
pub const SPEC: ThemeSpec = ThemeSpec::LIGHT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_colours_meet_wcag_aa() {
        let sheet = rgb_of(SURFACE_BASE);
        let panel = rgb_of(SURFACE_PANEL);
        let status_bg = rgb_of(SURFACE_STATUS);

        // Normal text (>= 4.5:1) on every surface it sits on.
        let ink = rgb_of(TEXT_PRIMARY);
        let muted = rgb_of(TEXT_MUTED);
        assert!(
            contrast_ratio(ink, sheet) >= 4.5,
            "TEXT_PRIMARY on SURFACE_BASE is {:.2}:1",
            contrast_ratio(ink, sheet)
        );
        assert!(
            contrast_ratio(ink, panel) >= 4.5,
            "TEXT_PRIMARY on SURFACE_PANEL is {:.2}:1",
            contrast_ratio(ink, panel)
        );
        assert!(
            contrast_ratio(muted, sheet) >= 4.5,
            "TEXT_MUTED on SURFACE_BASE is {:.2}:1",
            contrast_ratio(muted, sheet)
        );
        // TEXT_MUTED was corrected from (110,110,110) — ~4.49:1 on the panel
        // surface, just under AA — to (102,102,102), 5.1:1 on both light
        // surfaces.
        assert!(
            contrast_ratio(muted, panel) >= 4.5,
            "TEXT_MUTED on SURFACE_PANEL is {:.2}:1",
            contrast_ratio(muted, panel)
        );

        // Hint text (small, secondary) on the surfaces it appears on.
        let hint = rgb_of(TEXT_HINT);
        assert!(
            contrast_ratio(hint, sheet) >= 4.5,
            "TEXT_HINT on SURFACE_BASE is {:.2}:1",
            contrast_ratio(hint, sheet)
        );
        assert!(
            contrast_ratio(hint, panel) >= 4.5,
            "TEXT_HINT on SURFACE_PANEL is {:.2}:1",
            contrast_ratio(hint, panel)
        );
        assert!(
            contrast_ratio(hint, status_bg) >= 4.5,
            "TEXT_HINT on SURFACE_STATUS is {:.2}:1",
            contrast_ratio(hint, status_bg)
        );

        // Status message colours on the palette/panel surfaces.
        let ok = rgb_of(TEXT_SUCCESS);
        let err = rgb_of(TEXT_ERROR);
        let warn = rgb_of(TEXT_WARNING);
        for (name, col) in [("TEXT_SUCCESS", ok), ("TEXT_ERROR", err), ("TEXT_WARNING", warn)] {
            assert!(
                contrast_ratio(col, sheet) >= 4.5,
                "{name} on SURFACE_BASE is {:.2}:1",
                contrast_ratio(col, sheet)
            );
            assert!(
                contrast_ratio(col, panel) >= 4.5,
                "{name} on SURFACE_PANEL is {:.2}:1",
                contrast_ratio(col, panel)
            );
        }

        // Command palette row colours on the window background (SURFACE_BASE).
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
                contrast_ratio(col, sheet) >= 3.0,
                "{name} on SURFACE_BASE is {:.2}:1",
                contrast_ratio(col, sheet)
            );
        }

        // Meaningful UI boundaries / focus rings / accents (>= 3:1).
        let accent = rgb_of(ACCENT_DEFAULT);
        let success = rgb_of(SUCCESS_DEFAULT);
        assert!(
            contrast_ratio(accent, sheet) >= 3.0,
            "ACCENT_DEFAULT on SURFACE_BASE is {:.2}:1",
            contrast_ratio(accent, sheet)
        );
        assert!(
            contrast_ratio(accent, panel) >= 3.0,
            "ACCENT_DEFAULT on SURFACE_PANEL is {:.2}:1",
            contrast_ratio(accent, panel)
        );
        assert!(
            contrast_ratio(success, sheet) >= 3.0,
            "SUCCESS_DEFAULT on SURFACE_BASE is {:.2}:1",
            contrast_ratio(success, sheet)
        );
        assert!(
            contrast_ratio(success, panel) >= 3.0,
            "SUCCESS_DEFAULT on SURFACE_PANEL is {:.2}:1",
            contrast_ratio(success, panel)
        );

        // The selected-block-instance outline is a meaningful UI boundary on
        // the sheet (guide II.30.7.1, 3:1 floor).
        let block = rgb_of(BLOCK_INSTANCE_STROKE);
        assert!(
            contrast_ratio(block, sheet) >= 3.0,
            "BLOCK_INSTANCE_STROKE on SURFACE_BASE is {:.2}:1",
            contrast_ratio(block, sheet)
        );

        // Snap markers are UI graphics on the sheet (guide II.30.7.1, 3:1 floor).
        let paper = rgb_of(SURFACE_BASE);
        for (name, col) in [
            ("SNAP_MIDPOINT", rgb_of(SNAP_MIDPOINT)),
            ("SNAP_PERPENDICULAR", rgb_of(SNAP_PERPENDICULAR)),
            ("SNAP_TANGENT", rgb_of(SNAP_TANGENT)),
            ("SNAP_ORTHO", rgb_of(SNAP_ORTHO)),
            ("SNAP_GRID", rgb_of(SNAP_GRID)),
            ("CONSTRUCTION_STROKE", rgb_of(CONSTRUCTION_STROKE)),
        ] {
            assert!(
                contrast_ratio(col, paper) >= 3.0,
                "{name} on SURFACE_BASE is {:.2}:1",
                contrast_ratio(col, paper)
            );
        }
    }

    #[test]
    fn tier_two_roles_resolve_to_base_swatches() {
        // Tier 2 must never invent a colour: every role is one hop from the
        // base palette (guide II.30.6, one token source).
        let pairs = [
            ("SURFACE_BASE", SURFACE_BASE, BASE_PAPER),
            ("SURFACE_PANEL", SURFACE_PANEL, BASE_PANEL_BG),
            ("SURFACE_STATUS", SURFACE_STATUS, BASE_STATUS_BG),
            ("SURFACE_TRANSPARENT", SURFACE_TRANSPARENT, BASE_TRANSPARENT),
            ("SURFACE_SELECTION", SURFACE_SELECTION, BASE_SELECTION_WASH),
            ("BORDER_DEFAULT", BORDER_DEFAULT, BASE_PANEL_BORDER),
            ("BORDER_SELECTION", BORDER_SELECTION, BASE_SELECTION_EDGE),
            ("BORDER_STRONG", BORDER_STRONG, BASE_WALL_STROKE),
            ("TEXT_PRIMARY", TEXT_PRIMARY, BASE_INK),
            ("TEXT_MUTED", TEXT_MUTED, BASE_GRAPHITE_MUTED),
            ("TEXT_HINT", TEXT_HINT, BASE_GRAPHITE_HINT),
            ("TEXT_STRONG", TEXT_STRONG, BASE_BLACK),
            ("TEXT_DISABLED", TEXT_DISABLED, BASE_GRAPHITE_DISABLED),
            ("TEXT_SECONDARY", TEXT_SECONDARY, BASE_GRAPHITE_SECONDARY),
            ("TEXT_MATCH", TEXT_MATCH, BASE_MATCH_BLUE),
            ("TEXT_SUCCESS", TEXT_SUCCESS, BASE_VALID),
            ("TEXT_ERROR", TEXT_ERROR, BASE_ERROR),
            ("TEXT_WARNING", TEXT_WARNING, BASE_WARNING),
            ("ACCENT_DEFAULT", ACCENT_DEFAULT, BASE_COBALT),
            ("ACCENT_HOVER", ACCENT_HOVER, BASE_COBALT_HOVER),
            ("ACCENT_ACTIVE", ACCENT_ACTIVE, BASE_INK),
            ("FOCUS_RING", FOCUS_RING, ACCENT_DEFAULT),
            ("SUCCESS_DEFAULT", SUCCESS_DEFAULT, BASE_GREEN),
            ("DANGER_DEFAULT", DANGER_DEFAULT, BASE_ERROR),
            ("DANGER_HOVER", DANGER_HOVER, BASE_DANGER_HOVER),
            ("TEXT_ON_ACCENT", TEXT_ON_ACCENT, BASE_PAPER),
            ("STROKE_STRONG", STROKE_STRONG, BASE_INK),
            ("DRAFT_FILL", DRAFT_FILL, BASE_WALL_FILL),
            ("DRAFT_STROKE", DRAFT_STROKE, BASE_WALL_STROKE),
            ("DRAFT_PENDING", DRAFT_PENDING, BASE_WALL_PENDING),
            ("GRID_LINE_MINOR", GRID_LINE_MINOR, BASE_GRID_MINOR),
            ("GRID_LINE_MAJOR", GRID_LINE_MAJOR, BASE_GRID_MAJOR),
            ("OPENING_CUT", OPENING_CUT, SURFACE_BASE),
            ("OPENING_LEAF", OPENING_LEAF, TEXT_PRIMARY),
            ("OPENING_GLAZING", OPENING_GLAZING, BASE_WINDOW_LINE),
            ("SNAP_MIDPOINT", SNAP_MIDPOINT, BASE_SNAP_MIDPOINT),
            (
                "SNAP_PERPENDICULAR",
                SNAP_PERPENDICULAR,
                BASE_SNAP_PERPENDICULAR,
            ),
            ("SNAP_TANGENT", SNAP_TANGENT, BASE_SNAP_TANGENT),
            ("SNAP_ORTHO", SNAP_ORTHO, BASE_SNAP_ORTHO),
            ("SNAP_GRID", SNAP_GRID, BASE_SNAP_GRID),
            (
                "CONSTRUCTION_STROKE",
                CONSTRUCTION_STROKE,
                BASE_CONSTRUCTION,
            ),
        ];
        for (name, role, expected) in pairs {
            assert_eq!(role, expected, "{name} must resolve to its base swatch");
        }
    }

    #[test]
    fn tier_three_components_resolve_to_roles() {
        // Tier 3 must never hard-code a colour either: components compose the
        // semantic roles above them.
        let pairs = [
            ("WALL_FILL", WALL_FILL, DRAFT_FILL),
            ("WALL_STROKE", WALL_STROKE, DRAFT_STROKE),
            ("WALL_PENDING", WALL_PENDING, DRAFT_PENDING),
            ("GRID_MINOR", GRID_MINOR, GRID_LINE_MINOR),
            ("GRID_MAJOR", GRID_MAJOR, GRID_LINE_MAJOR),
            ("OPENING_GAP", OPENING_GAP, OPENING_CUT),
            ("DOOR_LEAF", DOOR_LEAF, OPENING_LEAF),
            ("WINDOW_LINE", WINDOW_LINE, OPENING_GLAZING),
            (
                "PALETTE_SELECTED_FILL",
                PALETTE_SELECTED_FILL,
                SURFACE_SELECTION,
            ),
            (
                "PALETTE_SELECTED_STROKE",
                PALETTE_SELECTED_STROKE,
                BORDER_SELECTION,
            ),
            ("PALETTE_HIT", PALETTE_HIT, TEXT_MATCH),
            ("PALETTE_LABEL", PALETTE_LABEL, TEXT_STRONG),
            ("PALETTE_DISABLED", PALETTE_DISABLED, TEXT_DISABLED),
            ("PALETTE_RECENT", PALETTE_RECENT, TEXT_SECONDARY),
            ("PALETTE_SHORTCUT", PALETTE_SHORTCUT, TEXT_MUTED),
            ("PALETTE_CATEGORY", PALETTE_CATEGORY, TEXT_DISABLED),
            (
                "BUTTON_PRIMARY_S.fill",
                BUTTON_PRIMARY_S.fill,
                ACCENT_DEFAULT,
            ),
            (
                "BUTTON_PRIMARY_M.fill",
                BUTTON_PRIMARY_M.fill,
                ACCENT_DEFAULT,
            ),
            (
                "BUTTON_PRIMARY_L.fill",
                BUTTON_PRIMARY_L.fill,
                ACCENT_DEFAULT,
            ),
            (
                "BUTTON_PRIMARY_M.hover",
                BUTTON_PRIMARY_M.hover,
                ACCENT_HOVER,
            ),
            (
                "BUTTON_PRIMARY_M.active",
                BUTTON_PRIMARY_M.active,
                ACCENT_ACTIVE,
            ),
            (
                "BUTTON_SECONDARY_M.fill",
                BUTTON_SECONDARY_M.fill,
                SURFACE_PANEL,
            ),
            (
                "BUTTON_SECONDARY_M.hover",
                BUTTON_SECONDARY_M.hover,
                SURFACE_BASE,
            ),
            (
                "BUTTON_GHOST_S.fill",
                BUTTON_GHOST_S.fill,
                SURFACE_TRANSPARENT,
            ),
            ("BUTTON_GHOST_L.hover", BUTTON_GHOST_L.hover, SURFACE_PANEL),
            ("BUTTON_DANGER_S.fill", BUTTON_DANGER_S.fill, DANGER_DEFAULT),
            ("BUTTON_DANGER_L.fill", BUTTON_DANGER_L.fill, DANGER_DEFAULT),
            ("BUTTON_PRIMARY_S.focus", BUTTON_PRIMARY_S.focus, FOCUS_RING),
            ("BUTTON_DANGER_L.focus", BUTTON_DANGER_L.focus, FOCUS_RING),
            (
                "BUTTON_PRIMARY_S.text",
                BUTTON_PRIMARY_S.text,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_PRIMARY_L.text",
                BUTTON_PRIMARY_L.text,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_SECONDARY_M.text",
                BUTTON_SECONDARY_M.text,
                TEXT_PRIMARY,
            ),
            ("BUTTON_GHOST_S.text", BUTTON_GHOST_S.text, TEXT_PRIMARY),
            ("BUTTON_DANGER_M.text", BUTTON_DANGER_M.text, TEXT_ON_ACCENT),
            ("BUTTON_DANGER_M.hover", BUTTON_DANGER_M.hover, DANGER_HOVER),
            (
                "BUTTON_PRIMARY_L.text_active",
                BUTTON_PRIMARY_L.text_active,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_SECONDARY_S.text_disabled",
                BUTTON_SECONDARY_S.text_disabled,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_GHOST_M.text_disabled",
                BUTTON_GHOST_M.text_disabled,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_DANGER_L.text_disabled",
                BUTTON_DANGER_L.text_disabled,
                TEXT_ON_ACCENT,
            ),
            (
                "BUTTON_PRIMARY_S.disabled",
                BUTTON_PRIMARY_S.disabled,
                TEXT_MUTED,
            ),
            ("SNAP_MIDPOINT", SNAP_MIDPOINT, BASE_SNAP_MIDPOINT),
            (
                "SNAP_PERPENDICULAR",
                SNAP_PERPENDICULAR,
                BASE_SNAP_PERPENDICULAR,
            ),
            ("SNAP_TANGENT", SNAP_TANGENT, BASE_SNAP_TANGENT),
            ("SNAP_ORTHO", SNAP_ORTHO, BASE_SNAP_ORTHO),
            ("SNAP_GRID", SNAP_GRID, BASE_SNAP_GRID),
            (
                "CONSTRUCTION_STROKE",
                CONSTRUCTION_STROKE,
                BASE_CONSTRUCTION,
            ),
            (
                "selection_stroke colour",
                selection_stroke().color,
                ACCENT_DEFAULT,
            ),
            // Field tokens (T-002)
            ("FIELD_LABEL", FIELD_LABEL, TEXT_PRIMARY),
            ("FIELD_LEGEND", FIELD_LEGEND, TEXT_STRONG),
            ("FIELD_HELP", FIELD_HELP, TEXT_MUTED),
            ("FIELD_ERROR_TEXT", FIELD_ERROR_TEXT, TEXT_ERROR),
            ("FIELD_ERROR_ICON", FIELD_ERROR_ICON, DANGER_DEFAULT),
            ("FIELD_INPUT_FILL", FIELD_INPUT_FILL, SURFACE_BASE),
            ("FIELD_INPUT_TEXT", FIELD_INPUT_TEXT, TEXT_PRIMARY),
            ("FIELD_BORDER", FIELD_BORDER, BORDER_STRONG),
            ("FIELD_BORDER_ERROR", FIELD_BORDER_ERROR, DANGER_DEFAULT),
            ("FIELD_BORDER_FOCUSED", FIELD_BORDER_FOCUSED, FOCUS_RING),
            ("FIELD_DISABLED_TEXT", FIELD_DISABLED_TEXT, TEXT_DISABLED),
            ("FIELD_GROUP_FILL", FIELD_GROUP_FILL, SURFACE_PANEL),
            ("FIELD_GROUP_BORDER", FIELD_GROUP_BORDER, BORDER_STRONG),
        ];
        for (name, component, expected) in pairs {
            assert_eq!(component, expected, "{name} must resolve to its role");
        }
    }

    #[test]
    fn spec_groups_the_three_tiers() {
        // The ThemeSpec is the grouped view of the same registry (T-006):
        // every tier subgroup must carry the flat constants, never a second
        // copy of a value. The deep field-by-field completeness check lives
        // in `tests/theme_tiers.rs`; this pins the grouping itself.
        assert_eq!(SPEC, ThemeSpec::LIGHT);
        assert_eq!(SPEC.base.paper, BASE_PAPER);
        assert_eq!(SPEC.semantic.surface_base, SURFACE_BASE);
        assert_eq!(SPEC.component.canvas.wall_fill, WALL_FILL);
        assert_eq!(SPEC.component.button.primary_sm, BUTTON_PRIMARY_S);
        assert_eq!(SPEC.component.button.corner_radius, BUTTON_CORNER_RADIUS);
        assert_eq!(SPEC.component.field.label, FIELD_LABEL);
        assert_eq!(SPEC.component.field.corner_radius, FIELD_CORNER_RADIUS);
    }

    #[test]
    fn contrast_maths_matches_wcag_reference_points() {
        let black = rgb_of(Color32::BLACK);
        let white = rgb_of(Color32::WHITE);
        assert!(relative_luminance(black).abs() < 1e-9);
        assert!((relative_luminance(white) - 1.0).abs() < 1e-9);
        assert!((contrast_ratio(white, white) - 1.0).abs() < 1e-9);
        assert!((contrast_ratio(black, white) - 21.0).abs() < 1e-6);
    }
}
