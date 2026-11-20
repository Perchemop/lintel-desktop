//! Icon widget over [`crate::theme::icon_texture`] — original SVG assets
//! rasterized at exact pixel size (spaceui pattern copy, ticket T-004,
//! spec v0.20.0 AC-05).
//!
//! Accessibility (guide II.30.7.4): an icon is never the only carrier of
//! meaning. Every icon carries a human-readable AccessKit label, and
//! [`Icon::icon_button`] attaches that label to the icon-only button through
//! [`egui::Response::labelled_by`], so a screen reader announces
//! "Snap to endpoint", never a bare "button".

// The widget API ships ahead of the shell slice (T-005) that wires the
// toolbar; until then the unit tests in `src/tests/icons.rs` are the only
// callers, so silence dead-code for the module as a whole.
#![allow(dead_code)]

use eframe::egui;
use eframe::egui::load::SizedTexture;

/// Default icon edge in points — the toolbar's icon size.
pub const DEFAULT_ICON_SIZE_PT: f32 = 20.0;

/// The human-readable name a screen reader announces for an icon.
pub fn label_of(name: &str) -> &'static str {
    match name {
        "snap_endpoint" => "Snap to endpoint",
        "snap_midpoint" => "Snap to midpoint",
        "snap_center" => "Snap to centre",
        "snap_intersection" => "Snap to intersection",
        "snap_grid" => "Snap to grid",
        "snap_ortho" => "Orthogonal lock",
        "layer_visible" => "Layer visibility",
        "layer_lock" => "Layer lock",
        "layer_print" => "Layer print setting",
        "dimension_aligned" => "Aligned dimension",
        "dimension_radial" => "Radial dimension",
        "block_insert" => "Insert block",
        _ => "Unknown icon",
    }
}

/// An icon rendered from `assets/icons/<name>.svg`.
///
/// Rasterization happens at `size * pixels_per_point` physical pixels (see
/// [`crate::theme::icon_texture`]), so the icon is painted one texel per
/// screen pixel and stays crisp at 16, 20, 24 and 32 pt on any display scale.
#[derive(Clone, Copy, Debug)]
pub struct Icon {
    name: &'static str,
    size: f32,
}

impl Icon {
    /// Create an icon at the default toolbar size.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            size: DEFAULT_ICON_SIZE_PT,
        }
    }

    /// Set the icon edge in points.
    pub fn size(mut self, pt: f32) -> Self {
        self.size = pt;
        self
    }

    /// The human-readable AccessKit label for this icon.
    pub fn label(&self) -> &'static str {
        label_of(self.name)
    }

    /// Paint the icon.
    ///
    /// The widget carries the AccessKit label, and the returned
    /// [`egui::Response`] id can be handed to
    /// [`egui::Response::labelled_by`] by an owning control.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let size = egui::vec2(self.size, self.size);
        let texture = crate::theme::icon_texture(ui.ctx(), self.name, self.size);
        ui.add(
            egui::Image::from_texture(SizedTexture::new(texture.id(), size))
                .fit_to_exact_size(size)
                .alt_text(self.label()),
        )
    }

    /// Icon-only button.
    ///
    /// Ghost role: no container until hover (guide II.30.10). The target is
    /// at least 24 pt on its shortest side (guide II.30.7.5). The AccessKit
    /// name arrives through [`egui::Response::labelled_by`] pointing at the
    /// icon's labelled node inside the button (guide II.30.7.4) — the icon is
    /// the button's name.
    pub fn icon_button(self, ui: &mut egui::Ui) -> egui::Response {
        let padding = ui.spacing().button_padding;
        let side = (self.size + 2.0 * padding.x).max(crate::theme::ICON_BUTTON_MIN_TARGET);
        let (rect, response) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::click());
        response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::Button));

        // Ghost chrome: transparent at rest, panel wash on hover, accent fill
        // while pressed. All colours are tier-3 component tokens.
        let fill = if response.is_pointer_button_down_on() {
            crate::theme::BUTTON_GHOST_S.active
        } else if response.hovered() {
            crate::theme::BUTTON_GHOST_S.hover
        } else {
            crate::theme::BUTTON_GHOST_S.fill
        };
        if fill != crate::theme::SURFACE_TRANSPARENT {
            ui.painter().rect_filled(
                rect,
                crate::theme::BUTTON_CORNER_RADIUS,
                fill,
            );
        }
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect,
                crate::theme::BUTTON_CORNER_RADIUS,
                egui::Stroke::new(2.0, crate::theme::FOCUS_RING),
                egui::StrokeKind::Inside,
            );
        }

        // The icon inside the button rect is a real, labelled widget: exactly
        // the node the button references for its accessible name.
        let icon_rect =
            egui::Rect::from_center_size(rect.center(), egui::vec2(self.size, self.size));
        let mut inner = ui.new_child(egui::UiBuilder::new().max_rect(icon_rect));
        let icon_response = self.show(&mut inner);
        response.labelled_by(icon_response.id)
    }
}
