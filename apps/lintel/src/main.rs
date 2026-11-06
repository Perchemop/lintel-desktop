//! Lintel desktop shell — eframe/egui GUI wired to `lintel-core` commands.
//! Every user interaction goes through a `Command`; the core model updates
//! headlessly, and the UI re-renders from the new state (addendum K.3).

use crate::canvas::{Canvas, Draft, View, screen_to_world};
use crate::inspector::{InspectorDraft, inspector};
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
    /// Draft state for the Inspector's new-storey fields.
    inspector_draft: InspectorDraft,
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
            inspector_draft: InspectorDraft {
                // A sensible default floor-to-floor height for a new storey.
                new_level_elevation: 3000.0,
                ..InspectorDraft::default()
            },
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

    /// The fixed path used by Open / Save / Save As. Native file dialogs are
    /// out of scope for this build (see DECISIONS.md D-025).
    const WORKFLOW_PATH: &str = "workflow.lintel.json";

    /// Start a brand-new empty plan, clearing selection, draft and message.
    fn new_plan(&mut self) {
        self.doc = Document::new("Untitled plan", now_epoch_s());
        self.selection = None;
        self.draft = Draft::default();
        self.status = None;
    }

    /// Open the plan at the fixed workflow path.
    fn open_plan(&mut self, now: i64) {
        match Document::open(PathBuf::from(Self::WORKFLOW_PATH)) {
            Ok(loaded) => {
                self.doc = loaded;
                self.selection = None;
                self.draft = Draft::default();
                self.view = View::default();
            }
            Err(e) => self.note_status(format!("Could not open the plan: {e}"), now),
        }
    }

    /// Save the document to its current path.
    fn save_plan(&mut self, now: i64) {
        if let Err(e) = self.doc.save() {
            self.note_status(format!("Could not save: {e}"), now);
        }
    }

    /// Save the document under the fixed workflow path.
    fn save_plan_as(&mut self, now: i64) {
        if let Err(e) = self.doc.save_as(PathBuf::from(Self::WORKFLOW_PATH)) {
            self.note_status(format!("Could not save: {e}"), now);
        }
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
                    if ui
                        .button("New plan")
                        .on_hover_text("Start a new empty plan (Ctrl+N)")
                        .clicked()
                    {
                        self.new_plan();
                    }
                    ui.separator();
                    if ui
                        .button("Open…")
                        .on_hover_text("Open a saved Lintel plan from disk (Ctrl+O)")
                        .clicked()
                    {
                        self.open_plan(now);
                    }
                    if ui
                        .button("Save")
                        .on_hover_text("Save the current plan (Ctrl+S)")
                        .clicked()
                    {
                        self.save_plan(now);
                    }
                    if ui
                        .button("Save As…")
                        .on_hover_text("Save the current plan under a new name (Ctrl+Shift+S)")
                        .clicked()
                    {
                        self.save_plan_as(now);
                    }
                    ui.separator();
                    if ui
                        .button("Export sheet…")
                        .on_hover_text("Render the active level as a PDF sheet (Ctrl+E)")
                        .clicked()
                    {
                        self.export_open = true;
                    }
                    ui.separator();
                    if ui.button("Exit").on_hover_text("Close Lintel").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    ui.add_enabled(self.doc.can_undo(), |ui: &mut egui::Ui| {
                        let response = ui
                            .button("Undo")
                            .on_hover_text("Undo the last change (Ctrl+Z)");
                        if response.clicked() {
                            self.doc.undo(now_epoch_s());
                            self.selection = None;
                            self.draft = Draft::default();
                        }
                        response
                    });
                    ui.add_enabled(self.doc.can_redo(), |ui: &mut egui::Ui| {
                        let response = ui
                            .button("Redo")
                            .on_hover_text("Redo the last undone change (Ctrl+Y)");
                        if response.clicked() {
                            self.doc.redo(now_epoch_s());
                            self.selection = None;
                            self.draft = Draft::default();
                        }
                        response
                    });
                });
                ui.menu_button("View", |ui| {
                    if ui
                        .button("Zoom to fit")
                        .on_hover_text("Zoom the canvas to fit the whole plan (F)")
                        .clicked()
                    {
                        self.zoom_to_fit();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui
                        .button("About Lintel…")
                        .on_hover_text("Show the About dialog with version and licence")
                        .clicked()
                    {
                        self.showing_about = true;
                    }
                    if ui
                        .button("Release notes…")
                        .on_hover_text("Show the full release history")
                        .clicked()
                    {
                        self.showing_notes = true;
                    }
                    ui.separator();
                    if ui.button("Clear message").on_hover_text("Dismiss the current status message").clicked()
                    {
                        self.status = None;
                    }
                    ui.separator();
                    if ui
                        .button("Check for updates…")
                        .on_hover_text("Check GitHub Releases for a newer version")
                        .clicked()
                    {
                        self.start_update_check();
                        self.showing_about = true;
                    }
                });
            });
            let summary = plan_summary(self.doc.project());
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Tool:");
                let tool_response = egui::ComboBox::from_label("Tool")
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
                tool_response.response.on_hover_text(
                    "Choose the active tool (W=Wall S=Select D=Door N=Window M=Dimension)",
                );
                ui.separator();
                ui.label(format!("Zoom: {}%", self.view.zoom_percent()));
                ui.separator();
                ui.label(egui::RichText::new(summary.clone()).small());
                ui.label(
                    egui::RichText::new("Shortcuts: W/S/D/N/M tools · Ctrl+N/O/S/Shift+S/E file · Z/Y undo-redo · F fit · Esc cancel · Del delete")
                        .small()
                        .color(egui::Color32::GRAY),
                );
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
                &summary,
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
                    if let Some(cmd) = inspector(
                        ui,
                        &self.doc,
                        &self.selection,
                        &mut self.snap,
                        &mut self.inspector_draft,
                    ) {
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
                Some(Hit::Dimension(id)) => {
                    // Dragging a dimension line moves it away from what it
                    // measures by changing the offset. The component of the
                    // drag perpendicular to the dimension line is used, so the
                    // line follows the pointer without sliding lengthways.
                    let level = self.doc.project().active_level().expect("level");
                    let dimension = level.dimension(id).expect("dimension").clone();
                    let shift = match dimension.kind {
                        lintel_core::dimension::DimensionKind::LinearHorizontal => mm.y,
                        lintel_core::dimension::DimensionKind::LinearVertical => mm.x,
                        lintel_core::dimension::DimensionKind::Aligned => {
                            let normal = (dimension.end - dimension.start)
                                .normalized()
                                .perpendicular();
                            mm.dot(normal)
                        }
                    };
                    // An offset must stay positive; clamping keeps the line on
                    // the far side of the measurement instead of erroring.
                    let new_offset = (dimension.offset + shift).max(1.0);
                    let _ = self.doc.apply(
                        Command::SetDimensionOffset {
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

        // Plain letter shortcuts must not fire while a modifier is held, or
        // Ctrl+S would also switch to the Select tool.
        let plain_keys =
            ui.input(|i| !i.modifiers.command && !i.modifiers.ctrl && !i.modifiers.alt);

        // Keyboard shortcuts apply when nothing has focus, or when the canvas
        // itself has it — but not while the user is typing in a text field.
        // (The canvas is focusable so screen readers and Tab can reach it.)
        let keyboard_scope =
            ui.memory(|m| m.focused().is_none_or(|id| Some(id) == response.widget_id));
        if keyboard_scope {
            // File shortcuts (Ctrl/Cmd + key).
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) {
                self.new_plan();
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O)) {
                self.open_plan(now);
            }
            if ui
                .input(|i| i.modifiers.command && i.key_pressed(egui::Key::S) && !i.modifiers.shift)
            {
                self.save_plan(now);
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S) && i.modifiers.shift)
            {
                self.save_plan_as(now);
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::E)) {
                self.export_open = true;
            }

            // Undo / redo.
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

            // Cancellation and deletion.
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

            // Plain tool / view shortcuts.
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::W)) {
                self.tool = Tool::Wall;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::S)) {
                self.tool = Tool::Select;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::D)) {
                self.tool = Tool::Door;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::N)) {
                self.tool = Tool::Window;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::M)) {
                self.tool = Tool::Dimension;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::G)) {
                self.snap.grid_enabled = !self.snap.grid_enabled;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::O)) {
                self.snap.ortho = !self.snap.ortho;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::F)) {
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
                    )
                    .on_hover_text("Export on an A3 sheet");
                    ui.radio_value(
                        &mut export_spec.paper,
                        lintel_core::pdf::PaperSize::A4,
                        "A4",
                    )
                    .on_hover_text("Export on an A4 sheet");
                    ui.label("Scale 1:N:");
                    ui.add(
                        egui::DragValue::new(&mut export_spec.scale_denominator)
                            .range(20..=2000)
                            .speed(10),
                    )
                    .on_hover_text("Scale denominator 1:N, from 1:20 to 1:2000");
                    ui.label("Sheet:");
                    ui.text_edit_singleline(&mut export_spec.sheet_number)
                        .on_hover_text("Sheet number printed on the exported sheet");
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut export_spec.sheet_title)
                        .on_hover_text("Sheet title printed on the exported sheet");
                    if ui
                        .button("Export PDF…")
                        .on_hover_text("Export the plan as a scaled PDF sheet")
                        .clicked()
                    {
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

/// One-line description of the active level. Used both as the canvas's
/// screen-reader label (guide II.21.2) and as a visible toolbar summary.
fn plan_summary(project: &lintel_core::project::Project) -> String {
    use lintel_core::opening::OpeningKind;

    let Some(level) = project.active_level() else {
        return "No active level".to_string();
    };
    let plural = |n: usize| if n == 1 { "" } else { "s" };
    let doors = level
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Door)
        .count();
    let windows = level
        .openings
        .iter()
        .filter(|o| o.kind == OpeningKind::Window)
        .count();
    format!(
        "{}: {} wall{}, {} door{}, {} window{}, {} dimension{}",
        level.name,
        level.walls.len(),
        plural(level.walls.len()),
        doors,
        plural(doors),
        windows,
        plural(windows),
        level.dimensions.len(),
        plural(level.dimensions.len()),
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use lintel_core::dimension::DimensionKind;
    use lintel_core::opening::OpeningKind;

    #[test]
    fn an_empty_level_summarises_to_zeroes() {
        let doc = Document::new("Plan", 1);
        assert_eq!(
            plan_summary(doc.project()),
            "Level 0: 0 walls, 0 doors, 0 windows, 0 dimensions"
        );
    }

    #[test]
    fn the_summary_uses_singular_words_for_single_items() {
        let mut doc = Document::new("Plan", 1);
        let wall = doc
            .apply(
                Command::AddWall {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(4000.0, 0.0),
                    thickness: 200.0,
                    height: 2800.0,
                },
                2,
            )
            .expect("add wall")
            .wall
            .expect("wall id");
        doc.apply(
            Command::PlaceOpening {
                kind: OpeningKind::Door,
                host: wall,
                offset: 2000.0,
            },
            3,
        )
        .expect("place door");
        doc.apply(
            Command::PlaceDimension {
                kind: DimensionKind::LinearHorizontal,
                start: Point2::new(0.0, 0.0),
                end: Point2::new(4000.0, 0.0),
                offset: 500.0,
            },
            4,
        )
        .expect("place dimension");

        assert_eq!(
            plan_summary(doc.project()),
            "Level 0: 1 wall, 1 door, 0 windows, 1 dimension"
        );
    }

    #[test]
    fn the_summary_follows_the_active_level() {
        let mut doc = Document::new("Plan", 1);
        let applied = doc
            .apply(
                Command::AddLevel {
                    name: "First floor".to_string(),
                    elevation: 3000.0,
                },
                2,
            )
            .expect("add level");
        assert!(
            plan_summary(doc.project()).starts_with("First floor:"),
            "a new storey becomes active: {}",
            plan_summary(doc.project())
        );

        // Switching back to the ground storey is reflected immediately.
        let ground = lintel_core::ids::LevelId(1);
        assert_ne!(ground, applied.level.expect("level id"));
        doc.apply(Command::SetActiveLevel { id: ground }, 3)
            .expect("set active level");
        assert!(plan_summary(doc.project()).starts_with("Level 0:"));
    }
}
