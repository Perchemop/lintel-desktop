//! Canvas panel: grid, walls, openings, selection, draft preview.
//!
//! Pure presentation: world millimetres → screen pixels, pointer events →
//! intents the app applies to the core document. The canvas never mutates the
//! model itself.

use eframe::egui;
use lintel_core::geom::{self, Point2};
use lintel_core::ids::Hit;
use lintel_core::join::PlanGeometry;
use lintel_core::snap::{self, SnapSettings};

use crate::theme;

/// What the canvas wants the app to do after one frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct CanvasResponse {
    pub pointer_world: Option<Point2>,
    pub click: Option<(egui::Pos2, egui::PointerButton)>,
    pub drag_delta: Option<egui::Vec2>,   // screen delta, primary button
    pub pan_delta: Option<egui::Vec2>,    // screen delta, middle button
    pub zoom: Option<(f64, egui::Pos2)>,  // factor, anchor screen pos
}

/// Draft geometry for the wall tool.
#[derive(Debug, Clone, Copy, Default)]
pub struct Draft {
    pub active: bool,
    pub start: Point2,
    pub current: Point2,
}

pub struct Canvas {}

impl Canvas {
    pub fn show(
        ui: &mut egui::Ui,
        view: &super::View,
        geometry: &PlanGeometry,
        selection: Option<Hit>,
        snap: &SnapSettings,
        draft: &Draft,
    ) -> (CanvasResponse, egui::Rect) {
        let mut response = CanvasResponse::default();

        let (rect, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        painter.rect_filled(rect, 0.0, theme::PAPER);

        let hover = ui.ctx().input(|i| i.pointer_hover_pos());
        response.pointer_world = hover.map(|p| screen_to_world(view, rect, p));
        let scroll = ui.ctx().input(|i| i.smooth_scroll_delta);
        if scroll.y.abs() > 1e-6 {
            if let Some(anchor) = hover {
                response.zoom = Some(((-scroll.y * 0.001).exp(), anchor));
            }
        }
        // Grid.
        if snap.grid_enabled && snap.grid.is_finite() && snap.grid > 0.0 {
            paint_grid(&painter, rect, view, snap.grid);
        }

        for wall in &geometry.walls {
            paint_wall(&painter, rect, view, wall, selection == Some(Hit::Wall(wall.wall.id)));
        }
        for opening in &geometry.openings {
            paint_opening(&painter, rect, view, opening, selection == Some(Hit::Opening(opening.id)));
        }

        // Draft preview (wall tool).
        if draft.active {
            let anchors = wall_anchors_snap(geometry);
            let snapped = snap::snap_point(
                response.pointer_world.unwrap_or(draft.current),
                &wall_anchors_snap(geometry),
                Some(draft.start),
                snap,
            );
            let start_screen = world_to_screen(view, rect, draft.start);
            let end_screen = world_to_screen(view, rect, snapped.point);
            painter.line_segment([start_screen, end_screen], theme::selection_stroke());
            let length = draft.start.distance(snapped.point);
            let angle = (snapped.point - draft.start).angle_degrees();
            let pos = end_screen + egui::Vec2::new(8.0, -6.0);
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                format!("{length:.0} mm  {angle:.0}°"),
                egui::FontId::proportional(11.0),
                theme::INK,
            );
            if let Some(anchor) = nearest_anchor(response.pointer_world, &anchors, snap.anchor_tolerance) {
                let screen = world_to_screen(view, rect, anchor);
                painter.circle_filled(screen, theme::snap_radius(), theme::GREEN);
            }
        } else if let Some(world) = response.pointer_world {
            let anchors = wall_anchors_snap(geometry);
            if let Some(anchor) = nearest_anchor(Some(world), &anchors, snap.anchor_tolerance) {
                let screen = world_to_screen(view, rect, anchor);
                painter.circle_filled(screen, 3.0, theme::GREEN);
            }
        }

        // Input intents.
        if response.pointer_world.is_some() {
            if ui.ctx().input(|i| i.button_pressed(egui::PointerButton::Primary)) {
                response.click = Some((hover.unwrap_or(rect.center()), egui::PointerButton::Primary));
            }
            if ui.ctx().input(|i| i.button_pressed(egui::PointerButton::Middle)) {
                response.click = Some((hover.unwrap_or(rect.center()), egui::PointerButton::Middle));
            }
        }
        if let Some(delta) = ui.ctx().input(|i| i.pointer_delta()) {
            if delta.length_sq() > 0.0 {
                if ui.ctx().input(|i| i.button_down(egui::PointerButton::Primary)) {
                    response.drag_delta = Some(delta);
                } else if ui.ctx().input(|i| i.button_down(egui::PointerButton::Middle)) {
                    response.pan_delta = Some(delta);
                }
            }
        }
        if scroll_delta.y.abs() > 1e-6 && response.hovered {
            response.zoom = Some(((-scroll_delta.y * 0.001).exp(), hover.unwrap_or(rect.center())));
        }

        (response, rect)
    }
}

/// Viewport transform: world millimetres ↔ screen pixels.
#[derive(Clone, Copy, Debug)]
pub struct View {
    /// World point at the centre of the viewport.
    pub centre: Point2,
    /// Screen pixels per model millimetre.
    pub px_per_mm: f32,
}

impl Default for View {
    fn default() -> Self {
        Self { centre: Point2::ORIGIN, px_per_mm: 0.4 }
    }
}

impl View {
    pub fn zoom_percent(&self) -> i32 {
        (self.px_per_mm as f64 / 0.4 * 100.0).round() as i32
    }

    pub fn clamp_zoom(&mut self) {
        self.px_per_mm = self.px_per_mm.clamp(0.02, 12.0);
    }
}

pub fn screen_to_world(view: &View, rect: egui::Rect, pos: egui::Pos2) -> Point2 {
    Point2::new(
        view.centre.x + (pos.x - rect.center().x) as f64 / view.px_per_mm as f64,
        view.centre.y - (pos.y - rect.center().y) as f64 / view.px_per_mm as f64,
    )
}

pub fn world_to_screen(view: &View, rect: egui::Rect, p: Point2) -> egui::Pos2 {
    egui::Pos2::new(
        rect.center().x + (p.x - view.centre.x) as f32 * view.px_per_mm,
        rect.center().y - (p.y - view.centre.y) as f32 * view.px_per_mm,
    )
}

fn paint_grid(painter: &egui::Painter, rect: egui::Rect, view: &View, spacing_mm: f64) {
    let px_per = view.px_per_mm as f64;
    if spacing_mm * px_per < 6.0 {
        return;
    }
    let wl = screen_to_world(view, rect, rect.left_top()).x;
    let wr = screen_to_world(view, rect, rect.right_top()).x;
    let wb = screen_to_world(view, rect, rect.left_bottom()).y;
    let wt = screen_to_world(view, rect, rect.left_top()).y;
    let g0 = (wl / spacing_mm).floor() as i64;
    let g1 = (wr / spacing_mm).ceil() as i64;
    for g in g0..=g1 {
        let x = g as f64 * spacing_mm;
        let p0 = world_to_screen(view, rect, Point2::new(x, wb));
        let p1 = world_to_screen(view, rect, Point2::new(x, wt));
        let stroke = if g % 5 == 0 {
            egui::Stroke::new(0.9, theme::GRID_MAJOR)
        } else {
            egui::Stroke::new(0.5, theme::GRID_MINOR)
        };
        painter.line_segment([p0, p1], stroke);
    }
    // horizontal grid lines
    let g0 = (wb / spacing_mm).floor() as i64;
    let g1 = (wt / spacing_mm).ceil() as i64;
    for g in g0..=g1 {
        let y = g as f64 * spacing_mm;
        let p0 = world_to_screen(view, rect, Point2::new(wl, y));
        let p1 = world_to_screen(view, rect, Point2::new(wr, y));
        let stroke = if g % 5 == 0 {
            egui::Stroke::new(0.9, theme::GRID_MAJOR)
        } else {
            egui::Stroke::new(0.5, theme::GRID_MINOR)
        };
        painter.line_segment([p0, p1], stroke);
    }
}

fn paint_wall(
    painter: &egui::Painter,
    rect: egui::Rect,
    view: &View,
    wall: &lintel_core::join::ResolvedWall,
    selected: bool,
) {
    let pts: Vec<egui::Pos2> = wall.corners.iter().map(|p| world_to_screen(view, rect, *p)).collect();
    let fill = if selected {
        theme::COBALT.linear_multiply(0.12)
    } else {
        theme::WALL_FILL
    };
    painter.add(egui::Shape::convex_polygon(pts.clone(), fill));
    let stroke = if selected {
        theme::selection_stroke()
    } else {
        egui::Stroke::new(0.9, theme::WALL_STROKE)
    };
    let n = pts.len();
    for i in 0..n {
        painter.line_segment([pts[i], pts[(i + 1) % n]], stroke);
    }
}

fn paint_opening(
    painter: &egui::Painter,
    rect: egui::Rect,
    view: &View,
    opening: &lintel_core::join::ResolvedOpening,
    selected: bool,
) {
    let half = opening.host_thickness * 0.5;
    let normal = opening.host_direction.perpendicular();
    let scale = view.px_per_mm as f64;
    let n_screen = normal.scaled(half * scale);
    let start = world_to_screen(view, rect, opening.start);
    let end = world_to_screen(view, rect, opening.end);

    // Paper gap over the wall band.
    let quad = [start + n_screen, end + n_screen, end - n_screen, start - n_screen];
    painter.add(egui::Shape::convex_polygon(quad.to_vec(), theme::OPENING_GAP));

    // Jambs.
    for jamb in [start, end] {
        painter.line_segment([jamb + n_screen, jamb - n_screen], egui::Stroke::new(0.9, theme::WALL_STROKE));
    }

    match opening.swing {
        Some(swing) => {
            let hinge = world_to_screen(view, rect, swing.hinge);
            let leaf = world_to_screen(view, rect, swing.leaf_end);
            painter.line_segment([hinge, leaf], egui::Stroke::new(0.9, theme::DOOR_LEAF));
            let arc: Vec<egui::Pos2> = geom::arc_points(
                swing.hinge,
                swing.radius,
                swing.from_degrees,
                swing.to_degrees,
                20,
            )
            .iter()
            .map(|p| world_to_screen(view, rect, *p))
            .collect();
            painter.add(egui::Shape::line(
                arc,
                egui::Stroke::new(0.7, theme::DOOR_LEAF),
            ));
        }
        None => {
            let inset = normal.scaled(half * scale * 0.5);
            painter.line_segment(
                [start + inset, end + inset],
                egui::Stroke::new(0.7, theme::WINDOW_LINE),
            );
            painter.line_segment(
                [start - inset, end - inset],
                egui::Stroke::new(0.7, theme::WINDOW_LINE),
            );
        }
    }

    if selected {
        let c = world_to_screen(view, rect, opening.centre);
        painter.circle_filled(c, 3.0, theme::selection_stroke().color);
    }
}

fn nearest_anchor<'a>(
    world: Option<Point2>,
    anchors: &'a [Point2],
    tolerance_mm: f64,
) -> Option<Point2> {
    let Some(w) = world else { return None };
    let mut best: Option<(f64, Point2)> = None;
    for &a in anchors {
        let d = w.distance(a);
        if d <= tolerance_mm && best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, a));
        }
    }
    best.map(|(_, a)| a)
}

/// Snap anchors for all walls in a level (both ends of each).
pub fn wall_anchors_snap(geometry: &PlanGeometry) -> Vec<Point2> {
    let mut anchors = Vec::with_capacity(geometry.walls.len() * 2);
    for wall in &geometry.walls {
        anchors.push(wall.wall.start);
        anchors.push(wall.wall.end);
    }
    anchors
}
