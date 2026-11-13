//! Command palette: the Ctrl+K launcher over every Lintel command.
//!
//! Pure UI over the same `lintel_core::command::Command` values the menus, the
//! inspector and the canvas apply: nothing here mutates the model, it asks
//! `LintelApp` to run an action. Actions that need a value (a new layer, a
//! renamed project) open a small form inside the same modal instead of
//! guessing one.

use eframe::egui;
use lintel_core::{
    command::Command,
    document::Document,
    ids::{EntityId, LayerId},
    layer::{EntitySelector, LayerProperty, Linetype, Rgb},
};

use crate::LintelApp;
use crate::Tool;
use crate::canvas::{Draft, Selection, SnapModeKind};

/// How many recent commands the palette remembers.
const MAX_RECENT: usize = 8;

/// How many rows fit before the list scrolls.
const ROW_HEIGHT: f32 = 26.0;

/// The number of rows the list shows before it scrolls.
const MAX_VISIBLE_ROWS: usize = 12;

/// The width of the palette modal.
const PALLETTE_WIDTH: f32 = 520.0;

/// The palette's own filter, picker and history state.
#[derive(Debug, Default)]
pub struct CommandPalette {
    /// Whether the modal is showing.
    open: bool,
    /// What the user has typed.
    query: String,
    /// Which filtered row is highlighted.
    selected: usize,
    /// Most recently used actions, newest last, capped at [`MAX_RECENT`].
    recent: Vec<PaletteAction>,
    /// A parameter form, shown in place of the results while it is open.
    pending: Option<PendingForm>,
}

impl CommandPalette {
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Show the palette, ready to type into.
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.pending = None;
    }

    /// Hide the palette and forget what was typed.
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.selected = 0;
        self.pending = None;
    }

    /// Remember an action as recently used, newest last and never duplicated.
    fn remember(&mut self, action: PaletteAction) {
        self.recent.retain(|recent| *recent != action);
        self.recent.push(action);
        while self.recent.len() > MAX_RECENT {
            self.recent.remove(0);
        }
    }

    /// The recent list, newest first.
    fn recent_newest_first(&self) -> Vec<PaletteAction> {
        self.recent.iter().rev().copied().collect()
    }
}

/// One command in the palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaletteAction {
    // Tools.
    SelectTool,
    WallTool,
    DoorTool,
    WindowTool,
    DimensionTool,
    LabelTool,
    MeasurementsTool,
    // File.
    NewPlan,
    OpenPlan,
    SavePlan,
    SavePlanAs,
    ExportSheet,
    ExportDrawingSet,
    // Edit.
    Undo,
    Redo,
    DeleteSelection,
    DuplicateSelection,
    SelectAll,
    ClearSelection,
    // View.
    ZoomFit,
    ToggleUnits,
    // Snapping.
    ToggleEndpoint,
    ToggleMidpoint,
    ToggleIntersection,
    TogglePerpendicular,
    ToggleTangent,
    ToggleGrid,
    ToggleOrtho,
    // Layers.
    CreateLayer,
    LayerVisible,
    LayerLocked,
    LayerPrintable,
    AssignSelectionToLayer,
    DeleteLayer,
    // Construction.
    ConstructionLineTool,
    ConstructionRayTool,
    ConstructionCircleTool,
    ConstructionArcTool,
    ConstructionVisible,
    DeleteConstruction,
    // Project.
    RenameProject,
    AddStorey,
    DeleteStorey,
    // Help.
    ShowAbout,
    ShowReleaseNotes,
    CheckForUpdates,
}

impl PaletteAction {
    /// Every action, in category order.
    pub const ALL: [PaletteAction; 46] = [
        PaletteAction::SelectTool,
        PaletteAction::WallTool,
        PaletteAction::DoorTool,
        PaletteAction::WindowTool,
        PaletteAction::DimensionTool,
        PaletteAction::LabelTool,
        PaletteAction::MeasurementsTool,
        PaletteAction::NewPlan,
        PaletteAction::OpenPlan,
        PaletteAction::SavePlan,
        PaletteAction::SavePlanAs,
        PaletteAction::ExportSheet,
        PaletteAction::ExportDrawingSet,
        PaletteAction::Undo,
        PaletteAction::Redo,
        PaletteAction::DeleteSelection,
        PaletteAction::DuplicateSelection,
        PaletteAction::SelectAll,
        PaletteAction::ClearSelection,
        PaletteAction::ZoomFit,
        PaletteAction::ToggleUnits,
        PaletteAction::ToggleEndpoint,
        PaletteAction::ToggleMidpoint,
        PaletteAction::ToggleIntersection,
        PaletteAction::TogglePerpendicular,
        PaletteAction::ToggleTangent,
        PaletteAction::ToggleGrid,
        PaletteAction::ToggleOrtho,
        PaletteAction::CreateLayer,
        PaletteAction::LayerVisible,
        PaletteAction::LayerLocked,
        PaletteAction::LayerPrintable,
        PaletteAction::AssignSelectionToLayer,
        PaletteAction::DeleteLayer,
        PaletteAction::ConstructionLineTool,
        PaletteAction::ConstructionRayTool,
        PaletteAction::ConstructionCircleTool,
        PaletteAction::ConstructionArcTool,
        PaletteAction::ConstructionVisible,
        PaletteAction::DeleteConstruction,
        PaletteAction::RenameProject,
        PaletteAction::AddStorey,
        PaletteAction::DeleteStorey,
        PaletteAction::ShowAbout,
        PaletteAction::ShowReleaseNotes,
        PaletteAction::CheckForUpdates,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PaletteAction::SelectTool => "Select tool",
            PaletteAction::WallTool => "Wall tool",
            PaletteAction::DoorTool => "Door tool",
            PaletteAction::WindowTool => "Window tool",
            PaletteAction::DimensionTool => "Dimension tool",
            PaletteAction::LabelTool => "Label tool",
            PaletteAction::MeasurementsTool => "Measurements tool",
            PaletteAction::NewPlan => "New plan",
            PaletteAction::OpenPlan => "Open plan",
            PaletteAction::SavePlan => "Save plan",
            PaletteAction::SavePlanAs => "Save plan as",
            PaletteAction::ExportSheet => "Export sheet",
            PaletteAction::ExportDrawingSet => "Export drawing set",
            PaletteAction::Undo => "Undo",
            PaletteAction::Redo => "Redo",
            PaletteAction::DeleteSelection => "Delete selection",
            PaletteAction::DuplicateSelection => "Duplicate selection",
            PaletteAction::SelectAll => "Select all",
            PaletteAction::ClearSelection => "Deselect all",
            PaletteAction::ZoomFit => "Zoom to fit",
            PaletteAction::ToggleUnits => "Switch Metric / Imperial",
            PaletteAction::ToggleEndpoint => "Snap: endpoint",
            PaletteAction::ToggleMidpoint => "Snap: midpoint",
            PaletteAction::ToggleIntersection => "Snap: intersection",
            PaletteAction::TogglePerpendicular => "Snap: perpendicular",
            PaletteAction::ToggleTangent => "Snap: tangent",
            PaletteAction::ToggleGrid => "Toggle grid",
            PaletteAction::ToggleOrtho => "Toggle orthogonal drawing",
            PaletteAction::CreateLayer => "Create layer",
            PaletteAction::LayerVisible => "Layer visibility",
            PaletteAction::LayerLocked => "Layer lock",
            PaletteAction::LayerPrintable => "Layer printable",
            PaletteAction::AssignSelectionToLayer => "Assign selection to layer",
            PaletteAction::DeleteLayer => "Delete layer",
             PaletteAction::ConstructionLineTool => "Construction line tool",
             PaletteAction::ConstructionRayTool => "Construction ray tool",
             PaletteAction::ConstructionCircleTool => "Construction circle tool",
             PaletteAction::ConstructionArcTool => "Construction arc tool",
             PaletteAction::ConstructionVisible => "Construction visibility",
             PaletteAction::DeleteConstruction => "Delete construction",
            PaletteAction::RenameProject => "Rename project",
            PaletteAction::AddStorey => "Add storey",
            PaletteAction::DeleteStorey => "Delete storey",
            PaletteAction::ShowAbout => "About Lintel",
            PaletteAction::ShowReleaseNotes => "Release notes",
            PaletteAction::CheckForUpdates => "Check for updates",
        }
    }

    /// The keyboard shortcut, shown on the right of the row.
    pub fn shortcut(self) -> &'static str {
        match self {
            PaletteAction::SelectTool => "S",
            PaletteAction::WallTool => "W",
            PaletteAction::DoorTool => "D",
            PaletteAction::WindowTool => "N",
            PaletteAction::DimensionTool => "M",
            PaletteAction::LabelTool => "L",
            PaletteAction::MeasurementsTool => "Shift+M",
            PaletteAction::NewPlan => "Ctrl+N",
            PaletteAction::OpenPlan => "Ctrl+O",
            PaletteAction::SavePlan => "Ctrl+S",
            PaletteAction::SavePlanAs => "Ctrl+Shift+S",
            PaletteAction::ExportSheet => "Ctrl+E",
            PaletteAction::ExportDrawingSet => "Ctrl+Shift+E",
            PaletteAction::Undo => "Ctrl+Z",
            PaletteAction::Redo => "Ctrl+Y",
            PaletteAction::DeleteSelection => "Del",
            PaletteAction::DuplicateSelection => "Ctrl+D",
            PaletteAction::SelectAll => "Ctrl+A",
            PaletteAction::ClearSelection => "Esc",
            PaletteAction::ZoomFit => "F",
            PaletteAction::ToggleEndpoint => "E",
            PaletteAction::ToggleMidpoint => "V",
            PaletteAction::ToggleIntersection => "I",
            PaletteAction::TogglePerpendicular => "P",
            PaletteAction::ToggleTangent => "T",
            PaletteAction::ToggleGrid => "G",
            PaletteAction::ToggleOrtho => "O",
            PaletteAction::ToggleUnits => "U",
            PaletteAction::ShowAbout => "?",
            PaletteAction::ShowReleaseNotes => "R",
            PaletteAction::CheckForUpdates => "U",
            _ => "",
        }
    }

    pub fn category(self) -> &'static str {
        match self {
            PaletteAction::SelectTool
            | PaletteAction::WallTool
            | PaletteAction::DoorTool
            | PaletteAction::WindowTool
            | PaletteAction::DimensionTool
            | PaletteAction::LabelTool
            | PaletteAction::MeasurementsTool
            | PaletteAction::ConstructionLineTool
            | PaletteAction::ConstructionRayTool
            | PaletteAction::ConstructionCircleTool
            | PaletteAction::ConstructionArcTool => "Tools",
            PaletteAction::NewPlan
            | PaletteAction::OpenPlan
            | PaletteAction::SavePlan
            | PaletteAction::SavePlanAs
            | PaletteAction::ExportSheet
            | PaletteAction::ExportDrawingSet => "File",
            PaletteAction::Undo
            | PaletteAction::Redo
            | PaletteAction::DeleteSelection
            | PaletteAction::DuplicateSelection
            | PaletteAction::SelectAll
            | PaletteAction::ClearSelection => "Edit",
            PaletteAction::ZoomFit | PaletteAction::ToggleUnits => "View",
            PaletteAction::ToggleEndpoint
            | PaletteAction::ToggleMidpoint
            | PaletteAction::ToggleIntersection
            | PaletteAction::TogglePerpendicular
            | PaletteAction::ToggleTangent
            | PaletteAction::ToggleGrid
            | PaletteAction::ToggleOrtho => "Snapping",
            PaletteAction::CreateLayer
            | PaletteAction::LayerVisible
            | PaletteAction::LayerLocked
            | PaletteAction::LayerPrintable
            | PaletteAction::AssignSelectionToLayer
            | PaletteAction::DeleteLayer
            | PaletteAction::ConstructionVisible
            | PaletteAction::DeleteConstruction => "Layers",
            PaletteAction::RenameProject
            | PaletteAction::AddStorey
            | PaletteAction::DeleteStorey => "Project",
            PaletteAction::ShowAbout
            | PaletteAction::ShowReleaseNotes
            | PaletteAction::CheckForUpdates => "Help",
        }
    }

    /// True when the action needs values before it can be applied: the palette
    /// opens a form instead of guessing.
    pub fn needs_parameters(self) -> bool {
        matches!(
            self,
            PaletteAction::CreateLayer
                | PaletteAction::LayerVisible
                | PaletteAction::LayerLocked
                | PaletteAction::LayerPrintable
                | PaletteAction::AssignSelectionToLayer
                | PaletteAction::DeleteLayer
                | PaletteAction::RenameProject
                | PaletteAction::AddStorey
                | PaletteAction::DeleteStorey
        )
    }

    /// True when the action can run in the current app state.
    pub fn is_enabled(self, doc: &Document, selection: &Selection) -> bool {
        let level = doc.project().active_level();
        let has_entities = level.is_some_and(|level| {
            !level.walls.is_empty()
                || !level.openings.is_empty()
                || !level.dimensions.is_empty()
                || !level.labels.is_empty()
        });
        match self {
            PaletteAction::Undo => doc.can_undo(),
            PaletteAction::Redo => doc.can_redo(),
            PaletteAction::DeleteSelection | PaletteAction::DuplicateSelection => {
                !selection.is_empty()
            }
            PaletteAction::SelectAll => has_entities,
            PaletteAction::ClearSelection => !selection.is_empty(),
            PaletteAction::CreateLayer => level.is_some_and(|level| level.layers.len() < 64),
            PaletteAction::LayerVisible
            | PaletteAction::LayerLocked
            | PaletteAction::LayerPrintable
            | PaletteAction::DeleteLayer => {
                level.is_some_and(|level| level.layers.iter().any(|layer| !layer.is_default()))
            }
            PaletteAction::AssignSelectionToLayer => {
                !selection.is_empty() && level.is_some_and(|level| level.layers.len() > 1)
            }
            PaletteAction::DeleteStorey => doc.project().levels.len() > 1,
            PaletteAction::ConstructionVisible | PaletteAction::DeleteConstruction => selection
                .primary()
                .is_some_and(|id| matches!(id, EntityId::Construction(_))),
            _ => true,
        }
    }
}

/// A fuzzy match: how well `target` matched and which characters did.
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzyMatch {
    /// Higher is a better match; zero for an empty query.
    pub score: i32,
    /// Indices into the target that the query covered.
    pub hits: Vec<usize>,
}

impl FuzzyMatch {
    /// The match every row gets when nothing has been typed.
    pub fn plain() -> Self {
        Self {
            score: 0,
            hits: Vec::new(),
        }
    }
}

/// Score `target` against `query`, or `None` when the query is not a
/// case-insensitive subsequence of it.
///
/// Consecutive hits, word starts and early first hits are worth more, so
/// "wall" ranks "Wall tool" above "Drawing set wall note" without any external
/// crate.
pub fn fuzzy_match(query: &str, target: &str) -> Option<FuzzyMatch> {
    let query: Vec<char> = query.trim().to_lowercase().chars().collect();
    if query.is_empty() {
        return Some(FuzzyMatch::plain());
    }
    let target_chars: Vec<char> = target.chars().collect();
    let target_lower: Vec<char> = target.to_lowercase().chars().collect();

    let mut hits = Vec::with_capacity(query.len());
    let mut score = 0;
    let mut search_from = 0;
    for (position, wanted) in query.iter().enumerate() {
        let found = target_lower
            .iter()
            .enumerate()
            .skip(search_from)
            .find(|(_, candidate)| *candidate == wanted)
            .map(|(index, _)| index);
        let index = found?;
        search_from = index + 1;
        hits.push(index);

        score += 1;
        if let Some(previous) = hits.len().checked_sub(2).map(|i| hits[i])
            && index == previous + 1
        {
            // A run of letters is worth more than scattered ones.
            score += 3;
        }
        if index == 0 {
            score += 5;
        } else if is_word_start(target_chars[index - 1], target_chars[index]) {
            score += 4;
        }
        if position == 0 {
            // The sooner the first letter lands, the better.
            score += 3 - (index as i32 / 4).min(3);
        }
    }
    // Shorter labels win a tie, so "Wall tool" beats a longer one containing it.
    score -= (target_chars.len() / 24) as i32;

    Some(FuzzyMatch { score, hits })
}

fn is_word_start(previous: char, current: char) -> bool {
    previous == ' '
        || previous == '-'
        || previous == '/'
        || previous == '('
        || previous == '.'
        || previous.is_lowercase() && current.is_uppercase()
}

/// Split a label into runs of matched and unmatched characters, for highlighting.
fn highlight_chunks(label: &str, hits: &[usize]) -> Vec<(String, bool)> {
    let mut chunks: Vec<(String, bool)> = Vec::new();
    for (index, character) in label.chars().enumerate() {
        let hit = hits.contains(&index);
        match chunks.last_mut() {
            Some(chunk) if chunk.1 == hit => chunk.0.push(character),
            _ => chunks.push((character.to_string(), hit)),
        }
    }
    chunks
}

/// One row of the palette.
#[derive(Debug, Clone)]
struct Row {
    action: PaletteAction,
    recent: bool,
    enabled: bool,
    score: i32,
    /// Indices of the matched characters, for highlighting.
    hits: Vec<usize>,
}

/// The rows to show: recent commands first while the query is empty, then every
/// command the query matches, best match first.
fn visible_rows(palette: &CommandPalette, doc: &Document, selection: &Selection) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    if palette.query.trim().is_empty() {
        for action in palette.recent_newest_first() {
            rows.push(Row {
                action,
                recent: true,
                enabled: action.is_enabled(doc, selection),
                score: 0,
                hits: Vec::new(),
            });
        }
    }
    for action in PaletteAction::ALL {
        if let Some(matched) = fuzzy_match(&palette.query, action.label()) {
            rows.push(Row {
                action,
                recent: false,
                enabled: action.is_enabled(doc, selection),
                score: matched.score,
                hits: matched.hits,
            });
        }
    }
    // A stable sort keeps the category order for equal scores.
    rows.sort_by_key(|a| std::cmp::Reverse(a.score));
    rows
}

/// Paint the palette modal. Nothing runs until a row is activated.
pub fn show(ui: &mut egui::Ui, app: &mut LintelApp, now: i64) {
    if !app.palette.is_open() {
        return;
    }

    let modal = egui::Modal::new(egui::Id::new("lintel_command_palette")).show(ui.ctx(), |ui| {
        ui.set_width(PALLETTE_WIDTH);
        // Escape steps out of a form before it closes the palette.
        let escape = ui
            .ctx()
            .input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        if escape {
            if app.palette.pending.is_some() {
                app.palette.pending = None;
            } else {
                ui.close();
            }
            return;
        }
        match app.palette.pending.clone() {
            Some(form) => form_ui(ui, app, form, now),
            None => search_ui(ui, app, now),
        }
    });
    if modal.should_close() {
        app.palette.close();
    }
}

/// The results stage: search field, recent commands and the filtered list.
fn search_ui(ui: &mut egui::Ui, app: &mut LintelApp, now: i64) {
    let rows = visible_rows(&app.palette, &app.doc, &app.selection);

    // Keyboard navigation is consumed before the search field is added, so its
    // own cursor never moves while the user is picking a command.
    let mut activate = false;
    ui.ctx().input_mut(|input| {
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
            app.palette.selected = step(app.palette.selected, &rows, true);
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
            app.palette.selected = step(app.palette.selected, &rows, false);
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
            activate = true;
        }
    });
    if app.palette.selected >= rows.len() {
        app.palette.selected = 0;
    }

    ui.heading("Command palette");
    ui.label(
        egui::RichText::new("Type to filter · ↑↓ to move · Enter to run · Esc to close")
            .small()
            .color(egui::Color32::GRAY),
    );
    ui.add_space(4.0);
    let search = ui.add(
        egui::TextEdit::singleline(&mut app.palette.query)
            .hint_text("Search commands and tools…")
            .desired_width(PALLETTE_WIDTH - 24.0),
    );
    search.request_focus();
    if search.changed() {
        app.palette.selected = 0;
    }
    ui.add_space(4.0);

    if rows.is_empty() {
        ui.label("No command matches that.");
    }

    // The selected row is kept on screen, or the list would scroll away from
    // under the keyboard.
    let scroll = egui::ScrollArea::vertical()
        .max_height(ROW_HEIGHT * MAX_VISIBLE_ROWS as f32)
        .show(ui, |ui| {
            let mut clicked = None;
            let mut selected_rect = None;
            for (index, row) in rows.iter().enumerate() {
                let response = row_ui(ui, row, index == app.palette.selected);
                if index == app.palette.selected {
                    selected_rect = Some(response.rect);
                }
                if response.clicked() {
                    clicked = Some(row.clone());
                } else if response.hovered() {
                    app.palette.selected = index;
                }
            }
            (clicked, selected_rect)
        });
    if let Some(rect) = scroll.inner.1 {
        ui.scroll_to_rect(rect, None);
    }

    if activate {
        if let Some(row) = rows.get(app.palette.selected) {
            activated(app, row.action, now);
        }
        return;
    }
    if let Some(row) = scroll.inner.0 {
        activated(app, row.action, now);
        return;
    }

    ui.separator();
    ui.label(
        egui::RichText::new(format!("{} of {} commands", rows.len(), rows.len()))
            .small()
            .color(egui::Color32::GRAY),
    );
}

/// Paint one row: highlighted label, then category and shortcut.
fn row_ui(ui: &mut egui::Ui, row: &Row, selected: bool) -> egui::Response {
    let frame = egui::Frame::new()
        .fill(if selected {
            egui::Color32::from_black_alpha(18)
        } else {
            egui::Color32::TRANSPARENT
        })
        .corner_radius(4.0)
        .inner_margin(egui::Margin::symmetric(6, 3))
        .stroke(if selected {
            egui::Stroke::new(1.0, egui::Color32::from_black_alpha(40))
        } else {
            egui::Stroke::NONE
        });
    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for (text, hit) in highlight_chunks(row.action.label(), &row.hits) {
                let colour = if hit {
                    egui::Color32::from_rgb(20, 90, 160)
                } else if row.enabled {
                    egui::Color32::BLACK
                } else {
                    egui::Color32::from_rgb(130, 130, 130)
                };
                let text = egui::RichText::new(text).color(colour);
                ui.label(if hit { text.strong() } else { text });
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if row.recent {
                    ui.label(
                        egui::RichText::new("recent")
                            .small()
                            .color(egui::Color32::from_rgb(140, 140, 140)),
                    );
                }
                if !row.action.shortcut().is_empty() {
                    ui.label(
                        egui::RichText::new(row.action.shortcut())
                            .small()
                            .color(egui::Color32::from_rgb(110, 110, 110)),
                    );
                }
                ui.label(
                    egui::RichText::new(row.action.category())
                        .small()
                        .color(egui::Color32::from_rgb(150, 150, 150)),
                );
            });
        });
    });
    // A frame is not interactive, so the click is registered against the row
    // rect it just painted.
    ui.interact(
        inner.response.rect,
        egui::Id::new(("palette_row", row.action)),
        egui::Sense::click(),
    )
}

/// The next enabled row after `from`, wrapping around.
fn step(from: usize, rows: &[Row], forward: bool) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let count = rows.len();
    let start = from.min(count - 1);
    for offset in 1..=count {
        let candidate = if forward {
            (start + offset) % count
        } else {
            (start + count - offset % count) % count
        };
        if rows[candidate].enabled {
            return candidate;
        }
    }
    start
}

/// Run an action, or open its parameter form when it needs one.
fn activated(app: &mut LintelApp, action: PaletteAction, now: i64) {
    app.palette.remember(action);
    if !action.is_enabled(&app.doc, &app.selection) {
        return;
    }
    if action.needs_parameters() {
        app.palette.pending = Some(PendingForm::for_action(app, action));
        app.palette.query.clear();
        app.palette.selected = 0;
        return;
    }
    apply_action(app, action, now);
    app.palette.close();
}

/// Apply an action that needs no parameters.
fn apply_action(app: &mut LintelApp, action: PaletteAction, now: i64) {
    match action {
        PaletteAction::SelectTool => app.tool = Tool::Select,
        PaletteAction::WallTool => app.tool = Tool::Wall,
        PaletteAction::DoorTool => app.tool = Tool::Door,
        PaletteAction::WindowTool => app.tool = Tool::Window,
        PaletteAction::DimensionTool => app.tool = Tool::Dimension,
        PaletteAction::LabelTool => app.tool = Tool::Label,
        PaletteAction::MeasurementsTool => app.tool = Tool::Measurements,
        PaletteAction::ConstructionLineTool => app.tool = Tool::ConstructionLine,
        PaletteAction::ConstructionRayTool => app.tool = Tool::ConstructionRay,
        PaletteAction::ConstructionCircleTool => app.tool = Tool::ConstructionCircle,
        PaletteAction::ConstructionArcTool => app.tool = Tool::ConstructionArc,
        PaletteAction::ConstructionVisible => {
            if let Some(EntityId::Construction(id)) = app.selection.primary() {
                let level = app.doc.project().active_level();
                let visible = level
                    .and_then(|l| l.construction_entity(id))
                    .map(|e| !e.visible)
                    .unwrap_or(true);
                let _ = app
                    .doc
                    .apply(Command::SetConstructionVisible { id, visible }, now);
            }
        }
        PaletteAction::DeleteConstruction => {
            if let Some(EntityId::Construction(id)) = app.selection.primary() {
                let _ = app.doc.apply(Command::DeleteConstruction { id }, now);
                app.selection.clear();
            }
        }
        PaletteAction::NewPlan => app.new_plan(),
        PaletteAction::OpenPlan => app.open_plan(now),
        PaletteAction::SavePlan => app.save_plan(now),
        PaletteAction::SavePlanAs => app.save_plan_as(now),
        PaletteAction::ExportSheet => app.export_open = true,
        PaletteAction::ExportDrawingSet => app.export_drawing_set(now),
        PaletteAction::Undo => app.undo_command(now),
        PaletteAction::Redo => app.redo_command(now),
        PaletteAction::DeleteSelection => app.delete_selection(now),
        PaletteAction::DuplicateSelection => app.duplicate_selection(now),
        PaletteAction::SelectAll => app.select_all(),
        PaletteAction::ClearSelection => {
            app.selection.clear();
            app.draft = Draft::default();
        }
        PaletteAction::ZoomFit => app.zoom_to_fit(),
        PaletteAction::ToggleUnits => app.toggle_units(now),
        PaletteAction::ToggleEndpoint => app.snap_modes.toggle(SnapModeKind::Endpoint),
        PaletteAction::ToggleMidpoint => app.snap_modes.toggle(SnapModeKind::Midpoint),
        PaletteAction::ToggleIntersection => app.snap_modes.toggle(SnapModeKind::Intersection),
        PaletteAction::TogglePerpendicular => app.snap_modes.toggle(SnapModeKind::Perpendicular),
        PaletteAction::ToggleTangent => app.snap_modes.toggle(SnapModeKind::Tangent),
        PaletteAction::ToggleGrid => app.snap.grid_enabled = !app.snap.grid_enabled,
        PaletteAction::ToggleOrtho => app.snap.ortho = !app.snap.ortho,
        PaletteAction::ShowAbout => app.showing_about = true,
        PaletteAction::ShowReleaseNotes => app.showing_notes = true,
        PaletteAction::CheckForUpdates => {
            app.start_update_check();
            app.showing_about = true;
        }
        // The parameterised actions never reach here: they open a form first.
        PaletteAction::CreateLayer
        | PaletteAction::LayerVisible
        | PaletteAction::LayerLocked
        | PaletteAction::LayerPrintable
        | PaletteAction::AssignSelectionToLayer
        | PaletteAction::DeleteLayer
        | PaletteAction::RenameProject
        | PaletteAction::AddStorey
        | PaletteAction::DeleteStorey => {}
    }
}

/// The linetypes a new layer can start with.
const LINETYPES: [(&str, Linetype); 4] = [
    ("Continuous", Linetype::Continuous),
    (
        "Dashed",
        Linetype::Dashed {
            dash: 6.0,
            gap: 3.0,
        },
    ),
    (
        "Hidden",
        Linetype::Hidden {
            dash: 3.0,
            gap: 1.5,
        },
    ),
    (
        "Center",
        Linetype::Center {
            dash: 12.0,
            gap: 2.0,
        },
    ),
];

/// Stand-in for "the layer this create just made". The shell swaps it for the
/// id the create returns, so the follow-up properties land on the new layer.
pub(crate) const NEW_LAYER_ID: LayerId = LayerId(u64::MAX);

/// A parameter form, shown inside the palette while the user fills it in.
#[derive(Debug, Clone)]
enum PendingForm {
    /// A new layer.
    Layer(LayerForm),
    /// One property of one existing layer.
    LayerPick {
        action: LayerPickAction,
        layer: usize,
        value: bool,
    },
    /// The project name.
    Project(String),
    /// A new storey.
    Storey { name: String, elevation: f64 },
}

#[derive(Debug, Clone)]
struct LayerForm {
    name: String,
    colour: String,
    linetype: usize,
    lineweight: f64,
    visible: bool,
    locked: bool,
    printable: bool,
}

impl Default for LayerForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            colour: String::new(),
            linetype: 0,
            lineweight: 0.25,
            visible: true,
            locked: false,
            printable: true,
        }
    }
}

/// Which layer property a picker form changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayerPickAction {
    Visible,
    Locked,
    Printable,
    Assign,
    Delete,
}

impl LayerPickAction {
    fn label(self) -> &'static str {
        match self {
            LayerPickAction::Visible => "Layer visibility",
            LayerPickAction::Locked => "Layer lock",
            LayerPickAction::Printable => "Layer printable",
            LayerPickAction::Assign => "Assign selection to layer",
            LayerPickAction::Delete => "Delete layer",
        }
    }
}

impl PendingForm {
    /// A fresh form for `action`, seeded from the current plan.
    fn for_action(app: &LintelApp, action: PaletteAction) -> Self {
        let layer = app.first_assignable_layer_index();
        match action {
            PaletteAction::CreateLayer => PendingForm::Layer(LayerForm::default()),
            PaletteAction::LayerVisible => PendingForm::LayerPick {
                action: LayerPickAction::Visible,
                layer,
                value: true,
            },
            PaletteAction::LayerLocked => PendingForm::LayerPick {
                action: LayerPickAction::Locked,
                layer,
                value: false,
            },
            PaletteAction::LayerPrintable => PendingForm::LayerPick {
                action: LayerPickAction::Printable,
                layer,
                value: true,
            },
            PaletteAction::AssignSelectionToLayer => PendingForm::LayerPick {
                action: LayerPickAction::Assign,
                layer,
                value: true,
            },
            PaletteAction::DeleteLayer => PendingForm::LayerPick {
                action: LayerPickAction::Delete,
                layer,
                value: false,
            },
            PaletteAction::RenameProject => PendingForm::Project(app.doc.project().name.clone()),
            _ => PendingForm::Storey {
                name: String::new(),
                elevation: 3000.0,
            },
        }
    }

    fn title(&self) -> &'static str {
        match self {
            PendingForm::Layer(_) => "Create layer",
            PendingForm::LayerPick { action, .. } => action.label(),
            PendingForm::Project(_) => "Rename project",
            PendingForm::Storey { .. } => "Add storey",
        }
    }
}

/// Render a parameter form. Apply runs the command and returns to the results;
/// Cancel steps back without changing anything. The typed values are stored
/// back each frame, so the form survives as the user types.
fn form_ui(ui: &mut egui::Ui, app: &mut LintelApp, mut form: PendingForm, now: i64) {
    ui.heading(form.title());
    ui.add_space(4.0);

    match &mut form {
        PendingForm::Layer(layer) => layer_form(ui, layer),
        PendingForm::LayerPick {
            action,
            layer,
            value,
        } => layer_pick_form(ui, app, *action, layer, value),
        PendingForm::Project(name) => {
            ui.add(
                egui::TextEdit::singleline(name)
                    .hint_text("Project name")
                    .desired_width(PALLETTE_WIDTH - 24.0),
            )
            .request_focus();
        }
        PendingForm::Storey { name, elevation } => {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.add(
                    egui::TextEdit::singleline(name)
                        .hint_text("New storey")
                        .desired_width(200.0),
                )
                .request_focus();
            });
            ui.horizontal(|ui| {
                ui.label("Elevation:");
                ui.add(
                    egui::DragValue::new(elevation)
                        .speed(100.0)
                        .range(-100_000.0..=100_000.0)
                        .suffix(" mm"),
                )
                .on_hover_text("Height of the new storey above the project datum.");
            });
        }
    }

    let commands = form_commands(&form, app);
    let valid = !commands.is_empty();
    ui.add_space(6.0);
    let mut done = false;
    ui.horizontal(|ui| {
        if ui
            .add_enabled(valid, egui::Button::new("Apply"))
            .on_hover_text(if valid {
                "Run the command"
            } else {
                "Fill in the missing values first"
            })
            .clicked()
        {
            let _ = app.run_commands(commands, now);
            done = true;
        }
        if ui
            .button("Cancel")
            .on_hover_text("Go back to the command list (Esc)")
            .clicked()
        {
            done = true;
        }
    });

    if done {
        app.palette.pending = None;
        app.palette.query.clear();
        app.palette.selected = 0;
    } else {
        app.palette.pending = Some(form);
    }
}

/// The body of the layer forms: name, colour, linetype and lineweight.
fn layer_form(ui: &mut egui::Ui, form: &mut LayerForm) {
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.add(
            egui::TextEdit::singleline(&mut form.name)
                .hint_text("Layer name")
                .desired_width(200.0),
        )
        .request_focus();
    });
    ui.horizontal(|ui| {
        ui.label("Colour:");
        ui.add(
            egui::TextEdit::singleline(&mut form.colour)
                .hint_text("#RRGGBB or blank")
                .desired_width(120.0),
        )
        .on_hover_text("Optional layer colour; blank uses the default ink.");
        if !form.colour.trim().is_empty() {
            let ok = Rgb::from_hex(form.colour.trim()).is_some();
            let text = if ok { "ok" } else { "bad hex" };
            let colour = if ok {
                egui::Color32::from_rgb(30, 130, 70)
            } else {
                egui::Color32::DARK_RED
            };
            ui.label(egui::RichText::new(text).small().color(colour));
        }
    });
    ui.horizontal(|ui| {
        ui.label("Linetype:");
        let selected = LINETYPES
            .get(form.linetype)
            .map_or("Continuous", |(label, _)| *label);
        egui::ComboBox::from_id_salt("palette_layer_linetype")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (index, (label, _)) in LINETYPES.iter().enumerate() {
                    if ui
                        .selectable_label(index == form.linetype, *label)
                        .clicked()
                    {
                        form.linetype = index;
                    }
                }
            });
    });
    ui.horizontal(|ui| {
        ui.label("Lineweight:");
        ui.add(
            egui::DragValue::new(&mut form.lineweight)
                .speed(0.05)
                .range(0.05..=5.0)
                .suffix(" mm"),
        )
        .on_hover_text("Plotted linewidth in millimetres (0.05–5).");
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut form.visible, "Visible")
            .on_hover_text("Hidden layers are neither drawn nor picked.");
        ui.checkbox(&mut form.locked, "Locked")
            .on_hover_text("A locked layer is drawn but cannot be edited.");
        ui.checkbox(&mut form.printable, "Printable")
            .on_hover_text("Printable layers reach the exported sheet.");
    });
}

/// The body of the layer picker forms: one layer, and a value where it needs one.
fn layer_pick_form(
    ui: &mut egui::Ui,
    app: &LintelApp,
    action: LayerPickAction,
    layer: &mut usize,
    value: &mut bool,
) {
    let layers = app.layer_labels();
    if layers.is_empty() {
        ui.label("This level has no layer to change.");
        return;
    }
    *layer = (*layer).min(layers.len() - 1);

    ui.horizontal(|ui| {
        ui.label("Layer:");
        let selected = layers
            .get(*layer)
            .map_or("", |(_, name)| name.as_str())
            .to_string();
        egui::ComboBox::from_id_salt("palette_layer_pick")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (index, (id, name)) in layers.iter().enumerate() {
                    let deletable = *id != LayerId::DEFAULT;
                    let text = if deletable {
                        name.clone()
                    } else {
                        format!("{name} (protected)")
                    };
                    if ui
                        .selectable_label(index == *layer, text)
                        .on_hover_text(if deletable {
                            "Choose this layer"
                        } else {
                            "The default layer always exists and cannot be deleted"
                        })
                        .clicked()
                    {
                        *layer = index;
                    }
                }
            });
    });

    match action {
        LayerPickAction::Visible | LayerPickAction::Locked | LayerPickAction::Printable => {
            let on_label = match action {
                LayerPickAction::Visible => "Visible",
                LayerPickAction::Locked => "Locked",
                _ => "Printable",
            };
            ui.horizontal(|ui| {
                ui.radio_value(value, true, on_label);
                ui.radio_value(value, false, format!("Not {on_label}"));
            });
        }
        LayerPickAction::Assign => {
            ui.label(
                egui::RichText::new(format!("{} selected", app.selection.len()))
                    .small()
                    .color(egui::Color32::GRAY),
            );
        }
        LayerPickAction::Delete => {
            if layers
                .get(*layer)
                .is_some_and(|(id, _)| *id == LayerId::DEFAULT)
            {
                ui.label(
                    egui::RichText::new("The default layer cannot be deleted.")
                        .small()
                        .color(egui::Color32::DARK_RED),
                );
            }
        }
    }
}

/// The commands a completed form runs, or an empty list while it is not
/// fillable yet.
fn form_commands(form: &PendingForm, app: &LintelApp) -> Vec<Command> {
    match form {
        PendingForm::Layer(layer) => {
            let name = layer.name.trim();
            if name.is_empty() {
                return Vec::new();
            }
            if !layer.colour.trim().is_empty() && Rgb::from_hex(layer.colour.trim()).is_none() {
                return Vec::new();
            }
            let linetype = LINETYPES
                .get(layer.linetype)
                .map_or(Linetype::Continuous, |(_, kind)| *kind);
            // A new layer is created with the defaults and then corrected, so
            // it can be born hidden, locked or non-printing.
            let mut commands = vec![Command::CreateLayer {
                name: name.to_string(),
                color: Rgb::from_hex(layer.colour.trim()),
                linetype,
                lineweight_mm: layer.lineweight,
            }];
            if !layer.visible {
                commands.push(Command::SetLayerProperty {
                    layer_id: NEW_LAYER_ID,
                    property: LayerProperty::Visible(false),
                });
            }
            if layer.locked {
                commands.push(Command::SetLayerProperty {
                    layer_id: NEW_LAYER_ID,
                    property: LayerProperty::Locked(true),
                });
            }
            if !layer.printable {
                commands.push(Command::SetLayerProperty {
                    layer_id: NEW_LAYER_ID,
                    property: LayerProperty::Printable(false),
                });
            }
            commands
        }
        PendingForm::LayerPick {
            action,
            layer,
            value,
        } => {
            let layers = app.layer_labels();
            let Some((id, _)) = layers.get(*layer) else {
                return Vec::new();
            };
            let id = *id;
            match action {
                LayerPickAction::Visible => vec![Command::SetLayerProperty {
                    layer_id: id,
                    property: LayerProperty::Visible(*value),
                }],
                LayerPickAction::Locked => vec![Command::SetLayerProperty {
                    layer_id: id,
                    property: LayerProperty::Locked(*value),
                }],
                LayerPickAction::Printable => vec![Command::SetLayerProperty {
                    layer_id: id,
                    property: LayerProperty::Printable(*value),
                }],
                LayerPickAction::Assign => {
                    if id == LayerId::DEFAULT {
                        return Vec::new();
                    }
                    app.selection
                        .primary()
                        .and_then(selector_for)
                        .map(|entity| {
                            vec![Command::AssignEntityToLayer {
                                entity,
                                layer_id: id,
                            }]
                        })
                        .unwrap_or_default()
                }
                LayerPickAction::Delete => {
                    if id == LayerId::DEFAULT {
                        return Vec::new();
                    }
                    vec![Command::DeleteLayer { layer_id: id }]
                }
            }
        }
        PendingForm::Project(name) => {
            let name = name.trim();
            if name.is_empty() {
                return Vec::new();
            }
            vec![Command::RenameProject {
                name: name.to_string(),
            }]
        }
        PendingForm::Storey { name, elevation } => {
            let name = name.trim();
            if name.is_empty() {
                return Vec::new();
            }
            vec![Command::AddLevel {
                name: name.to_string(),
                elevation: *elevation,
            }]
        }
    }
}

/// The domain selector for a canvas pick, so a selection can be assigned.
///
/// A construction guide has no selector — it is not layered — so assigning the
/// most recent pick does nothing for one.
fn selector_for(id: EntityId) -> Option<EntitySelector> {
    match id {
        EntityId::Wall(id) => Some(EntitySelector::Wall(id)),
        EntityId::Opening(id) => Some(EntitySelector::Opening(id)),
        EntityId::Dimension(id) => Some(EntitySelector::Dimension(id)),
        EntityId::Label(id) => Some(EntitySelector::Label(id)),
        EntityId::Construction(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_query_matches_everything_without_highlighting() {
        let matched = fuzzy_match("", "Wall tool").expect("empty query matches");
        assert_eq!(matched.score, 0);
        assert!(matched.hits.is_empty());
    }

    #[test]
    fn a_query_that_is_not_a_subsequence_does_not_match() {
        assert_eq!(fuzzy_match("zzz", "Wall tool"), None);
        assert_eq!(fuzzy_match("lwa", "Wall tool"), None, "order matters");
    }

    #[test]
    fn matching_ignores_case() {
        let matched = fuzzy_match("WALL", "Wall tool").expect("case-insensitive");
        assert_eq!(matched.hits, vec![0, 1, 2, 3]);
    }

    #[test]
    fn a_consecutive_run_beats_scattered_letters() {
        let query = "port";
        let consecutive = fuzzy_match(query, "Export sheet").expect("matches");
        // "Support" has "port" scattered: p at 2, o at 4 (gap), r at 5, t at 6
        let scattered = fuzzy_match(query, "Support").expect("matches");
        assert!(
            consecutive.score > scattered.score,
            "consecutive {} should beat scattered {}",
            consecutive.score,
            scattered.score
        );
    }

    #[test]
    fn a_word_start_ranks_a_label_above_a_buried_hit() {
        let word_start = fuzzy_match("es", "Export sheet").expect("matches");
        let buried = fuzzy_match("es", "Layer notes").expect("matches");
        assert!(word_start.score > buried.score);
    }

    #[test]
    fn a_shorter_label_wins_a_tie() {
        let short = fuzzy_match("a", "A").expect("matches");
        let long = fuzzy_match("a", "A very very very very long label").expect("matches");
        assert!(short.score > long.score);
    }

    #[test]
    fn highlights_group_runs_of_matched_characters() {
        let matched = fuzzy_match("walt", "Wall tool").expect("matches");
        let chunks = highlight_chunks("Wall tool", &matched.hits);
        // "walt" matches W(0), a(1), l(2), t(5) -> "Wal" run then "t"
        assert_eq!(chunks[0], ("Wal".to_string(), true));
        assert!(chunks.iter().any(|(text, hit)| text == "t" && *hit));
        assert!(chunks.iter().any(|(text, hit)| text == "l " && !*hit));
    }

    #[test]
    fn every_action_is_named_and_grouped() {
        let mut labels = Vec::new();
        for action in PaletteAction::ALL {
            assert!(!action.label().is_empty(), "{action:?} needs a label");
            assert!(!action.category().is_empty());
            assert!(!action.shortcut().is_empty() || action.needs_parameters());
            labels.push(action.label());
        }
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), PaletteAction::ALL.len(), "labels are unique");
    }
}
