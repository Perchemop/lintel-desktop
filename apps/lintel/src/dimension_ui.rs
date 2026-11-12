//! Dimension rendering on the canvas panel.
//!
//! Pure presentation: world millimetres → screen pixels. The dimension UI
//! never mutates the model itself.

use crate::canvas::{Selection, View, world_to_screen};
use crate::theme;
use eframe::egui;
use lintel_core::ids::EntityId;
use lintel_core::join::PlanGeometry;
use lintel_core::level::Level;

/// Draw every dimension in `geometry` onto `painter`.
pub fn render_dimensions(
    painter: &egui::Painter,
    rect: egui::Rect,
    view: &View,
    geometry: &PlanGeometry,
    selection: &Selection,
    level: &Level,
) {
    for dim in geometry.dimensions() {
        let selected = selection.contains(EntityId::Dimension(dim.id));
        let Some((a, b)) = dim.dimension_line_at(Some(level)) else {
            continue;
        };
        let sa = world_to_screen(view, rect, a);
        let sb = world_to_screen(view, rect, b);
        let ss = world_to_screen(view, rect, dim.effective_start(Some(level)));
        let se = world_to_screen(view, rect, dim.effective_end(Some(level)));

        let stroke_color = if selected { theme::COBALT } else { theme::INK };
        let line_stroke = egui::Stroke::new(1.0, stroke_color);

        // Witness lines (extension lines) from measured points to the
        // dimension line endpoints, when enabled and valid.
        if let Some((from, to)) = dim.start_witness_line(Some(level)) {
            let sfrom = world_to_screen(view, rect, from);
            let sto = world_to_screen(view, rect, to);
            painter.line_segment([sfrom, sto], line_stroke);
        }
        if let Some((from, to)) = dim.end_witness_line(Some(level)) {
            let sfrom = world_to_screen(view, rect, from);
            let sto = world_to_screen(view, rect, to);
            painter.line_segment([sfrom, sto], line_stroke);
        }

        // Dimension line itself.
        painter.line_segment([sa, sb], line_stroke);

        // Tick marks at each end: small perpendicular segments.
        let dir = sb - sa;
        let len = dir.length().max(1.0);
        let perp = egui::Vec2::new(-dir.y, dir.x) / len * 6.0;
        painter.line_segment([sa - perp, sa + perp], line_stroke);
        painter.line_segment([sb - perp, sb + perp], line_stroke);

        // Text label rotated to follow the dimension line.
        let mid = (sa.to_vec2() + sb.to_vec2()) * 0.5;
        let normal = egui::Vec2::new(-dir.y, dir.x) / len;
        let mut angle = dir.angle();
        if angle > std::f32::consts::FRAC_PI_2 {
            angle -= std::f32::consts::PI;
        } else if angle <= -std::f32::consts::FRAC_PI_2 {
            angle += std::f32::consts::PI;
        }
        let label = format!("{:.0} mm", dim.length_at(Some(level)));
        let font = egui::FontId::proportional(11.0);
        let galley = painter.layout_no_wrap(label, font, stroke_color);
        let pos = egui::Pos2::new(mid.x, mid.y) + normal * 8.0;
        let shape = egui::epaint::TextShape::new(pos, galley, stroke_color)
            .with_angle_and_anchor(angle, egui::Align2::CENTER_CENTER);
        painter.add(shape);

        // Grip markers on anchored measured points.
        if dim.start_anchor.is_some() {
            let r = egui::Rect::from_center_size(ss, egui::vec2(4.0, 4.0));
            painter.rect_filled(r, 0.0, theme::COBALT);
        }
        if dim.end_anchor.is_some() {
            let r = egui::Rect::from_center_size(se, egui::vec2(4.0, 4.0));
            painter.rect_filled(r, 0.0, theme::COBALT);
        }
    }
}
