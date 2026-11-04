//! Dimension rendering on the canvas panel.
//!
//! Pure presentation: world millimetres → screen pixels. The dimension UI
//! never mutates the model itself.

use crate::canvas::{View, world_to_screen};
use crate::theme;
use eframe::egui;
use lintel_core::join::{Hit, PlanGeometry};

/// Draw every dimension in `geometry` onto `painter`.
pub fn render_dimensions(
    painter: &egui::Painter,
    rect: egui::Rect,
    view: &View,
    geometry: &PlanGeometry,
    selection: Option<Hit>,
) {
    for dim in geometry.dimensions() {
        let selected = matches!(selection, Some(Hit::Dimension(id)) if id == dim.id);
        let Some((a, b)) = dim.dimension_line() else {
            continue;
        };
        let sa = world_to_screen(view, rect, a);
        let sb = world_to_screen(view, rect, b);
        let ss = world_to_screen(view, rect, dim.start);
        let se = world_to_screen(view, rect, dim.end);

        let stroke_color = if selected { theme::COBALT } else { theme::INK };
        let line_stroke = egui::Stroke::new(1.0, stroke_color);

        // Extension lines (witness lines) from measured points to the
        // dimension line endpoints.
        if dim.extension_lines {
            painter.line_segment([ss, sa], line_stroke);
            painter.line_segment([se, sb], line_stroke);
        }

        // Dimension line itself.
        painter.line_segment([sa, sb], line_stroke);

        // Tick marks at each end: small perpendicular segments.
        let dir = sb - sa;
        let len = dir.length().max(1.0);
        let perp = egui::Vec2::new(-dir.y, dir.x) / len * 6.0;
        painter.line_segment([sa - perp, sa + perp], line_stroke);
        painter.line_segment([sb - perp, sb + perp], line_stroke);

        // Text label centred above/below the line.
        let mid = (sa.to_vec2() + sb.to_vec2()) * 0.5;
        let label_offset = if perp.y <= 0.0 {
            egui::Vec2::new(0.0, -12.0)
        } else {
            egui::Vec2::new(0.0, 12.0)
        };
        let label = format!("{:.0} mm", dim.length());
        painter.text(
            egui::Pos2::new(mid.x, mid.y) + label_offset,
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(11.0),
            stroke_color,
        );
    }
}
