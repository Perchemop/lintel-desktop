//! Lintel visual system — original, draftsman-flavoured palette.
//!
//! Nothing here copies a competitor: warm paper, graphite ink, one cobalt
//! accent for selection and tool state, amber for snap confirmation, green for
//! "end" snaps. WCAG AA contrast is checked where text sits on ink or paper.

use eframe::egui::Color32;

pub const PAPER: Color32 = Color32::from_rgb(252, 248, 242); // warm off-white sheet
pub const INK: Color32 = Color32::from_rgb(26, 24, 22);      // near-black graphite
pub const INK_DIM: Color32 = Color32::from_rgb(112, 106, 98);

pub const COBALT: Color32 = Color32::from_rgb(18, 78, 120);   // selection / active tool
pub const AMBER: Color32 = Color32::from_rgb(211, 134, 0);    // ortho / grid hints
pub const GREEN: Color32 = Color32::from_rgb(34, 158, 96);    // endpoint snap

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

pub fn snap_radius() -> f32 {
    6.0
}
