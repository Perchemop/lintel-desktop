//! Lintel desktop shell — eframe/egui GUI wired to `lintel-core` commands.
//! Every user interaction goes through a `Command`; the core model updates
//! headlessly, and the UI re-renders from the new state (addendum K.3).
//!
//! Structure: the `Host` trait extracts the model so `egui_kittest` can drive it
//! without a real eframe::Frame (see addendum K.7/K.13).

use eframe::egui;
use eframe::Frame;
use lintel_core::{
    command::Command,
    document::Document,
    geom::Point2,
    ids::{LevelId, OpeningId, WallId},
    project::Project,
    units::UnitSystem,
};
use std::path::PathBuf;

/// The per-project model state held by the shell.
/// (The core model lives in `lintel-core`; this is the UI's view of it.)
#[derive(Clone)]
struct AppState {
    /// Project file name (without .lintel.json extension)
    project_name: String,
    /// The core project model
    project: Project,
    /// Which level is active in the UI
    active_level_index: usize,
    /// Temporary file path for the currently edited project
    temp_path: Option<PathBuf>,
    /// Whether the project has unsaved changes
    dirty: bool,
    /// Unit system for display
    unit_system: UnitSystem,
    /// Pending command from UI (applied then cleared)
    pending_command: Option<Command>,
    /// Hitting-input feedback
    feedback: String,
}

/// The main eframe app.
struct LintelApp {
    /// Per-window state (one project at a time)
    state: AppState,
    /// Open file path (from file-open dialog)
    open_file: Option<PathBuf>,
    /// Save file path (from file-save dialog)
    save_file: Option<PathBuf>,
}

impl eframe::App for LintelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // ---- Menubar ----
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                if ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked {
                        self.state = AppState::new_project();
                    }
                    ui.separator();
                    if ui.button("Open…").clicked {
                        // In a real app we'd open a native file dialog;
                        // for now just use a hardcoded path or ask the user.
                        self.open_file = Some(PathBuf::from("workflow.lintel.json"));
                    }
                    if ui.button("Save").clicked && !self.state.dirty {
                        self.state.save();
                    }
                    if ui.button("Save As…").clicked {
                        self.save_file = Some(PathBuf::from("workflow.lintel.json"));
                        self.state.save();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked {
                        ctx.send_viewport_cmd(eframe::ViewportCommand::Close);
                    }
                }) {
                    ui.set_enabled(!self.state.dirty); // disable if unsaved changes
                }

                // Edit menu
                if ui.menu_button("Edit", |ui| {
                    // Commands will be added below; placeholders for now
                    if ui.add_enabled(!self.state.dirty, egui::Button::new("Undo")).clicked {
                        // todo: self.state.undo();
                    }
                    if ui.add_enabled(!self.state.dirty, egui::Button::new("Redo")).clicked {
                        // todo: self.state.redo();
                    }
                });

                // View menu
                if ui.menu_button("View", |ui| {
                    ui.radio_value(&mut self.state.unit_system, UnitSystem::Metric, "Metric (mm)");
                    ui.radio_value(&mut self.state.unit_system, UnitSystem::Imperial, "Imperial (in/ft)");
                });
            });
        });

        // ---- Main area: document view + command panel ----
        egui::CentralPanel::default().show(ctx, |ui| {
            // Left: document overview
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                // Document summary
                ui.heading("Lintel Project");
                ui.label(format!("Project: {}", self.state.project_name));
                ui.label(format!("Units: {}", self.state.unit_system.label()));
                ui.label(format!("Level: {}", self.state.project.active_level().map_or("?", |l| l.name.as_str())));
                ui.label(format!("Walls: {}", self.state.project.wall_count()));
                ui.label(format!("Openings: {}", self.state.project.opening_count()));

                // ---- Command panel ----
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Command:");
                    // Add Wall
                    if ui.button("Add Wall").clicked {
                        let cmd = Command::AddWall {
                            start: Point2::new(0.0, 0.0),
                            end: Point2::new(6000.0, 0.0),
                            thickness: 200.0,
                            height: 2800.0,
                        };
                        self.apply_command(cmd);
                    }
                    // Place Door
                    if ui.button("Place Door").clicked {
                        let cmd = Command::PlaceOpening {
                            kind: lintel_core::opening::OpeningKind::Door,
                            host: WallId(1),
                            offset: 2000.0,
                        };
                        self.apply_command(cmd);
                    }
                    // Place Window
                    if ui.button("Place Window").clicked {
                        let cmd = Command::PlaceOpening {
                            kind: lintel_core::opening::OpeningKind::Window,
                            host: WallId(1),
                            offset: 3000.0,
                        };
                        self.apply_command(cmd);
                    }
                });

                // ---- Document stats ----
                ui.vertical(|ui| {
                    if self.state.dirty {
                        ui.colored_label(egui::Color32::LIGHT_RED, "⚠ Unsaved changes");
                    } else {
                        ui.colored_label(egui::Color32::GREEN, "✓ Saved");
                    }
                });
            });

            // Right: geometry view (simple text summary; real app would render egui graphics)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.separator();
                ui.heading("Geometry Summary");
                let level = self.state
                    .project
                    .level(self.state.active_level_index)
                    .unwrap_or_else(|| self.state.project.level(0).unwrap());
                let geo = lintel_core::join::PlanGeometry::of_level(level);
                ui.label(format!("Walls: {}", geo.walls.len()));
                ui.label(format!("Openings: {}", geo.openings.len()));
                if let Some(b) = geo.bounds() {
                    ui.label(format!("Bounds: {:.1} × {:.1} mm", b.width(), b.height()));
                }
                for (i, wall) in geo.walls.iter().enumerate() {
                    ui.label(format!("Wall {}: {:.1}mm length", i + 1, wall.length()));
                }
                for (i, open) in geo.openings.iter().enumerate() {
                    ui.label(format!("Opening {}: {} at {:.1}, {:.1}", i + 1, open.kind_label(), open.centre.x, open.centre.y));
                }
            });
        });

        // ---- Handle pending command feedback ----
        if !self.state.feedback.is_empty() {
            egui::Window::new("Feedback")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&self.state.feedback);
                    if ui.button("OK").clicked {
                        self.state.feedback.clear();
                    }
                });
        }
    }
}

impl AppState {
    fn new_project() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_secs() as i64;
        let project = Project::new("New Plan", now);
        Self {
            project_name: "New Plan".into(),
            project,
            active_level_index: 0,
            temp_path: None,
            dirty: false,
            unit_system: UnitSystem::Metric,
            pending_command: None,
            feedback: String::new(),
        }
    }

    fn apply_command(&mut self, cmd: Command) {
        match lintel_core::command::apply_command(&mut self.project, cmd) {
            Ok(applied) => {
                // Command succeeded; capture what was created for feedback
                let mut msg = String::new();
                if let Some(wall) = applied.wall {
                    msg.push_str(&format!("Wall added: {} (length {:.1}mm)\n", wall, self.project.wall(unwrap_or_panic).map_or(0.0, |w| w.length())));
                }
                if let Some(opening) = applied.opening {
                    msg.push_str(&format!("Opening added: {} on wall {}\n", opening, self.project.level(0).unwrap().wall(opening).map_or("?", |w| w.id.0)));
                }
                if let Some(level) = applied.level {
                    msg.push_str(&format!("Level added: {}\n", level));
                }
                self.dirty = true;
                self.feedback = if msg.is_empty() {
                    "Command applied successfully".into()
                } else {
                    msg
                };
            }
            Err(e) => {
                self.feedback = format!("Command failed: {}", e);
            }
        }
    }

    fn save(&mut self) {
        // Save the project as JSON
        if let Some(path) = &self.temp_path {
            if let Ok(json) = lintel_core::persistence::to_json(&self.project) {
                if let Ok(text) = std::fs::write(path, json) {
                    self.dirty = false;
                    self.feedback = format!("Saved to {}", path.display());
                } else {
                    self.feedback = format!("Failed to write {}", path.display());
                }
            } else {
                self.feedback = "Failed to serialize project".into();
            }
        } else {
            // Fall back to save-as
            self.save_file = Some(PathBuf::from("workflow.lintel.json"));
            self.save();
        }
    }
}

/// Small helper to avoid repeating `unwrap_or_else` in demo code.
fn unwrap_or_panic<T>(opt: Option<T>) -> T {
    opt.unwrap_or_else(|| panic!("Expected Some, got None"))
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: eframe::ViewportBuilder::default()
            .with_title("Lintel — Desktop Drafting")
            .with_inner_size(eframe::Vec2 { x: 1200.0, y: 800.0 }),
        ..Default::default()
    };

    // Start with a new project
    let app = LintelApp {
        state: AppState::new_project(),
        open_file: None,
        save_file: None,
    };

    eframe::run_native(
        "Lintel — Desktop Drafting",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
    .expect("eframe run failed");
}