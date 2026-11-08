# Implementation Plan — Lintel

This document tracks the vertical-slice implementation order used for each release. It is the working bridge between `ROADMAP.md` (what we ship) and `REQUIREMENTS_TRACEABILITY.md` (why we ship it).

## Approach
Lintel follows the guide §K.3 thick-core / thin-shell layering:
- `crates/lintel-core` owns domain, commands, persistence, PDF export, and headless tests.
- `apps/lintel` owns egui/eframe UI, input handling, and presentation.

Each release is a vertical slice:
1. Domain types + commands in core with headless tests.
2. PDF export if the slice affects the drawing set.
3. UI wiring: canvas, inspector, menus, keyboard shortcuts.
4. CI green → PR → merge → release.

## Active Slice — v0.11.0 “Measurements”

**Status:** In Progress — feature code is on `main`; release pending.

**Core**
- `crates/lintel-core/src/measurements.rs` — `Measurements`, `LevelTotals`,
  `ProjectTotals`: room area/perimeter, level wall/opening length totals, and
  project-wide aggregation.
- `crates/lintel-core/src/units.rs` — `format_area` honours `UnitSystem`
  (m² / ft² / in²).
- `crates/lintel-core/src/lib.rs` — re-exports `Measurements`, `LevelTotals`,
  `ProjectTotals`, `format_area`.

**Tests**
- `measurements::tests` — room area/perimeter, wall and opening totals, level
  and project aggregation, missing level, out-of-range room index, JSON
  round-trip.

**UI**
- `apps/lintel/src/main.rs` — `Tool::Measurements`, Shift+M, Inspector flag.
- `apps/lintel/src/inspector.rs` — Measurements section: selected room, level
  totals and per-room list, project totals.

**Release gate**
- [ ] Bump `apps/lintel/Cargo.toml` 0.10.1 → 0.11.0 (this checkout still reads
      0.10.1)
- [ ] CI green and PR merged
- [ ] Signed release artefacts + `SHA256SUMS.txt`
- [ ] `docs/tokens.json` updated by CI and quoted in the release notes
- [ ] GitHub Release published for v0.11.0

## Completed Slices

### v0.10.1 Window Icon & Audit Fixes
**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.1
- Window icon embedded via `with_icon()` + `image` crate
- `include_bytes!` path corrected to repo root
- `resources/icons/lintel.svg` SVG source added
- Windows `.iss` version corrected
- Command-layer proptest fuzz tests added

### v0.10.0 Suite Standards Conformance
**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.0
- `docs/tokens.json` CI-managed ledger
- Release signing step added to `release.yml`
- `docs/development.md` added
- Property-based geometry and command-layer fuzz tests
- Version UI bottom-right, Release Notes popup with Update button

### v0.9.0 “Drawing Set”
**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.9.0
**Core**
- `crates/lintel-core/src/room.rs` — planar face traversal for auto-detected rooms.
- `crates/lintel-core/src/schedule.rs` — door/window schedule rows.
- `crates/lintel-core/src/label.rs` — free text plan labels.
- `crates/lintel-core/src/join.rs` — `PlanGeometry` now exposes `labels` and `rooms`.
- `crates/lintel-core/src/command.rs` — `PlaceLabel/SetLabelText/MoveLabel/DeleteLabel`.
- `crates/lintel-core/src/pdf.rs` — north arrow, scale bar, per-sheet schedule, room/label rendering, `render_set` multi-page PDF.

**UI**
- `apps/lintel/src/canvas.rs` — room area captions, label rendering, selection colour.
- `apps/lintel/src/main.rs` — `Tool::Label`, `L` shortcut, `Ctrl+D` duplicate, `Ctrl+Shift+E` Export drawing set, `plan_summary` room count.
- `apps/lintel/src/inspector.rs` — label editor, read-only Rooms and Schedule collapsing headers.

## Next Slices

### v0.11.1 “Phase A: Table Stakes (Layers + Construction Geometry)”
**Status:** Planned. Design source: `ELITE_CAD_RESEARCH_SUMMARY.md`,
`DOMAIN_INVESTIGATION_ELITE_CAD.md`, `DOMAIN_DESIGN_SKETCHES.md`,
`docs/research/elite-ui-investigation.md`.

**Core**
- `crates/lintel-core/src/layer.rs` — `Layer`, `LayerId`, `Linetype`; protected
  default layer.
- `crates/lintel-core/src/style.rs` — `EntityStyle` with `layer_id` and style
  overrides on every entity; `#[serde(default)]` migration and default-layer
  assignment on load.
- `crates/lintel-core/src/construction.rs` — `ConstructionEntity`,
  `ConstructionKind` (line, ray, circle); excluded from PDF, included as snap
  candidates.
- `crates/lintel-core/src/snap.rs` — `SnapCandidateKind` gains midpoint,
  intersection and perpendicular; `SnapSource` and the priority order are the
  foundation for v0.12.0 wiring.
- `crates/lintel-core/src/project.rs` — validation for layer references and
  construction entities.

**UI**
- Right dock: Layers palette (create, delete, visibility, lock, assign
  selection).
- Construction tools on the tool palette; construction geometry drawn in a
  distinct non-printing style.
- Persistent status bar: cursor coordinates, snap/ortho/grid chips, zoom %.
- Shell restructure into docked panels, the prerequisite identified in the UI
  investigation.

**Gate:** headless tests for layer migration, construction/PDF filtering and
snap candidates; CI green; signed release with `SHA256SUMS.txt` and
`docs/tokens.json` updated.

### v0.12.0 “Phase A continued (Snapping + Selection + Command Palette)”
**Status:** Planned.

**Core**
- Entity snapping generation for midpoint, intersection and perpendicular
  (pre-computed intersections, priority resolution).
- `crates/lintel-core/src/selection.rs` — `select_by_window` (window and
  crossing modes), pure and headless.
- Command registry metadata (name, shortcut, description) to feed a palette
  without duplicating the `Command` enum.
- `crates/lintel-core/src/dimension.rs` — `DimensionAnchor`
  (`EntityPoint` / `Free`), witness-line gap and overrun, backward-compatible
  load.

**UI**
- Snap markers and per-kind toggles.
- Window/crossing drag selection with add/remove.
- Command palette (Ctrl+K) with fuzzy search and keyboard navigation.
- Anchored dimension placement and witness-line rendering.

**Gate:** anchor-migration and selection property tests; CI green; signed
release with `SHA256SUMS.txt` and `docs/tokens.json` updated.

### v1.0.0 “Pavement”
- Polish, README screenshots, accessibility spot-checks, per-release legal
  checklist.

## Dependencies
- **Layers and construction geometry are prerequisites for enhanced snapping.**
  v0.11.1 must land the layer model (stable entity ownership/style) and the
  construction entities that snapping consumes before v0.12.0 wires midpoint,
  intersection and perpendicular snapping.
- `DimensionAnchor` must preserve JSON compatibility: earlier files load with
  `DimensionAnchor::Free`; save-time upgrading is heuristic only.
- `crates/update-check` and `crates/crash-report` are now git submodules
  (DECISIONS.md D-029); CI checks out with `submodules: recursive`.
- Branch protection is documented in guide §II.25; repo currently has no
  protection. Follow PR flow pending protection being enabled.
