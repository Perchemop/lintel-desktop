//! Lintel desktop shell — eframe/egui GUI wired to `lintel-core` commands.
//! Every user interaction goes through a `Command`; the core model updates
//! headlessly, and the UI re-renders from the new state (addendum K.3).

use crate::canvas::{
    Canvas, Draft, HoveredEntity, Selection, SelectionBand, SelectionModeKind, SnapModeKind,
    SnapModes, View, entity_of, resolve_snap, screen_to_world,
};
use crate::command_palette::CommandPalette;
use crate::inspector::InspectorDraft;
use crate::panels::{Panel, PanelVisibility};
use crate::theme::SPEC;
use eframe::egui;
use eframe::egui::{SidePanel, TopBottomPanel};
use lintel_core::{
    BlockId,
    command::Command,
    document::Document,
    geom::{Bounds, Point2, Vec2},
    ids::{DimensionId, EntityId},
    join::{Hit, PlanGeometry},
};

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};

mod about;
mod app;
mod canvas;
mod command_palette;
mod components;
mod dimension_ui;
mod inspector;
mod layout;
mod constraints_panel;
mod layers;
mod output;
mod panels;
mod release_notes;
mod status_bar;
#[cfg(test)]
mod tests;
mod theme;
#[cfg(test)]
mod theme_tests;

use crate::app::tool::Tool;

const MIN_WALL_LENGTH_DRAG: f64 = 2.0;

/// Main eframe app state.
pub struct LintelApp {
    doc: Document,
    view: View,
    tool: Tool,
    /// Everything the canvas has selected: a plain click replaces it, Shift
    /// adds to it, Ctrl removes, and a rubber band sets it.
    selection: Selection,
    /// Which snap modes offer candidates while drawing.
    snap_modes: SnapModes,
    snap: lintel_core::snap::SnapSettings,
    /// The rubber-band selection drag in progress, if any.
    band: Option<SelectionBand>,
    /// What the snapper settled on this frame, for the status bar readout.
    last_snap: Option<crate::canvas::SnapReadout>,
    draft: Draft,
    palette: CommandPalette,
    showing_about: bool,
    showing_notes: bool,
    export_open: bool,
    export_spec: lintel_core::pdf::SheetSpec,
    /// Receives the result of an in-flight update check, if any.
    update_rx: Option<Receiver<update_check::UpdateStatus>>,
    /// The shared update checker, kept for the life of the app so its
    /// [`update_check::CACHE_TTL`] cache actually holds across checks. Shared
    /// behind a mutex because the blocking check runs on a worker thread.
    update_checker: std::sync::Arc<std::sync::Mutex<update_check::UpdateChecker>>,
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
    /// Last pointer position in world coordinates, for the status bar readout.
    pointer_world: Option<lintel_core::geom::Point2>,
    /// Currently selected block instance, if any.
    selected_block_instance: Option<BlockId>,
    /// Panel visibility state for the workbench shell (v0.22.0).
    panel_visibility: PanelVisibility,
    /// Runtime theme mode (v0.22.0).
    theme_mode: theme::ThemeMode,
    /// Canvas hover hints state (v0.22.0).
    canvas_hints: crate::canvas::CanvasHints,
    /// Active sheet name (none means "Plan").
    active_sheet: Option<String>,
    /// Settings dialog open flag.
    showing_settings: bool,
}

/// Project a pointer position onto a wall's centreline and return the
/// resulting offset along that centreline, clamped to the wall's length.
///
/// Opening placement uses this rather than the raw Euclidean distance from the
/// wall start: clicking off the centreline must still land on the wall.
pub(crate) fn wall_offset_along_centerline(wall: &lintel_core::wall::Wall, world: Point2) -> f64 {
    let wall_vec = wall.end - wall.start;
    let click_vec = world - wall.start;
    let t = click_vec.dot(wall_vec) / wall_vec.length_sq();
    wall_vec.length() * t.clamp(0.0, 1.0)
}

impl LintelApp {
    fn new() -> Self {
        let now = now_epoch_s();
        let doc = Document::new("Untitled plan", now);
        let update_checker =
            std::sync::Arc::new(std::sync::Mutex::new(update_check::UpdateChecker::new()));
        let mut app = Self {
            doc,
            view: View::default(),
            tool: Tool::Wall,
            selection: Selection::default(),
            snap_modes: SnapModes::default(),
            snap: lintel_core::snap::SnapSettings::default(),
            band: None,
            last_snap: None,
            draft: Draft::default(),
            palette: CommandPalette::default(),
            showing_about: false,
            showing_notes: false,
            export_open: false,
            export_spec: lintel_core::pdf::SheetSpec::default(),
            update_rx: None,
            update_checker,
            update_status: None,
            update_open: false,
            status: None,
            status_at: 0,
            inspector_draft: InspectorDraft {
                // A sensible default floor-to-floor height for a new storey.
                new_level_elevation: 3000.0,
                ..InspectorDraft::default()
            },
            pointer_world: None,
            selected_block_instance: None,
            panel_visibility: PanelVisibility::all_visible(),
            theme_mode: theme::ThemeMode::default(),
            canvas_hints: crate::canvas::CanvasHints::default(),
            active_sheet: None,
            showing_settings: false,
        };
        // One quiet check at startup; it fails soft when offline and never
        // blocks the UI thread.
        app.start_update_check();
        app
    }

    /// Kick off an update check on a worker thread. A no-op while one is
    /// already in flight.
    ///
    /// The shared [`update_check::UpdateChecker`] is reused, so the cached
    /// result stands for `CACHE_TTL`: repeated checks inside that window return
    /// the cache instead of hitting the GitHub API again.
    fn start_update_check(&mut self) {
        if self.update_rx.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.update_rx = Some(rx);
        let checker = std::sync::Arc::clone(&self.update_checker);
        std::thread::spawn(move || {
            let status = match checker.lock() {
                Ok(mut checker) => checker.check(about::REPO, env!("CARGO_PKG_VERSION")),
                // A poisoned lock only happens if a previous check panicked;
                // fail soft rather than panicking the UI thread too.
                Err(_) => {
                    update_check::UpdateStatus::Unknown("update checker lock poisoned".to_string())
                }
            };
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
        self.selection.clear();
        self.draft = Draft::default();
        self.band = None;
        self.status = None;
        self.selected_block_instance = None;
    }

    /// Open the plan at the fixed workflow path.
    fn open_plan(&mut self, now: i64) {
        match Document::open(PathBuf::from(Self::WORKFLOW_PATH)) {
            Ok(loaded) => {
                self.doc = loaded;
                self.selection.clear();
                self.draft = Draft::default();
                self.band = None;
                self.view = View::default();
                self.selected_block_instance = None;
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

    /// Select every entity on the active level. The domain runs the pick; this
    /// only keeps the answer.
    fn select_all(&mut self) {
        let geometry = PlanGeometry::of_level(self.doc.project().active_level().expect("level"));
        self.selection.pick_all(&geometry);
    }

    /// Select a block instance by id (keyboard palette entry point).
    ///
    /// Clears the ordinary entity selection first, matching canvas click
    /// behaviour: an instance is a distinct selection kind.
    pub fn select_block_instance(&mut self, id: Option<lintel_core::block::BlockId>) {
        self.selection.clear();
        self.selected_block_instance = id;
    }

    /// Delete every selected entity, reporting the first refusal.
    ///
    /// Construction guides own their own delete command, so a selection that
    /// holds one deletes it too.
    ///
    /// A selected block instance takes precedence: it has no entry in the
    /// ordinary selection set, so delete it directly by id and clear the
    /// selection. A `BlockInstanceNotFound` refusal (the instance is already
    /// gone) is reported like any other delete failure, but the selection is
    /// cleared regardless.
    fn delete_selection(&mut self, now: i64) {
        if let Some(id) = self.selected_block_instance.take() {
            if let Err(e) = self.doc.apply(
                Command::DeleteBlockInstance {
                    block_instance_id: id,
                },
                now,
            ) {
                self.note_status(format!("Could not delete: {e}"), now);
            }
            return;
        }
        let mut refusal: Option<String> = None;
        for id in self.selection.iter() {
            let command = match id {
                EntityId::Wall(id) => Command::DeleteWall { id },
                EntityId::Opening(id) => Command::DeleteOpening { id },
                EntityId::Dimension(id) => Command::DeleteDimension { id },
                EntityId::Label(id) => Command::DeleteLabel { id },
                EntityId::Construction(id) => Command::DeleteConstruction { id },
            };
            if let Err(e) = self.doc.apply(command, now)
                && refusal.is_none()
            {
                refusal = Some(format!("Could not delete: {e}"));
            }
        }
        if let Some(message) = refusal {
            self.note_status(message, now);
        }
        self.selection.clear();
    }

    /// Copy every selected entity, leaving the copies selected.
    ///
    /// The level is cloned once, so reading an entity to copy never fights the
    /// mutable borrow of the document.
    fn duplicate_selection(&mut self, now: i64) {
        let offset = Vec2::new(1000.0, 1000.0);
        let mut copies: Vec<EntityId> = Vec::new();
        let mut refusal: Option<String> = None;
        let level = self.doc.project().active_level().cloned();
        let Some(level) = level else {
            return;
        };
        for id in self.selection.iter() {
            let command = match id {
                EntityId::Wall(id) => {
                    let Some(wall) = level.wall(id).cloned() else {
                        continue;
                    };
                    Command::AddWall {
                        start: wall.start + offset,
                        end: wall.end + offset,
                        thickness: wall.thickness,
                        height: wall.height,
                    }
                }
                EntityId::Dimension(id) => {
                    let Some(dimension) = level.dimension(id).cloned() else {
                        continue;
                    };
                    Command::PlaceDimension {
                        kind: dimension.kind,
                        start: dimension.start + offset,
                        end: dimension.end + offset,
                        offset: dimension.offset,
                    }
                }
                EntityId::Label(id) => {
                    let Some(label) = level.label(id).cloned() else {
                        continue;
                    };
                    Command::PlaceLabel {
                        anchor: label.anchor + offset,
                        text: label.text.clone(),
                    }
                }
                EntityId::Opening(id) => {
                    let Some(opening) = level.opening(id).cloned() else {
                        continue;
                    };
                    let Some(host) = level.wall(opening.host) else {
                        continue;
                    };
                    let host_length = host.length();
                    let new_offset = (opening.offset + 1000.0).min(host_length);
                    Command::PlaceOpening {
                        kind: opening.kind,
                        host: opening.host,
                        offset: new_offset,
                    }
                }
                // A construction guide has no copy command: it is a helper, and
                // duplicating one would clutter the plan rather than help it.
                EntityId::Construction(_) => continue,
            };
            match self.doc.apply(command, now) {
                Ok(applied) => {
                    if let Some(id) = applied.wall {
                        copies.push(EntityId::Wall(id));
                    }
                    if let Some(id) = applied.opening {
                        copies.push(EntityId::Opening(id));
                    }
                    if let Some(id) = applied.dimension {
                        copies.push(EntityId::Dimension(id));
                    }
                    if let Some(id) = applied.label {
                        copies.push(EntityId::Label(id));
                    }
                }
                Err(e) => {
                    if refusal.is_none() {
                        refusal = Some(format!("Duplicate rejected: {e}"));
                    }
                }
            }
        }
        if let Some(message) = refusal {
            self.note_status(message, now);
        }
        self.selection.set(copies);
    }

    /// Write the multi-sheet PDF drawing set next to the project file.
    fn export_drawing_set(&mut self, now: i64) {
        let path = PathBuf::from(format!("{}-set.pdf", self.export_spec.sheet_number));
        match lintel_core::pdf::render_set(self.doc.project(), &self.export_spec, now) {
            Ok(bytes) => {
                if let Err(e) = std::fs::write(&path, &bytes) {
                    self.note_status(format!("Could not write {}: {e}", path.display()), now);
                }
            }
            Err(e) => self.note_status(format!("Could not export the PDF: {e}"), now),
        }
    }

    /// Flip between Metric and Imperial.
    fn toggle_units(&mut self, now: i64) {
        let units = self.doc.project().units;
        let next = match units {
            lintel_core::units::UnitSystem::Metric => lintel_core::units::UnitSystem::Imperial,
            lintel_core::units::UnitSystem::Imperial => lintel_core::units::UnitSystem::Metric,
        };
        let _ = self.doc.apply(Command::SetUnits { units: next }, now);
    }

    /// Where a pointer position lands under the current snap settings.
    fn snap_at(&self, world: Point2, origin: Option<Point2>) -> Point2 {
        let geometry = PlanGeometry::of_level(self.doc.project().active_level().expect("level"));
        resolve_snap(world, &geometry, &self.snap_modes, origin, &self.snap).point
    }

    /// Simple hit test for block instances: returns the instance whose
    /// transformed definition bounds contain `world`.
    fn hit_block_instance(&self, world: Point2) -> Option<BlockId> {
        let level = self.doc.project().active_level()?;
        // Iterate in reverse so the topmost (last placed) instance wins.
        for instance in level.block_instances.iter().rev() {
            let def = level.block_def(instance.block_id);
            // Build a bounding box from the transformed local points. Fall back
            // to a small square around the instance origin when the definition
            // has no geometry of its own.
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
            } else {
                // Pad the bounds a little so thin geometry is still clickable.
                const PAD: f64 = 50.0;
                min_x -= PAD;
                min_y -= PAD;
                max_x += PAD;
                max_y += PAD;
            }
            if world.x >= min_x && world.x <= max_x && world.y >= min_y && world.y <= max_y {
                return Some(instance.id);
            }
        }
        None
    }

    /// Labels for the active level's layers, as `(LayerId, name)`.
    pub(crate) fn layer_labels(&self) -> Vec<(lintel_core::ids::LayerId, String)> {
        self.doc
            .project()
            .active_level()
            .map(|level| {
                level
                    .layers
                    .iter()
                    .map(|layer| (layer.id, layer.name.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The layer a picker form starts on: the first one that is not the
    /// protected default, so the destructive actions are never pointed at it.
    pub(crate) fn first_assignable_layer_index(&self) -> usize {
        let layers = self.layer_labels();
        layers
            .iter()
            .position(|(id, _)| *id != lintel_core::ids::LayerId::DEFAULT)
            .unwrap_or(0)
    }

    /// Apply one command, reporting a refusal in the status bar.
    ///
    /// Returns what the command created, so a follow-up command can name a new
    /// entity without the UI guessing an id.
    pub(crate) fn run_command(
        &mut self,
        command: Command,
        now: i64,
    ) -> Option<lintel_core::command::Applied> {
        match self.doc.apply(command, now) {
            Ok(applied) => Some(applied),
            Err(e) => {
                self.note_status(format!("Command rejected: {e}"), now);
                None
            }
        }
    }

    /// Apply a form's commands in order, rewriting the new-layer placeholder to
    /// the id the create returned, so its follow-up properties land on the new
    /// layer instead of on whatever layer happens to be there.
    pub(crate) fn run_commands(&mut self, commands: Vec<Command>, now: i64) -> bool {
        let mut created: Option<lintel_core::ids::LayerId> = None;
        let mut all_ok = true;
        for command in commands {
            let command = match command {
                Command::SetLayerProperty { layer_id, property }
                    if layer_id == crate::command_palette::NEW_LAYER_ID =>
                {
                    match created {
                        Some(id) => Command::SetLayerProperty {
                            layer_id: id,
                            property,
                        },
                        None => {
                            all_ok = false;
                            continue;
                        }
                    }
                }
                other => other,
            };
            match self.run_command(command, now) {
                Some(applied) => {
                    if let Some(id) = applied.layer {
                        created = Some(id);
                    }
                }
                None => all_ok = false,
            }
        }
        all_ok
    }

    /// Step one undo back.
    pub(crate) fn undo_command(&mut self, now: i64) {
        if self.doc.undo(now) {
            self.selection.clear();
            self.draft = Draft::default();
            self.band = None;
        }
    }

    /// Step one redo forward.
    pub(crate) fn redo_command(&mut self, now: i64) {
        if self.doc.redo(now) {
            self.selection.clear();
            self.draft = Draft::default();
            self.band = None;
        }
    }

    /// Open or close the command palette, sharing one rule everywhere.
    fn toggle_palette(&mut self) {
        if self.palette.is_open() {
            self.palette.close();
        } else {
            self.palette.open();
        }
    }
}

impl eframe::App for LintelApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = now_epoch_s();

        // Apply the runtime theme mode to the egui context (T-225).
        self.apply_theme_mode(ui.ctx());

        // Global keyboard shortcuts.
        self.handle_global_shortcuts(ui);

        // Workbench shell layout.
        TopBottomPanel::top("editor_tabs").show(ui, |ui| {
            self.editor_tabs(ui);
        });

        SidePanel::left("activity_bar")
            .resizable(false)
            .default_width(48.0)
            .show(ui, |ui| {
                self.activity_bar(ui);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            self.canvas_panel(ui, now);
        });

        TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .default_height(28.0)
            .show(ui, |ui| {
                self.status_bar(ui, now);
            });

        // Dialogs render as floating windows on top of everything.
        self.dialogs(ui, now);

        // Command palette renders as a modal overlay on the context.
        crate::command_palette::show(ui.ctx(), self, now);
    }
}

impl LintelApp {
    /// Apply the runtime theme mode to the egui context.
    fn apply_theme_mode(&mut self, ctx: &egui::Context) {
        let visuals = match self.theme_mode {
            theme::ThemeMode::Light => egui::Visuals::light(),
            theme::ThemeMode::Dark => egui::Visuals::dark(),
            theme::ThemeMode::HighContrast => {
                let mut v = egui::Visuals::dark();
                v.widgets.noninteractive.bg_fill = crate::theme::BASE_PAPER;
                v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, crate::theme::TEXT_PRIMARY);
                v
            }
            theme::ThemeMode::System => {
                // egui 0.36 doesn't expose platform dark-mode detection via
                // Context::style(); default to light and let the user switch
                // manually via the theme selector.
                egui::Visuals::light()
            }
        };
        ctx.set_visuals(visuals);
    }

    /// Handle global keyboard shortcuts (T-227: Ctrl+T theme cycle).
    fn handle_global_shortcuts(&mut self, ui: &egui::Ui) {
        let input = ui.input(|i| i.clone());
        if input.modifiers.command && input.key_pressed(egui::Key::T) {
            self.theme_mode = match self.theme_mode {
                theme::ThemeMode::Light => theme::ThemeMode::Dark,
                theme::ThemeMode::Dark => theme::ThemeMode::HighContrast,
                theme::ThemeMode::HighContrast => theme::ThemeMode::System,
                theme::ThemeMode::System => theme::ThemeMode::Light,
            };
        }
    }

    /// Editor tabs: Plan (pinned) + Sheet tabs.
    fn editor_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Plan tab — always present, pinned.
            let plan_active = self.active_sheet().map_or(true, |s| s == "Plan");
            let plan_btn = egui::Button::new("Plan");
            let plan_btn = if plan_active {
                plan_btn.fill(SPEC.component.button.primary_sm.fill)
            } else {
                plan_btn
            };
            if ui.add(plan_btn).clicked() {
                self.active_sheet = Some("Plan".to_string());
            }
            ui.separator();

            // Sheet tabs from the drawing set.
            for sheet in self.drawing_set_sheets() {
                let active = self.active_sheet().as_deref() == Some(sheet.as_str());
                let sheet_btn = egui::Button::new(sheet.as_str());
                let sheet_btn = if active {
                    sheet_btn.fill(SPEC.component.button.primary_sm.fill)
                } else {
                    sheet_btn
                };
                if ui.add(sheet_btn).clicked() {
                    self.active_sheet = Some(sheet.clone());
                }
                ui.separator();
            }

            // + button: add new sheet.
            if ui.button("+").clicked() {
                // TODO: add new sheet to drawing set
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("Lintel v").small());
                ui.label(egui::RichText::new(env!("CARGO_PKG_VERSION")).small().monospace());
            });
        });
        ui.separator();
    }

    /// Activity bar: tool icons + panel toggles.
    fn activity_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 8.0;

            // Tool icons.
            for tool in crate::app::tool::Tool::ALL {
                let selected = self.tool == tool;
                let btn = crate::components::buttons::StyledButton::new(tool.label())
                    .role(tool.role())
                    .size(crate::components::buttons::ButtonSize::Sm)
                    .selected(selected);
                if ui.add(btn).on_hover_text(format!("{} ({})", tool.label(), tool.shortcut())).clicked() {
                    self.tool = tool;
                }
            }

            ui.separator();

            // Panel toggles.
            for panel in Panel::ALL {
                let visible = self.panel_visible(panel);
                let btn = crate::components::buttons::StyledButton::new(panel.title())
                    .role(crate::components::buttons::ButtonRole::Ghost)
                    .size(crate::components::buttons::ButtonSize::Sm)
                    .selected(visible);
                if ui.add(btn).on_hover_text(panel.title()).clicked() {
                    self.toggle_panel(panel);
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("⚙").on_hover_text("Settings").clicked() {
                    self.showing_settings = true;
                }
            });
        });
    }

    /// Render the canvas in the central panel: walls, openings, dimensions,
    /// snap markers, empty state, and canvas hints.
    fn canvas_panel(&mut self, ui: &mut egui::Ui, now: i64) {
        let level = self
            .doc
            .project()
            .active_level()
            .expect("level");
        let geometry = PlanGeometry::of_level(level);
        let summary = crate::plan_summary(self.doc.project());

        let (resp, rect) = Canvas::show(
            ui,
            &self.view,
            &geometry,
            &self.selection,
            &self.snap_modes,
            &self.snap,
            &self.draft,
            &summary,
            self.band.as_ref(),
        );

        let dimension_painter = ui.painter_at(rect);
        crate::dimension_ui::render_dimensions(
            &dimension_painter,
            rect,
            &self.view,
            &geometry,
            &self.selection,
            level,
        );

        // Empty state.
        if geometry.walls.is_empty() && !self.draft.active {
            let msg1 = "Pick the Wall tool (W) and click twice to draw a wall";
            let msg2 = "Then place doors and windows from the toolbar";
            let center = rect.center();
            dimension_painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                msg1,
                egui::FontId::proportional(16.0),
                SPEC.component.canvas.text,
            );
            dimension_painter.text(
                center + egui::Vec2::new(0.0, 22.0),
                egui::Align2::CENTER_CENTER,
                msg2,
                egui::FontId::proportional(16.0),
                SPEC.component.canvas.text,
            );
        }

        // Highlight selected block instance.
        if let Some(id) = self.selected_block_instance
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
            let a_pt = screen_to_world(&self.view, rect, egui::Pos2::new(min_x as f32, min_y as f32));
            let b_pt = screen_to_world(&self.view, rect, egui::Pos2::new(max_x as f32, max_y as f32));
            let a = egui::Pos2::new(a_pt.x as f32, a_pt.y as f32);
            let b = egui::Pos2::new(b_pt.x as f32, b_pt.y as f32);
            let r = egui::Rect::from_two_pos(a, b);
            let painter = ui.painter_at(rect);
            painter.rect_stroke(
                r,
                0.0,
                egui::Stroke::new(2.0, SPEC.component.canvas.block_instance_stroke),
                egui::StrokeKind::Outside,
            );
        }

        self.pointer_world = resp.pointer_world;
        self.last_snap = resp.snap;
        self.handle_canvas_input(ui, resp, rect, now);

        // Canvas hints (T-228/T-229).
        let hovered = resp.pointer_world.and_then(|world| {
            geometry
                .hit_test(world, self.snap.anchor_tolerance)
                .map(|hit| match hit {
                    Hit::Wall(id) => HoveredEntity::Wall(id),
                    Hit::Opening(id) => HoveredEntity::Opening(id),
                    Hit::Dimension(id) => HoveredEntity::Dimension(id),
                    Hit::Label(_) => HoveredEntity::AnchorGrip(DimensionId(0)),
                })
        });
        let dt = ui.input(|i| i.predicted_dt) as f64;
        self.canvas_hints.update(hovered, dt);
        let pointer_pos = resp.pointer_world.map(|p| egui::Pos2::new(p.x as f32, p.y as f32));
        self.canvas_hints.render(ui, pointer_pos, &self.view, rect);
    }

    /// Status bar: coordinates, snap chips, selection, zoom, theme.
    fn status_bar(&mut self, ui: &mut egui::Ui, now: i64) {
        crate::status_bar::render(ui, self, now);
    }

    /// Active sheet name (none means "Plan").
    fn active_sheet(&self) -> Option<&str> {
        self.active_sheet.as_deref()
    }

    /// Drawing set sheet names (placeholder — full sheet management is a later slice).
    fn drawing_set_sheets(&self) -> Vec<String> {
        vec!["Plan".to_string()]
    }

    /// Whether a panel is visible (from panel visibility state).
    fn panel_visible(&self, panel: Panel) -> bool {
        self.panel_visibility.is_visible(panel)
    }

    /// Toggle a panel's visibility.
    fn toggle_panel(&mut self, panel: Panel) {
        let visible = self.panel_visibility.is_visible(panel);
        self.panel_visibility.set_visible(panel, !visible);
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
        // The palette owns the keyboard while it is open.
        let canvas_focused =
            ui.memory(|m| m.focused().is_none_or(|id| Some(id) == response.widget_id));
        let keyboard_scope = canvas_focused && !self.palette.is_open();

        // A click places or selects, unless a rubber band took the press.
        if let Some(click_screen) = response.click
            && self.band.is_none()
        {
            let world = screen_to_world(&self.view, rect, click_screen);
            let modifiers = ui.input(|i| i.modifiers);
            match self.tool {
                Tool::Select => {
                    // Block instances are picked before ordinary entities: a
                    // click inside an instance's bounds selects the instance
                    // rather than whatever happens to lie beneath it.
                    let instance_hit = self.hit_block_instance(world);
                    if let Some(id) = instance_hit {
                        if modifiers.shift || modifiers.command || modifiers.ctrl {
                            // Keep single-selection semantics for instances.
                        } else {
                            self.selection.clear();
                        }
                        self.selected_block_instance = Some(id);
                    } else {
                        self.selected_block_instance = None;
                        let hit = geometry.hit_test(world, self.snap.anchor_tolerance);
                        match hit {
                            Some(hit) => {
                                let id = entity_of(hit);
                                if modifiers.shift {
                                    self.selection.toggle(id);
                                } else if modifiers.command || modifiers.ctrl {
                                    self.selection.remove(id);
                                } else {
                                    self.selection.replace(id);
                                }
                            }
                            // Clicking empty canvas clears the set, unless the click
                            // was building one up.
                            None => {
                                if !modifiers.shift && !modifiers.command && !modifiers.ctrl {
                                    self.selection.clear();
                                }
                            }
                        }
                    }
                    self.draft = Draft::default();
                }
                Tool::Wall => self.place_wall_at(world, now),
                Tool::Dimension => self.place_dimension_at(world, now),
                Tool::Label => self.place_label_at(world, now),
                Tool::Door | Tool::Window => self.place_opening_at(world, now),
                Tool::ConstructionLine
                | Tool::ConstructionRay
                | Tool::ConstructionCircle
                | Tool::ConstructionArc => self.place_construction_at(world, now),
                Tool::Block => self.place_block_at(world, now),
                Tool::Measurements => {}
            }
        }

        // Rubber-band selection: a drag that starts on empty canvas draws a
        // box instead of moving something. Window (left to right) keeps only
        // what the box encloses; crossing (right to left) takes whatever it
        // touches; Shift adds and Ctrl removes.
        if let Tool::Select = self.tool {
            if response.drag_started
                && let Some(press) = response.press
            {
                let press_world = screen_to_world(&self.view, rect, press);
                let on_entity = geometry
                    .hit_test(press_world, self.snap.anchor_tolerance)
                    .is_some();
                if !on_entity {
                    let modifiers = ui.input(|i| i.modifiers);
                    self.band = Some(SelectionBand {
                        start: press,
                        add: modifiers.shift,
                        remove: modifiers.command || modifiers.ctrl,
                    });
                }
            }
            if let Some(band) = self.band.clone() {
                if let Some(hover) = response.pointer_world {
                    let start_world = screen_to_world(&self.view, rect, band.start);
                    let mode = SelectionModeKind::from_direction(start_world, hover);
                    let bounds = Bounds::from_points(&[start_world, hover]).expect("bounds");
                    self.selection.pick_box(
                        &geometry,
                        bounds,
                        mode == SelectionModeKind::Crossing,
                        band.add,
                        band.remove,
                    );
                }
                if response.drag_stopped {
                    self.band = None;
                }
            }
        }

        // Dragging a selection moves it.
        if let Some(delta) = response.drag_delta
            && self.band.is_none()
            && let Tool::Select = self.tool
        {
            let mm = Vec2::new(
                delta.x as f64 / self.view.px_per_mm as f64,
                -delta.y as f64 / self.view.px_per_mm as f64,
            );
            for id in self.selection.iter() {
                match id {
                    EntityId::Wall(id) => {
                        let level = self.doc.project().active_level().expect("level");
                        if let Some(wall) = level.wall(id) {
                            let start = wall.start + mm;
                            let end = wall.end + mm;
                            let _ = self.doc.apply(Command::MoveWall { id, start, end }, now);
                        }
                    }
                    EntityId::Opening(id) => {
                        let level = self.doc.project().active_level().expect("level");
                        if let Some(opening) = level.opening(id)
                            && let Some(host) = level.wall(opening.host)
                        {
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
                    }
                    EntityId::Dimension(id) => {
                        // Dragging a dimension line moves it away from what it
                        // measures by changing the offset. The component of the
                        // drag perpendicular to the dimension line is used, so
                        // the line follows the pointer without sliding
                        // lengthways.
                        let level = self.doc.project().active_level().expect("level");
                        if let Some(dimension) = level.dimension(id) {
                            let dimension = dimension.clone();
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
                            // An offset must stay positive; clamping keeps the
                            // line on the far side instead of erroring.
                            let new_offset = (dimension.offset + shift).max(1.0);
                            let _ = self.doc.apply(
                                Command::SetDimensionOffset {
                                    id,
                                    offset: new_offset,
                                },
                                now,
                            );
                        }
                    }
                    EntityId::Label(id) => {
                        let level = self.doc.project().active_level().expect("level");
                        if let Some(label) = level.label(id) {
                            let anchor = label.anchor + mm;
                            let _ = self.doc.apply(Command::MoveLabel { id, anchor }, now);
                        }
                    }
                    // A construction guide is dragged by its own tool, not by
                    // the selection.
                    EntityId::Construction(_) => {}
                }
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
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::E))
                && !ui.input(|i| i.modifiers.shift)
            {
                self.export_open = true;
            }
            if ui.input(|i| i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::E))
            {
                self.export_drawing_set(now);
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::A)) {
                self.select_all();
            }

            // Undo / redo. Accept plain Z/Y as well as Ctrl+Z/Ctrl+Y; the
            // keyboard_scope guard already keeps these from firing while typing.
            if ui.input(|i| {
                (i.modifiers.command || plain_keys)
                    && i.key_pressed(egui::Key::Z)
                    && !i.modifiers.shift
            }) && self.doc.can_undo()
            {
                self.doc.undo(now);
                self.selection.clear();
                self.draft = Draft::default();
            }
            if ui.input(|i| (i.modifiers.command || plain_keys) && i.key_pressed(egui::Key::Y))
                && self.doc.can_redo()
            {
                self.doc.redo(now);
                self.selection.clear();
                self.draft = Draft::default();
            }

            // Cancellation and deletion.
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.selection.clear();
                self.draft = Draft::default();
                self.band = None;
                self.selected_block_instance = None;
            }
            if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace))
            {
                self.delete_selection(now);
            }

            // Plain tool / view / snap shortcuts.
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::D)) {
                self.duplicate_selection(now);
            }

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
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::M) && !i.modifiers.shift) {
                self.tool = Tool::Dimension;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::M) && i.modifiers.shift) {
                self.tool = Tool::Measurements;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::L)) {
                self.tool = Tool::Label;
            }
            // Construction tools (CAP-108): the C family, plus Q and X for the
            // round and curved kinds.
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::C) && !i.modifiers.shift) {
                self.tool = Tool::ConstructionLine;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.shift) {
                self.tool = Tool::ConstructionRay;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::Q)) {
                self.tool = Tool::ConstructionCircle;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::X)) {
                self.tool = Tool::ConstructionArc;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::V) && i.modifiers.shift) {
                // Toggle visibility of the selected construction (CAP-108).
                if let Some(EntityId::Construction(id)) = self.selection.primary() {
                    let visible = self
                        .doc
                        .project()
                        .active_level()
                        .and_then(|level| level.construction_entity(id))
                        .map(|entity| !entity.visible)
                        .unwrap_or(true);
                    let _ = self
                        .doc
                        .apply(Command::SetConstructionVisible { id, visible }, now);
                }
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::G)) {
                self.snap.grid_enabled = !self.snap.grid_enabled;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::O)) {
                self.snap.ortho = !self.snap.ortho;
            }
            // Function-key aliases (QCAD/AutoCAD convention): F7 grid, F8 ortho.
            if ui.input(|i| i.key_pressed(egui::Key::F7)) {
                self.snap.grid_enabled = !self.snap.grid_enabled;
            }
            if ui.input(|i| i.key_pressed(egui::Key::F8)) {
                self.snap.ortho = !self.snap.ortho;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::F)) {
                self.zoom_to_fit();
            }
            // Snap modes. The letters are free of the tool keys and keep the
            // drawing hand on the left of the keyboard, as CAD tools do.
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::E)) {
                self.snap_modes.toggle(SnapModeKind::Endpoint);
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::V) && !i.modifiers.shift) {
                self.snap_modes.toggle(SnapModeKind::Midpoint);
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::I)) {
                self.snap_modes.toggle(SnapModeKind::Intersection);
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::P)) {
                self.snap_modes.toggle(SnapModeKind::Perpendicular);
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::T)) {
                self.snap_modes.toggle(SnapModeKind::Tangent);
            }
            // Release notes (palette + menu share the flow; R mirrors the
            // palette hint).
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::R)) {
                self.showing_notes = true;
            }
            // Check for updates (Ctrl+U mirrors the palette hint).
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::U)) {
                self.start_update_check();
                self.showing_about = true;
            }
        }
    }

    /// Place a wall: the first click starts it, the second commits it.
    fn place_wall_at(&mut self, world: Point2, now: i64) {
        if !self.draft.active {
            self.draft = Draft {
                active: true,
                start: self.snap_at(world, None),
                current: world,
            };
            return;
        }
        let end = self.snap_at(world, Some(self.draft.start));
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
                .map_err(|e| self.note_status(format!("Wall rejected: {e}"), now))
                .ok();
            if let Some(applied) = applied
                && let Some(wall_id) = applied.wall
            {
                self.selection.replace(EntityId::Wall(wall_id));
            }
        }
        self.draft = Draft::default();
    }

    /// Place an aligned dimension between two picked points.
    fn place_dimension_at(&mut self, world: Point2, now: i64) {
        if !self.draft.active {
            self.draft = Draft {
                active: true,
                start: self.snap_at(world, None),
                current: world,
            };
            return;
        }
        let end = self.snap_at(world, Some(self.draft.start));
        if end.distance(self.draft.start) > MIN_WALL_LENGTH_DRAG {
            let start_world = self.draft.start;
            let applied = self
                .doc
                .apply(
                    Command::PlaceDimension {
                        kind: lintel_core::dimension::DimensionKind::Aligned,
                        start: start_world,
                        end,
                        offset: lintel_core::dimension::DEFAULT_DIMENSION_OFFSET,
                    },
                    now,
                )
                .map_err(|e| self.note_status(format!("Dimension rejected: {e}"), now))
                .ok();
            if let Some(applied) = applied
                && let Some(dim_id) = applied.dimension
            {
                // Anchor the measured points to any wall they land on, so the
                // dimension follows the wall when it moves.
                let geometry = lintel_core::join::PlanGeometry::of_level(
                    self.doc.project().active_level().expect("level"),
                );
                for (point, is_start) in [(start_world, true), (end, false)] {
                    if let Some(hit) = geometry.hit_test(point, self.snap.anchor_tolerance)
                        && let lintel_core::join::Hit::Wall(wall_id) = hit
                    {
                        let wall = geometry.wall(wall_id).expect("wall");
                        let dx = wall.wall.end.x - wall.wall.start.x;
                        let dy = wall.wall.end.y - wall.wall.start.y;
                        let len_sq = dx * dx + dy * dy;
                        let param = if len_sq > 0.0 {
                            (((point.x - wall.wall.start.x) * dx
                                + (point.y - wall.wall.start.y) * dy)
                                / len_sq)
                                .clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        let anchor = lintel_core::dimension::DimensionAnchor::anchored(
                            wall_id, param, false,
                        );
                        let cmd = if is_start {
                            Command::SetDimensionStartAnchor {
                                id: dim_id,
                                anchor: Some(anchor),
                            }
                        } else {
                            Command::SetDimensionEndAnchor {
                                id: dim_id,
                                anchor: Some(anchor),
                            }
                        };
                        let _ = self.doc.apply(cmd, now);
                    }
                }
                self.selection.replace(EntityId::Dimension(dim_id));
            }
        }
        self.draft = Draft::default();
    }

    /// Place a construction guide: first click sets the anchor, second commits.
    fn place_construction_at(&mut self, world: Point2, now: i64) {
        if !self.draft.active {
            self.draft = Draft {
                active: true,
                start: self.snap_at(world, None),
                current: world,
            };
            return;
        }
        let end = self.snap_at(world, Some(self.draft.start));
        let start = self.draft.start;
        let kind = match self.tool {
            Tool::ConstructionLine => {
                lintel_core::construction::ConstructionKind::Line { start, end }
            }
            Tool::ConstructionRay => lintel_core::construction::ConstructionKind::Ray {
                origin: start,
                direction: lintel_core::geom::Vec2::new(end.x - start.x, end.y - start.y),
            },
            Tool::ConstructionCircle => lintel_core::construction::ConstructionKind::Circle {
                center: start,
                radius: start.distance(end).max(1.0),
            },
            Tool::ConstructionArc => {
                let radius = start.distance(end).max(1.0);
                let start_deg = (end - start).angle_degrees();
                lintel_core::construction::ConstructionKind::Arc {
                    center: start,
                    radius,
                    start_deg,
                    end_deg: start_deg + 90.0,
                }
            }
            _ => {
                self.draft = Draft::default();
                return;
            }
        };
        if start.distance(end) > MIN_WALL_LENGTH_DRAG
            || matches!(self.tool, Tool::ConstructionCircle | Tool::ConstructionArc)
        {
            let applied = self
                .doc
                .apply(Command::CreateConstruction { kind }, now)
                .map_err(|e| self.note_status(format!("Construction rejected: {e}"), now))
                .ok();
            if let Some(applied) = applied
                && let Some(cid) = applied.construction
            {
                self.selection.replace(EntityId::Construction(cid));
            }
        }
        self.draft = Draft::default();
    }

    /// Place a block instance at the pointer.
    fn place_block_at(&mut self, world: Point2, now: i64) {
        let level = self.doc.project().active_level().expect("level");
        let block_id = if self.inspector_draft.block_def_id != 0 {
            lintel_core::BlockId(self.inspector_draft.block_def_id as u32)
        } else {
            match level.block_defs.first() {
                Some(def) => def.id,
                None => {
                    self.note_status("No block definition selected.".to_string(), now);
                    return;
                }
            }
        };
        // Make sure the def exists on this level.
        if level.block_def(block_id).is_none() {
            self.note_status("Selected block definition not found.".to_string(), now);
            return;
        }
        let position = self.snap_at(world, None);
        let applied = self
            .doc
            .apply(Command::AddBlockInstance { block_id, position }, now)
            .map_err(|e| self.note_status(format!("Block rejected: {e}"), now))
            .ok();
        let _ = applied;
        self.draft = Draft::default();
    }

    /// Drop a text label at the pointer.
    fn place_label_at(&mut self, world: Point2, now: i64) {
        let anchor = self.snap_at(world, None);
        let applied = self
            .doc
            .apply(
                Command::PlaceLabel {
                    anchor,
                    text: "Note".to_string(),
                },
                now,
            )
            .map_err(|e| self.note_status(format!("Label rejected: {e}"), now))
            .ok();
        if let Some(applied) = applied
            && let Some(label_id) = applied.label
        {
            self.selection.replace(EntityId::Label(label_id));
        }
        self.draft = Draft::default();
    }

    /// Place a door or window on the selected wall, at the offset nearest the
    /// pointer; a bare click just picks what is under it.
    fn place_opening_at(&mut self, world: Point2, now: i64) {
        let geometry = PlanGeometry::of_level(self.doc.project().active_level().expect("level"));
        let host = match self.selection.primary() {
            Some(EntityId::Wall(id)) if geometry.wall(id).is_some() => Some(id),
            _ => match geometry.hit_test(world, self.snap.anchor_tolerance) {
                Some(Hit::Wall(id)) => Some(id),
                _ => None,
            },
        };
        let Some(host) = host else {
            let hit = geometry.hit_test(world, self.snap.anchor_tolerance);
            self.selection.set_from_hit(hit);
            self.note_status("Click on a wall to place a door or window".to_string(), now);
            return;
        };
        let kind = match self.tool {
            Tool::Door => lintel_core::opening::OpeningKind::Door,
            Tool::Window => lintel_core::opening::OpeningKind::Window,
            _ => unreachable!("openings only come from the door or window tool"),
        };
        let wall = geometry.wall(host).expect("host wall");
        let offset = wall_offset_along_centerline(&wall.wall, world);
        match self
            .doc
            .apply(Command::PlaceOpening { kind, host, offset }, now)
        {
            Ok(_) => {
                self.selection.replace(EntityId::Wall(host));
                self.draft = Draft::default();
            }
            Err(e) => self.note_status(
                format!("Cannot place {}: {}", kind.label().to_ascii_lowercase(), e),
                now,
            ),
        };
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
pub fn plan_summary(project: &lintel_core::project::Project) -> String {
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
    let rooms = lintel_core::room::detect_rooms(&level.walls).len();
    let base = format!(
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
    );
    if rooms > 0 {
        format!("{}, {} room{}", base, rooms, plural(rooms))
    } else {
        base
    }
}

/// Return the LintelApp instance wrapped for egui_kittest::harness
/// accessibility testing (guide II.27.12 / K.7).
pub fn app() -> LintelApp {
    LintelApp::new()
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

    // Embed the app icon and set it as the window icon (guide §10: original visual design).
    const ICON_BYTES: &[u8] = include_bytes!("../../../resources/linux/lintel.png");
    let icon_img = image::load_from_memory(ICON_BYTES)
        .expect("load embedded icon")
        .into_rgba8();
    let (icon_width, icon_height) = icon_img.dimensions();
    let icon_data = egui::IconData {
        rgba: icon_img.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Lintel — architectural drafting")
            .with_inner_size(egui::Vec2 {
                x: 1300.0,
                y: 850.0,
            })
            .with_icon(icon_data),
        ..Default::default()
    };
    let app = LintelApp::new();
    eframe::run_native("Lintel", native_options, Box::new(|_cc| Ok(Box::new(app))))
        .expect("eframe run failed");
}

#[cfg(test)]
mod kittest {
    // egui_kittest v0.36 API: the crate provides a test harness through
    // egui_kittest::harness or similar. The exact API surface is being
    // validated against egui 0.36.2. Placeholder test that verifies the
    // app can be constructed headlessly.
    #[test]
    fn app_constructs_headlessly() {
        let _app = crate::app();
    }

    #[test]
    fn plan_summary_returns_non_empty() {
        use lintel_core::project::Project;
        let summary = crate::plan_summary(&Project::new("Test".to_string(), 0));
        assert!(!summary.is_empty());
        assert!(summary.contains("Level 0"));
    }
}
