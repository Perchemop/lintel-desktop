//! Application shell: title bar, rail (tool palette), and canvas regions.
//!
//! Pattern copied from `spaceui` `Theme::install()` returning `Regions`
//! (spec v0.20.0 AC-06, T-005). All colours/spacing from theme tokens.

use crate::canvas::{Canvas, CanvasResponse, Selection, SelectionBand, SnapModeKind, SnapModes, View, Draft, world_to_screen, screen_to_world};
use crate::app::tool::Tool;
use crate::components::buttons::{ButtonRole, ButtonSize, StyledButton};
use crate::inspector::{InspectorDraft, inspector};
use crate::layout::helpers::{card, tool_group, vstack};
use crate::theme;
use crate::theme::SPEC;
use eframe::egui;
use lintel_core::{
    BlockId,
    command::Command,
    document::Document,
    geom::{Bounds, Point2, Vec2},
    ids::EntityId,
    join::{Hit, PlanGeometry},
};

/// The three main layout regions returned by `Shell::regions()`.
///
/// - `title_bar`: Top strip with app name, window controls, breadcrumb
/// - `rail`: Left sidebar with tool palette (draw, modify, snap, layers)
/// - `canvas`: Center drawing area + rulers + status
pub struct Regions {
    /// Top title bar response.
    pub title_bar: egui::Response,
    /// Left rail (tool palette) response.
    pub rail: egui::Response,
    /// Center canvas response.
    pub canvas: CanvasResponse,
    /// Status bar at bottom of canvas.
    pub status_bar: egui::Response,
    /// Inspector window response (floating, right side).
    pub inspector: Option<egui::InnerResponse<()>>,
    /// Command palette response (modal overlay).
    pub palette: Option<egui::ModalResponse<()>>,
}

/// Layout the shell and return the three regions for the caller to fill.
///
/// This follows the `spaceui` pattern: `Theme::install(ui) -> Regions`
/// where the caller fills each region with content.
pub fn regions(
    ui: &mut egui::Ui,
    app: &mut crate::LintelApp,
    now: i64,
) -> Regions {
    // Poll update check
    app.poll_update();
    if app.update_rx.is_some() {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
    }

    // Handle global shortcuts
    if ui.ctx().input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
        app.toggle_palette();
    }

    // We'll use a CentralPanel for the main layout and build regions inside
    let mut title_bar_response = None;
    let mut rail_response = None;
    let mut canvas_response = None;
    let mut status_bar_response = None;
    let mut inspector_response = None;
    let mut palette_response = None;

    egui::CentralPanel::default().show(ui, |ui| {
        // Top: Title bar
        title_bar_response = Some(title_bar(ui, app, now));

        ui.separator();

        // Main content area: rail (left) + canvas (center)
        ui.horizontal(|ui| {
            // Left rail
            rail_response = Some(rail(ui, app));

            ui.separator();

            // Canvas (takes remaining space)
            let level = app.doc.project().active_level().expect("level");
            let geometry = PlanGeometry::of_level(level);
            let summary = crate::plan_summary(app.doc.project());

            let (resp, rect) = Canvas::show(
                ui,
                &app.view,
                &geometry,
                &app.selection,
                &app.snap_modes,
                &app.snap,
                &app.draft,
                &summary,
                app.band.as_ref(),
            );
            canvas_response = Some(resp);

            let dimension_painter = ui.painter_at(rect);
            crate::dimension_ui::render_dimensions(
                &dimension_painter,
                rect,
                &app.view,
                &geometry,
                &app.selection,
                level,
            );

            // Empty state
            if geometry.walls.is_empty() && !app.draft.active {
                let msg1 = "Pick the Wall tool (W) and click twice to draw a wall";
                let msg2 = "Then place doors and windows from the toolbar";
                let center = rect.center();
                dimension_painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    msg1,
                    egui::FontId::proportional(16.0),
                    SPEC.component.text.primary,
                );
                dimension_painter.text(
                    center + egui::Vec2::new(0.0, 22.0),
                    egui::Align2::CENTER_CENTER,
                    msg2,
                    egui::FontId::proportional(16.0),
                    SPEC.component.text.primary,
                );
            }

            // Highlight selected block instance
            if let Some(id) = app.selected_block_instance {
                if let Some(level) = app.doc.project().active_level()
                    && let Some(instance) = level.block_instance(id)
                {
                    let def = level.block_def(instance.block_id);
                    let mut min_x = f64::INFINITY;
                    let mut min_y = f64::INFINITY;
                    let mut max_x = f64::NEG_INFINITY;
                    let mut max_y = f64::NEG_INFINITY;
                    let mut any = false;
                    if let Some(def) = def {
                        for local in def.local_points() {
                            let p = instance.transform_point(local);
                            min_x = min_x.min(p.x);
                            min_y = min_y.min(p.y);
                            max_x = max_x.max(p.x);
                            max_y = max_y.max(p.y);
                            any = true;
                        }
                    }
                    if !any {
                        const HALF: f64 = 250.0;
                        min_x = instance.param.x - HALF;
                        min_y = instance.param.y - HALF;
                        max_x = instance.param.x + HALF;
                        max_y = instance.param.y + HALF;
                    }
                    let a = world_to_screen(&app.view, rect, Point2::new(min_x, min_y));
                    let b = world_to_screen(&app.view, rect, Point2::new(max_x, max_y));
                    let r = egui::Rect::from_two_pos(a, b);
                    let painter = ui.painter_at(rect);
                    painter.rect_stroke(
                        r,
                        0.0,
                        egui::Stroke::new(2.0, SPEC.component.canvas.block_instance_stroke),
                        egui::StrokeKind::Outside,
                    );
                }
            }

            app.pointer_world = canvas_response.as_ref().unwrap().pointer_world;
            app.last_snap = canvas_response.as_ref().unwrap().snap;

            app.handle_canvas_input(ui, canvas_response.as_ref().unwrap().clone(), rect, now);
        });

        ui.separator();

        // Bottom: Status bar
        status_bar_response = Some(crate::status_bar::render(ui, app, now));
    });

    // Inspector window (floating on right)
    if !app.palette.is_open() {
        let resp = egui::Window::new("Inspector")
            .default_pos(egui::Pos2::new(4.0, 40.0))
            .show(ui.ctx(), |ui| {
                let cmd = inspector(
                    ui,
                    &app.doc,
                    &app.selection,
                    &mut app.snap,
                    &mut app.inspector_draft,
                    app.tool == Tool::Measurements,
                    app.selected_block_instance,
                );
                if let Some(c) = cmd.as_ref() {
                    let _ = app.doc.apply(c.clone(), now);
                }
            });
        inspector_response = resp;
    }

    // Command palette
    palette_response = crate::command_palette::show(ui.ctx(), app, now);

    Regions {
        title_bar: title_bar_response.unwrap(),
        rail: rail_response.unwrap(),
        canvas: canvas_response.unwrap(),
        status_bar: status_bar_response.unwrap(),
        inspector: inspector_response,
        palette: palette_response,
    }
}

/// Title bar region: app name, window controls, breadcrumb.
fn title_bar(ui: &mut egui::Ui, app: &mut crate::LintelApp, _now: i64) -> egui::Response {
    let resp = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = theme::SPACING_MD;

        // App name
        ui.label(
            egui::RichText::new("Lintel")
                .heading()
                .color(SPEC.component.text.primary),
        );

        ui.separator();

        // Breadcrumb: active level name
        if let Some(level) = app.doc.project().active_level() {
            ui.label(
                egui::RichText::new(&level.name)
                    .small()
                    .color(SPEC.component.text.muted),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Window controls (cross-platform, not OS native)
            if ui
                .add(StyledButton::new("✕").role(ButtonRole::Ghost).size(ButtonSize::Sm))
                .on_hover_text("Close Lintel")
                .clicked()
            {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            // Update status indicator
            if let Some(status) = app.status.clone()
                && now_epoch_s().saturating_sub(app.status_at) < 8
            {
                ui.label(
                    egui::RichText::new(status)
                        .small()
                        .color(SPEC.component.text.error),
                );
            }
        });
    });
    resp.response
}

/// Rail region: tool palette with groups (draw, modify, snap, layers).
fn rail(ui: &mut egui::Ui, app: &mut crate::LintelApp) -> egui::Response {
    let resp = ui.vertical(|ui| {
        ui.set_min_width(200.0);
        ui.set_max_width(280.0);
        ui.spacing_mut().item_spacing.y = theme::SPACING_LG;

        // Tool palette groups using vstack
        vstack(ui, theme::SPACING_LG, |ui| {
            // Draw tools
            tool_group(ui, "Draw", |ui| {
                vstack(ui, theme::SPACING_SM, |ui| {
                    tool_button(ui, app, "Wall", Tool::Wall, "W", ButtonRole::Primary);
                    tool_button(ui, app, "Door", Tool::Door, "D", ButtonRole::Secondary);
                    tool_button(ui, app, "Window", Tool::Window, "N", ButtonRole::Secondary);
                    tool_button(ui, app, "Dimension", Tool::Dimension, "M", ButtonRole::Secondary);
                    tool_button(ui, app, "Label", Tool::Label, "L", ButtonRole::Secondary);
                    tool_button(ui, app, "Block", Tool::Block, "B", ButtonRole::Secondary);
                });
            });

            // Construction tools
            tool_group(ui, "Construction", |ui| {
                vstack(ui, theme::SPACING_SM, |ui| {
                    tool_button(ui, app, "Line", Tool::ConstructionLine, "C", ButtonRole::Ghost);
                    tool_button(ui, app, "Ray", Tool::ConstructionRay, "Shift+C", ButtonRole::Ghost);
                    tool_button(ui, app, "Circle", Tool::ConstructionCircle, "Q", ButtonRole::Ghost);
                    tool_button(ui, app, "Arc", Tool::ConstructionArc, "X", ButtonRole::Ghost);
                });
            });

            // Snap modes
            tool_group(ui, "Snap", |ui| {
                vstack(ui, theme::SPACING_SM, |ui| {
                    snap_chip(ui, app, SnapModeKind::Endpoint, "E");
                    snap_chip(ui, app, SnapModeKind::Midpoint, "V");
                    snap_chip(ui, app, SnapModeKind::Intersection, "I");
                    snap_chip(ui, app, SnapModeKind::Perpendicular, "P");
                    snap_chip(ui, app, SnapModeKind::Tangent, "T");
                    snap_chip(ui, app, SnapModeKind::Grid, "G");
                    snap_chip(ui, app, SnapModeKind::Ortho, "O");
                });
            });

            // Layers (placeholder for now)
            tool_group(ui, "Layers", |ui| {
                vstack(ui, theme::SPACING_SM, |ui| {
                    ui.label(
                        egui::RichText::new("Layer management")
                            .small()
                            .color(SPEC.component.text.hint),
                    );
                });
            });
        });
    });
    resp.response
}

/// A tool button in the rail.
fn tool_button(
    ui: &mut egui::Ui,
    app: &mut crate::LintelApp,
    label: &str,
    tool: Tool,
    shortcut: &str,
    role: ButtonRole,
) {
    let selected = app.tool == tool;
    let btn = StyledButton::new(label)
        .role(role)
        .size(ButtonSize::Md)
        .selected(selected);
    if ui
        .add(btn)
        .on_hover_text(format!("{} ({})", label, shortcut))
        .clicked()
    {
        app.tool = tool;
    }
}

/// A snap mode chip in the rail.
fn snap_chip(ui: &mut egui::Ui, app: &mut crate::LintelApp, kind: SnapModeKind, key: &str) {
    let active = app.snap_modes.enabled(kind);
    let label = format!("{} ({})", kind.label(), key);
    let resp = ui
        .add(
            StyledButton::new(label)
                .role(if active { ButtonRole::Primary } else { ButtonRole::Ghost })
                .size(ButtonSize::Sm),
        )
        .on_hover_text(format!("Toggle {} snap ({})", kind.label(), key));
    if resp.clicked() {
        app.snap_modes.toggle(kind);
    }
}

fn now_epoch_s() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_secs() as i64
}