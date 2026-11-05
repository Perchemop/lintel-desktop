//! Lintel desktop shell — eframe/egui GUI wired to `lintel-core` commands.
//! Every user interaction goes through a `Command`; the core model updates
//! headlessly, and the UI re-renders from the new state (addendum K.3).

use crate::canvas::{Canvas, Draft, View, screen_to_world};
use crate::inspector::inspector;
use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    geom::{Point2, Vec2},
    join::{Hit, PlanGeometry},
};

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};

mod about;
mod canvas;
mod dimension_ui;
mod inspector;
mod release_notes;
mod theme;

const MIN_WALL_LENGTH_DRAG: f64 = 2.0;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Select,
    Wall,
    Door,
    Window,
    Dimension,
}

impl Tool {
    fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Wall => "Wall",
            Tool::Door => "Door",
            Tool::Window => "Window",
            Tool::Dimension => "Dimension",
        }
    }
}

/// Main eframe app state.
struct LintelApp {
    doc: Document,
    view: View,
    tool: Tool,
    selection: Option<Hit>,
    snap: lintel_core::snap::SnapSettings,
    draft: Draft,
    showing_about: bool,
    showing_notes: bool,
    export_open: bool,
    export_spec: lintel_core::pdf::SheetSpec,
    /// Receives the result of an in-flight update check, if any.
    update_rx: Option<Receiver<update_check::UpdateStatus>>,
    /// The last completed update check, if any.
    update_status: Option<update_check::UpdateStatus>,
    /// Whether the "update available" prompt is showing.
    update_open: bool,
    /// Last user-visible status message, cleared after a short while.
    status: Option<String>,
    /// Unix seconds when `status` was set, so it can expire.
    status_at: i64,
}

impl LintelApp {
    fn new() -> Self {
        let now = now_epoch_s();
        let doc = Document::new("Untitled plan", now);
        let mut app = Self {
            doc,
            view: View::default(),
            tool: Tool::Wall,
            selection: None,
            snap: lintel_core::snap::SnapSettings::default(),
            draft: Draft::default(),
            showing_about: false,
            showing_notes: false,
            export_open: false,
            export_spec: lintel_core::pdf::SheetSpec::default(),
            update_rx: None,
            update_status: None,
            update_open: false,
            status: None,
            status_at: 0,
        };
        // One quiet check at startup; it fails soft when offline and never
        // blocks the UI thread.
        app.start_update_check();
        app
    }

    /// Kick off an update check on a worker thread. A no-op while one is
    /// already in flight.
    fn start_update_check(&mut self) {
        if self.update_rx.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.update_rx = Some(rx);
        std::thread::spawn(move || {
            let checker = update_check::UpdateChecker::new();
            let status = checker.check_uncached(about::REPO, env!("CARGO_PKG_VERSION"));
            let _ = tx.send(status);
        });
    }

    /// Collect a finished update check without blocking the UI thread.
    fn poll_update(&mut self) {
        let result = self.update_rx.as_ref().map(|rx| rx.try_recv());
        match result {
            None => {}
            Some(Ok(status)) => {
                self.update_open = matches!(status, update_check::UpdateStatus::UpdateAvailable(_));
                self.update_status = Some(status);
                self.update_rx = None;
            }
            Some(Err(mpsc::TryRecvError::Empty)) => {}
            Some(Err(mpsc::TryRecvError::Disconnected)) => self.update_rx = None,
        }
    }

    /// Show a short status/error message in the toolbar and log it.
    fn note_status(&mut self, message: String, now: i64) {
        eprintln!("lintel: {message}");
        self.status = Some(message);
        self.status_at = now;
    }
}

impl eframe::App for LintelApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = now_epoch_s();
        self.poll_update();
        if self.update_rx.is_some() {
            // Poll for the worker's result without spinning the CPU.
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(500));
        }

        egui::CentralPanel::default().show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New plan").clicked() {
                        self.doc = Document::new("Untitled plan", now_epoch_s());
                        self.selection = None;
                        self.draft = Draft::default();
                    }
                    ui.separator();
                    if ui.button("Open…").clicked()
                        && let Some(path) = Some(PathBuf::from("workflow.lintel.json"))
                    {
                        match Document::open(&path) {
                            Ok(loaded) => {
                                self.doc = loaded;
                                self.selection = None;
                                self.view = View::default();
                            }
                            Err(e) => {
                                self.note_status(format!("Could not open the plan: {e}"), now)
                            }
                        }
                    }
                    if ui.button("Save").clicked()
                        && let Err(e) = self.doc.save()
                    {
                        self.note_status(format!("Could not save: {e}"), now);
                    }
                    if ui.button("Save As…").clicked()
                        && let Some(path) = Some(PathBuf::from("workflow.lintel.json"))
                        && let Err(e) = self.doc.save_as(path)
                    {
                        self.note_status(format!("Could not save: {e}"), now);
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    ui.add_enabled(self.doc.can_undo(), |ui: &mut egui::Ui| {
                        let response = ui.button("Undo");
                        if response.clicked() {
                            self.doc.undo(now_epoch_s());
                            self.selection = None;
                            self.draft = Draft::default();
                        }
                        response
                    });
                    ui.add_enabled(self.doc.can_redo(), |ui: &mut egui::Ui| {
                        let response = ui.button("Redo");
                        if response.clicked() {
                            self.doc.redo(now_epoch_s());
                            self.selection = None;
                            self.draft = Draft::default();
                        }
                        response
                    });
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Zoom to fit").clicked() {
                        self.zoom_to_fit();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About Lintel…").clicked() {
                        self.showing_about = true;
                    }
                    if ui.button("Release notes…").clicked() {
                        self.showing_notes = true;
                    }
                    ui.separator();
                    if ui.button("Clear message").clicked() {
                        self.status = None;
                    }
                    ui.separator();
                    if ui.button("Check for updates…").clicked() {
                        self.start_update_check();
                        self.showing_about = true;
                    }
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Tool:");
                egui::ComboBox::from_label("Tool")
                    .selected_text(self.tool.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.tool, Tool::Select, Tool::Select.label());
                        ui.selectable_value(&mut self.tool, Tool::Wall, Tool::Wall.label());
                        ui.selectable_value(&mut self.tool, Tool::Door, Tool::Door.label());
                        ui.selectable_value(&mut self.tool, Tool::Window, Tool::Window.label());
                        ui.selectable_value(
                            &mut self.tool,
                            Tool::Dimension,
                            Tool::Dimension.label(),
                        );
                    });
                ui.separator();
                ui.label(format!("Zoom: {}%", self.view.zoom_percent()));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(status) = self.status.clone()
                        && now_epoch_s().saturating_sub(self.status_at) < 8
                    {
                        ui.label(
                            egui::RichText::new(status)
                                .small()
                                .color(egui::Color32::DARK_RED),
                        );
                    }
                    let version = ui.link(
                        egui::RichText::new(format!("Lintel v{}", env!("CARGO_PKG_VERSION")))
                            .small(),
                    );
                    if version
                        .on_hover_text("Release notes and update status")
                        .clicked()
                    {
                        self.showing_notes = true;
                    }
                });
            });

            let geometry =
                PlanGeometry::of_level(self.doc.project().active_level().expect("level"));
            let (response, rect) = Canvas::show(
                ui,
                &self.view,
                &geometry,
                self.selection,
                &self.snap,
                &self.draft,
            );
            let dimension_painter = ui.painter_at(rect);
            crate::dimension_ui::render_dimensions(
                &dimension_painter,
                rect,
                &self.view,
                &geometry,
                self.selection,
            );
            if geometry.walls.is_empty() && !self.draft.active {
                let msg1 = "Pick the Wall tool (W) and click twice to draw a wall";
                let msg2 = "Then place doors and windows from the toolbar";
                let center = rect.center();
                dimension_painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    msg1,
                    egui::FontId::proportional(16.0),
                    crate::theme::INK,
                );
                dimension_painter.text(
                    center + egui::Vec2::new(0.0, 22.0),
                    egui::Align2::CENTER_CENTER,
                    msg2,
                    egui::FontId::proportional(16.0),
                    crate::theme::INK,
                );
            }
            self.handle_canvas_input(ui, response, rect, now);

            egui::Window::new("Inspector")
                .default_pos(egui::Pos2::new(4.0, 40.0))
                .show(ui, |ui| {
                    if let Some(cmd) = inspector(ui, &self.doc, &self.selection, &mut self.snap) {
                        let _ = self.doc.apply(cmd, now);
                    }
                });
        });

        self.dialogs(ui, now);
    }
}

impl LintelApp {
    fn handle_canvas_input(
        &mut self,
        ui: &egui::Ui,
        response: crate::canvas::CanvasResponse,
        rect: egui::Rect,
        now: i64,
    ) {
        let geometry = PlanGeometry::of_level(self.doc.project().active_level().expect("level"));

        if let Some(click_screen) = response.click {
            let world = screen_to_world(&self.view, rect, click_screen);
            match (self.tool, Some(world)) {
                (Tool::Select, Some(world)) => {
                    self.selection = geometry.hit_test(world, self.snap.anchor_tolerance);
                    self.draft = Draft::default();
                }
                (Tool::Wall, Some(world)) => {
                    let anchors = crate::canvas::wall_anchors_snap(&geometry);
                    if !self.draft.active {
                        let start =
                            lintel_core::snap::snap_point(world, &anchors, None, &self.snap).point;
                        self.draft = Draft {
                            active: true,
                            start,
                            current: world,
                        };
                    } else {
                        let end = lintel_core::snap::snap_point(
                            world,
                            &anchors,
                            Some(self.draft.start),
                            &self.snap,
                        )
                        .point;
                        if end.distance(self.draft.start) > MIN_WALL_LENGTH_DRAG {
                            let applied = self
                                .doc
                                .apply(
                                    Command::AddWall {
                                        start: self.draft.start,
                                        end,
                                        thickness: 200.0,
                                        height: 2800.0,
                                    },
                                    now,
                                )
                                .map_err(|e| {
                                    self.note_status(format!("Wall rejected: {e}"), now);
                                })
                                .ok();
                            if let Some(applied) = applied
                                && let Some(wall_id) = applied.wall
                            {
                                self.selection = Some(Hit::Wall(wall_id));
                            }
                        }
                        self.draft = Draft::default();
                    }
                }
                (Tool::Dimension, Some(world)) => {
                    let anchors = crate::canvas::wall_anchors_snap(&geometry);
                    if !self.draft.active {
                        let start =
                            lintel_core::snap::snap_point(world, &anchors, None, &self.snap).point;
                        self.draft = Draft {
                            active: true,
                            start,
                            current: world,
                        };
                    } else {
                        let end = lintel_core::snap::snap_point(
                            world,
                            &anchors,
                            Some(self.draft.start),
                            &self.snap,
                        )
                        .point;
                        if end.distance(self.draft.start) > MIN_WALL_LENGTH_DRAG {
                            let applied = self
                                .doc
                                .apply(
                                    Command::PlaceDimension {
                                        kind: lintel_core::dimension::DimensionKind::Aligned,
                                        start: self.draft.start,
                                        end,
                                        offset: lintel_core::dimension::DEFAULT_DIMENSION_OFFSET,
                                    },
                                    now,
                                )
                                .map_err(|e| {
                                    self.note_status(format!("Dimension rejected: {e}"), now);
                                })
                                .ok();
                            if let Some(applied) = applied
                                && let Some(dim_id) = applied.dimension
                            {
                                self.selection = Some(Hit::Dimension(dim_id));
                            }
                        }
                        self.draft = Draft::default();
                    }
                }
                (Tool::Door | Tool::Window, Some(world)) => match self.selection {
                    Some(Hit::Wall(id)) => {
                        let level = self.doc.project().active_level().expect("level");
                        let wall = level.wall(id).expect("wall");
                        let offset = wall.start.distance(world).clamp(0.0, wall.length());
                        let kind = match self.tool {
                            Tool::Door => lintel_core::opening::OpeningKind::Door,
                            Tool::Window => lintel_core::opening::OpeningKind::Window,
                            _ => unreachable!(),
                        };
                        let _ = self.doc.apply(
                            Command::PlaceOpening {
                                kind,
                                host: id,
                                offset,
                            },
                            now,
                        );
                        self.draft = Draft::default();
                    }
                    _ => {
                        self.selection = geometry.hit_test(world, self.snap.anchor_tolerance);
                    }
                },
                _ => {}
            }
        }

        if let Some(delta) = response.drag_delta
            && let Tool::Select = self.tool
        {
            let mm = Vec2::new(
                delta.x as f64 / self.view.px_per_mm as f64,
                -delta.y as f64 / self.view.px_per_mm as f64,
            );
            match self.selection {
                Some(Hit::Wall(id)) => {
                    let level = self.doc.project().active_level().expect("level");
                    let wall = level.wall(id).expect("wall");
                    let start = wall.start + mm;
                    let end = wall.end + mm;
                    let _ = self.doc.apply(Command::MoveWall { id, start, end }, now);
                }
                Some(Hit::Opening(id)) => {
                    let level = self.doc.project().active_level().expect("level");
                    let opening = level.opening(id).expect("opening");
                    let host = level.wall(opening.host).expect("host wall");
                    let dir = host.direction();
                    let progress = ((delta.x as f64) * dir.x - (delta.y as f64) * dir.y)
                        / self.view.px_per_mm as f64;
                    let new_offset = (opening.offset + progress).clamp(0.0, host.length());
                    let _ = self.doc.apply(
                        Command::MoveOpening {
                            id,
                            offset: new_offset,
                        },
                        now,
                    );
                }
                _ => {}
            }
        }

        if let Some(delta) = response.pan_delta {
            self.view.centre = Point2::new(
                self.view.centre.x - delta.x as f64 / self.view.px_per_mm as f64,
                self.view.centre.y + delta.y as f64 / self.view.px_per_mm as f64,
            );
        }

        if let Some((factor, anchor)) = response.zoom
            && let Some(hover) = response.pointer_world
        {
            let before = hover;
            self.view.px_per_mm = (self.view.px_per_mm as f64 * factor).clamp(0.02, 12.0) as f32;
            let after = screen_to_world(&self.view, rect, anchor);
            self.view.centre = Point2::new(
                self.view.centre.x + (before.x - after.x),
                self.view.centre.y + (before.y - after.y),
            );
        }

        if self.draft.active {
            if let Some(world) = response.pointer_world {
                self.draft.current = world;
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.draft = Draft::default();
            }
        }

        if ui.memory(|m| m.focused().is_none()) {
            if ui
                .input(|i| i.modifiers.command && i.key_pressed(egui::Key::Z) && !i.modifiers.shift)
                && self.doc.can_undo()
            {
                self.doc.undo(now);
                self.selection = None;
                self.draft = Draft::default();
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Y))
                && self.doc.can_redo()
            {
                self.doc.redo(now);
                self.selection = None;
                self.draft = Draft::default();
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.selection = None;
                self.draft = Draft::default();
            }
            if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace))
            {
                if let Some(Hit::Wall(id)) = self.selection {
                    let _ = self.doc.apply(Command::DeleteWall { id }, now);
                    self.selection = None;
                }
                if let Some(Hit::Opening(id)) = self.selection {
                    let _ = self.doc.apply(Command::DeleteOpening { id }, now);
                    self.selection = None;
                }
                if let Some(Hit::Dimension(id)) = self.selection {
                    let _ = self.doc.apply(Command::DeleteDimension { id }, now);
                    self.selection = None;
                }
            }
            if ui.input(|i| i.key_pressed(egui::Key::W)) {
                self.tool = Tool::Wall;
            }
            if ui.input(|i| i.key_pressed(egui::Key::S)) {
                self.tool = Tool::Select;
            }
            if ui.input(|i| i.key_pressed(egui::Key::D)) {
                self.tool = Tool::Door;
            }
            if ui.input(|i| i.key_pressed(egui::Key::N)) {
                self.tool = Tool::Window;
            }
            if ui.input(|i| i.key_pressed(egui::Key::M)) {
                self.tool = Tool::Dimension;
            }
            if ui.input(|i| i.key_pressed(egui::Key::G)) {
                self.snap.grid_enabled = !self.snap.grid_enabled;
            }
            if ui.input(|i| i.key_pressed(egui::Key::O)) {
                self.snap.ortho = !self.snap.ortho;
            }
            if ui.input(|i| i.key_pressed(egui::Key::F)) {
                self.zoom_to_fit();
            }
        }
    }

    fn dialogs(&mut self, ui: &mut egui::Ui, now: i64) {
        if self.showing_about {
            let status = about::status_line(self.update_status.as_ref());
            let mut do_check = false;
            egui::Window::new("About Lintel")
                .open(&mut self.showing_about)
                .collapsible(false)
                .show(ui, |ui| {
                    ui.label("Lintel — free, offline architectural drafting.");
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("MIT OR Apache-2.0. Your plan stays on your disk.");
                    ui.separator();
                    ui.label(status);
                    ui.hyperlink_to(
                        "github.com/Perchemop/lintel-desktop",
                        format!("https://github.com/{}", about::REPO),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Release notes…").clicked() {
                            self.showing_notes = true;
                        }
                        if ui.button("Check for updates").clicked() {
                            do_check = true;
                        }
                    });
                });
            if do_check {
                self.start_update_check();
            }
        }
        about::release_notes_window(ui, &mut self.showing_notes);
        about::update_window(ui, &mut self.update_open, self.update_status.as_ref());
        if self.export_open {
            // The window edits the spec in place and only reports that the
            // user asked for an export, so the closure never needs `*self`
            // (it is already borrowed by `.open`).
            let export_spec = &mut self.export_spec;
            let mut do_export = false;
            egui::Window::new("Export sheet")
                .open(&mut self.export_open)
                .collapsible(false)
                .show(ui, |ui| {
                    ui.label("Paper size:");
                    ui.radio_value(
                        &mut export_spec.paper,
                        lintel_core::pdf::PaperSize::A3,
                        "A3",
                    );
                    ui.radio_value(
                        &mut export_spec.paper,
                        lintel_core::pdf::PaperSize::A4,
                        "A4",
                    );
                    ui.label("Scale 1:N:");
                    ui.add(
                        egui::DragValue::new(&mut export_spec.scale_denominator)
                            .range(20..=2000)
                            .speed(10),
                    );
                    ui.label("Sheet:");
                    ui.text_edit_singleline(&mut export_spec.sheet_number);
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut export_spec.sheet_title);
                    if ui.button("Export PDF…").clicked() {
                        do_export = true;
                    }
                });
            if do_export {
                let path = PathBuf::from(format!("{}.pdf", self.export_spec.sheet_number));
                match lintel_core::pdf::render_plan(self.doc.project(), &self.export_spec, now) {
                    Ok(bytes) => {
                        if let Err(e) = std::fs::write(&path, &bytes) {
                            self.note_status(
                                format!("Could not write {}: {e}", path.display()),
                                now,
                            );
                        }
                    }
                    Err(e) => self.note_status(format!("Could not export the PDF: {e}"), now),
                }
            }
        }
    }

    fn zoom_to_fit(&mut self) {
        let geometry = PlanGeometry::of_level(self.doc.project().active_level().expect("level"));
        if let Some(bounds) = geometry.bounds() {
            let padding = 40.0;
            let rect_w = 1100.0;
            let rect_h = 700.0;
            let sx = (rect_w - 2.0 * padding) / bounds.width().max(1.0);
            let sy = (rect_h - 2.0 * padding) / bounds.height().max(1.0);
            let scale = sx.min(sy).clamp(0.02, 12.0) as f32;
            self.view.px_per_mm = scale;
            self.view.centre = bounds.centre();
        }
    }
}

fn now_epoch_s() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_secs() as i64
}

fn main() {
    // Capture a crash report before anything else can panic (guide section 7).
    crash_report::install_panic_hook("lintel", env!("CARGO_PKG_VERSION"), &about::crash_log_dir());
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Lintel — architectural drafting")
            .with_inner_size(egui::Vec2 {
                x: 1300.0,
                y: 850.0,
            }),
        ..Default::default()
    };
    let app = LintelApp::new();
    eframe::run_native("Lintel", native_options, Box::new(|_cc| Ok(Box::new(app))))
        .expect("eframe run failed");
}
