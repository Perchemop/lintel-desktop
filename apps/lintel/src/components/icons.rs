//! Lintel icon system — original SVG icons rasterized at exact pixel size via
//! `resvg`/`tiny-skia`, exposed as `egui::TextureHandle` through a theme
//! helper. No Lucide/Tabler/Phosphor dependency (guide II.30.1, II.30.4, spec AC-05).
//!
//! Icons live as `assets/icons/*.svg` (compiled into the binary via
//! `include_str!`), are parsed once per size via `usvg`, rendered to a
//! `tiny_skia::Pixmap`, converted to `egui::ColorImage`, and uploaded as a
//! texture. The registry is a static map from semantic names to the embedded
//! SVG strings.

use crate::theme;
use eframe::egui::{self, ColorImage, Context, TextureHandle, TextureOptions};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Canonical icon names (semantic, not visual). Use these in UI code.
/// Adding a new icon:
/// 1. Add `assets/icons/<name>.svg` (24×24 viewBox, stroke=#1A1816, 2px).
/// 2. Add an entry to `ICON_SVGS` below.
/// 3. Add the name to `IconName` if you want compile-time checking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconName {
    // Tools
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
    /// The semantic key used in the registry map and texture names.
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

/// Embedded SVG source strings — compiled into the binary.
/// Keep alphabetical by key for easy scanning.
const ICON_SVGS: &[(&str, &str)] = &[
    ("block", include_str!("../../assets/icons/block.svg")),
    ("block_insert", include_str!("../../assets/icons/block_insert.svg")),
    ("construction_arc", include_str!("../../assets/icons/construction_arc.svg")),
    ("construction_circle", include_str!("../../assets/icons/construction_circle.svg")),
    ("construction_line", include_str!("../../assets/icons/construction_line.svg")),
    ("construction_ray", include_str!("../../assets/icons/construction_ray.svg")),
    ("dimension", include_str!("../../assets/icons/dimension.svg")),
    ("dimension_aligned", include_str!("../../assets/icons/dimension_aligned.svg")),
    ("dimension_radial", include_str!("../../assets/icons/dimension_radial.svg")),
    ("door", include_str!("../../assets/icons/door.svg")),
    ("label", include_str!("../../assets/icons/label.svg")),
    ("layer_lock", include_str!("../../assets/icons/layer_lock.svg")),
    ("layer_print", include_str!("../../assets/icons/layer_print.svg")),
    ("layer_visible", include_str!("../../assets/icons/layer_visible.svg")),
    ("measurements", include_str!("../../assets/icons/measurements.svg")),
    ("select", include_str!("../../assets/icons/select.svg")),
    ("snap_center", include_str!("../../assets/icons/snap_center.svg")),
    ("snap_endpoint", include_str!("../../assets/icons/snap_endpoint.svg")),
    ("snap_grid", include_str!("../../assets/icons/snap_grid.svg")),
    ("snap_intersection", include_str!("../../assets/icons/snap_intersection.svg")),
    ("snap_midpoint", include_str!("../../assets/icons/snap_midpoint.svg")),
    ("snap_ortho", include_str!("../../assets/icons/snap_ortho.svg")),
    ("snap_perpendicular", include_str!("../../assets/icons/snap_perpendicular.svg")),
    ("snap_tangent", include_str!("../../assets/icons/snap_tangent.svg")),
    ("wall", include_str!("../../assets/icons/wall.svg")),
    ("window", include_str!("../../assets/icons/window.svg")),
];

/// Internal cache key: (icon_key, size_pt).
type CacheKey = (&'static str, u32);

/// Texture cache: one entry per (icon, size) per process lifetime.
static TEXTURE_CACHE: LazyLock<Mutex<HashMap<CacheKey, TextureHandle>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Rasterize an embedded SVG to a `ColorImage` at `size_pt` (points).
/// `resvg` renders to a `tiny_skia::Pixmap`; we convert to `ColorImage`.
fn rasterize_svg(svg: &str, size_pt: u32) -> Result<ColorImage, String> {
    // Parse SVG with usvg (no text, no system fonts — icons are pure paths).
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg.as_bytes(), &opt)
        .map_err(|e| format!("usvg parse failed: {e}"))?;

    // Create pixmap at exact pixel size. The SVG viewBox is 24×24, so
    // tree.size() will be 24×24; we scale to the requested pt.
    let scale = size_pt as f32 / 24.0;
    let pixmap_size = tree.size().to_int_size();
    let width = (pixmap_size.width() as f32 * scale).round() as u32;
    let height = (pixmap_size.height() as f32 * scale).round() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| format!("tiny_skia pixmap alloc failed for {width}×{height}"))?;

    // Render with a scale transform so the SVG fills the requested size.
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, pixmap.as_mut());

    // Convert premultiplied RGBA bytes to egui ColorImage.
    let rgba = pixmap.data(); // premultiplied sRGBA
    let img = ColorImage::from_rgba_premultiplied([width as usize, height as usize], rgba);
    Ok(img)
}

/// Get or create a texture handle for `icon` at `size_pt`.
/// The texture is cached per (icon, size) for the process lifetime.
pub fn icon_texture(
    ctx: &Context,
    icon: IconName,
    size_pt: u32,
) -> Result<TextureHandle, String> {
    let key = icon.key();
    let cache_key = (key, size_pt);

    // Fast path: already cached.
    if let Some(handle) = TEXTURE_CACHE.lock().unwrap().get(&cache_key) {
        return Ok(handle.clone());
    }

    // Find the embedded SVG.
    let svg = ICON_SVGS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, s)| *s)
        .ok_or_else(|| format!("icon '{key}' not found in ICON_SVGS"))?;

    // Rasterize at the requested size.
    let img = rasterize_svg(svg, size_pt)?;

    // Upload to GPU once.
    let name = format!("lintel_icon::{key}::{size_pt}pt");
    let handle = ctx.load_texture(name, img, TextureOptions::default());

    // Cache and return.
    TEXTURE_CACHE.lock().unwrap().insert(cache_key, handle.clone());
    Ok(handle)
}

/// Convenience: draw an icon at `size_pt` inside a button or label.
/// Returns the widget response for interaction handling.
pub fn show_icon(ui: &mut egui::Ui, icon: IconName, size_pt: u32) -> egui::Response {
    match icon_texture(ui.ctx(), icon, size_pt) {
        Ok(handle) => {
            let size = egui::Vec2::splat(size_pt as f32);
            ui.add(egui::Image::new((handle.id(), size)).fit_to_exact_size(size))
        }
        Err(e) => {
            // Fallback: draw the name in red so the missing icon is obvious.
            ui.label(egui::RichText::new(format!("⚠ {e}")).color(crate::theme::DANGER_DEFAULT))
        }
    }
}

/// Convenience: draw an icon button with accessible name for AccessKit (II.30.7.4).
/// Uses a standard `egui::Button` with an image; the accessible name is set via
/// `Response::widget_info` so screen readers announce it correctly.
pub fn icon_button(
    ui: &mut egui::Ui,
    icon: IconName,
    size_pt: u32,
    accessible_name: &str,
) -> egui::Response {
    let img_size = egui::Vec2::splat(size_pt as f32);
    let handle = icon_texture(ui.ctx(), icon, size_pt)
        .unwrap_or_else(|_| ui.ctx().load_texture(
            "lintel_icon::missing",
            ColorImage::example(),
            TextureOptions::default(),
        ));
    let img = egui::Image::new((handle.id(), img_size)).fit_to_exact_size(img_size);

    let resp = ui.add(egui::Button::new("").icon(img).min_size(img_size));

    // Set accessible name on the widget info so AccessKit reads it.
    resp.widget_info(|info| {
        info.label = accessible_name.into();
        info.role = egui::WidgetInfo::role_from_label(&info.label, &resp);
    });

    resp.on_hover_text(accessible_name)
}

/// Preload all icons at common sizes (16, 20, 24, 32 pt) to avoid first-frame lag.
/// Call once at startup (e.g. from `LintelApp::new` after the context exists).
pub fn preload_all(ctx: &Context) {
    const SIZES: &[u32] = &[16, 20, 24, 32];
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
    fn all_icons_have_embedded_svg() {
        for icon in IconName::all() {
            let key = icon.key();
            let found = ICON_SVGS.iter().any(|(k, _)| *k == key);
            assert!(found, "icon '{key}' missing from ICON_SVGS");
        }
    }

    #[test]
    fn icon_names_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for icon in IconName::all() {
            assert!(seen.insert(icon.key()), "duplicate icon key: {}", icon.key());
        }
    }

    #[test]
    fn svg_sources_contain_expected_viewbox() {
        // Every embedded SVG should declare viewBox="0 0 24 24"
        for (key, svg) in ICON_SVGS {
            assert!(
                svg.contains("viewBox=\"0 0 24 24\""),
                "icon '{key}' missing 24×24 viewBox"
            );
            assert!(
                svg.contains("stroke=\"#1A1816\""),
                "icon '{key}' should use BASE_INK stroke"
            );
        }
    }

    #[test]
    fn all_tool_icons_present() {
        // The 12 tool icons required by T-004
        let tool_keys = [
            "wall", "door", "window", "dimension", "label", "measurements",
            "select", "construction_line", "construction_ray",
            "construction_circle", "construction_arc", "block",
        ];
        for key in tool_keys {
            assert!(
                ICON_SVGS.iter().any(|(k, _)| *k == key),
                "tool icon '{key}' missing"
            );
        }
    }
}