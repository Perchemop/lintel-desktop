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

## Active Slice — v0.11.1 “Phase A continued: Snapping + Selection + Command Palette”

**Status:** **Complete** — published as 0.12.1 (PR #21 merged to `main`, CI green).

**Core**
- `crates/lintel-core/src/snap.rs` — extend snapping from endpoints to midpoint,
  intersection, perpendicular, tangent and quadrant candidates, with an explicit
  candidate-kind priority order.
- `crates/lintel-core/src/selection.rs` (new) — `select_by_window` for window,
  crossing and fence modes with add/remove sets, plus an entity-type and layer
  filter; pure and headless.
- `crates/lintel-core/src/lib.rs` — re-export the selection API.
- Foundation already on the branch (part of this slice): `layer.rs`,
  `construction.rs` and `apps/lintel/src/status_bar.rs`.

**UI**
- `apps/lintel/src/command_palette.rs` (new) — Ctrl+K palette: fuzzy match over
  the command registry, keyboard navigation and a recent-commands list.
- `apps/lintel/src/canvas.rs` — on-canvas snap indicators per candidate kind; the
  window/crossing/fence selection rectangle.
- `apps/lintel/src/main.rs` — wire Ctrl+K, the palette, the selection modes and
  the snap-indicator toggle.

**Tests**
- `crates/lintel-core/src/snap.rs` tests — candidate generation and priority
  ordering.
- `crates/lintel-core/src/selection.rs` tests — window vs crossing vs fence,
  add/remove sets, and entity-type/layer filters.

**Release gate**
- [x] Bump `apps/lintel/Cargo.toml` (and the core) to 0.11.1 (later 0.12.1)
- [x] CI green: formatting, clippy, build and tests
- [x] PR merged to `main`
- [x] Published as tag `v0.12.1` (no `v0.11.1` tag; see DECISIONS.md D-030)

---

## Next Slice — v0.12.0 “Phase B: Constraint-Driven Dimensions”

**Status:** Planned

**Core**
- `crates/lintel-core/src/dimension.rs` — add `DimensionAnchor` (point on wall +
  offset or perpendicular distance), witness-line gap/overrun, and serialization
  backward-compatibility.
- `crates/lintel-core/src/command.rs` — `SetDimensionAnchor`, `SetWitnessGap`,
  `SetWitnessOverrun` commands.
- `crates/lintel-core/src/constraint.rs` (new) — constraint solver for
  dimension-driven geometry (perpendicular distance, fixed angle, equal length).

**UI**
- `apps/lintel/src/dimension_ui.rs` — anchor placement UX, grip editing for
  witness lines, constraint badges on canvas.
- `apps/lintel/src/inspector.rs` — anchor property fields, constraint toggle.

**Tests**
- Dimension anchor round-trip serialization.
- Constraint solver: perpendicular distance holds under wall moves.
- [ ] `docs/tokens.json` updated by CI and quoted in the release notes

## Released — v0.11.0 “Measurements” (tag `v0.11.0`, commit `803b590`)

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
- [x] Bump `apps/lintel/Cargo.toml` 0.10.1 → 0.11.0 (commit `803b590`)
- [x] CI green and PR merged
- [x] Release artefacts + `SHA256SUMS.txt`
- [x] `docs/tokens.json` updated by CI
- [x] GitHub Release published for v0.11.0

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

### v0.12.0 / 0.12.1 “Phase A (Snapping + Selection + Command Palette)” — published as 0.12.1
**Status:** Planned — being re-scoped; the snapping, selection and
command-palette work below is now the v0.11.1 active slice, leaving the
constraint-dimension foundation here.

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
  The core layer model and construction entities landed as the Phase A
  foundation; the v0.11.1 active slice (CAP-094) builds the enhanced snapping on
  top of them. The layers/construction UI wiring is still outstanding.
- `DimensionAnchor` must preserve JSON compatibility: earlier files load with
  `DimensionAnchor::Free`; save-time upgrading is heuristic only.
- `crates/update-check` and `crates/crash-report` are now git submodules
  (DECISIONS.md D-029); CI checks out with `submodules: recursive`.
- Branch protection is documented in guide §II.25; repo currently has no
  protection. Follow PR flow pending protection being enabled.
