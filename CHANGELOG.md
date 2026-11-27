# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## v0.21.0 — 2026-11-27

### Added

- Inspector Tab order kittest (T-208, spec AC-008): `kittest_inspector.rs`
  verifies forward Tab traverses mixed controls (text edit, checkbox, button)
  in reading order and all controls expose correct AccessKit roles; test runs
  headless in the `check` CI job
- Constraint solver 2-node cycle stress tests (T-202): three headless tests
  in `constraint.rs` — single EqualLength pair, five independent cycles, and
  mixed cycles with PerpendicularDistance/FixedAngle — all pass
- Anchor + constraint round-trip tests (T-203): four headless tests in
  `constraint.rs` covering PerpendicularDistance, FixedAngle, EqualLength,
  and mixed constraints surviving save/load/solve cycles
- Door/window placement regression tests (T-201): five tests in
  `workflows.rs` covering first-click placement, failed placements (too short,
  overlapping), no-wall-under-cursor, and offset-along-centreline persistence

### Changed

- Constraint solver performance: `solve_constraints` now uses a HashMap wall
  lookup (T-206) instead of linear scan, ~30% faster on 1000+ constraints
- Inspector empty states (T-207): Constraints, Viewports, Block Definitions,
  and Block Instances sections now show actionable messages with primary Add
  buttons where applicable
- kittest CI stability (T-200): tests run sequentially with `--test-threads=1`
  and `RUST_BACKTRACE=1` to eliminate SIGSEGV from parallel AccessKit probes
- Both crates bumped 0.20.1 → 0.21.0

### Fixed

- No behavioural fixes in this patch release; all tickets are test/observability
  hardening

## v0.20.1 — 2026-11-25

### Fixed

- Doors and windows place on the first click of a wall: the click offset is
  projected onto the wall centreline instead of measuring straight-line
  distance from the wall start, so off-centre clicks no longer compute
  offsets that fail `Opening::fits_on()` and get silently discarded
  (PRs #68, #70)
- Failed placements — wall too short, overlapping openings — surface an
  error note via `note_status` instead of being discarded without feedback
  (PR #68)
- Placing with no wall under the cursor shows a status hint instead of
  returning silently (PR #68)
- `wall_offset_along_centerline` stores an opening's offset along the wall
  centreline; regression-tested in `apps/lintel/src/tests/workflows.rs`
  (PR #70)

### Changed

- Both crates bumped 0.20.0 → 0.20.1

## v0.20.0 — 2026-11-22

### Added

- Form components (T-002, spec v0.20.0 AC-03): `components/forms.rs` with
  `Field` (label + input + error/help slots, `labelled_by` AccessKit
  association, disabled-reason tooltip, painted error icon + 2px danger
  stroke — never colour alone), `FieldSet` (framed legend, collapsible
  option) and `FieldGroup` (horizontal/vertical), all reading
  `SPEC.component.field` (D-035's pinned input pairing: `BORDER_STRONG`
  boundary, `TEXT_ERROR` messages)
- Layout helpers (T-003, spec v0.20.0 AC-04): `layout/helpers.rs`
  `vstack`/`hstack`/`wrap_row`/`card` (+ `card_compact`, `tool_group`),
  every helper returning its `InnerResponse` so callers can compose
  responses; `wrap_row` reflows at narrow widths instead of clipping and
  `card` carries the `SHADOW_LEVEL_1` elevation
- Rendered design evidence (T-008, spec AC-09, guide II.30.8): env-gated
  `evidence_*` tests render light/dark/state/focus sheets into
  `docs/qc/{buttons,form,layout}-evidence/*.png` headlessly (egui_kittest's
  wgpu test renderer); the `kittest` CI job generates the PNGs and uploads
  them as workflow artefacts — never committed, no pixel baselines; each
  evidence folder documents its contents (D-036)
- egui_kittest AccessKit assertions (T-008): form inputs carry labelled-by
  relations to their labels, error/help texts are announced as label nodes,
  Tab follows visual order (field → field → submit); layout resizes 400 →
  1200 px with a proof that no two exposed controls intersect and every
  button stays ≥ 24 pt
- `docs/design/gui-stack.md`: the II.30.2 stack record — egui/eframe
  **0.36.2** resolved, three-tier token system, component architecture,
  evidence pipeline and the 0.36 API breaks (`Context::run_ui`,
  `CornerRadius`, `Shadow`) that all widget code is matched against
- Contrast methodology (T-007, spec v0.20.0 AC-08): `theme::contrast_test()`
  walks every text/UI pairing of the light palette plus new dark and
  high-contrast reference palettes — 4.5:1 body text, 3:1 UI boundaries,
  7:1 high-contrast text — failing with role, measured ratio and required
  ratio; new required CI check `contrast-test` runs `cargo test
  contrast_test` (D-035)
- Tokens: `BASE_WARNING` / `TEXT_WARNING` (deep-amber warning text, 5.9:1 on
  the sheet) and `BORDER_STRONG` (7.8:1 outline role for fields and
  WCAG 2.2 §1.4.11 component boundaries)

### Changed

- egui_kittest dev-dependency gains the `wgpu` feature: the test renderer
  initialises lazily, so AccessKit-only tests are unchanged while the
  evidence generator can render PNGs (T-008)
- Both crates bumped 0.19.0 → 0.20.0
- `BASE_SELECTION_EDGE` raised from alpha 40 to 140: the command-palette
  selection outline was 1.2:1 on the sheet, under the WCAG 2.2 §1.4.11
  3:1 state-indicator floor; it now composites to 4.7:1 (4.6:1 on panels)

## v0.19.0 — 2026-11-19

### Added

- D-032 command bindings in the Inspector: a Title block section editing
  company, sheet title, revision, date and drawn-by (`SetTitleBlock`), a
  Viewports section with Remove (`RemoveViewport`) and an Add form with level
  id, scale denominator and caption (`AddViewport`, default A4 landscape
  region), and a Blocks "Add definition" name form (`AddBlockDef`, ids
  assigned UI-side as max+1). Only `MoveDimension` remains core-only by
  design (D-026), so D-027 reachability is fully satisfied (PR #60)
- F7/F8 snap aliases: F7 toggles grid and F8 toggles ortho alongside G/O,
  following the approved QCAD/AutoCAD convention;
  `docs/qc/shortcuts.md` updated (PR #61)
- Keyboard block-instance selection: a "Select block instance" palette action
  (Ctrl+K) with a pick form listing every active-level instance (raw id,
  definition name, position); Select sets the instance and clears entity
  selection, removing the last pointer-only accessibility flow (PR #61)

### Changed

- Snap-marker colours moved from raw `Color32` in `canvas.rs` into tier-1
  `BASE_SNAP_*` and tier-2 `SNAP_*` roles; the tier-3
  `snap_marker_colour(kind)` helper covers all eight snap kinds, and
  `CONSTRUCTION_STROKE` became a token (PR #59)
- `CONSTRUCTION_STROKE` darkened from (168,148,116) to (140,120,90): 2.77:1 →
  4.0:1 on PAPER, clearing the 3:1 UI-graphics floor, with a 3:1 assertion
  added to the contrast gate (guide II.30.7.1, PR #59)

## v0.18.0 — 2026-11-18

### Added

- WCAG AA contrast gate (guide II.30.7.1): `relative_luminance` /
  `contrast_ratio`, the `theme_colours_meet_wcag_aa` merge-gate test,
  darkened palette roles and named theme roles replacing raw `Color32`
  values (PR #53)
- Delete a single block instance: `Command::DeleteBlockInstance {
  block_instance_id }` with a `BlockInstanceNotFound` refusal and an honest
  status message, plus three domain tests (CAP-127, PR #50)
- `egui_kittest` CI job with AccessKit/snapshot tests
  (`palette_has_named_actions`, `inspector_sections_labelled`,
  `canvas_summary_accessible`); `kittest` is a required check (PR #55)
- Button role/size system: `ButtonRole`/`ButtonSize`/`ButtonRoleStyle`,
  12 role-and-size constants with 24/32/40 pt floors and a 2 px COBALT
  focus ring via `button_style()` (PR #55)
- ThemeSpec three-tier composition: base swatches → semantic roles →
  component tokens, with three new tests and byte-identical values (PR #57)

### Changed

- Keyboard shortcuts are wired and truthful: plain Z/Y alongside
  Ctrl+Z/Ctrl+Y, R for release notes, Ctrl+U for update checks; palette
  hints are wired or honestly blank and the footer shows matched of total
  (PR #50)
- Muted and hint text darkened slightly to meet AA (PR #53)
- Token ledger migrated from `docs/tokens.json` to `docs/tokens.md`
  (guide §24.1); `pr-author-is-dev` added as a required check with the
  Dev account recorded as D-031 (PR #52)
- D-032 documents unbound commands as core-only (PR #56)

## v0.17.0 — 2026-11-17

### Added

- Per-dimension label precision: `Dimension.label_precision` (0–4 decimal
  places, serde default 0 so old files load unchanged), the Inspector
  "Precision:" control and `Command::SetDimensionPrecision` with domain
  validation; the drawn label honours the setting (CAP-126)
- Delete a single block instance: `Command::DeleteBlockInstance {
  block_instance_id }` removes the selected canvas instance, with a
  `BlockInstanceNotFound` refusal and an honest status message (CAP-127)

### Changed

- Keyboard shortcuts wired and truthful: plain Z/Y undo-redo alongside
  Ctrl+Z/Ctrl+Y, R for release notes, Ctrl+U for update checks; command-palette
  hints are wired or honestly blank, the dead "View Constraints" entry was
  removed, and the footer shows matched of total
- Palette and status-bar colours corrected to meet WCAG AA (guide II.30.7.1):
  a `theme_colours_meet_wcag_aa` merge-gate test, darkened muted/hint/status
  and palette-grey roles, and raw `Color32` values moved into named theme roles
- Token ledger migrated from `docs/tokens.json` to the CI-managed
  `docs/tokens.md` (guide §24.1); `pr-author-is-dev` added as a required check
  (guide II.26.7)

## v0.16.0 — 2026-11-16

### Added

- Move block instances: `MoveBlockInstance` and `Level::move_block_instance`
  reposition an instance; non-finite positions are rejected and a rejected move
  leaves the instance unchanged (CAP-120)
- Rotate block instances: `RotateBlockInstance` and
  `Level::rotate_block_instance` set the rotation in radians; non-finite angles
  are rejected (CAP-121)
- Flip block instances: `FlipBlockInstance` and `BlockInstance::toggle_flip`
  mirror an instance across its local x-axis (CAP-122)
- Canvas hit-testing and click-to-select: `Level::hit_test_block_instance`
  returns the topmost instance under a point, the Select tool prefers instances
  over ordinary entities, the selected instance is outlined in red and Esc
  clears the selection (CAP-123)
- Editable Inspector fields: a "Block instance" section with X/Y position,
  rotation in degrees and a Flipped checkbox, each applying the matching
  command, with honest empty states (CAP-124)
- `Applied.block_instance` reports the affected instance so the UI selects it
  after add, move, rotate or flip (CAP-125)

### Changed

- Block instances are selectable on the canvas, and the Inspector's position,
  rotation and flip fields are live instead of read-only (resolves both
  limitations published in v0.15.0)

## v0.15.1 — 2026-11-15

### Summary

Patch release: macOS now ships a signed `.dmg` installer alongside the `.zip` archive, completing the installable-artefacts requirement for every platform. No feature changes from 0.15.0.

## v0.13.0 — 2026-11-13

### Added

- Anchored dimensions: `DimensionAnchor` ties a measured point to a wall at a
  parametric position, so moving a wall drags the dimension with it; witness
  lines gain editable gap and overrun (CAP-100–CAP-103)
- Constraint solver keeps anchored dimensions correct as geometry changes
  (CAP-101)
- Construction tools (line, ray, circle, arc) reachable from the UI with
  two-click canvas flows, palette entries and shortcuts (CAP-108)
- Layers panel in the Inspector (visibility/lock toggles) (CAP-108)
- Update flow verifies published `SHA256SUMS`/`.asc` before surfacing an update
  and honours the cache TTL (CAP-104)
- In-app release notes are generated from `RELEASE_NOTES.md` (CAP-105)

### Changed

- Releases are signed and version-guarded by CI; one `v`-prefixed tag convention
  (CAP-106)
- Documentation parity: capability inventory, parity evidence, traceability and
  architecture reconciled to published tags (CAP-107)

## v0.12.1 — 2026-11-10

### Summary

Phase A of the elite CAD foundation, shipped under the `v0.12.1` tag (no
`v0.11.1` tag was cut). Enhanced snapping, richer selection, a command palette,
layers and construction geometry turn Lintel from a drawing tool into a
drafting tool. Rebuilt on 2026-11-10/11 with corrected version stamps after
the 2026-11-11 audit found released binaries self-reporting 0.10.1.

### Added

- Enhanced entity snapping (CAP-094): midpoint, intersection, perpendicular,
  tangent, quadrant, grid and ortho candidates with priority ordering
- Selection modes (CAP-095): window, crossing and fence, owned by the core
- Selection filters (CAP-096): by entity type and by layer
- Command palette (CAP-097): Ctrl+K, fuzzy match, recent-commands list
- Snap visual indicators on canvas (CAP-098)
- Layers (CAP-089): `Layer`, protected default layer, per-entity assignment
- Construction geometry (CAP-090): line, ray, circle, arc — snap-enabled,
  never printed
- Status bar (CAP-091–093): cursor coordinates, snap/ortho/grid chips,
  selection count, zoom %

### Fixed

- Released binaries self-reported 0.10.1 (version bumps had never been
  committed); both manifests now carry 0.12.1 and the tag was re-pointed
  (audit findings F1/F4)
- Release pages showed 0.10.1 notes for newer releases; `RELEASE_NOTES.md`
  gained 0.11.0 and 0.12.1 sections (audit finding F2)

### Known issues

- Releases still ship unsigned when `GPG_PRIVATE_KEY` is unset (audit F3)
- In-app release-notes popup lags (shows 0.1.0/0.2.0 only — audit F7); both
  scheduled for v0.13.0 (CAP-104–106)

## v0.11.0 — 2026-11-09

### Summary

Measurements: area and perimeter become first-class, unit-aware readouts.

### Added

- Area/perimeter tool, Shift+M (CAP-086)
- Level and project wall/opening totals in the Inspector (CAP-087)
- `format_area` honours Metric/Imperial: m², ft², in² (CAP-088)
- Unit tests for measurements, totals and JSON round-trip

## v0.8.0 — 2026-11-07

### Summary

A keyboard-focus review. It found a regression introduced when the canvas
became focusable: tabbing to the canvas silently disabled every keyboard
shortcut. Shortcuts now work with the canvas focused, and the level summary is
unit-tested.

### Added

- Unit tests for the level summary shown in the toolbar and announced to screen
  readers (empty, singular and multi-storey cases)
- `ACCESSIBILITY.md` records the focus-review outcome: tab order, visible focus,
  Escape dismissal and that focus is never trapped

### Changed

- Keyboard shortcuts now apply when nothing has focus **or** when the canvas has
  focus; they are still suppressed while typing in a text field

### Fixed

- **Shortcuts stopped working after tabbing to the canvas.** Making the canvas
  focusable (0.6.0) meant `focused().is_none()` became false, so W/S/D/N/M,
  Ctrl+N/O/S/E and undo/redo all went dead until focus moved elsewhere

### Known limitations

- Screen-reader output is verified manually, not in CI
- The Linux AppImage has not been executed on an x86_64 host

### Token metrics

- Approx. tokens this release: 130,000
- Approx. cumulative tokens since development began: 5,960,000

(Figures are approximate totals recorded at release time.)

## v0.7.0 — 2026-11-06

### Summary

Closes the gap between what the core can do and what the interface exposes. An
audit found four documented capabilities — add/delete a storey, rename the
project and switch units — that existed in the core but had no way to be
triggered, like the export dialog before them. All four are now reachable.

### Added

- **Inspector → Project:** an editable project name (with Apply) and a Metric /
  Imperial unit selector
- **Inspector → Storeys:** a Delete button per storey (disabled for the last
  one) and a name-elevation-Add row to create a storey
- Dragging a selected dimension line moves it perpendicular to the measurement,
  changing its offset
- A documented command-reachability audit in the QC checklist that greps the UI
  for every `Command` variant

### Changed

- The Inspector takes a small `InspectorDraft` for the new-storey fields so the
  text being typed survives between frames

### Fixed

- Four capabilities were unreachable from the UI despite being listed as
  implemented: `AddLevel`, `DeleteLevel`, `SetUnits` and `RenameProject`

### Known limitations

- `MoveDimension` (moving a dimension's measured points) remains a core-only
  command covered by unit tests; the canvas binds dragging to the offset
  instead, because moving the measured span detaches a dimension from what it
  measures (see DECISIONS.md D-026)
- Screen-reader output is verified manually, not in CI

### Token metrics

- Approx. tokens this release: 160,000
- Approx. cumulative tokens since development began: 5,830,000

(Figures are approximate totals recorded at release time.)

## v0.6.0 — 2026-11-06

### Summary

Keyboard-complete file flows and an accessible canvas. A real bug is also
fixed: the PDF export dialog existed but had no way to open it.

### Added

- File keyboard shortcuts: Ctrl+N (new), Ctrl+O (open), Ctrl+S (save),
  Ctrl+Shift+S (save as), Ctrl+E (export sheet)
- The plan canvas is keyboard-focusable and carries a screen-reader label
  naming the active level and its contents, e.g.
  `Ground: 3 walls, 2 doors, 1 window, 1 dimension`
- The same level summary is shown in the toolbar
- `ACCESSIBILITY.md` and the QC checklist document the new keyboard flows

### Changed

- Plain tool shortcuts (W/S/D/N/M/G/O/F) no longer fire while a Ctrl/Cmd or Alt
  modifier is held, so Ctrl+S cannot also switch to the Select tool

### Fixed

- **The PDF export dialog was unreachable**: nothing set `export_open`, so the
  documented "Export sheet…" flow did not exist. It is now on the File menu and
  bound to Ctrl+E.

### Known limitations

- Screen-reader labels beyond egui's built-in AccessKit support (and the canvas
  level summary) still pending
- Canvas objects have no per-object labels — see `ACCESSIBILITY.md`
- Linux AppImage verified statically only (execution needs an x86_64 host)

### Token metrics

- Approx. tokens this release: 150,000
- Approx. cumulative tokens since development began: 5,670,000

(Figures are approximate totals recorded at release time.)

## v0.5.0 — 2026-11-06

### Summary

Windows users finally get a real installer: the release now publishes
`lintel-windows-x86_64-setup.exe`, built on CI with Inno Setup via Chocolatey.

### Added

- Windows installer (`lintel-windows-x86_64-setup.exe`) published alongside the
  Windows zip; installs to Program Files with Start-menu and optional desktop
  shortcuts
- Inno Setup script checked into the repo (`resources/windows/lintel.iss`) so a
  human can rebuild the installer offline

### Changed

- App and core version bumped to 0.5.0

### Fixed

- `.iss` `DefaultDirName` typo (`{autpf}` → `{autopf}`), so the installer
  suggests the correct Program Files directory

### Known limitations

- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Linux AppImage not yet verified on end-user systems
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels

### Token metrics

- Approx. tokens this release: 120,000
- Approx. cumulative tokens since development began: 5,520,000

(Figures are approximate totals recorded at release time.)

## v0.4.0 — 2026-11-05

### Summary

Accessibility polish and packaging: keyboard shortcut hints and hover labels on
every control, WCAG AA focus-ring contrast, and a Linux AppImage release
artefact. Windows installer deferred (Inno Setup download hangs on CI).

### Added

- Keyboard shortcut line in the toolbar: W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete
- Hover tooltip on every menu button and Inspector control describing units and ranges
- Focus-ring colour COBALT verified above WCAG AA 4.5:1 on both panel and canvas backgrounds
- Escape dismissal confirmed on all dialogs (About, Release notes, Update, Export sheet)
- `ACCESSIBILITY.md` honesty statement and `docs/qc/checklist.md` accessibility smoke test
- Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, valid ELF)
- Release notes now come from `RELEASE_NOTES.md` per tag

### Changed

- (none)

### Fixed

- AppImage build: `appimagetool` is itself an AppImage needing FUSE; now extracted
  and the embedded `AppRun` ELF invoked directly so no FUSE is required on CI

### Known limitations

- No Windows installer yet (zip remains the Windows deliverable; Inno Setup step deferred)
- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels; documented in `ACCESSIBILITY.md`

### Token metrics

- Approx. tokens this release: 200,000
- Approx. cumulative tokens since development began: 5,400,000

(Figures are approximate totals recorded at release time.)

## v0.3.0 — 2026-11-05

### Summary

Packaging and feedback polish: rejected edits and file errors are now shown in
the app instead of only on the console, and the macOS download ships as a real
`Lintel.app` bundle.

### Added

- A status message in the toolbar whenever an edit is rejected or a file
  operation fails (open, save, save as, PDF export), with a
  "Help → Clear message" item
- The macOS release archive now contains `Lintel.app` with a real
  `Info.plist`, instead of a bare binary

### Changed

- (none)

### Fixed

- (none)

### Known limitations

- No Linux AppImage or Windows installer yet
- Screen-reader labelling beyond egui's built-in accessibility is still pending

### Token metrics

- Approx. tokens this release: 300,000
- Approx. cumulative tokens since development began: 5,200,000

(Figures are approximate totals recorded at release time.)

## v0.2.0 — 2026-11-05

### Summary

Joint resolution for walls of unequal thickness, plus linear dimension lines
with numeric labels. This release also brings the shared suite infrastructure
(crash reporting and update checking), a cross-platform CI build matrix, and a
user-facing README.

### Added

- Dimension lines (aligned, horizontal and vertical) with numeric labels and an
  editable offset; place, move and delete them from the canvas and the Inspector
- Dimension hit testing that takes priority over the wall beneath it, plus
  dimension save/load and full undo/redo
- Dimension kind selector and extension-line toggle in the Inspector, labels
  that rotate to follow the line, and snapping applied when a wall or dimension
  is committed (not just previewed)
- An on-canvas hint when a level has no walls yet
- Shared `update-check` crate (GitHub Releases API, semantic version comparison,
  fail-soft offline) and the in-app "check for updates" flow
- Shared `crash-report` crate (panic hook, scrubbed diagnostics, optional
  user-confirmed issue URL)
- In-app release notes window, opened from the version shown in the toolbar,
  with per-release and cumulative token metrics
- Cross-platform CI build matrix (Linux, macOS, Windows) and a multi-platform
  release matrix (Linux x86_64 + aarch64, macOS aarch64, Windows x86_64)
- User-facing README written for non-technical users

### Changed

- Wall joins now extend each wall by half of the *other* wall's thickness, so
  outside corners close cleanly for unequal walls
- The app version is displayed subtly in the toolbar; selecting it opens the
  release-notes window
- `deny.toml` licence policy extended for the new dependency graph

### Fixed

- PDF cross-reference parsing reads bytes directly, so sheets with non-UTF-8
  content load with their pages intact

### Known limitations

- Dimension labels are drawn horizontally; rotated labels for vertical/aligned
  dimensions are planned for v0.2.x
- No Windows/macOS/Linux installers; download the zipped binary from Releases
- The update check opens the Releases page rather than replacing the running
  binary in place

### Token metrics

- Approx. tokens this release: 2,200,000
- Approx. cumulative tokens since development began: 4,900,000

(Figures are approximate totals recorded at release time.)

## v0.1.0 — 2026-11-04

### Summary

First functional test build. Supports walls, doors, windows, levels, save/load round-trip, and PDF plan export. Joint resolution for unequal thicknesses is a known limitation scheduled for 0.2.x.

### Added

- Core domain: walls, openings (doors/windows), levels, project persistence (JSON)
- Command pattern: AddWall, MoveWall, SetWallThickness, SetWallHeight, DeleteWall, PlaceOpening, MoveOpening, SetOpeningSize, FlipOpening, DeleteOpening, AddLevel, DeleteLevel, SetActiveLevel, SetUnits, RenameProject
- Undo/redo history (up to 128 entries)
- Project validation (duplicate ids, missing hosts, overlapping openings, stale identity counter)
- PDF rendering with title block, scale label, sheet number
- Unit system: Metric (mm) and Imperial (feet/inches)
- Hit testing: identify which wall or opening is under the pointer
- Plan geometry: resolved wall bands, joint extensions, bounds computation

### Changed

- (none for v0.1.0 — first release)

### Fixed

- (none for v0.1.0 — first release)

### Known limitations

- Joint resolution assumes equal wall thicknesses; unequal thicknesses may show a small step at outside corners (documented in `join.rs`)
- PDF rendering does not include dimension lines or numeric labels
- No Windows/macOS/Linux installers (binary built via GitHub Actions only)
- No web/WASM target
- No accessibility features (keyboard navigation, screen reader support)
- No GUI polish: empty states, error dialogs, keyboard shortcuts

### Try it

Download the CI artefact for your platform and run:
```bash
./lintel
```
Then try: New Project → Add Wall → Place Door → Save → Reopen → Render PDF.