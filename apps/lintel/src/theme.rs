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
//! Re-hueing the product is a tier-1 edit; re-styling one component is a tier-3
//! edit; nothing in between needs to move.

use eframe::egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions};

// ---------------------------------------------------------------------------
// Tier 1 — base palette (raw swatches, no meaning attached)
// ---------------------------------------------------------------------------

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
pub const BASE_SELECTION_EDGE: Color32 = Color32::from_black_alpha(40);
pub const BASE_TRANSPARENT: Color32 = Color32::TRANSPARENT;

pub const BASE_SNAP_MIDPOINT: Color32 = Color32::from_rgb(176, 106, 22);
pub const BASE_SNAP_PERPENDICULAR: Color32 = Color32::from_rgb(120, 74, 160);
pub const BASE_SNAP_TANGENT: Color32 = Color32::from_rgb(24, 128, 140);
pub const BASE_SNAP_ORTHO: Color32 = Color32::from_rgb(90, 82, 74);
pub const BASE_SNAP_GRID: Color32 = Color32::from_rgb(130, 130, 130);
/// Construction-geometry stroke. Darkened from (168,148,116) — 2.77:1 on
/// PAPER, below the 3:1 UI-graphics floor — to 4.0:1 (guide II.30.7.1).
pub const BASE_CONSTRUCTION: Color32 = Color32::from_rgb(140, 120, 90);
// ---------------------------------------------------------------------------
// Tier 2 — semantic roles (what a colour means)
// ---------------------------------------------------------------------------

// Surfaces.
/// Canvas sheet and app background.
pub const SURFACE_BASE: Color32 = BASE_PAPER;
/// Inspector / toolbar panel fill.
#[allow(dead_code)]
pub const SURFACE_PANEL: Color32 = BASE_PANEL_BG;
/// Status-bar strip fill.
#[allow(dead_code)]
pub const SURFACE_STATUS: Color32 = BASE_STATUS_BG;
/// "Paint nothing" — the fill of a widget that has no background.
pub const SURFACE_TRANSPARENT: Color32 = BASE_TRANSPARENT;
/// Selected-row wash in the command palette.
pub const SURFACE_SELECTION: Color32 = BASE_SELECTION_WASH;

// Borders.
/// Panel divider.
#[allow(dead_code)]
pub const BORDER_DEFAULT: Color32 = BASE_PANEL_BORDER;
/// Outline of a selected row.
pub const BORDER_SELECTION: Color32 = BASE_SELECTION_EDGE;

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
pub const TEXT_DANGER: Color32 = BASE_ERROR;

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

// ---------------------------------------------------------------------------
// Tier 3 — component tokens (where a role is used)
// ---------------------------------------------------------------------------

// Canvas geometry: walls, grid and openings.
pub const WALL_FILL: Color32 = DRAFT_FILL;
pub const WALL_STROKE: Color32 = DRAFT_STROKE;
#[allow(dead_code)]
pub const WALL_DRAFT: Color32 = DRAFT_PENDING;
pub const GRID_MINOR: Color32 = GRID_LINE_MINOR;
pub const GRID_MAJOR: Color32 = GRID_LINE_MAJOR;
pub const OPENING_GAP: Color32 = OPENING_CUT;
pub const DOOR_LEAF: Color32 = OPENING_LEAF;
pub const WINDOW_LINE: Color32 = OPENING_GLAZING;

// Command palette: a command row and its parts.
pub const PALETTE_SELECTED_FILL: Color32 = SURFACE_SELECTION;
pub const PALETTE_SELECTED_STROKE: Color32 = BORDER_SELECTION;
pub const PALETTE_HIT: Color32 = TEXT_MATCH; // matched search text
pub const PALETTE_LABEL: Color32 = TEXT_STRONG; // enabled command label
pub const PALETTE_DISABLED: Color32 = TEXT_DISABLED; // 5.2:1 on the sheet
pub const PALETTE_RECENT: Color32 = TEXT_SECONDARY; // 5.6:1 on the sheet
pub const PALETTE_SHORTCUT: Color32 = TEXT_MUTED;
pub const PALETTE_CATEGORY: Color32 = TEXT_DISABLED; // 5.2:1 on the sheet

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

/// Button corner radius (design-token radii 2/4/8/12 — guide II.30.6).
pub const BUTTON_CORNER_RADIUS: f32 = 4.0;

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
    offset: egui::Vec2::new(0.0, 2.0),
    blur: 4.0,
    spread: 0.0,
};
/// Level 2: medium lift (dropdowns, tooltips).
pub const SHADOW_LEVEL_2: egui::Shadow = egui::Shadow {
    color: Color32::from_black_alpha(35),
    offset: egui::Vec2::new(0.0, 6.0),
    blur: 12.0,
    spread: 0.0,
};
/// Level 3: high lift (modals, overlays).
pub const SHADOW_LEVEL_3: egui::Shadow = egui::Shadow {
    color: Color32::from_black_alpha(50),
    offset: egui::Vec2::new(0.0, 16.0),
    blur: 24.0,
    spread: 0.0,
};

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
        let err = rgb_of(TEXT_DANGER);
        for (name, col) in [("TEXT_SUCCESS", ok), ("TEXT_DANGER", err)] {
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
            ("TEXT_PRIMARY", TEXT_PRIMARY, BASE_INK),
            ("TEXT_MUTED", TEXT_MUTED, BASE_GRAPHITE_MUTED),
            ("TEXT_HINT", TEXT_HINT, BASE_GRAPHITE_HINT),
            ("TEXT_STRONG", TEXT_STRONG, BASE_BLACK),
            ("TEXT_DISABLED", TEXT_DISABLED, BASE_GRAPHITE_DISABLED),
            ("TEXT_SECONDARY", TEXT_SECONDARY, BASE_GRAPHITE_SECONDARY),
            ("TEXT_MATCH", TEXT_MATCH, BASE_MATCH_BLUE),
            ("TEXT_SUCCESS", TEXT_SUCCESS, BASE_VALID),
            ("TEXT_DANGER", TEXT_DANGER, BASE_ERROR),
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
            ("WALL_DRAFT", WALL_DRAFT, DRAFT_PENDING),
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
        ];
        for (name, component, expected) in pairs {
            assert_eq!(component, expected, "{name} must resolve to its role");
        }
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
