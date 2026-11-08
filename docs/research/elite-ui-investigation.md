# Elite UI Investigation — Lintel

**Author:** UI/UX subagent (rubric UI dev stage)
**Date:** 2026-11-08
**Repo:** `~/lintel` (Lintel v0.10.1, egui/eframe 0.36)
**Scope:** Investigation + design report only. No code changes.
**Companion docs:** `DOMAIN_INVESTIGATION_ELITE_CAD.md`, `DOMAIN_DESIGN_SKETCHES.md`, `ELITE_CAD_RESEARCH_SUMMARY.md` (domain side).

This report assesses 14 UI/UX feature areas that separate an elite paid
architectural drafting tool from a free/demo one, maps each to egui 0.36
patterns, estimates effort, and assigns it to the v0.11.1 / v0.12.0 slices.
It also gives an honest read of where the current shell feels free/demo.

---

## 0. Executive summary

Lintel's **domain layer** is genuinely strong: command-driven, headless-tested,
undo/redo, multi-storey, PDF export, snapping. The **shell** is the opposite:
a single `CentralPanel` containing a `MenuBar` plus one toolbar `horizontal`,
a floating `Window::new("Inspector")`, and a bottom-right version link. There
is no docking, no status bar, no context menu, no command line, no palette, no
onboarding. Every feature is reachable but the surface reads like a
"developer's test harness" rather than a product.

**The single highest-leverage change is not any one feature — it is
restructuring the shell into the standard CAD chrome layout** (top ribbon +
left tool palette + right docked inspector/layers/properties + bottom status
bar + optional command line). That is a UI-only refactor that makes every
subsequent feature land in the right place and is a prerequisite for most
items below.

**Recommended split:**

- **v0.11.1 (UI "chrome + workflow" slice):** shell restructure (panels),
  status bar, context menus, selection modes, grips on canvas, dynamic input
  during draw, onboarding tour + tooltips. All of these are pure UI and
  consume domain APIs that already exist or are landing in v0.11.1
  (selection filtering, enhanced snapping, construction geometry).
- **v0.12.0 (UI "productivity" slice):** command palette, ribbon tabs,
  properties palette live-edit, layers palette, command line, tool palettes
  (blocks/hatches), model/layout tabs, view cube + named views. These need
  domain support (layers, blocks, layouts) that is v0.12.0 scope.

---

## 1. Honest gap analysis — free/demo vs. paid

| Area | Current state | Why it reads "free/demo" |
|---|---|---|
| Window chrome | `MenuBar` with File/Edit/View/Help + one toolbar row | Menus are the only navigation; no ribbon, no quick-access toolbar |
| Tool selection | `ComboBox` labelled "Tool" in the toolbar | A combo box for the primary verb is a prototype affordance, not a tool |
| Inspector | Floating `Window::new("Inspector")` at `(4,40)` | Draggable floating window over the canvas; not docked; can be lost |
| Properties editing | `Apply` button per field; `opening_fields` returns a command only when a value differs | Not live; user must click Apply; multi-select impossible |
| Layers | None | No layer concept at all in UI or (yet) domain |
| Status bar | None | No coordinate readout, no snap/ortho/grid toggles as persistent chips, no zoom % in a bar |
| Command line | None | No type-to-command; only single-letter shortcuts |
| Context menus | None | Right-click does nothing |
| Tool palettes | None | No drag-drop blocks/hatches; no recent-commands strip |
| Model/layout tabs | Storeys live inside the Inspector's collapsible "Storeys" | No tab strip; switching storeys is buried |
| View controls | Scroll-wheel zoom, middle-drag pan, `F` fit | No zoom-window, no named views, no view cube/compass |
| Selection | Single `Option<Hit>`; click replaces | No window/crossing/fence, no add/remove, no select-all |
| Grips | None | Entities cannot be edited on-canvas except whole-wall drag |
| Dynamic input | Draft preview shows `"{length:.0} mm  {angle:.0}°"` text near cursor | Good start, but no editable on-canvas fields, no relative/absolute entry |
| Onboarding | Empty-state hint text on canvas only | No tour, no sample file, no shortcut overlay |
| Undo/redo | Menu items + Ctrl+Z/Y | No toolbar buttons with enabled state in a visible strip |
| Themes | Hard-coded warm paper theme | No dark mode, no theme switcher (acceptable for v0.1, but paid tools offer it) |

**Bottom line:** Lintel already has the *depth* of a paid tool in its model, but
the *surface* is a developer demo. The gap is almost entirely chrome and
interaction polish, which is exactly what this report addresses.

---

## 2. Cross-cutting shell architecture (prerequisite for all items)

Before any feature, restructure `LintelApp::ui` into docked panels using egui
0.36's Ui-first panels:

```rust
// Top: menu bar + ribbon tabs + quick-access toolbar
egui::TopBottomPanel::top("menubar").show(ui, |ui| { egui::MenuBar::new().ui(ui, …); });
egui::TopBottomPanel::top("ribbon").show(ui, |ui| { ribbon(ui, self); });
// Left: tool palette (vertical icon strip)
egui::SidePanel::left("tools").show(ui, |ui| { tool_palette(ui, self); });
// Right: docked inspector + layers + properties (tabbed)
egui::SidePanel::right("right_dock").show(ui, |ui| { right_dock(ui, self); });
// Bottom: status bar + optional command line
egui::TopBottomPanel::bottom("status").show(ui, |ui| { status_bar(ui, self); });
egui::TopBottomPanel::bottom("cmdline").show(ui, |ui| { command_line(ui, self); });
// Centre: canvas
egui::CentralPanel::default().show(ui, |ui| { canvas + overlays; });
```

Notes:
- egui 0.36 panels are shown in nesting order; each call borrows `&mut Ui`
  and the closure receives `&mut Ui`. This matches the existing App trait
  signature `fn ui(&mut self, ui, frame)` — no `RefCell` needed; keep state
  on `LintelApp` and pass `&mut self` fields into panel closures carefully
  (split borrows).
- Persist panel widths/visibility in a small `UiState` struct serialized to
  `~/.config/lintel/ui.json` (or beside the project). egui's `Memory` is
  session-only; a JSON file survives restarts. This is a domain-adjacent
  concern but lives in the UI crate (it is presentation state, not model
  state) — acceptable.
- Use `egui::containers::panel::Panel` resize handles; set
  `resizable(true)` on side panels.

**Effort:** M (1–2 days). **Fit:** v0.11.1 prerequisite. No domain change.

---

## 3. Feature-by-feature design

### 3.1 Command palette (Ctrl+K)

**What:** Searchable launcher over every command/action: tools, file ops, view
ops, inspector actions, recent commands, with fuzzy match and keyboard-only
navigation (↑↓ Enter Esc).

**egui patterns:**
- `egui::Window::new("Command Palette").fixed_pos(center).fixed_size(...)`
  or `egui::Area` with `Order::Foreground` + manual dimming rect.
- Single `TextEdit::singleline` with focus on open (`ui.memory_mut(|m| m.request_focus(id))`).
- Fuzzy matcher: implement a tiny `fuzzy_score(query, label) -> Option<i32>`
  (subsequence match + bonus for consecutive/word-boundary hits). No external
  crate needed; keep it in `src/command_palette.rs`.
- Results list in a `ScrollArea::vertical()`; highlight matches with
  `RichText` coloured spans. `ui.selectable_label` per row; track selected
  index; Enter runs, Esc closes.
- Recent commands: `Vec<CommandId>` (a new `enum CommandId` mirroring
  actions) capped at 10, persisted in `UiState`.

**Work:** M. **Fit:** v0.12.0 (needs the full action inventory settled; the
ribbon work in 3.2 defines the canonical action list). Could ship a minimal
version in v0.11.1 (tools + file + view only) if desired.

**Domain gaps:** None — purely UI over existing commands. Needs a stable
`CommandId`/action registry; recommend domain-dev adds a `Command::label()`
and `Command::shortcut()` helper so the palette and status bar share one
source of truth. (Currently labels live ad-hoc in main.rs.)

### 3.2 Ribbon / toolbar organization

**What:** Contextual tabs (Draw / Modify / Annotate / View / Output) with
large/small button modes and a quick-access toolbar (QAT) of pinned actions.

**egui patterns:**
- Top panel with `egui::MenuBar` (existing) + a tab strip:
  `ui.horizontal(|ui| { for tab in tabs { ui.selectable_value(&mut ribbon_tab, tab, tab.label()); } })`.
- Per-tab content: `match ribbon_tab { Draw => draw_ribbon(ui), … }` rendering
  icon+label buttons in `ui.horizontal_wrapped` or a `Grid`.
- Button sizes: a `RibbonSize { Large, Small }` toggle stored in `UiState`;
  large = icon over label (two-line), small = icon + tooltip. Use
  `egui::Button::new(RichText)` with `ui.add_sized`.
- QAT: a row of small icon buttons above the ribbon, persisted pinned list.
- Icons: egui has no built-in icon font. Options: (a) text glyphs from a
  bundled icon font (e.g. embed a subset of Lucide as `.ttf` via
  `ctx.set_fonts`), (b) small `include_bytes!` PNGs as `egui::TextureHandle`,
  (c) text labels only. Recommend (a) for crisp scaling; needs an asset.
  Current repo has `resources/icons/lintel.svg` only.

**Work:** L. **Fit:** v0.12.0 (icon asset pipeline + tab definitions). The
existing single toolbar row can be promoted incrementally.

**Domain gaps:** None.

### 3.3 Properties palette (modeless, live-edit)

**What:** Right-docked panel showing editable properties of the selected
entity; live (no Apply), multi-select common properties, drag-to-change.

**egui patterns:**
- Replace the floating `Window::new("Inspector")` "Element" section with a
  docked `SidePanel`/tab. Keep the inspector's other sections (Project, View,
  Storeys, Rooms, Schedule, Measurements) as separate tabs or a scrollable
  stack.
- Live editing: bind `DragValue`/`TextEdit` directly to a *local* copy, and
  on `.changed()` emit the corresponding `Command` immediately (apply each
  frame the value changes). The current `wall_fields` already mutates locals
  and only commits on Apply — change to commit on change. Concern: undo
  granularity. Recommendation: domain-dev adds a `Command` variant or a
  "commit on drag-end" hook; UI tracks `dragging: bool` (egui's
  `response.dragged()`) and pushes undo snapshot only when drag ends. The
  rule "push undo snapshots only on committed actions" is satisfied if we
  apply commands during drag but snapshot only at drag-end — **this requires
  domain support** (a `Document::apply_without_snapshot` or a
  `begin_drag`/`commit_drag` pair). Flag as domain gap.
- Multi-select: change `selection: Option<Hit>` → `Selection: Vec<Hit>` (or
  a `HashSet<Hit>`). Common properties across selected entities of the same
  kind are shown; mixed selections show only universal props (layer, colour).
  Needs domain support for multi-entity commands (or UI loops single-entity
  commands — acceptable for v0.11.1).
- Drag-to-change values: `DragValue` with `.speed()` already supports drag;
  add `.suffix()` and range. For on-canvas drag of a numeric property, use
  `response.dragged()` delta.

**Work:** M (UI) + S domain (drag-commit hook, multi-select). **Fit:**
v0.12.0 for full; v0.11.1 can ship live single-select editing on top of
existing commands (drop Apply buttons) — low risk, high perceived quality.

**Domain gaps:** (a) drag-end undo snapshot hook; (b) multi-select model
(`Selection` collection) and multi-entity commands; (c) `Command::label()`.

### 3.4 Layers palette

**What:** Layer list with colour/linetype/visibility toggles, current-layer
indicator, layer states (freeze/thaw, lock/unlock).

**egui patterns:**
- Docked right panel tab "Layers": `ScrollArea` of rows. Each row:
  visibility eye icon button, lock icon button, colour swatch (`ui.color_edit_button_srgb`),
  linetype combo, name `TextEdit`, and a "current" indicator (bold + accent
  bar) when `layer.id == active_layer`.
- New layer / delete layer / rename buttons in a footer row.
- Layer states: freeze (not rendered, not selectable), lock (rendered, not
  editable), hidden (not rendered). Store as bitflags on `Layer`.

**Work:** M (UI) + M (domain: `Layer`, `Linetype`, entity `layer_id`,
migration). **Fit:** v0.12.0 (domain lists layer system for v0.12.0). Minimal
version (visibility toggle per entity group) could ship v0.11.1 but the
domain investigation explicitly schedules full layers for v0.12.0 — respect
that.

**Domain gaps:** entire layer subsystem (see `ELITE_CAD_RESEARCH_SUMMARY.md`
v0.12.0 scope).

### 3.5 Status bar

**What:** Coordinate readout (cursor world X,Y), snap/grid/ortho toggles as
chips, selection count, unit mode, zoom level.

**egui patterns:**
- `TopBottomPanel::bottom("status")` with a `ui.horizontal`:
  - left: `format!("X: {:.0}  Y: {:.0}", world.x, world.y)` (from
    `response.pointer_world`), unit suffix.
  - chips: `ui.toggle_value(&mut snap.grid_enabled, "Grid")` etc. with
    `.small()`.
  - middle: `format!("{} selected", selection.len())`.
  - right: `format!("{}%  {}", view.zoom_percent(), units.label())`.
- Cursor coordinate needs the canvas's last `pointer_world`; store on app
  state each frame (`self.last_cursor_world`).
- Keep the existing toolbar shortcut hint line or move it into the status
  bar's hover tooltips.

**Work:** S. **Fit:** v0.11.1. Pure UI, consumes existing state.

**Domain gaps:** None.

### 3.6 Command line (type-to-command)

**What:** Type "LINE"/"L" to invoke tools/commands; history (↑↓), autocomplete
dropdown, aliases.

**egui patterns:**
- Bottom panel (below status or replacing it) with a single `TextEdit` in
  "command mode". When focused, keystrokes accumulate; Enter commits.
- Autocomplete: filter a static `&[(&str, CommandId)]` alias table; render as
  a popup `Area` above the input listing matches; Tab/Enter accepts top.
- History: `Vec<String>` capped; ↑ recalls.
- Alias table includes tool letters (W/S/D/N/M/L/G/O/F) and full names
  ("WALL", "LINE", "DOOR", …). Also file ops ("NEW", "OPEN", "SAVE",
  "EXPORT"), view ops ("ZOOM", "PAN", "FIT").
- While the command line has focus, disable global single-letter shortcuts
  (extend the existing `keyboard_scope` check to also exclude when the
  command-line widget is focused).

**Work:** M. **Fit:** v0.12.0 (depends on the action registry from 3.1).
Could land a minimal version in v0.11.1 (tool switching only).

**Domain gaps:** None if `CommandId`/action registry exists (3.1).

### 3.7 Context menus

**What:** Right-click menus: different for selected entity vs empty canvas;
"repeat last command".

**egui patterns:**
- egui 0.36: `response.context_menu(|ui| { … })` on the canvas response,
  or `ui.ctx().is_pointer_button_down_on(…)` + manual `Area`. The simplest:
  `egui_response.context_menu(|ui| { … })` where `egui_response` is the
  canvas's allocated response. Provide menu items based on `self.selection`.
- Entity menu: Properties, Delete, Duplicate, Move to layer (v0.12), Bring
  to front, Zoom to selection.
- Canvas menu: Paste, Select all, Zoom extents, Grid/snap toggles, "Repeat
  last command" (stores `last_command: Option<CommandId>`).
- Use `ui.menu_button` for submenus (e.g. "Zoom" → Extents/Window/Previous).

**Work:** S–M. **Fit:** v0.11.1 (entity/canvas menus + repeat-last). Layer
items wait for v0.12.0.

**Domain gaps:** "Repeat last command" needs the app to remember the last
applied `Command` (or its `CommandId` + params). For parametric repeat
(e.g. repeat wall with same thickness), store the full `Command` clone —
`Command` must be `Clone`. Check: `Command` derives? Verify in
`crates/lintel-core/src/command.rs`. Likely `#[derive(Clone)]` already (used
in tests?). Flag to domain-dev to confirm/add `Clone` + `Debug`.

### 3.8 Tool palettes (blocks, recent, hatches)

**What:** Drag-and-drop blocks/symbols, recent commands strip, hatch patterns.

**egui patterns:**
- A floating/docked palette window with a `Grid` of icon buttons; drag source
  via `ui.dnd_drag_source` (egui 0.36 has built-in DnD: `ui.dnd_drag_source`
  / `ui.dnd_drop_zone`). Drop on canvas creates the entity.
- Recent commands: horizontal strip of last N tool buttons.
- Hatch patterns: preview swatches; apply as fill to a closed room/region
  (domain support needed).

**Work:** L (UI) + L (domain: blocks/hatches). **Fit:** v0.12.0. Blocks and
hatches are v0.12.0 domain scope per the elite CAD summary.

**Domain gaps:** block library + hatch pattern definitions.

### 3.9 Model / layout tabs

**What:** Tab strip to switch between model space and paper-space layouts
(sheets).

**egui patterns:**
- Top panel row of `selectable_label` tabs: "Model" + one tab per layout.
  Active tab styled with accent underline.
- Switching sets `self.space: Space { Model } | { Layout(LayoutId) }` and
  the canvas renders the appropriate geometry (model geometry vs. sheet
  viewport).
- Layouts need domain support: `Layout` struct with viewport (centre, scale,
  clip), title block, and a list of viewports. This is a significant domain
  addition not currently in the roadmap — recommend either (a) scope as
  v0.13.0, or (b) ship "storey tabs" now (model space per storey) which is
  already domain-supported and gives 80% of the value. Storey tabs are a
  cheap, high-value UI win on top of existing `SetActiveLevel`.

**Work:** S (storey tabs) / XL (full paper space). **Fit:** storey tabs in
v0.11.1; full layouts in v0.13.0+ (not currently scheduled — raise with
orchestrator).

**Domain gaps:** paper-space layout model (new domain module). Storey tabs
need none.

### 3.10 View controls

**What:** Pan/zoom/zoom-extents/zoom-window, view cube or compass, named
views.

**egui patterns:**
- Toolbar buttons calling existing `zoom_to_fit()` plus new helpers:
  `zoom_window` (rubber-band rect → compute scale/centre from bounds),
  `zoom_previous` (view stack), `zoom_in`/`zoom_out` (×1.5).
- View cube: a small `Area` in the top-right of the canvas drawing an
  isometric-ish cube (2D drafting → compass rose is more appropriate: N
  arrow). Use the painter; clicking a face sets view centre rotation (2D has
  no rotation, so the compass is a north indicator + reset-orientation).
- Named views: a combo box + save/delete; persisted in `UiState` or project.
  Domain support: `NamedView { name, centre, px_per_mm }` stored in project
  (project-level, not per-level) — small domain addition.

**Work:** M (UI) + S (domain: named views). **Fit:** v0.11.1 for
zoom-window/extents/compass; named views v0.12.0.

**Domain gaps:** named views persistence (optional; can live in UiState for
v0.11.1).

### 3.11 Selection modes

**What:** Window (left-to-right) / crossing (right-to-left) / fence selection;
add/remove (Shift/Ctrl); select-all; deselect.

**egui patterns:**
- Canvas drag with no hit → rubber-band rectangle. Determine direction:
  left→right = window (fully enclosed), right→left = crossing (intersects).
  Color the rect blue (window) / green (crossing) per AutoCAD convention.
- Hit-test geometry against the rect in world space: `PlanGeometry` needs
  `entities_in_rect(rect, mode)` — domain function (selection filtering is
  already in the v0.11.1 domain scope per the elite CAD summary).
- Modifier keys: Shift = add, Ctrl = remove, plain = replace.
- Select all: Ctrl+A → select all entities on active level.
- Change `selection: Option<Hit>` → `Selection` collection (same gap as 3.3).

**Work:** M (UI) + S (domain: selection filtering functions already planned).
**Fit:** v0.11.1 (domain selection filtering is a v0.11.1 must-have).

**Domain gaps:** `Selection` collection type + multi-entity commands; the
filtering pure functions are already in the v0.11.1 plan.

### 3.12 Grips / handles

**What:** On-canvas grips for direct manipulation (move endpoints, resize
openings, offset dimensions).

**egui patterns:**
- Paint small squares (grips) at entity control points when selected, using
  the canvas painter (`painter.circle_filled` / `rect_filled`).
- Hit-test grips first (higher priority than entity hit-test) during pointer
  down; on drag, apply the relevant `Command` (e.g. drag a wall endpoint →
  `MoveWall` with one end fixed). Needs domain commands for endpoint moves
  (`SetWallEndpoints` or `MoveWallEndpoint`) — currently only `MoveWall`
  (whole wall). **Domain gap.**
- Grip types: move (square), stretch (hollow square), rotate (not needed in
  2D drafting with axis-aligned), scale. Keep to move + stretch for v0.11.1.

**Work:** M (UI) + S (domain: endpoint-move commands). **Fit:** v0.11.1 for
wall endpoints and opening offsets; dimension grips v0.12.0.

**Domain gaps:** `MoveWallEndpoint { id, which, new_point }` command (and
validation: no zero-length, no overlap). Currently only whole-wall move.

### 3.13 Dynamic input

**What:** On-canvas dimension entry while drawing: type length/angle, relative
or absolute coordinate entry.

**egui patterns:**
- During `draft.active`, render an editable `TextEdit` (or capture keystrokes)
  near the cursor showing the current length/angle; Tab cycles fields
  (length → angle), Enter commits, Esc cancels.
- egui immediate mode: place the text edit in an `Area` positioned at
  `end_screen + offset`; call `ui.memory_mut(|m| m.request_focus(id))` on
  first frame of draft.
- Parse input with `lintel_core::units::parse_length` (exists) and a small
  angle parser.
- Relative entry: prefix `@` (e.g. `@4000<90`) sets delta from last point;
  absolute: `4000,3000`. This is standard CAD command-line syntax.

**Work:** M. **Fit:** v0.11.1 (high perceived quality, low domain cost —
reuses `snap_point` + `AddWall`). Needs a small parser in UI (or domain
helper `parse_coordinate_input`).

**Domain gaps:** optional `parse_coordinate_input` helper in `units.rs`/
`geom.rs` for `@dx,dy` and `length<angle` syntax. Low priority; can live in
UI.

### 3.14 Onboarding

**What:** First-run tour, sample drawings, tooltips with shortcut hints.

**egui patterns:**
- First-run detection: `UiState::first_run` flag; on true, show a modal
  `Window` tour with Next/Back/Skip and highlight targets (dim canvas + arrow).
  egui has no built-in spotlight; implement by painting a semi-transparent
  rect over everything except a target rect, and an `Area` with the tooltip.
- Sample drawings: ship 1–2 `.lintel.json` files in `resources/samples/`;
  "Open sample" in empty state and File menu loads them via `Document::open`.
- Tooltips: already present via `.on_hover_text`; ensure every control has
  one including shortcut (already a v0.4.0 capability). Audit for gaps.
- Keyboard shortcut overlay: `?` or F1 opens a cheat-sheet window listing
  all shortcuts grouped by category.

**Work:** M. **Fit:** v0.11.1 (tour + sample + cheat sheet). High impact on
first impression.

**Domain gaps:** None (samples are data files).

---

## 4. Cross-cutting concerns

### 4.1 State management

- `LintelApp` already holds all state. Keep it that way; add `UiState`
  (panels, ribbon tab, theme, recent commands, first-run) as a field.
- Multi-select: change `selection: Option<Hit>` → `selection: Selection`
  (`Vec<Hit>` or `HashSet<Hit>`). This touches `main.rs`, `canvas.rs`,
  `inspector.rs`, `dimension_ui.rs`. Coordinate with domain-dev so the
  `Selection` type and helpers (`select_all`, `clear`, `toggle`) live in
  `lintel-core::selection` (domain owns logic). **This is the biggest
  single UI refactor and should be its own slice.**

### 4.2 Theming / dark mode

- Add a theme toggle in View menu or status bar; store in `UiState`.
- egui: `ctx.set_visuals(Visuals::dark())` / `Visuals::light()`; but Lintel
  has a custom warm palette. Create `theme::apply(ctx, mode)` that sets
  `ctx.set_visuals` plus overrides `style.spacing`, `widgets` colours.
- Recommend deferring to v0.12.0 (not in the 14 items' core ask) but note
  it as a "paid feel" multiplier.

### 4.3 Persistence of UI state

- `UiState` serialized to JSON in the user config dir
  (`directories` crate or `std::env::var("HOME")`/`APPDATA`). Currently no
  config-dir dependency. Could reuse the same path logic as crash-report.
  Low priority; can store beside the project file for v0.11.1.

### 4.4 Accessibility

- All new panels/buttons must carry `.on_hover_text` with shortcut hints
  (v0.4.0 capability, keep the standard).
- Floating palettes must be keyboard-dismissable (Esc) and not trap focus.
- Canvas overlays (tour spotlight) must not break screen-reader labels.

### 4.5 Performance

- Command palette fuzzy match over ~80 actions is trivial.
- Selection rectangle hit-testing: O(n) per frame during drag is fine for
  <500 entities; domain-dev's pre-computed spatial index (if added) helps.
- Grip rendering: only for selected entities; cap grip count.

---

## 5. Domain gaps to raise with domain-dev

Consolidated list (so the orchestrator can hand to domain-dev):

1. **`Command: Clone + Debug`** — needed for "repeat last command" and
   command palette recent list. Verify/derive in `command.rs`.
2. **`Command::label()` and `Command::shortcut()`** — single source of truth
   for palette, status bar, tooltips. Currently labels are ad-hoc strings in
   `main.rs`.
3. **Drag-end undo snapshot hook** — `Document::apply` snapshots every call.
   For live drag editing we need either `apply_transient` (no snapshot) +
   `commit_drag` (snapshot), or a `begin_drag`/`end_drag` pair. Without this,
   live property editing will flood the undo stack.
4. **Multi-select model** — `Selection` collection type in `lintel-core`,
   plus multi-entity command variants or a documented "UI loops single-entity
   commands" contract. Domain should own selection logic (filtering,
   select-all, crossing/window) per the thick-core rule.
5. **Grip endpoint-move commands** — `MoveWallEndpoint { id, end, point }`,
   `MoveOpeningOffset` already exists. Plus validation (no zero-length wall).
6. **Named views** — `NamedView { name, centre, px_per_mm }` stored at
   project level; commands `AddNamedView`/`DeleteNamedView`/`ApplyNamedView`.
7. **Paper-space layouts** — only if we commit to model/layout tabs fully;
   otherwise ship storey tabs (no domain change). Recommend raising as a
   v0.13.0 decision.
8. **Layers subsystem** — already in v0.12.0 domain scope; UI depends on it.
9. **Blocks / hatches** — already in v0.12.0 domain scope; UI depends on it.
10. **`parse_coordinate_input` helper** (optional) for `@dx,dy` and
    `length<angle` dynamic input syntax.

---

## 6. Suggested slice assignment

### v0.11.1 — "Chrome + workflow" (UI-heavy, low domain cost)

- Shell restructure into docked panels (§2).
- Status bar (3.5).
- Context menus: entity + canvas + repeat-last (3.7) — needs `Command: Clone`.
- Selection modes: window/crossing/fence, add/remove, select-all (3.11) —
  needs domain selection filtering (already planned) + multi-select model.
- Grips: wall endpoints + opening offsets (3.12) — needs endpoint-move
  commands.
- Dynamic input during draw (3.13).
- Onboarding: tour + sample drawings + shortcut cheat sheet (3.14).
- Storey tabs (3.9 minimal).
- View controls: zoom-window, extents, compass (3.10 partial).
- Live single-select property editing without Apply (3.3 partial) — needs
  drag-end snapshot hook or accept undo flooding (not recommended).

**Domain dependencies for v0.11.1:** items 1, 3, 5, and the already-planned
selection filtering. Raise these first.

### v0.12.0 — "Productivity" (UI + domain)

- Command palette (3.1).
- Ribbon tabs + QAT + icon pipeline (3.2).
- Full properties palette: live, multi-select (3.3).
- Layers palette (3.4) — needs layers domain.
- Command line (3.6).
- Tool palettes: blocks, recent, hatches (3.8) — needs blocks/hatches domain.
- Named views (3.10).
- Full model/layout tabs (3.9) — only if paper-space domain lands; otherwise
  keep storey tabs.

---

## 7. Effort legend

- **S** = 1–3 days, one developer, no domain change.
- **M** = 3–7 days, may need a small domain addition.
- **L** = 1–3 weeks, needs domain support and tests.
- **XL** = 3+ weeks or a new domain subsystem.

## 8. Risks

- **Multi-select refactor blast radius:** changing `Option<Hit>` to a
  collection touches canvas, inspector, main, dimension_ui. Do it once, in a
  dedicated slice, with tests. Do not interleave with other features.
- **Undo flooding from live editing:** without the drag-end hook, live
  `DragValue` editing will push one snapshot per frame. Must resolve domain
  item 3 before shipping live editing.
- **Icon asset pipeline:** ribbon needs icons. Decision needed: bundle an icon
  font (recommended) vs. PNGs. PNGs are simpler and match the existing
  `resources/` pattern; icon font scales better. Raise as a question if the
  orchestrator has a preference.
- **egui_dock:** a third-party docking crate exists (`egui_dock`) but adds a
  dependency. The cross-cutting shell (§2) uses built-in panels only, which is
  enough for the proposed layout. If tabbed docking inside the right panel is
  wanted, evaluate `egui_dock` in v0.12.0.

---

## 9. What I did not change

No code, no Cargo.toml, no tests. This is a design report only, per the
investigation mandate. The next step is for the orchestrator to (a) confirm
the v0.11.1/v0.12.0 split, (b) hand the domain gaps in §5 to domain-dev, and
(c) decide the icon-pipeline question in §8.
