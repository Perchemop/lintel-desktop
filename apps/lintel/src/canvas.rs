//! Canvas panel: grid, construction guides, walls, openings, selection, draft
//! preview, snap markers and the rubber-band selection box.
//!
//! Pure presentation: world millimetres → screen pixels, pointer events →
//! intents the app applies to the core document. The canvas never mutates the
//! model itself.

use crate::theme;
use eframe::egui;
use lintel_core::construction::{ConstructionEntity, ConstructionKind};
use lintel_core::geom::{self, Bounds, Point2};
use lintel_core::ids::EntityId;
use lintel_core::join::{Hit, PlanGeometry};
use lintel_core::selection::{SelectionFilter, SelectionMode, select};
use lintel_core::snap::{self, SnapSettings};

/// How far apart two snap candidates may be, in millimetres, and still count as
/// the same point. It keeps a midpoint that sits 0.2 mm from a wall end from
/// being treated as the *nearer* candidate: within this band the priority order
/// decides.
const SNAP_TIE_MM: f64 = 0.5;

/// Radius of the marker drawn on a winning snap candidate, in pixels.
const SNAP_MARKER_PX: f32 = 4.5;

/// Radius of the marker drawn on a candidate that lost to a nearer one.
const SNAP_HINT_PX: f32 = 2.5;

/// How far an unbounded construction guide is drawn and tested for
/// intersections, in millimetres. Far beyond any viewport, so an infinite
/// construction line still offers a crossing with a short one.
const GUIDE_EXTENT_MM: f64 = 1.0e7;

/// Construction guides are drawn thin and warm-grey so they read as a guide,
/// never as the building. They never reach the sheet whatever the canvas shows.
const CONSTRUCTION_STROKE: egui::Color32 = egui::Color32::from_rgb(168, 148, 116);

/// What the canvas wants the app to do after one frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct CanvasResponse {
    pub pointer_world: Option<Point2>,
    pub click: Option<egui::Pos2>, // primary-button click, in screen space
    pub drag_delta: Option<egui::Vec2>, // primary-button drag, screen delta
    pub pan_delta: Option<egui::Vec2>, // middle-button drag, screen delta
    pub zoom: Option<(f64, egui::Pos2)>, // factor, anchor in screen space
    /// Where the primary button went down, in screen space. An entity drag and
    /// a rubber-band selection both begin with a press, and only this tells
    /// them apart.
    pub press: Option<egui::Pos2>,
    /// True on the frame a primary drag begins.
    pub drag_started: bool,
    /// True on the frame a primary drag ends.
    pub drag_stopped: bool,
    /// What the snapper settled on this frame, for the status bar readout.
    pub snap: Option<SnapReadout>,
    /// The canvas widget's id, so the shell can tell keyboard focus on the
    /// canvas apart from focus inside a text field.
    pub widget_id: Option<egui::Id>,
}

/// The point the snapper settled on and why, handed to the status bar.
#[derive(Debug, Clone, Copy)]
pub struct SnapReadout {
    pub point: Point2,
    pub kind: SnapModeKind,
}

/// Draft geometry for the wall tool.
#[derive(Debug, Clone, Copy, Default)]
pub struct Draft {
    pub active: bool,
    pub start: Point2,
    pub current: Point2,
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
        Self {
            centre: Point2::ORIGIN,
            px_per_mm: 0.4,
        }
    }
}

impl View {
    pub fn zoom_percent(&self) -> i32 {
        (self.px_per_mm as f64 / 0.4 * 100.0).round() as i32
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

/// What the canvas has selected.
///
/// The ids are presentation state — the model has no notion of selection — but
/// every *pick* is run by the domain: `lintel_core::selection::select` decides
/// what a window, crossing or fence touches, and this type only remembers the
/// answer, in the order it was found, so "the most recent pick" is well
/// defined.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Selection {
    /// Selected ids, oldest first, so `primary()` is the most recent pick.
    ids: Vec<EntityId>,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.ids.contains(&id)
    }

    /// The most recent pick: what the Inspector edits.
    pub fn primary(&self) -> Option<EntityId> {
        self.ids.last().copied()
    }

    /// Drop everything and select one entity.
    pub fn replace(&mut self, id: EntityId) {
        self.ids.clear();
        self.ids.push(id);
    }

    /// Adopt a hit-or-nothing result from a click on empty canvas.
    pub fn set_from_hit(&mut self, hit: Option<Hit>) {
        match hit {
            Some(hit) => self.replace(entity_of(hit)),
            None => self.clear(),
        }
    }

    /// Add, unless it is already selected.
    pub fn add(&mut self, id: EntityId) {
        if !self.contains(id) {
            self.ids.push(id);
        }
    }

    /// Remove, if present.
    pub fn remove(&mut self, id: EntityId) {
        self.ids.retain(|selected| *selected != id);
    }

    /// Add when absent, remove when present: the Shift+click behaviour.
    pub fn toggle(&mut self, id: EntityId) {
        if self.contains(id) {
            self.remove(id);
        } else {
            self.add(id);
        }
    }

    pub fn set(&mut self, ids: impl IntoIterator<Item = EntityId>) {
        self.ids = ids.into_iter().collect();
    }

    pub fn clear(&mut self) {
        self.ids.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.ids.iter().copied()
    }

    /// Run a rubber-band pick and merge it into the set: a plain box replaces,
    /// Shift adds and Ctrl removes.
    pub fn pick_box(
        &mut self,
        geometry: &PlanGeometry,
        bounds: Bounds,
        crossing: bool,
        add: bool,
        remove: bool,
    ) {
        let picked = pick_sorted(geometry, bounds, crossing);
        if !add && !remove {
            self.ids.clear();
        }
        for id in picked {
            if remove {
                self.remove(id);
            } else {
                self.add(id);
            }
        }
    }

    /// Select everything the filter allows on the active level.
    pub fn pick_all(&mut self, geometry: &PlanGeometry) {
        let picked = select(geometry, SelectionMode::All, SelectionFilter::All);
        let mut picked: Vec<EntityId> = picked.into_iter().collect();
        picked.sort_by_key(|id| id.value());
        self.ids = picked;
    }
}

/// A domain pick, in a stable order so the shell's reports do not jitter.
fn pick_sorted(geometry: &PlanGeometry, bounds: Bounds, crossing: bool) -> Vec<EntityId> {
    let mode = SelectionMode::Window { bounds, crossing };
    let picked = select(geometry, mode, SelectionFilter::All);
    let mut picked: Vec<EntityId> = picked.into_iter().collect();
    picked.sort_by_key(|id| id.value());
    picked
}

/// Which way a rubber-band drag was started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionModeKind {
    /// Left to right: only what the box fully encloses.
    Window,
    /// Right to left: whatever the box touches.
    Crossing,
}

impl SelectionModeKind {
    /// The direction the pointer is travelling, in world x. Screen x and world
    /// x always run the same way, so a world test decides the mode.
    pub fn from_direction(from: Point2, to: Point2) -> Self {
        if to.x >= from.x {
            SelectionModeKind::Window
        } else {
            SelectionModeKind::Crossing
        }
    }

    /// AutoCAD's convention: a window box is blue, a crossing box green.
    pub fn colour(self) -> egui::Color32 {
        match self {
            SelectionModeKind::Window => theme::COBALT,
            SelectionModeKind::Crossing => theme::GREEN,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SelectionModeKind::Window => "Window selection",
            SelectionModeKind::Crossing => "Crossing selection",
        }
    }
}

/// A rubber-band selection drag in progress, owned by the shell. The far corner
/// is always the pointer, so the canvas can draw it without any extra state.
#[derive(Debug, Clone)]
pub struct SelectionBand {
    /// Where the primary button went down, in screen space.
    pub start: egui::Pos2,
    /// Shift was held: the box adds to the selection instead of replacing it.
    pub add: bool,
    /// Ctrl/Cmd was held: the box removes from it.
    pub remove: bool,
}

/// The identity of a hit, so a click can name what it found.
pub fn entity_of(hit: Hit) -> EntityId {
    match hit {
        Hit::Wall(id) => EntityId::Wall(id),
        Hit::Opening(id) => EntityId::Opening(id),
        Hit::Dimension(id) => EntityId::Dimension(id),
        Hit::Label(id) => EntityId::Label(id),
    }
}

/// A snap candidate the pointer can lock on to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapCandidate {
    pub point: Point2,
    pub kind: SnapModeKind,
}

/// Why a point landed where it did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapModeKind {
    Endpoint,
    Intersection,
    Midpoint,
    Perpendicular,
    Tangent,
    Ortho,
    Grid,
    Free,
}

impl SnapModeKind {
    /// The order candidates are offered in: the order a drafter expects an
    /// endpoint to win.
    pub const PRIORITY: [SnapModeKind; 8] = [
        SnapModeKind::Endpoint,
        SnapModeKind::Intersection,
        SnapModeKind::Midpoint,
        SnapModeKind::Perpendicular,
        SnapModeKind::Tangent,
        SnapModeKind::Ortho,
        SnapModeKind::Grid,
        SnapModeKind::Free,
    ];

    pub fn label(self) -> &'static str {
        match self {
            SnapModeKind::Endpoint => "Endpoint",
            SnapModeKind::Intersection => "Intersection",
            SnapModeKind::Midpoint => "Midpoint",
            SnapModeKind::Perpendicular => "Perpendicular",
            SnapModeKind::Tangent => "Tangent",
            SnapModeKind::Ortho => "Ortho",
            SnapModeKind::Grid => "Grid",
            SnapModeKind::Free => "Free",
        }
    }

    /// Lower is offered first. Ortho and grid are constraints, not candidates,
    /// so they are considered after every entity snap.
    pub fn priority(self) -> usize {
        Self::PRIORITY
            .iter()
            .position(|kind| *kind == self)
            .unwrap_or(Self::PRIORITY.len())
    }

    /// Marker colour on the canvas: endpoint keeps the green Lintel has always
    /// used, and each other kind gets its own so the label is never the only
    /// signal.
    pub fn colour(self) -> egui::Color32 {
        match self {
            SnapModeKind::Endpoint => theme::GREEN,
            SnapModeKind::Intersection => theme::COBALT,
            SnapModeKind::Midpoint => egui::Color32::from_rgb(176, 106, 22),
            SnapModeKind::Perpendicular => egui::Color32::from_rgb(120, 74, 160),
            SnapModeKind::Tangent => egui::Color32::from_rgb(24, 128, 140),
            SnapModeKind::Ortho => egui::Color32::from_rgb(90, 82, 74),
            SnapModeKind::Grid => egui::Color32::from_rgb(130, 130, 130),
            SnapModeKind::Free => theme::INK,
        }
    }
}

/// The switchable snap modes.
///
/// Ortho and grid are deliberately absent: they are constraints rather than
/// candidates and stay owned by `lintel_core::snap::SnapSettings`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapModes {
    pub endpoint: bool,
    pub midpoint: bool,
    pub intersection: bool,
    pub perpendicular: bool,
    pub tangent: bool,
}

impl Default for SnapModes {
    fn default() -> Self {
        // The three that keep a plan connected come first; the drafting ones
        // stay off until asked for, as every CAD default does.
        Self {
            endpoint: true,
            midpoint: true,
            intersection: true,
            perpendicular: false,
            tangent: false,
        }
    }
}

impl SnapModes {
    /// The five switchable modes, in the order the status bar lists them.
    pub const KINDS: [SnapModeKind; 5] = [
        SnapModeKind::Endpoint,
        SnapModeKind::Midpoint,
        SnapModeKind::Intersection,
        SnapModeKind::Perpendicular,
        SnapModeKind::Tangent,
    ];

    pub fn enabled(self, kind: SnapModeKind) -> bool {
        match kind {
            SnapModeKind::Endpoint => self.endpoint,
            SnapModeKind::Midpoint => self.midpoint,
            SnapModeKind::Intersection => self.intersection,
            SnapModeKind::Perpendicular => self.perpendicular,
            SnapModeKind::Tangent => self.tangent,
            SnapModeKind::Ortho | SnapModeKind::Grid | SnapModeKind::Free => false,
        }
    }

    pub fn toggle(&mut self, kind: SnapModeKind) {
        match kind {
            SnapModeKind::Endpoint => self.endpoint = !self.endpoint,
            SnapModeKind::Midpoint => self.midpoint = !self.midpoint,
            SnapModeKind::Intersection => self.intersection = !self.intersection,
            SnapModeKind::Perpendicular => self.perpendicular = !self.perpendicular,
            SnapModeKind::Tangent => self.tangent = !self.tangent,
            SnapModeKind::Ortho | SnapModeKind::Grid | SnapModeKind::Free => {}
        }
    }

    /// The short name the status bar chips use.
    pub fn label(kind: SnapModeKind) -> &'static str {
        match kind {
            SnapModeKind::Endpoint => "End",
            SnapModeKind::Midpoint => "Mid",
            SnapModeKind::Intersection => "Int",
            SnapModeKind::Perpendicular => "Perp",
            SnapModeKind::Tangent => "Tan",
            SnapModeKind::Ortho | SnapModeKind::Grid | SnapModeKind::Free => kind.label(),
        }
    }

    /// The single letter that toggles this mode, shown in the status bar hint.
    pub fn key_hint(kind: SnapModeKind) -> &'static str {
        match kind {
            SnapModeKind::Endpoint => "E",
            SnapModeKind::Midpoint => "V",
            SnapModeKind::Intersection => "I",
            SnapModeKind::Perpendicular => "P",
            SnapModeKind::Tangent => "T",
            SnapModeKind::Ortho | SnapModeKind::Grid | SnapModeKind::Free => "",
        }
    }
}

/// Where two segments cross, or `None` when they do not.
pub fn segment_intersection(a: Point2, b: Point2, c: Point2, d: Point2) -> Option<Point2> {
    let r = b - a;
    let s = d - c;
    let denominator = r.cross(s);
    // Parallel or collinear: no single crossing to offer.
    if denominator.abs() < f64::EPSILON {
        return None;
    }
    let q = c - a;
    let t = q.cross(s) / denominator;
    let u = q.cross(r) / denominator;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(a + r * t)
    } else {
        None
    }
}

/// The straight guides of a level, in millimetres: wall centrelines plus the
/// bounded shape of any visible construction line.
fn straight_guides(geometry: &PlanGeometry) -> Vec<(Point2, Point2)> {
    let mut guides: Vec<(Point2, Point2)> = geometry
        .walls
        .iter()
        .map(|wall| (wall.wall.start, wall.wall.end))
        .collect();
    for guide in geometry.construction.iter().filter(|guide| guide.visible) {
        let kind = guide.kind.clone();
        match kind {
            ConstructionKind::Line { start, end } => guides.push((start, end)),
            ConstructionKind::InfiniteLine { point, direction } => {
                let step = direction.normalized().scaled(GUIDE_EXTENT_MM);
                guides.push((point - step, point + step));
            }
            ConstructionKind::Ray { origin, direction } => {
                guides.push((
                    origin,
                    origin + direction.normalized().scaled(GUIDE_EXTENT_MM),
                ));
            }
            ConstructionKind::Circle { .. } | ConstructionKind::Arc { .. } => {}
        }
    }
    guides
}

/// Every snap candidate the enabled modes offer for `pointer`.
///
/// `origin` is the last picked point, which is what perpendicular snapping
/// measures from. Construction geometry is a first-class source: its own
/// `anchors()` decides what it offers, so nothing here re-derives geometry that
/// the domain already knows.
pub fn snap_candidates(
    geometry: &PlanGeometry,
    modes: &SnapModes,
    pointer: Option<Point2>,
    origin: Option<Point2>,
) -> Vec<SnapCandidate> {
    let mut candidates = Vec::new();
    // A hidden guide is neither drawn nor picked, so it offers nothing.
    let guides_visible: Vec<&ConstructionEntity> = geometry
        .construction
        .iter()
        .filter(|guide| guide.visible)
        .collect();

    if modes.endpoint {
        for wall in &geometry.walls {
            candidates.push(SnapCandidate {
                point: wall.wall.start,
                kind: SnapModeKind::Endpoint,
            });
            candidates.push(SnapCandidate {
                point: wall.wall.end,
                kind: SnapModeKind::Endpoint,
            });
        }
        for guide in &guides_visible {
            for anchor in guide.anchors() {
                candidates.push(SnapCandidate {
                    point: anchor,
                    kind: SnapModeKind::Endpoint,
                });
            }
        }
    }

    if modes.midpoint {
        for wall in &geometry.walls {
            candidates.push(SnapCandidate {
                point: midpoint(wall.wall.start, wall.wall.end),
                kind: SnapModeKind::Midpoint,
            });
        }
        for guide in &guides_visible {
            let kind = guide.kind.clone();
            if let ConstructionKind::Line { start, end } = kind {
                candidates.push(SnapCandidate {
                    point: midpoint(start, end),
                    kind: SnapModeKind::Midpoint,
                });
            }
        }
    }

    let guides = straight_guides(geometry);

    if modes.intersection {
        for (index, (a, b)) in guides.iter().enumerate() {
            for (c, d) in &guides[index + 1..] {
                if let Some(point) = segment_intersection(*a, *b, *c, *d) {
                    candidates.push(SnapCandidate {
                        point,
                        kind: SnapModeKind::Intersection,
                    });
                }
            }
        }
    }

    if modes.perpendicular {
        let from = origin.or(pointer);
        if let Some(from) = from {
            for (a, b) in &guides {
                let (t, foot) = geom::closest_point_on_segment(from, *a, *b);
                // Strictly inside: an end of the guide is already an endpoint
                // candidate, and offering it twice would only blur the marker.
                if t > 0.0 && t < 1.0 {
                    candidates.push(SnapCandidate {
                        point: foot,
                        kind: SnapModeKind::Perpendicular,
                    });
                }
            }
        }
    }

    if modes.tangent {
        for guide in &guides_visible {
            let (center, radius) = match guide.kind.clone() {
                ConstructionKind::Circle { center, radius } => (center, radius),
                ConstructionKind::Arc { center, radius, .. } => (center, radius),
                _ => continue,
            };
            if let Some(pointer) = pointer {
                let outward = pointer - center;
                if outward.length_sq() > f64::EPSILON {
                    candidates.push(SnapCandidate {
                        point: center + outward.normalized().scaled(radius),
                        kind: SnapModeKind::Tangent,
                    });
                }
            }
        }
    }

    candidates
}

fn midpoint(a: Point2, b: Point2) -> Point2 {
    Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)
}

/// Where a pointer position should actually land, and which candidate won.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapResolution {
    pub point: Point2,
    pub kind: SnapModeKind,
    /// The candidate that won, if the win came from a candidate rather than
    /// from the domain's constraint snapping.
    pub candidate: Option<SnapCandidate>,
}

/// Nearest candidate wins; within `SNAP_TIE_MM` the priority order decides, so a
/// wall end still beats a midpoint that shares its position.
fn pick_snap(
    raw: Point2,
    candidates: &[SnapCandidate],
    origin: Option<Point2>,
    settings: &SnapSettings,
) -> SnapResolution {
    let mut best: Option<(f64, usize, SnapCandidate)> = None;
    for candidate in candidates {
        let distance = raw.distance(candidate.point);
        if distance > settings.anchor_tolerance {
            continue;
        }
        let better = match best {
            None => true,
            Some((best_distance, best_priority, _)) => {
                distance < best_distance - SNAP_TIE_MM
                    || (distance < best_distance + SNAP_TIE_MM
                        && candidate.kind.priority() < best_priority)
            }
        };
        if better {
            best = Some((distance, candidate.kind.priority(), *candidate));
        }
    }
    if let Some((_, _, candidate)) = best {
        return SnapResolution {
            point: candidate.point,
            kind: candidate.kind,
            candidate: Some(candidate),
        };
    }

    // No candidate: the domain decides the constraint snapping, which is the
    // part that has always lived in the core.
    let snapped = snap::snap_point(raw, &[], origin, settings);
    let kind = match snapped.kind {
        snap::SnapKind::Ortho => SnapModeKind::Ortho,
        snap::SnapKind::Grid => SnapModeKind::Grid,
        _ => SnapModeKind::Free,
    };
    SnapResolution {
        point: snapped.point,
        kind,
        candidate: None,
    }
}

/// Where a pointer position should actually land, and why.
pub fn resolve_snap(
    raw: Point2,
    geometry: &PlanGeometry,
    modes: &SnapModes,
    origin: Option<Point2>,
    settings: &SnapSettings,
) -> SnapResolution {
    let candidates = snap_candidates(geometry, modes, Some(raw), origin);
    pick_snap(raw, &candidates, origin, settings)
}

pub struct Canvas {}

impl Canvas {
    /// Paint one frame of the plan.
    ///
    /// `band` is the rubber-band selection drag in progress, drawn from its
    /// start corner to the pointer. The canvas only ever reads state: every
    /// pointer intent comes back in the [`CanvasResponse`] for the shell to
    /// turn into commands.
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        ui: &mut egui::Ui,
        view: &View,
        geometry: &PlanGeometry,
        selection: &Selection,
        modes: &SnapModes,
        snap: &SnapSettings,
        draft: &Draft,
        summary: &str,
        band: Option<&SelectionBand>,
    ) -> (CanvasResponse, egui::Rect) {
        let mut response = CanvasResponse::default();

        // The canvas is focusable so keyboard users can Tab to it, and it
        // carries a screen-reader label describing the active level (guide
        // II.21.2). Individual plan objects cannot be labelled: they are drawn
        // by a custom painter, which is recorded in ACCESSIBILITY.md.
        let (egui_response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag() | egui::Sense::focusable_noninteractive(),
        );
        let rect = egui_response.rect;
        response.widget_id = Some(egui_response.id);
        response.press = egui_response.interact_pointer_pos();
        response.drag_started = egui_response.drag_started_by(egui::PointerButton::Primary);
        response.drag_stopped = egui_response.drag_stopped_by(egui::PointerButton::Primary);
        painter.rect_filled(rect, 0.0, theme::PAPER);

        let hover = egui_response.hover_pos();
        response.pointer_world = hover.map(|p| screen_to_world(view, rect, p));

        // Grid.
        if snap.grid_enabled && snap.grid.is_finite() && snap.grid > 0.0 {
            paint_grid(&painter, rect, view, snap.grid);
        }

        // Construction guides: drawn first, so the building sits on top of the
        // guides that placed it.
        for guide in geometry.construction.iter().filter(|g| g.visible) {
            paint_construction(&painter, rect, view, guide);
        }

        for wall in &geometry.walls {
            paint_wall(
                &painter,
                rect,
                view,
                wall,
                selection.contains(EntityId::Wall(wall.wall.id)),
            );
        }
        for opening in &geometry.openings {
            paint_opening(
                &painter,
                rect,
                view,
                opening,
                selection.contains(EntityId::Opening(opening.id)),
            );
        }

        // Rooms: area label at each centroid.
        for room in geometry.rooms() {
            let pos = world_to_screen(view, rect, room.centroid);
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                room.area_label(),
                egui::FontId::proportional(11.0),
                theme::INK.linear_multiply(0.55),
            );
        }

        // Labels.
        for label in geometry.labels() {
            let pos = world_to_screen(view, rect, label.anchor);
            let colour = if selection.contains(EntityId::Label(label.id)) {
                theme::COBALT
            } else {
                theme::INK
            };
            painter.text(
                pos,
                egui::Align2::LEFT_BOTTOM,
                &label.text,
                egui::FontId::proportional(
                    (label.height_mm as f32 * view.px_per_mm).clamp(9.0, 96.0),
                ),
                colour,
            );
        }

        // Snap markers: a small square on every candidate near the pointer, and
        // a filled marker with the kind beside the one that wins. Same
        // resolution the shell will apply on the next click.
        let origin = draft.active.then_some(draft.start);
        let candidates = snap_candidates(geometry, modes, response.pointer_world, origin);
        let resolution = response
            .pointer_world
            .filter(|point| point.is_finite())
            .map(|world| pick_snap(world, &candidates, origin, snap));
        response.snap = resolution.map(|resolved| SnapReadout {
            point: resolved.point,
            kind: resolved.kind,
        });
        if let Some(resolved) = resolution
            && let Some(pointer) = response.pointer_world
        {
            for candidate in &candidates {
                let distance = pointer.distance(candidate.point);
                if distance > snap.anchor_tolerance {
                    continue;
                }
                let screen = world_to_screen(view, rect, candidate.point);
                let is_winner = matches!(resolved.candidate, Some(winner) if winner == *candidate);
                if is_winner {
                    painter.circle_filled(screen, SNAP_MARKER_PX, candidate.kind.colour());
                    painter.text(
                        screen + egui::vec2(9.0, -7.0),
                        egui::Align2::LEFT_TOP,
                        candidate.kind.label(),
                        egui::FontId::proportional(11.0),
                        candidate.kind.colour(),
                    );
                } else {
                    painter.circle_filled(
                        screen,
                        SNAP_HINT_PX,
                        candidate.kind.colour().linear_multiply(0.55),
                    );
                }
            }
        }

        // Draft preview (wall tool).
        if draft.active {
            let start_screen = world_to_screen(view, rect, draft.start);
            let end = resolved_or(draft.current, resolution);
            let end_screen = world_to_screen(view, rect, end);
            painter.line_segment([start_screen, end_screen], theme::selection_stroke());
            let length = draft.start.distance(end);
            let angle = (end - draft.start).angle_degrees();
            let pos = end_screen + egui::Vec2::new(8.0, -6.0);
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                format!("{length:.0} mm  {angle:.0}°"),
                egui::FontId::proportional(11.0),
                theme::INK,
            );
        }

        // Rubber-band selection box.
        if let Some(band) = band
            && let Some(hover) = hover
        {
            let box_rect = egui::Rect::from_two_pos(band.start, hover);
            let mode = SelectionModeKind::from_direction(
                screen_to_world(view, rect, band.start),
                screen_to_world(view, rect, hover),
            );
            let colour = mode.colour();
            painter.rect_filled(box_rect, 0.0, colour.linear_multiply(0.12));
            painter.rect_stroke(
                box_rect,
                0.0,
                egui::Stroke::new(1.0, colour),
                egui::StrokeKind::Inside,
            );
            painter.text(
                box_rect.left_top() + egui::vec2(4.0, 4.0),
                egui::Align2::LEFT_TOP,
                mode.label(),
                egui::FontId::proportional(11.0),
                colour,
            );
        }

        // Input intents.
        if egui_response.clicked_by(egui::PointerButton::Primary) {
            response.click = hover;
        }
        let delta = egui_response.drag_delta();
        if delta.length_sq() > 0.0 {
            if egui_response.dragged_by(egui::PointerButton::Primary) {
                response.drag_delta = Some(delta);
            } else if egui_response.dragged_by(egui::PointerButton::Middle) {
                response.pan_delta = Some(delta);
            }
        }
        let scroll = ui.input(|i| i.smooth_scroll_delta);
        if scroll.y.abs() > 1e-6
            && let Some(anchor) = hover
        {
            response.zoom = Some(((-scroll.y as f64 * 0.001).exp(), anchor));
        }

        // Screen-reader label for the plan canvas (guide II.21.2).
        egui_response
            .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, summary));

        (response, rect)
    }
}

/// The point the draft should preview: the snapped one when the pointer is over
/// the canvas, the last known drag point otherwise.
fn resolved_or(fallback: Point2, resolution: Option<SnapResolution>) -> Point2 {
    resolution.map_or(fallback, |resolved| resolved.point)
}

fn paint_construction(
    painter: &egui::Painter,
    rect: egui::Rect,
    view: &View,
    guide: &ConstructionEntity,
) {
    let stroke = egui::Stroke::new(1.0, CONSTRUCTION_STROKE);
    let kind = guide.kind.clone();
    match kind {
        ConstructionKind::Line { start, end } => {
            painter.line_segment(
                [
                    world_to_screen(view, rect, start),
                    world_to_screen(view, rect, end),
                ],
                stroke,
            );
        }
        ConstructionKind::InfiniteLine { point, direction } => {
            let step = direction.normalized().scaled(GUIDE_EXTENT_MM);
            painter.line_segment(
                [
                    world_to_screen(view, rect, point - step),
                    world_to_screen(view, rect, point + step),
                ],
                stroke,
            );
        }
        ConstructionKind::Ray { origin, direction } => {
            let far = origin + direction.normalized().scaled(GUIDE_EXTENT_MM);
            painter.line_segment(
                [
                    world_to_screen(view, rect, origin),
                    world_to_screen(view, rect, far),
                ],
                stroke,
            );
        }
        ConstructionKind::Circle { .. } | ConstructionKind::Arc { .. } => {
            // Segments per shape: fine enough that a circle never reads as a
            // polygon at any sane zoom.
            let segments = match kind {
                ConstructionKind::Circle { .. } => 96,
                _ => 48,
            };
            if let Some(points) = kind.polyline(segments) {
                let screen: Vec<egui::Pos2> = points
                    .iter()
                    .map(|point| world_to_screen(view, rect, *point))
                    .collect();
                painter.add(egui::Shape::line(screen, stroke));
            }
        }
    }
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
    let pts: Vec<egui::Pos2> = wall
        .corners
        .iter()
        .map(|p| world_to_screen(view, rect, *p))
        .collect();
    let fill = if selected {
        theme::COBALT.linear_multiply(0.12)
    } else {
        theme::WALL_FILL
    };
    painter.add(egui::Shape::convex_polygon(
        pts.clone(),
        fill,
        egui::Stroke {
            width: 0.0,
            color: egui::Color32::TRANSPARENT,
        },
    ));
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
    let n_screen = egui::Vec2::new(
        (normal.x * half * scale) as f32,
        (normal.y * half * scale) as f32,
    );
    let start = world_to_screen(view, rect, opening.start);
    let end = world_to_screen(view, rect, opening.end);

    let quad = [
        start + n_screen,
        end + n_screen,
        end - n_screen,
        start - n_screen,
    ];
    painter.add(egui::Shape::convex_polygon(
        quad.to_vec(),
        theme::OPENING_GAP,
        egui::Stroke {
            width: 0.0,
            color: egui::Color32::TRANSPARENT,
        },
    ));

    for jamb in [start, end] {
        painter.line_segment(
            [jamb + n_screen, jamb - n_screen],
            egui::Stroke::new(0.9, theme::WALL_STROKE),
        );
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
            let inset = egui::Vec2::new(
                (normal.x * half * scale * 0.5) as f32,
                (normal.y * half * scale * 0.5) as f32,
            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use lintel_core::construction::ConstructionKind;
    use lintel_core::dimension::{Dimension, DimensionKind};
    use lintel_core::ids::{ConstructionId, DimensionId, LabelId, OpeningId, WallId};
    use lintel_core::label::PlanLabel;
    use lintel_core::level::Level;
    use lintel_core::opening::{Opening, OpeningKind};
    use lintel_core::snap::SnapSettings;
    use lintel_core::wall::Wall;

    fn level() -> Level {
        Level::new(lintel_core::ids::LevelId(1), "Level 0", 0.0)
    }

    fn wall(id: u64, start: (f64, f64), end: (f64, f64)) -> Wall {
        Wall::new(
            WallId(id),
            Point2::new(start.0, start.1),
            Point2::new(end.0, end.1),
            200.0,
            2800.0,
        )
    }

    fn settings() -> SnapSettings {
        SnapSettings {
            grid: 0.0,
            grid_enabled: false,
            ortho: false,
            anchor_tolerance: 150.0,
        }
    }

    fn all_modes() -> SnapModes {
        SnapModes {
            endpoint: true,
            midpoint: true,
            intersection: true,
            perpendicular: true,
            tangent: true,
        }
    }

    fn geometry_with_two_walls() -> PlanGeometry {
        let mut level = level();
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.walls.push(wall(2, (2000.0, 0.0), (2000.0, 4000.0)));
        PlanGeometry::of_level(&level)
    }

    #[test]
    fn an_endpoint_within_tolerance_wins() {
        let geometry = geometry_with_two_walls();
        let modes = SnapModes {
            midpoint: false,
            intersection: false,
            perpendicular: false,
            tangent: false,
            ..SnapModes::default()
        };
        let resolved = resolve_snap(
            Point2::new(4060.0, 40.0),
            &geometry,
            &modes,
            None,
            &settings(),
        );
        assert_eq!(resolved.kind, SnapModeKind::Endpoint);
        assert_eq!(resolved.point, Point2::new(4000.0, 0.0));
    }

    #[test]
    fn a_midpoint_is_offered_only_when_the_mode_is_on() {
        let geometry = geometry_with_two_walls();
        let off = SnapModes {
            midpoint: false,
            intersection: false,
            perpendicular: false,
            tangent: false,
            ..SnapModes::default()
        };
        // Use wall 2's midpoint at (2000, 2000) which is not an endpoint.
        let raw = Point2::new(2050.0, 2030.0);
        let resolved = resolve_snap(raw, &geometry, &off, None, &settings());
        assert_eq!(resolved.kind, SnapModeKind::Free);

        let modes = SnapModes {
            perpendicular: false,
            ..all_modes()
        };
        let resolved = resolve_snap(raw, &geometry, &modes, None, &settings());
        assert_eq!(resolved.kind, SnapModeKind::Midpoint);
        assert_eq!(resolved.point, Point2::new(2000.0, 2000.0));
    }

    #[test]
    fn a_wall_crossing_is_offered_as_an_intersection() {
        let geometry = geometry_with_two_walls();
        let modes = SnapModes {
            endpoint: false,
            midpoint: false,
            perpendicular: false,
            tangent: false,
            intersection: true,
        };
        let resolved = resolve_snap(
            Point2::new(1980.0, 60.0),
            &geometry,
            &modes,
            None,
            &settings(),
        );
        assert_eq!(resolved.kind, SnapModeKind::Intersection);
        assert_eq!(resolved.point, Point2::new(2000.0, 0.0));
    }

    #[test]
    fn an_endpoint_beats_a_midpoint_at_the_same_place() {
        let mut level = level();
        // A wall whose midpoint is exactly the other wall's end.
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level
            .walls
            .push(wall(2, (4000.0, -2000.0), (4000.0, 2000.0)));
        let geometry = PlanGeometry::of_level(&level);
        let resolved = resolve_snap(
            Point2::new(4000.0, 0.0),
            &geometry,
            &all_modes(),
            None,
            &settings(),
        );
        assert_eq!(resolved.kind, SnapModeKind::Endpoint);
    }

    #[test]
    fn perpendicular_measures_from_the_last_picked_point() {
        let mut level = level();
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        let geometry = PlanGeometry::of_level(&level);
        let modes = SnapModes {
            endpoint: false,
            midpoint: false,
            intersection: false,
            tangent: false,
            perpendicular: true,
        };
        let origin = Point2::new(1000.0, 800.0);
        // The pointer is far from the wall; the candidate is the foot of the
        // perpendicular from the previous point.
        let resolved = resolve_snap(
            Point2::new(1000.0, 120.0),
            &geometry,
            &modes,
            Some(origin),
            &settings(),
        );
        assert_eq!(resolved.kind, SnapModeKind::Perpendicular);
        assert_eq!(resolved.point, Point2::new(1000.0, 0.0));
    }

    #[test]
    fn a_construction_circle_offers_a_tangent_touch_point() {
        let kind = ConstructionKind::Circle {
            center: Point2::new(0.0, 0.0),
            radius: 1000.0,
        };
        assert!(kind.is_valid());
        let mut level = level();
        level
            .construction
            .push(ConstructionEntity::new(ConstructionId(9), kind));
        let geometry = PlanGeometry::of_level(&level);
        let resolved = resolve_snap(
            Point2::new(1070.0, 0.0),
            &geometry,
            &all_modes(),
            None,
            &settings(),
        );
        assert_eq!(resolved.kind, SnapModeKind::Tangent);
        assert_eq!(resolved.point, Point2::new(1000.0, 0.0));
    }

    #[test]
    fn a_hidden_guide_is_not_a_candidate() {
        let kind = ConstructionKind::Line {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(4000.0, 0.0),
        };
        let mut level = level();
        let mut guide = ConstructionEntity::new(ConstructionId(4), kind);
        guide.visible = false;
        level.construction.push(guide);
        let geometry = PlanGeometry::of_level(&level);
        let candidates = snap_candidates(
            &geometry,
            &all_modes(),
            Some(Point2::new(2000.0, 0.0)),
            None,
        );
        assert!(candidates.is_empty());
    }

    #[test]
    fn two_crossing_walls_have_one_intersection() {
        let candidates = snap_candidates(
            &geometry_with_two_walls(),
            &SnapModes {
                endpoint: false,
                midpoint: false,
                perpendicular: false,
                tangent: false,
                intersection: true,
            },
            None,
            None,
        );
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].point, Point2::new(2000.0, 0.0));
    }

    #[test]
    fn parallel_segments_never_cross() {
        let cross = segment_intersection(
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            Point2::new(0.0, 500.0),
            Point2::new(1000.0, 500.0),
        );
        assert_eq!(cross, None);
        let cross = segment_intersection(
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            Point2::new(500.0, -500.0),
            Point2::new(500.0, 500.0),
        );
        assert_eq!(cross, Some(Point2::new(500.0, 0.0)));
    }

    #[test]
    fn a_domain_window_picks_what_it_encloses() {
        let geometry = geometry_with_two_walls();
        let bounds = Bounds {
            min: Point2::new(-100.0, -100.0),
            max: Point2::new(4100.0, 100.0),
        };
        let mut selection = Selection::default();
        selection.pick_box(&geometry, bounds, false, false, false);
        assert!(selection.contains(EntityId::Wall(WallId(1))));
        // A window takes only what it fully encloses.
        assert!(!selection.contains(EntityId::Wall(WallId(2))));
    }

    #[test]
    fn a_crossing_box_takes_what_it_touches() {
        let geometry = geometry_with_two_walls();
        let bounds = Bounds {
            min: Point2::new(-100.0, -100.0),
            max: Point2::new(4100.0, 100.0),
        };
        let mut selection = Selection::default();
        selection.pick_box(&geometry, bounds, true, false, false);
        assert!(selection.contains(EntityId::Wall(WallId(1))));
        assert!(
            selection.contains(EntityId::Wall(WallId(2))),
            "a crossing keeps what it touches"
        );
    }

    #[test]
    fn shift_adds_and_ctrl_removes_from_the_set() {
        let geometry = geometry_with_two_walls();
        let bounds = Bounds {
            min: Point2::new(-100.0, -100.0),
            max: Point2::new(4100.0, 100.0),
        };
        let mut selection = Selection::default();
        selection.replace(EntityId::Wall(WallId(9)));
        selection.pick_box(&geometry, bounds, true, true, false);
        assert!(selection.contains(EntityId::Wall(WallId(9))), "kept");
        assert!(selection.contains(EntityId::Wall(WallId(1))), "added");

        selection.pick_box(&geometry, bounds, true, false, true);
        assert!(!selection.contains(EntityId::Wall(WallId(1))), "removed");
        assert!(selection.contains(EntityId::Wall(WallId(9))), "kept");
    }

    #[test]
    fn a_plain_box_replaces_what_was_selected() {
        let geometry = geometry_with_two_walls();
        // Bounds that include wall 1 center (2000, 0) but not wall 2 center (2000, 2000)
        let bounds = Bounds {
            min: Point2::new(0.0, -100.0),
            max: Point2::new(3000.0, 100.0),
        };
        let mut selection = Selection::default();
        selection.replace(EntityId::Wall(WallId(9)));
        selection.pick_box(&geometry, bounds, false, false, false);
        assert_eq!(selection.len(), 1);
        assert!(selection.contains(EntityId::Wall(WallId(1))));
    }

    #[test]
    fn select_all_takes_every_entity_on_the_level() {
        let mut level = level();
        level.walls.push(wall(1, (0.0, 0.0), (4000.0, 0.0)));
        level.walls.push(wall(2, (4000.0, 0.0), (4000.0, 4000.0)));
        level.openings.push(Opening::new(
            OpeningId(2),
            OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        level.dimensions.push(Dimension::new(
            DimensionId(3),
            DimensionKind::Aligned,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        level.labels.push(PlanLabel::new(
            LabelId(4),
            Point2::new(1000.0, 1000.0),
            "Hall",
        ));
        level.construction.push(ConstructionEntity::new(
            ConstructionId(5),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(100.0, 0.0),
            },
        ));
        let geometry = PlanGeometry::of_level(&level);

        let mut selection = Selection::default();
        selection.pick_all(&geometry);
        assert_eq!(
            selection.len(),
            6,
            "walls(2), opening, dimension, label, guide"
        );
        assert!(selection.contains(EntityId::Opening(OpeningId(2))));
        assert!(selection.contains(EntityId::Label(LabelId(4))));
    }

    #[test]
    fn a_selection_replaces_then_toggles() {
        let mut selection = Selection::default();
        assert!(selection.is_empty());

        selection.replace(EntityId::Wall(WallId(1)));
        assert_eq!(selection.len(), 1);
        assert_eq!(selection.primary(), Some(EntityId::Wall(WallId(1))));

        selection.replace(EntityId::Wall(WallId(2)));
        assert_eq!(selection.primary(), Some(EntityId::Wall(WallId(2))));
        assert_eq!(selection.len(), 1);

        selection.toggle(EntityId::Wall(WallId(1)));
        assert_eq!(selection.len(), 2);
        assert_eq!(
            selection.primary(),
            Some(EntityId::Wall(WallId(1))),
            "newest last"
        );

        selection.toggle(EntityId::Wall(WallId(1)));
        assert_eq!(selection.len(), 1);

        selection.set(vec![EntityId::Wall(WallId(3)), EntityId::Wall(WallId(4))]);
        assert_eq!(selection.len(), 2);
        assert!(selection.contains(EntityId::Wall(WallId(4))));

        selection.remove(EntityId::Wall(WallId(3)));
        assert_eq!(selection.len(), 1);

        selection.clear();
        assert!(selection.is_empty());
        assert_eq!(selection.primary(), None);
    }

    #[test]
    fn a_click_on_empty_canvas_either_clears_or_does_nothing() {
        let mut selection = Selection::default();
        selection.replace(EntityId::Wall(WallId(1)));
        selection.set_from_hit(None);
        assert!(selection.is_empty());

        selection.replace(EntityId::Wall(WallId(2)));
        selection.set_from_hit(Some(Hit::Dimension(DimensionId(9))));
        assert_eq!(
            selection.primary(),
            Some(EntityId::Dimension(DimensionId(9)))
        );
    }

    #[test]
    fn a_hit_names_the_entity_it_found() {
        assert_eq!(entity_of(Hit::Wall(WallId(1))), EntityId::Wall(WallId(1)));
        assert_eq!(
            entity_of(Hit::Opening(OpeningId(2))),
            EntityId::Opening(OpeningId(2))
        );
        assert_eq!(
            entity_of(Hit::Dimension(DimensionId(3))),
            EntityId::Dimension(DimensionId(3))
        );
        assert_eq!(
            entity_of(Hit::Label(LabelId(4))),
            EntityId::Label(LabelId(4))
        );
    }

    #[test]
    fn the_drag_direction_decides_the_selection_mode() {
        let from = Point2::new(0.0, 0.0);
        assert_eq!(
            SelectionModeKind::from_direction(from, Point2::new(100.0, 0.0)),
            SelectionModeKind::Window
        );
        assert_eq!(
            SelectionModeKind::from_direction(from, Point2::new(-100.0, 0.0)),
            SelectionModeKind::Crossing
        );
        assert_eq!(SelectionModeKind::Window.label(), "Window selection");
    }

    #[test]
    fn every_mode_has_a_distinct_marker_colour_and_label() {
        let mut labels = Vec::new();
        let mut colours = Vec::new();
        for kind in SnapModeKind::PRIORITY {
            labels.push(kind.label());
            colours.push(kind.colour());
        }
        let unique_labels = labels.len();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), unique_labels, "every kind is named");
        assert_ne!(colours[0], colours[1], "kinds are told apart by colour too");
    }

    #[test]
    fn priority_puts_an_endpoint_ahead_of_everything() {
        assert!(SnapModeKind::Endpoint.priority() < SnapModeKind::Midpoint.priority());
        assert!(SnapModeKind::Intersection.priority() < SnapModeKind::Perpendicular.priority());
        assert!(SnapModeKind::Tangent.priority() < SnapModeKind::Ortho.priority());
        assert!(SnapModeKind::Grid.priority() < SnapModeKind::Free.priority());
    }

    #[test]
    fn modes_toggle_one_at_a_time() {
        let mut modes = SnapModes::default();
        assert!(modes.endpoint);
        assert!(modes.intersection);
        assert!(!modes.tangent);

        modes.toggle(SnapModeKind::Tangent);
        assert!(modes.tangent);
        assert!(modes.endpoint, "the others are untouched");

        // Grid is not a switchable mode in SnapModes; toggling it is a no-op.
        modes.toggle(SnapModeKind::Grid);
        assert!(modes.tangent, "grid toggle does not affect other modes");
        assert!(modes.endpoint);
        assert_eq!(SnapModes::key_hint(SnapModeKind::Midpoint), "V");
    }
}
