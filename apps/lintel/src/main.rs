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

mod canvas;
mod inspector;
mod theme;

const MIN_WALL_LENGTH_DRAG: f64 = 2.0;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Select,
    Wall,
    Door,
    Window,
}

impl Tool {
    fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Wall => "Wall",
            Tool::Door => "Door",
            Tool::Window => "Window",
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
    export_open: bool,
    export_spec: lintel_core::pdf::SheetSpec,
}

impl LintelApp {
    fn new() -> Self {
        let now = now_epoch_s();
        let doc = Document::new("Untitled plan", now);
        Self {
            doc,
            view: View::default(),
            tool: Tool::Wall,
            selection: None,
            snap: lintel_core::snap::SnapSettings::default(),
            draft: Draft::default(),
            showing_about: false,
            export_open: false,
            export_spec: lintel_core::pdf::SheetSpec::default(),
        }
    }
}

impl eframe::App for LintelApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = now_epoch_s();

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
                        && let Some(path) = rfd::FileDialog::new()
                            .add_filter("Lintel plan", &["lintel.json"])
                            .pick_file()
                    {
                        match Document::open(&path) {
                            Ok(loaded) => {
                                self.doc = loaded;
                                self.selection = None;
                                self.view = View::default();
                            }
                            Err(e) => eprintln!("open: {e}"),
                        }
                    }
                    if ui.button("Save").clicked()
                        && let Err(e) = self.doc.save()
                    {
                        eprintln!("save: {e}");
                    }
                    if ui.button("Save As…").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .add_filter("Lintel plan", &["lintel.json"])
                            .save_file()
                    {
                        if let Err(e) = self.doc.save_as(path) {
                            eprintln!("save as: {e}");
                        }
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
                    });
                ui.separator();
                ui.label(format!("Zoom: {}%", self.view.zoom_percent()));
            });

            let (response, rect) = Canvas::show(
                ui,
                &self.view,
                &PlanGeometry::of_level(self.doc.project().active_level().expect("level")),
                self.selection,
                &self.snap,
                &self.draft,
            );
            self.handle_canvas_input(ui, response, rect, now);

            egui::Window::new("Inspector")
                .default_pos(egui::Pos2::new(4.0, 40.0))
                .show(ui, |ui| {
                    if let Some(cmd) = inspector(ui, &self.doc, &self.selection, &self.snap) {
                        self.doc.apply(cmd, now);
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
                    if !self.draft.active {
                        self.draft = Draft {
                            active: true,
                            start: world,
                            current: world,
                        };
                    } else {
                        let end = world;
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
                                .map_err(|e| eprintln!("wall: {e}"))
                                .ok();
                            if let Some(applied) = applied {
                                if let Some(wall_id) = applied.wall {
                                    self.selection = Some(Hit::Wall(wall_id));
                                }
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

        if let Some(delta) = response.drag_delta {
            if let Tool::Select = self.tool {
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
        }

        if let Some(delta) = response.pan_delta {
            self.view.centre = Point2::new(
                self.view.centre.x - delta.x as f64 / self.view.px_per_mm as f64,
                self.view.centre.y + delta.y as f64 / self.view.px_per_mm as f64,
            );
        }

        if let Some((factor, anchor)) = response.zoom {
            if let Some(hover) = response.pointer_world {
                let before = hover;
                self.view.px_per_mm =
                    (self.view.px_per_mm as f64 * factor).clamp(0.02, 12.0) as f32;
                let after = screen_to_world(&self.view, rect, anchor);
                self.view.centre = Point2::new(
                    self.view.centre.x + (before.x - after.x),
                    self.view.centre.y + (before.y - after.y),
                );
            }
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
            {
                if self.doc.can_undo() {
                    self.doc.undo(now);
                    self.selection = None;
                    self.draft = Draft::default();
                }
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Y)) {
                if self.doc.can_redo() {
                    self.doc.redo(now);
                    self.selection = None;
                    self.draft = Draft::default();
                }
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
            egui::Window::new("About Lintel")
                .open(&mut self.showing_about)
                .collapsible(false)
                .show(ui, |ui| {
                    ui.label("Lintel — free, offline architectural drafting.");
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("MIT OR Apache-2.0. Your plan stays on your disk.");
                    ui.separator();
                    let _ = ui.link("github.com/Perchemop/lintel-desktop");
                });
        }
        if self.export_open {
            egui::Window::new("Export sheet")
                .open(&mut self.export_open)
                .collapsible(false)
                .show(ui, |ui| {
                    ui.label("Paper size:");
                    ui.radio_value(
                        &mut self.export_spec.paper,
                        lintel_core::pdf::PaperSize::A3,
                        "A3",
                    );
                    ui.radio_value(
                        &mut self.export_spec.paper,
                        lintel_core::pdf::PaperSize::A4,
                        "A4",
                    );
                    ui.label("Scale 1:N:");
                    ui.add(
                        egui::DragValue::new(&mut self.export_spec.scale_denominator)
                            .range(20..=2000)
                            .speed(10),
                    );
                    ui.label("Sheet:");
                    ui.text_edit_singleline(&mut self.export_spec.sheet_number);
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.export_spec.sheet_title);
                    if ui.button("Export PDF…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PDF", &["pdf"])
                            .set_file_name(format!("{}.pdf", self.export_spec.sheet_number))
                            .save_file()
                        {
                            match lintel_core::pdf::render_plan(
                                self.doc.project(),
                                &self.export_spec,
                                now,
                            ) {
                                Ok(bytes) => {
                                    if std::fs::write(&path, &bytes).is_err() {
                                        eprintln!("write pdf: {}", path.display());
                                    }
                                }
                                Err(e) => eprintln!("export: {e}"),
                            }
                        }
                    }
                });
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
