//! Lintel desktop shell — eframe/egui GUI wired to `lintel-core` commands.
//! Every user interaction goes through a `Command`; the core model updates
//! headlessly, and the UI re-renders from the new state (addendum K.3).

use crate::canvas::{
    Canvas, Draft, Selection, SelectionBand, SelectionModeKind, SnapModeKind, SnapModes, SnapReadout,
    View, entity_of, resolve_snap, screen_to_world,
};
use crate::command_palette::CommandPalette;
use crate::inspector::{InspectorDraft, inspector};
use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    geom::{Bounds, Point2, Vec2},
    ids::EntityId,
    join::{Hit, PlanGeometry},
};

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};

mod about;
mod canvas;
mod command_palette;
mod dimension_ui;
mod inspector;
mod release_notes;
mod status_bar;
mod theme;

const MIN_WALL_LENGTH_DRAG: f64 = 2.0;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Select,
    Wall,
    Door,
    Window,
    Dimension,
    Label,
    Measurements,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Wall => "Wall",
            Tool::Door => "Door",
            Tool::Window => "Window",
            Tool::Dimension => "Dimension",
            Tool::Label => "Label",
            Tool::Measurements => "Measurements",
        }
    }
}

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
}

impl LintelApp {
    fn new() -> Self {
        let now = now_epoch_s();
        let doc = Document::new("Untitled plan", now);
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
        self.selection.clear();
        self.draft = Draft::default();
        self.band = None;
        self.status = None;
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

    /// Delete every selected entity, reporting the first refusal.
    ///
    /// Construction guides own their own delete command, so a selection that
    /// holds one deletes it too.
    fn delete_selection(&mut self, now: i64) {
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
        self.poll_update();
        if self.update_rx.is_some() {
            // Poll for the worker's result without spinning the CPU.
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(500));
        }
        // Ctrl+K opens the command palette from anywhere, even while typing in
        // the Inspector: it is the one shortcut that is never swallowed. This
        // is the only place it is handled, so a press can never double-toggle.
        if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
            self.toggle_palette();
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
                    if ui
                        .button("Export drawing set…")
                        .on_hover_text("Write a multi-sheet PDF with a cover page (Ctrl+Shift+E)")
                        .clicked()
                    {
                        self.export_drawing_set(now);
                    }
                    ui.separator();
                    if ui.button("Exit").on_hover_text("Close Lintel").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui
                        .button("Command palette…")
                        .on_hover_text("Search every command by name (Ctrl+K)")
                        .clicked()
                    {
                        self.palette.open();
                    }
                    ui.separator();
                    ui.add_enabled(self.doc.can_undo(), |ui: &mut egui::Ui| {
                        let response = ui
                            .button("Undo")
                            .on_hover_text("Undo the last change (Ctrl+Z)");
                        if response.clicked() {
                            self.doc.undo(now_epoch_s());
                            self.selection.clear();
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
                            self.selection.clear();
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
                        ui.selectable_value(&mut self.tool, Tool::Label, Tool::Label.label());
                        ui.selectable_value(
                            &mut self.tool,
                            Tool::Measurements,
                            Tool::Measurements.label(),
                        );
                    });
                tool_response.response.on_hover_text(
                    "Choose the active tool (W=Wall S=Select D=Door N=Window M=Dimension Shift+M=Measurements)",
                );
                ui.separator();
                if ui
                    .button("Commands")
                    .on_hover_text("Search every command by name (Ctrl+K)")
                    .clicked()
                {
                    self.palette.open();
                }
                ui.separator();
                ui.label(format!("Zoom: {}%", self.view.zoom_percent()));
                ui.separator();
                ui.label(egui::RichText::new(summary.clone()).small());
                ui.label(
                    egui::RichText::new("Shortcuts: W/S/D/N/M/L tools | E/V/I/P/T snap modes | Ctrl+K palette | Ctrl+A select all | Shift-click and drag-box multi-select | Ctrl+N/O/S/Shift+S/E file | Ctrl+D duplicate | Z/Y undo-redo | F fit | Esc cancel | Del delete")
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
                });
            });

            let level = self.doc.project().active_level().expect("level");
            let geometry = PlanGeometry::of_level(level);
            let (response, rect) = Canvas::show(
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
            self.pointer_world = response.pointer_world;
            self.last_snap = response.snap;
            self.handle_canvas_input(ui, response, rect, now);

            // Status bar (bottom) with version link in the bottom-right corner
            // per suite standard.
            crate::status_bar::render(ui, self, now);

            egui::Window::new("Inspector")
                .default_pos(egui::Pos2::new(4.0, 40.0))
                .show(ui, |ui| {
                    if let Some(cmd) = inspector(
                        ui,
                        &self.doc,
                        &self.selection,
                        &mut self.snap,
                        &mut self.inspector_draft,
                        self.tool == Tool::Measurements,
                    ) {
                        let _ = self.doc.apply(cmd, now);
                    }
                });
        });

        self.dialogs(ui, now);
        crate::command_palette::show(ui, self, now);
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
                    self.draft = Draft::default();
                }
                Tool::Wall => self.place_wall_at(world, now),
                Tool::Dimension => self.place_dimension_at(world, now),
                Tool::Label => self.place_label_at(world, now),
                Tool::Door | Tool::Window => self.place_opening_at(world, now),
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

            // Undo / redo.
            if ui
                .input(|i| i.modifiers.command && i.key_pressed(egui::Key::Z) && !i.modifiers.shift)
                && self.doc.can_undo()
            {
                self.doc.undo(now);
                self.selection.clear();
                self.draft = Draft::default();
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Y))
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
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::G)) {
                self.snap.grid_enabled = !self.snap.grid_enabled;
            }
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::O)) {
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
            if plain_keys && ui.input(|i| i.key_pressed(egui::Key::V)) {
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
                .map_err(|e| self.note_status(format!("Dimension rejected: {e}"), now))
                .ok();
            if let Some(applied) = applied
                && let Some(dim_id) = applied.dimension
            {
                self.selection.replace(EntityId::Dimension(dim_id));
            }
        }
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
            return;
        };
        let kind = match self.tool {
            Tool::Door => lintel_core::opening::OpeningKind::Door,
            Tool::Window => lintel_core::opening::OpeningKind::Window,
            _ => unreachable!("openings only come from the door or window tool"),
        };
        let wall = geometry.wall(host).expect("host wall");
        let offset = wall
            .wall
            .start
            .distance(world)
            .clamp(0.0, wall.wall.length());
        let _ = self
            .doc
            .apply(Command::PlaceOpening { kind, host, offset }, now);
        self.selection.replace(EntityId::Wall(host));
        self.draft = Draft::default();
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

    #[test]
    fn the_summary_appends_room_count_when_rooms_exist() {
        let mut doc = Document::new("Plan", 1);
        // Four walls enclosing a 4000 x 4000 square.
        let ids = [
            (Point2::new(0.0, 0.0), Point2::new(4000.0, 0.0)),
            (Point2::new(4000.0, 0.0), Point2::new(4000.0, 4000.0)),
            (Point2::new(4000.0, 4000.0), Point2::new(0.0, 4000.0)),
            (Point2::new(0.0, 4000.0), Point2::new(0.0, 0.0)),
        ];
        for (s, e) in ids {
            doc.apply(
                Command::AddWall {
                    start: s,
                    end: e,
                    thickness: 200.0,
                    height: 2800.0,
                },
                2,
            )
            .expect("add wall");
        }
        assert_eq!(
            plan_summary(doc.project()),
            "Level 0: 4 walls, 0 doors, 0 windows, 0 dimensions, 1 room"
        );
    }
}
