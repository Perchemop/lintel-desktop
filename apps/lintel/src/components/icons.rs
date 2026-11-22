//! Lintel icon components — type-safe wrapper around `theme::icon_texture`.
//!
//! The rasterization and caching live in `theme.rs` (T-004, spec AC-05).
//! This module provides the `IconName` enum for compile-time safety,
//! convenience functions `show_icon`/`icon_button`, and AccessKit wiring.

#![allow(dead_code)] // icon component library pending shell adoption (v0.20.1); exercised by src/tests/icons.rs (D-034/D-035)

use crate::theme;
use eframe::egui::{self, Context, TextureHandle};

/// Canonical icon names (semantic, not visual). Use these in UI code.
/// Adding a new icon:
/// 1. Add `assets/icons/<name>.svg` (24×24 viewBox, stroke=#1A1816, 2px).
/// 2. Add an entry to `theme::ICON_NAMES` and `theme::ICON_SVGS`.
/// 3. Add the name to this enum if you want compile-time checking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconName {
    // Tools (T-004)
    Wall,
    Door,
    Window,
    Dimension,
    Label,
    Measurements,
    Select,
    ConstructionLine,
    ConstructionRay,
    ConstructionCircle,
    ConstructionArc,
    Block,
    // Snap / Layer / UI (pre-existing)
    SnapEndpoint,
    SnapMidpoint,
    SnapIntersection,
    SnapGrid,
    SnapCenter,
    SnapOrtho,
    SnapPerpendicular,
    SnapTangent,
    LayerVisible,
    LayerPrint,
    LayerLock,
    DimensionAligned,
    DimensionRadial,
    BlockInsert,
}

impl IconName {
    /// The semantic key used in `theme::ICON_SVGS` and `theme::icon_texture`.
    pub fn key(self) -> &'static str {
        match self {
            IconName::Wall => "wall",
            IconName::Door => "door",
            IconName::Window => "window",
            IconName::Dimension => "dimension",
            IconName::Label => "label",
            IconName::Measurements => "measurements",
            IconName::Select => "select",
            IconName::ConstructionLine => "construction_line",
            IconName::ConstructionRay => "construction_ray",
            IconName::ConstructionCircle => "construction_circle",
            IconName::ConstructionArc => "construction_arc",
            IconName::Block => "block",
            IconName::SnapEndpoint => "snap_endpoint",
            IconName::SnapMidpoint => "snap_midpoint",
            IconName::SnapIntersection => "snap_intersection",
            IconName::SnapGrid => "snap_grid",
            IconName::SnapCenter => "snap_center",
            IconName::SnapOrtho => "snap_ortho",
            IconName::SnapPerpendicular => "snap_perpendicular",
            IconName::SnapTangent => "snap_tangent",
            IconName::LayerVisible => "layer_visible",
            IconName::LayerPrint => "layer_print",
            IconName::LayerLock => "layer_lock",
            IconName::DimensionAligned => "dimension_aligned",
            IconName::DimensionRadial => "dimension_radial",
            IconName::BlockInsert => "block_insert",
        }
    }

    /// All known icon names (for iteration / testing).
    pub const fn all() -> &'static [IconName] {
        &[
            IconName::Wall,
            IconName::Door,
            IconName::Window,
            IconName::Dimension,
            IconName::Label,
            IconName::Measurements,
            IconName::Select,
            IconName::ConstructionLine,
            IconName::ConstructionRay,
            IconName::ConstructionCircle,
            IconName::ConstructionArc,
            IconName::Block,
            IconName::SnapEndpoint,
            IconName::SnapMidpoint,
            IconName::SnapIntersection,
            IconName::SnapGrid,
            IconName::SnapCenter,
            IconName::SnapOrtho,
            IconName::SnapPerpendicular,
            IconName::SnapTangent,
            IconName::LayerVisible,
            IconName::LayerPrint,
            IconName::LayerLock,
            IconName::DimensionAligned,
            IconName::DimensionRadial,
            IconName::BlockInsert,
        ]
    }
}

/// Get a texture handle for `icon` at `size_pt` points.
/// Delegates to `theme::icon_texture` which caches per (name, physical pixels).
pub fn icon_texture(ctx: &Context, icon: IconName, size_pt: f32) -> TextureHandle {
    theme::icon_texture(ctx, icon.key(), size_pt)
}

/// Convenience: draw an icon at `size_pt` inside a button or label.
/// Returns the widget response for interaction handling.
pub fn show_icon(ui: &mut egui::Ui, icon: IconName, size_pt: f32) -> egui::Response {
    let handle = icon_texture(ui.ctx(), icon, size_pt);
    let size = egui::Vec2::splat(size_pt);
    ui.add(egui::Image::new((handle.id(), size)).fit_to_exact_size(size))
}

/// Convenience: draw an icon button with accessible name for AccessKit (II.30.7.4).
/// Uses a standard `egui::Button` with an image; the accessible name is set via
/// `Response::widget_info` so screen readers announce it correctly.
pub fn icon_button(
    ui: &mut egui::Ui,
    icon: IconName,
    size_pt: f32,
    accessible_name: &str,
) -> egui::Response {
    let handle = icon_texture(ui.ctx(), icon, size_pt);
    let img_size = egui::Vec2::splat(size_pt);
    let img = egui::Image::new((handle.id(), img_size)).fit_to_exact_size(img_size);

    let resp = ui.add(egui::Button::image(img).min_size(img_size));

    // Set accessible name on the widget info so AccessKit reads it.
    // In egui 0.36, widget_info closure takes 0 args; use WidgetInfo::labeled.
    resp.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, accessible_name.to_owned())
    });

    resp.on_hover_text(accessible_name)
}

/// Preload all icons at common sizes (16, 20, 24, 32 pt) to avoid first-frame lag.
/// Call once at startup (e.g. from `LintelApp::new` after the context exists).
pub fn preload_all(ctx: &Context) {
    const SIZES: &[f32] = &[16.0, 20.0, 24.0, 32.0];
    for icon in IconName::all() {
        for &sz in SIZES {
            let _ = icon_texture(ctx, *icon, sz);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_icons_have_theme_entries() {
        for icon in IconName::all() {
            let key = icon.key();
            assert!(
                theme::ICON_NAMES.contains(&key),
                "icon '{key}' missing from theme::ICON_NAMES"
            );
        }
    }

    #[test]
    fn icon_names_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for icon in IconName::all() {
            assert!(
                seen.insert(icon.key()),
                "duplicate icon key: {}",
                icon.key()
            );
        }
    }

    #[test]
    fn all_tool_icons_present() {
        // The 12 tool icons required by T-004
        let tool_keys = [
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
        ];
        for key in tool_keys {
            assert!(
                theme::ICON_NAMES.contains(&key),
                "tool icon '{key}' missing from theme::ICON_NAMES"
            );
        }
    }
}
