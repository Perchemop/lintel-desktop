//! Lintel visual system — original, draftsman-flavoured palette.
//!
//! Nothing here copies a competitor: warm paper, graphite ink, one cobalt
//! accent for selection and tool state, amber for snap confirmation, green for
//! "end" snaps. WCAG AA contrast is checked where text sits on ink or paper.

use eframe::egui::Color32;

pub const PAPER: Color32 = Color32::from_rgb(252, 248, 242); // warm off-white sheet
pub const INK: Color32 = Color32::from_rgb(26, 24, 22); // near-black graphite

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

pub fn selection_stroke() -> egui::Stroke {
    egui::Stroke::new(2.0, COBALT)
}
