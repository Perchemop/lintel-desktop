## Lintel 0.20.0 — Design System Extensions

**Summary.** This release finishes the v0.20.0 design-system slice: the
button, form and layout component layer is complete, every colour resolves
through the three token tiers, contrast is enforced per palette by a
dedicated CI check, and the interface is now proven with rendered evidence
and AccessKit assertions instead of claims.

### Added
- Form components (T-002): `Field` (label + input + error/help, screen-reader
  label association, disabled-reason tooltip, painted error icon — never
  colour alone), `FieldSet` (framed legend, collapsible) and `FieldGroup`
- Layout helpers (T-003): `vstack`, `hstack`, `wrap_row` (narrow-window
  reflow) and `card` (padding, border, radius, elevation shadow)
- Rendered design evidence (T-008, guide II.30.8): light/dark/state/focus
  screenshots of the buttons, forms and layout helpers generated headlessly
  in CI and attached to the release PR; PNGs are artefacts, never committed
- AccessKit assertions for forms and layout: inputs are announced with their
  labels, errors are announced as text, Tab follows visual order, and a
  400 → 1200 px resize is proven overlap-free
- `docs/design/gui-stack.md` records the stack (egui/eframe 0.36.2), the
  three-tier token system and the component architecture
- Contrast methodology (T-007) with a required `contrast-test` CI check
  across light, dark and high-contrast reference palettes

### Changed
- Widget code reads `SPEC.component.*` component tokens; no raw `Color32`
  outside `theme.rs` (CI grep gate)
- egui_kittest gains the `wgpu` test-renderer feature for evidence rendering
- Both crates bumped 0.19.0 → 0.20.0

### Known limitations
- Lintel ships the light drafting palette only; the dark and high-contrast
  palettes are asserted reference palettes until the runtime theme switch
  ships (dark evidence screenshots show egui's dark chrome)
- Shell workbench (activity bar, status bar, editor tabs) and docking remain
  separate milestones per the v0.20.0 spec

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Measured from the OpenCode session ledger and recorded in `docs/tokens.md`
(guide §24.1; supersedes the earlier per-release estimates):

- This release: 29,514,710 tokens (input + output + reasoning)
- Cumulative since development began: 1,236,787,671 tokens

---
## Lintel 0.19.0 — Snaps, Sheets & Full Keyboard Reach

**Summary.** Snap markers now resolve through the theme instead of hard-coded colours, and construction strokes pass the UI-graphics contrast floor. The Inspector gains the remaining D-032 bindings — title block, viewports and block definitions — so every documented command is reachable. F7/F8 join the snap shortcuts, and a keyboard-only pick flow removes the last pointer-only accessibility path.

### Added
- D-032 command bindings (PR #60): Inspector "Title block" section edits company, sheet title, revision, date and drawn-by (`SetTitleBlock`); "Viewports" section lists viewports with Remove (`RemoveViewport`) and adds them with a level id, scale denominator and caption (`AddViewport`), defaulting to an A4 landscape region; Blocks gains an "Add definition" name form (`AddBlockDef`, ids assigned UI-side as max+1). D-032 now records only `MoveDimension` as core-only, by design (D-026), so D-027 reachability is fully satisfied
- F7/F8 snap aliases (PR #61): F7 toggles the grid and F8 toggles ortho, alongside the existing G/O keys, following the approved QCAD/AutoCAD convention; `docs/qc/shortcuts.md` updated
- Keyboard block-instance selection (PR #61): a "Select block instance" palette action (Ctrl+K) offers a pick form listing every active-level instance with its raw id, definition name and position. Selecting sets the instance and clears entity selection, matching canvas-click behaviour; the pointer-only accessibility limitation is replaced with this keyboard path

### Changed
- CONSTRUCTION_STROKE darkened (168,148,116) → (140,120,90), taking contrast on PAPER from 2.77:1 to 4.0:1 — above the 3:1 UI-graphics floor; a 3:1 assertion was added to the contrast gate (guide II.30.7.1, PR #59)
- Snap markers are theme tokens (PR #59): five colours (Midpoint/Perpendicular/Tangent/Ortho/Grid) moved from raw `Color32` in `canvas.rs` into tier-1 `BASE_SNAP_*` and tier-2 `SNAP_*` roles; the tier-3 `snap_marker_colour(kind)` helper covers all eight snap kinds, and no raw `Color32` remains in `canvas.rs`. `CONSTRUCTION_STROKE` is a token as well

### Known limitations
- Viewport regions use a default A4 landscape rectangle; editing their on-sheet placement is a future slice
- `MoveDimension` remains core-only by design (D-026: moving a dimension would detach it from what it measures)

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Measured from the OpenCode session ledger and recorded in `docs/tokens.md`
(guide §24.1; supersedes the earlier per-release estimates):

- This release: 1,353,455 tokens (input + output + reasoning)
- Cumulative since development began: 1,207,272,961 tokens

---
## Lintel 0.18.0 — The Design System

**Summary.** This release turns the interface's visual language into an explicit design system: contrast is a merge gate, colours resolve through three token tiers, and buttons follow one documented role-and-size system. Accessibility evidence moves into CI with `egui_kittest`, and a single selected block instance can now be deleted with an honest refusal.

### Added
- Delete a single block instance (CAP-127): `Command::DeleteBlockInstance { block_instance_id }` removes the selected canvas instance; an already-gone instance is refused with `BlockInstanceNotFound` and the refusal is reported in the status bar, with three domain tests (PR #50)
- WCAG AA contrast gate (guide II.30.7.1): `relative_luminance` / `contrast_ratio` in `theme.rs` and the `theme_colours_meet_wcag_aa` merge-gate test; darkened palette roles and raw `Color32` values in the status bar, command palette and main toolbar moved into named theme roles (PR #53)
- `egui_kittest` CI job (guide II.30.8): AccessKit and snapshot testing with `palette_has_named_actions`, `inspector_sections_labelled` and `canvas_summary_accessible`; `kittest` is now a required check in `.ci-required.txt` (PR #55)
- Button patterns: `ButtonRole`/`ButtonSize`/`ButtonRoleStyle` with 12 role-and-size constants (Primary/Secondary/Ghost/Danger × S/M/L, 24/32/40 pt minimums) and `button_style()` with a 2 px COBALT focus ring, applied to the Inspector, command palette and status bar (PR #55)
- ThemeSpec three-tier composition: 24 `BASE_*` swatches → 31 semantic roles → component tokens (`WALL_*`, `PALETTE_*`, `BUTTON_*`), with every raw literal isolated in tier 1 and three new tests; values are byte-identical, so there is no visual change (PR #57)

### Changed
- Keyboard shortcuts are wired and truthful: plain Z/Y undo-redo alongside Ctrl+Z/Ctrl+Y, R opens the release notes, Ctrl+U checks for updates (PR #50)
- Command palette shortcut hints are wired or honestly blank, the dead "View Constraints" entry is gone and the footer shows matched of total (PR #50)
- Palette roles were darkened to meet AA; muted and hint text appears slightly darker than before (PR #53)
- Token metrics now come from the CI-managed `docs/tokens.md` ledger instead of `docs/tokens.json` (PR #52)
- D-032 records unbound commands as core-only (PR #56)

### Known limitations
- Block instances are still selected by pointer only — there is no keyboard-only instance selection yet; once an instance is selected, its Inspector fields are Tab-reachable
- Block instance bounds remain bounding-box approximations, so clicking inside the bounding rectangle of a thin rotated wall still selects it

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.md` (CI-managed ledger, guide §24.1):

- This release: 5,000 tokens
- Cumulative since development began: 75,000 tokens (70,000 + 5,000)

---
## Lintel 0.17.0 — Dimension Precision, Keyboard Truth & the Contrast Gate

**Summary.** Dimensions carry their own label precision, so one drawing can show whole millimetres in one place and decimals in another without changing project-wide units. A selected block instance can now be deleted, and the keyboard shortcuts and palette hints say what they actually do. The palette and status bar also pass a WCAG AA contrast gate.

### Added
- Per-dimension label precision (CAP-126): `Dimension.label_precision` holds 0–4 decimal places (serde default 0, so older project files load unchanged); the Inspector "Precision:" control applies `Command::SetDimensionPrecision` and the drawn label honours the setting
- Delete a single block instance (CAP-127): `Command::DeleteBlockInstance { block_instance_id }` removes the selected canvas instance; an already-gone instance is refused with `BlockInstanceNotFound` and the refusal is reported plainly in the status bar

### Changed
- Keyboard shortcuts wired: plain Z/Y undo-redo alongside Ctrl+Z/Ctrl+Y, R opens the release notes, Ctrl+U checks for updates
- Command palette shortcut hints are now truthful — each is wired or honestly blank (`palette_shortcuts_are_unique`); the dead "View Constraints" entry was removed, since constraints are read in the Inspector solver section
- Palette and status-bar colours pass a WCAG AA contrast gate: the muted, hint, valid-input and palette-grey roles were darkened and raw colours moved into named theme roles. Muted and hint text appears slightly darker than before
- The footer shows matched of total results; token metrics now come from the CI-managed `docs/tokens.md` ledger (guide §24.1)

### Known limitations
- Block instances are still selected by pointer only — there is no keyboard-only instance selection yet; once an instance is selected, its Inspector fields are Tab-reachable (recorded in `ACCESSIBILITY.md`)
- Block instance bounds remain bounding-box approximations, so clicking inside the bounding rectangle of a thin rotated wall still selects it

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.md` (CI-managed ledger, guide §24.1):

- This release: 5,000 tokens
- Cumulative since development began: 70,000 tokens (65,000 + 5,000)

---
## Lintel 0.16.0 — Block Instance Editing & Canvas Selection

**Summary.** Block instances become editable objects rather than fixed placements: click one on the canvas to select it, then move, rotate and flip it from the Inspector. This resolves both limitations published in 0.15.0 — the instance fields are now live, and instances are hit-tested by clicking.

### Added
- Move a block instance (CAP-120): set a new X/Y position from the Inspector or with `MoveBlockInstance`; non-finite positions are rejected and a rejected move leaves the instance where it was, with undo/redo and JSON round-trip
- Rotate a block instance (CAP-121): set the rotation in degrees from the Inspector or with `RotateBlockInstance`; non-finite angles are rejected and the instance keeps its previous rotation
- Flip a block instance (CAP-122): the Inspector's Flipped checkbox and `FlipBlockInstance` mirror an instance across its local x-axis
- Select an instance on the canvas (CAP-123): the Select tool hit-tests block instances — the topmost instance under the pointer wins, it is drawn with a red outline, and Esc clears the selection
- Editable Inspector fields (CAP-124): a "Block instance" section with X/Y position, rotation in degrees and a Flipped checkbox, each applying the matching command; it shows "No block instance selected." when nothing is selected and "Selected instance no longer exists." when the instance has gone
- Auto-select after an edit (CAP-125): the command result reports the affected instance, so the Inspector stays on the instance you just added, moved, rotated or flipped

### Changed
- Block instances are now clickable on the canvas in the Select tool and are tested before ordinary entities
- The Inspector's block-instance position, rotation and flip fields are live instead of read-only

### Known limitations
- Block instance bounds are bounding-box approximations, so clicking inside the bounding rectangle of a thin rotated wall still selects it
- Multi-selection of instances is not supported; one instance is selected at a time

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 65,000 tokens

---
## Lintel 0.15.1 — macOS Installer Fix

**Summary.** Patch release: the macOS release now ships a signed `.dmg` installer alongside the `.zip` archive, satisfying the installable-artefacts requirement for every platform. No feature changes from 0.15.0.

### Changed
- Release pipeline builds and attaches `lintel-macos-aarch64.dmg` for macOS (replacing zip-only macOS artefacts)

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-macos-aarch64.dmg`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 60,000 tokens

---
## Lintel 0.15.0 — Blocks & Symbols, Print Polish

**Summary.** Blocks give you repeatable drafting: define a wall-opening-detail once, place it everywhere, change it once. Sheets are properly framed with clipped viewports and viewport-anchored north arrows and scale bars. Constraints enter the command palette with a solver readout — pinned geometry is visible, not invisible.

### Added
- Block definitions (CAP-116): named groups of entities (wall, opening, dimension, label) saved on the project; `AddBlockDef`/`RemoveBlockDef`/`AddBlockInstance`/`RemoveBlockInstance` commands with validation (non-empty name, duplicate rejection, in-use guard); undo/redo and JSON round-trip
- Block instance UI (CAP-117): `Tool::Block` click-to-place instances on the canvas from an inspector-selected definition; inspector "Blocks" section lists definitions (name, entity count, selectable) and instances (position, rotation, flip, remove)
- Viewport print polish (CAP-118): real PDF clipping at the viewport region (frames cut instead of drawing whole), north arrow and scale bar anchored to each viewport instead of page corners, viewport placement guards (off-sheet/malformed skipped, fallback to scaled plan)
- Constraint palette & solver readout (CAP-119): `AddPerpendicularConstraint`/`RemoveConstraint`/`ViewConstraints` palette entries with labels, shortcuts and "Constraints" category; read-only inspector "Constraint solver" section showing count and validation status

### Changed
- Block definition names are trimmed on add; duplicate names are rejected case-insensitively
- Page-corner north arrow/scale bar now only drawn on plain (non-viewport) sheets; viewport sheets carry orientation inside each viewport

### Known limitations
- Instance fields (position, rotation, flip) are read-only in this release; editable transforms require `MoveBlockInstance`/rotate/flip commands (deferred)
- Block instances aren't yet hit-tested on canvas, so they can't be selected by clicking
- `RemoveConstraint` via palette removes the last constraint (no index picker yet)

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 55,000 tokens

---
## Lintel 0.14.0 — Constraint Persistence & Sheet Layout

**Summary.** Constraints become part of the drawing: they are saved with the project, re-solved on load and after every wall move, and managed from the Inspector. Sheets gain proper title blocks and level-bound viewports, so a drawing set reads like an issued document. A release QC checklist codifies what the pipeline cannot see.

### Added
- Constraint persistence (CAP-111): a level stores its constraints; they survive save/load, are re-solved the moment a file opens and after every wall edit; validation rejects dangling references, out-of-range parameters, self-equal-length and duplicate pins
- Constraint management (CAP-112): an Inspector panel lists a level's constraints with remove buttons and an add-form for perpendicular, fixed-angle and equal-length rules; shared validation rules live once in the core
- Sheet title blocks (CAP-113): every exported sheet carries a bordered bottom-right block — company, project, sheet title, level, scale, revision, drawn-by, date and sheet number; blank fields stay blank
- Sheet viewports (CAP-114): a level can be placed on a sheet multiple times through framed, scale-fit viewports (1:N) with optional captions; viewports are validated (level exists, positive scale, on-sheet regions)
- QC codified (CAP-115): `docs/qc/release-checklist.md` — signed-artefact verification, AppImage smoke, screen-reader pass, ledger consistency, each with executable commands and sign-off lines

### Changed
- The release-notes popup and the published GitHub release both derive from `RELEASE_NOTES.md`, which now carries the 0.14.0 section
- Title-block sheet numbering: an optional base number in the title block drives A-101-style numbering across the drawing set

### Known limitations
- Walls straddling a viewport boundary are drawn whole rather than clipped
- Circle and arc construction flows remain simplified (radius from two clicks; fixed 90° arc)
- Constraint objects do not yet appear in the command palette (Inspector is the home)

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 50,000 tokens

---
## Lintel 0.13.0 — Anchored Dimensions & Release Governance

**Summary.** Dimensions become anchored to the geometry they measure: an anchor ties a measured point to a wall at a parametric position, so moving a wall drags the dimension with it. Witness lines gain editable gap and overrun, a constraint solver keeps anchored dimensions correct as geometry changes, and the whole elite-CAD Phase A surface (construction tools, layers, snapping, selection, command palette) is now fully reachable from the UI. The release pipeline is governed: signed artefacts are enforced, tags must match crate versions, and the in-app release notes are generated from the same file that feeds GitHub releases.

### Added
- Constraint-driven dimensions core (CAP-100): `DimensionAnchor` ties a measured point to a wall at a fraction along it; witness-line gap and overrun (200 mm / 100 mm defaults); anchored dimensions follow wall moves; old project files load unchanged
- Constraint solver (CAP-101): `PerpendicularDistance`, `FixedAngle` and `EqualLength` constraints with `solve_constraints`; applying a constraint moves the anchored point and its stored fallback
- Anchor commands (CAP-102): set/clear start and end anchors, set witness gap and overrun — validated, undoable, persisted
- Anchor UI (CAP-103): inspector anchor rows ("anchored to Wall N at P%"), witness margin controls, canvas witness lines drawn with gap and overrun, cobalt grip markers on anchored points
- Construction tools (CAP-108): line, ray, circle and arc with two-click canvas flows; Layers panel with visibility/lock toggles; command-palette entries for every construction action
- Keyboard reachability (CAP-108): C/Shift+C/Q/X select the construction tools, Shift+V toggles the selected construction's visibility, Del deletes it; clicking near a wall while placing a dimension anchors that endpoint to the wall
- Update verification metadata (CAP-104): the update popup surfaces the release's `SHA256SUMS.txt` manifest and digest and links the verification guide; Lintel states plainly that it never installs updates for you
- Release governance (CAP-106): CI refuses to publish when the tag does not match the crate versions or when signing is unavailable, verifies its own signature, anchors releases to the real git tag, and warns when the token figure is the default
- Documentation parity (CAP-107): capability inventory, parity evidence, traceability and architecture docs extended through CAP-108

### Changed
- In-app release notes are parsed at runtime from `RELEASE_NOTES.md` (CAP-105), so the popup can never drift from the published notes; releases before the token ledger are marked "predates the token ledger"
- The update check honours its one-hour cache instead of hitting GitHub on every click
- GitHub releases are anchored to `v`-prefixed git tags going forward

### Fixed
- Released binaries self-reporting a stale version is now structurally impossible: CI fails the release when tag and crate versions disagree
- Silent unsigned releases are no longer possible: the workflow fails without `GPG_PRIVATE_KEY` and verifies its own signature before publishing

### Known limitations
- Constraint objects are not yet persisted on the level (the solver takes an explicit list); sheet layout with title blocks (CAP-099) and constraint persistence are the next planning targets
- Circle and arc construction flows are simplified (radius from two clicks; fixed 90-degree arc)

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, and `SHA256SUMS.txt.asc` (signed).

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 45,000 tokens

---
<!-- This file is attached verbatim to every GitHub Release by
     .github/workflows/release.yml (--notes-file). The newest release belongs at
     the top. Everything above the first release heading stays invisible on
     GitHub because it is an HTML comment. -->

## Lintel 0.12.1 — Constraint-Driven Dimensions

**Summary.** The constraint-driven dimensions milestone's alignment release. The workspace manifests move to 0.12.1 so the shipped binaries, the Windows installer metadata source and the CI token ledger agree, and this release collects the working-environment capabilities that landed on `main` between published releases — layers and construction geometry, the persistent status bar, enhanced snapping, window/crossing/fence selection with filters, and the Ctrl+K command palette.

### Added
- Phase A working environment (CAP-089–093): a layer model with a protected default layer and per-entity assignment (`layer.rs`), line/ray/circle/arc construction geometry that snaps but never prints (`construction.rs`), and a persistent status bar showing cursor coordinates, snap/ortho/grid chips, selection count and zoom % (`status_bar.rs`).
- Enhanced entity snapping (CAP-094, CAP-098): endpoint, midpoint, intersection, perpendicular, tangent, quadrant, grid and ortho candidates resolved through an explicit priority order, with on-canvas snap indicators.
- Selection modes (CAP-095, CAP-096): window, crossing and fence selection owned by the core (`selection.rs`), with add/remove sets and entity-type/layer filters.
- Command palette (CAP-097): Ctrl+K opens a searchable palette with fuzzy matching, recent commands and keyboard navigation.

### Changed
- `crates/lintel-core/Cargo.toml` and `apps/lintel/Cargo.toml` now report 0.12.1 (the manifests were stale at 0.10.1 while the release ledger already recorded 0.12.1).

### Fixed
- `RELEASE_NOTES.md` gained the missing 0.11.0 and 0.12.1 sections; earlier releases in this milestone were published with the 0.10.1 section still at the top of the notes file.

### Known limitations
- The constraint-driven dimensions foundation itself (`DimensionAnchor`, witness-line gap/overrun, the constraint solver) is designed and scoped (`DOMAIN_DESIGN_SKETCHES.md` §3, IMPLEMENTATION_PLAN "Phase B") but not yet implemented; it is the next slice.
- Screen-reader output is verified manually, not in CI.

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 15,000 tokens

---
## Lintel 0.11.0 — Constraint-Driven Dimensions

**Summary.** The constraint-driven dimensions milestone's measurement slice: a Measurements tool that reports a selected room's area and perimeter and swaps the Inspector to live level and project totals, all shown in the project's Metric or Imperial units.

### Added
- Measurements tool (Shift+M, CAP-086): select a room and read its area and perimeter. `crates/lintel-core/src/measurements.rs` provides `Measurements`, `LevelTotals` and `ProjectTotals` — room area/perimeter, level wall and opening length totals, and project-wide aggregation.
- Inspector → Measurements section (CAP-087): the selected room, per-level totals, a per-room list and the project totals.
- Unit-aware area formatting (CAP-088): `format_area` honours `UnitSystem`, formatting areas consistently as m², ft² or in².

### Changed
- Area and length readouts follow the project's unit system instead of raw millimetres.

### Fixed
- Measurements inspector room detection and caching: the selected-room readout could go stale or pick the wrong room.
- Release notes: this section restores the notes that were missing when the release was first published (the 0.10.1 section was still at the top of the file).

### Known limitations
- Rooms have no persistent identity; naming is done via plan labels anchored inside the room (unchanged from 0.9.0).
- Screen-reader output is verified manually, not in CI.

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 10,000 tokens

---
## Lintel 0.10.1 — Window icon and installer version

**Summary.** The application window and taskbar now show the Lintel icon instead of the default toolkit icon, and the Windows installer metadata reports the correct version.

### Added
- Window icon: the embedded Lintel PNG is set on the eframe viewport via `.with_icon()`, so the window title bar, taskbar and app switcher show the Lintel mark on every platform.
- `resources/icons/lintel.svg` — original scalable icon source (clean-room floor-plan motif in the Lintel palette) for future platform icon generation.

### Changed
- Windows installer script (`resources/windows/lintel.iss`) now reports version 0.10.1 (was stale at 0.8.0).
- `ATTRIBUTION.md` records the icon assets and their original licence.

### Fixed
- `include_bytes!` path for the embedded icon resolves from the source file's directory (three levels to the repository root).

### Known limitations
- macOS app bundle still ships without a `.icns` icon (planned next release).
- Release signing still requires the `GPG_PRIVATE_KEY` repository secret to produce `SHA256SUMS.txt.asc`.

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 10,000 tokens

---
## Lintel 0.10.0 — Suite-standard conformance

**Summary.** Brings Lintel into conformance with guide §12.1 suite-wide standards: CI-managed token ledger, signed releases with SHA256SUMS verification, property-based geometry fuzz tests, and development documentation.

### Added
- `docs/development.md` — build, test, and contribution guide for developers.
- `crates/lintel-core/tests/properties.rs` — property-based fuzz tests for room detection edge cases (degenerate walls, NaN coordinates, collinear overlaps, extreme values).
- Release signing: `SHA256SUMS.txt` is GPG-signed and the `.asc` artefact is attached to every release (guide §12.1 auto-update security).
- CI updates `docs/tokens.json` on each semver tag (`release_version`, `release_tokens`, `total_tokens_since_start`).

### Changed
- `crates/lintel-core/Cargo.toml` adds `proptest = "1"` as a dev-dependency for fuzz tests.
- `.github/workflows/release.yml` adds token-ledger update and release-signing steps to the publish job.

### Fixed
- Release notes now reference `docs/tokens.json` ledger values for every release.

### Known limitations
- `crates/update-check` is still in-repo; guide §12.1 requires a shared git submodule (deviation recorded in DECISIONS.md D-012).
- In-app auto-update verifies SHA256SUMS but does not yet verify the GPG signature before download (verification wiring planned post-v0.10.0).

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, `SHA256SUMS.txt`, `SHA256SUMS.txt.asc`.

### Token metrics
Per `docs/tokens.json` (CI-managed ledger, guide §12.1):

- This release: 5,000 tokens
- Cumulative since development began: 5,000 tokens

---
## Lintel 0.9.1 — Version fix

**Summary.** Corrects the shipped binary version to 0.9.1 (fixes update check reporting the wrong installed version).

### Fixed
- Bumped crate versions to 0.9.1 so release binaries report the correct version.

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Approximate totals recorded at release time.

- This release: ~5,000 tokens
- Cumulative since development began: ~6,085,000 tokens

---
## Lintel 0.9.0 — Drawing Set

**Summary.** Lintel now issues a real drawing set: auto-detected rooms with area captions, free text plan labels, an opening schedule, north arrow and graphic scale bar on every sheet, multi-sheet PDF export with a cover sheet, and duplicate (Ctrl+D).

### Added
- Auto-detected rooms via planar face traversal; area captions rendered on canvas and PDF.
- Free text plan labels with CRUD, drag to move, Delete key.
- Opening schedule grouped by kind/size with D1/W1 marks; shown in Inspector and PDF.
- Multi-sheet drawing-set export: cover sheet + one sheet per storey via `render_set`.
- Sheet furniture: north arrow and graphic scale bar on plan sheets.
- Duplicate selection (Ctrl+D) for wall/dimension/label/opening.
- Export drawing set menu (Ctrl+Shift+E) and Label tool (L).

### Changed
- `PlanGeometry` now exposes `labels` and `rooms`; hit testing prioritises labels.
- PDF `escape_text` maps superscripts and dashes to ASCII for WinAnsi fidelity.

### Fixed
- Room detection orientation bug: disconnected wall groups previously produced duplicate rooms; now all non-positive walks are dropped.
- `LabelNotFound` error message no longer duplicates “label”.
- Room JSON round-trip uses approximate centroid comparison.

### Known limitations
- Rooms have no persistent identity; naming is done via plan labels anchored inside the room.
- `crates/update-check` is still in-repo; guide §12.1 requires a shared git submodule (deviation recorded).
- Version UI is still top-toolbar; guide §12.1 requires bottom-right version with Release Notes popup containing token metrics and Update button (planned for v0.10.0).

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Approximate totals recorded at release time.

- This release: ~120,000 tokens
- Cumulative since development began: ~6,080,000 tokens

---
## Lintel 0.8.0 — Keyboard-focus review

**Summary.** A focus review found a regression: after tabbing to the drawing
canvas, every keyboard shortcut went dead. Shortcuts now work with the canvas
focused and are still suppressed while you type in a field.

### Added

- Unit tests for the level summary shown in the toolbar and announced to screen
  readers
- `ACCESSIBILITY.md` now records the focus-review outcome: tab order, visible
  focus, Escape dismissal, and that focus is never trapped

### Changed

- Keyboard shortcuts apply when nothing has focus **or** when the canvas has
  focus; they are still suppressed while typing in a text field

### Fixed

- **Shortcuts stopped working after tabbing to the canvas** (a regression from
  0.6.0, when the canvas became focusable)

### Known limitations

- Screen-reader output is verified manually, not in CI
- The Linux AppImage has not been executed on an x86_64 host

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~130,000 tokens
- Cumulative since development began: ~5,960,000 tokens

---

## Lintel 0.7.0 — Every documented capability is now reachable

**Summary.** An audit found four things Lintel could do but that had no way to
be triggered in the interface — add or delete a storey, rename the project, and
switch between metric and imperial units. All four are now in the Inspector,
and dragging a dimension moves its line.

### Added

- **Project panel:** rename the project, and switch between Metric (mm) and
  Imperial (ft-in) units
- **Storeys panel:** delete a storey, and add a new one with a name and
  elevation
- Dragging a selected dimension line moves it perpendicular to what it
  measures (its offset)

### Fixed

- Add storey, delete storey, rename project and switch units were implemented in
  the core but unreachable from the interface — the same class of defect as the
  unreachable export dialog fixed in 0.6.0

### Known limitations

- Moving a dimension's measured points (`MoveDimension`) is a core-only command
  covered by unit tests; dragging adjusts the offset instead, because moving the
  measured span would detach a dimension from what it measures
- Screen-reader output is verified manually, not in CI

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~160,000 tokens
- Cumulative since development began: ~5,830,000 tokens

---

## Lintel 0.6.0 — Keyboard-complete file flows and an accessible canvas

**Summary.** Everything you do with files is now one keyboard shortcut away,
and the canvas announces what is on the plan to a screen reader. This release
also fixes a real bug: the PDF export dialog existed but could not be opened.

### Added

- File keyboard shortcuts: **Ctrl+N** new, **Ctrl+O** open, **Ctrl+S** save,
  **Ctrl+Shift+S** save as, **Ctrl+E** export sheet
- The plan canvas is keyboard-focusable and carries a screen-reader label
  describing the active level, e.g. `Ground: 3 walls, 2 doors, 1 window, 1 dimension`
- The same level summary is shown in the toolbar, so you can see the model's
  contents at a glance

### Changed

- Plain tool shortcuts (W/S/D/N/M/G/O/F) no longer fire while Ctrl/Cmd or Alt is
  held — previously Ctrl+S would also switch to the Select tool

### Fixed

- **PDF export is reachable again.** The export dialog was wired up but nothing
  opened it, so the documented "Export sheet…" flow did not exist. It is now on
  the File menu and bound to Ctrl+E.

### Known limitations

- Canvas objects have no per-object labels (the canvas carries a level summary
  instead); documented in `ACCESSIBILITY.md`
- Screen-reader output is verified manually, not in CI
- The Linux AppImage is verified statically (ELF + SquashFS + checksum);
  running it requires an x86_64 Linux host

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~150,000 tokens
- Cumulative since development began: ~5,670,000 tokens

---

## Lintel 0.5.0 — Windows installer

**Summary.** Windows users get a real installer: the release now publishes
`lintel-windows-x86_64-setup.exe`, built on CI with Inno Setup, alongside the
Windows zip. Install to Program Files with Start-menu and optional desktop
shortcuts.

### Added

- Windows installer (`lintel-windows-x86_64-setup.exe`) published with the
  release, in addition to `lintel-windows-x86_64.zip`
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
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels;
  documented in `ACCESSIBILITY.md`

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~120,000 tokens
- Cumulative since development began: ~5,520,000 tokens

---

## Lintel 0.4.0 — Accessibility labels, keyboard shortcuts, AppImage

**Summary.** Keyboard shortcut hints and hover labels on every control,
WCAG AA focus-ring contrast, and a Linux AppImage release artefact. The
Windows installer is deferred (Inno Setup download hangs on CI).

### Added

- Keyboard shortcut line in the toolbar: W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete
- Hover tooltip on every menu button and Inspector control describing units and ranges
- Focus-ring colour COBALT verified above WCAG AA 4.5:1 on both panel and canvas backgrounds
- Escape dismissal confirmed on all dialogs (About, Release notes, Update, Export sheet)
- `ACCESSIBILITY.md` honesty statement and `docs/qc/checklist.md` accessibility smoke test
- Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, valid ELF)

### Changed

- (none)

### Fixed

- AppImage build: `appimagetool` is itself an AppImage needing FUSE; now extracted
  and the embedded `AppRun` ELF invoked directly so no FUSE is required on CI

### Known limitations

- No Windows installer yet — the Windows zip remains the deliverable; the Inno
  Setup installer step is deferred to a future release
- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels;
  documented in `ACCESSIBILITY.md`

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage),
`lintel-windows-x86_64.zip`, and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~200,000 tokens
- Cumulative since development began: ~5,400,000 tokens

---

## Lintel 0.3.0 — Error feedback and macOS bundle

**Summary.** When something cannot be done — a rejected edit, a file that will
not open or save, a PDF that cannot be written — Lintel now says so in the
toolbar instead of staying silent. The macOS download is packaged as a real
`Lintel.app` application.

### Added

- A status message in the toolbar whenever an edit is rejected or a file
  operation fails (open, save, save as, PDF export). It clears itself after a
  few seconds, and **Help → Clear message** hides it immediately.
- The macOS release archive now contains `Lintel.app` with a real
  `Info.plist`, so it launches like a normal Mac application.

### Known limitations

- No Linux AppImage or Windows installer yet — unzip and run the binary.
- Screen-reader labelling beyond the toolkit's built-in support is pending.

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-windows-x86_64.zip`, and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~300,000 tokens
- Cumulative since development began: ~5,200,000 tokens

---

## Lintel 0.2.0 — Joint resolution, dimension lines, suite infrastructure

**Summary.** Wall joins now close cleanly even when the two walls have
different thicknesses, and you can annotate the plan with dimension lines that
show real measurements. Under the hood this release adds the shared crash
reporting and update-checking infrastructure and builds on more platforms.

### Added

- Dimension lines (aligned, horizontal and vertical) with numeric labels and an
  editable offset. Place them with the Dimension tool, then move, re-offset or
  delete them from the canvas or the Inspector.
- Dimension kind selector and an extension-line (witness line) toggle in the
  Inspector.
- Rotated dimension labels that follow the line, so vertical and aligned
  measurements read correctly.
- Snapping now also applies when you *commit* a wall or dimension, so both ends
  land exactly on existing wall corners.
- An on-canvas hint when a level is empty.
- In-app release notes: the version shown in the toolbar opens a window with
  the notes for every release and token-usage metrics.
- Update checking: Lintel quietly checks the GitHub Releases API for a newer
  version and offers a one-click link to the download page. It fails soft when
  you are offline and never blocks the app.
- Crash reporting: a panic writes a scrubbed JSON log locally. Nothing is sent
  anywhere unless you open the pre-filled bug report yourself.
- Cross-platform CI build matrix (Linux, macOS, Windows) and release artefacts
  for Linux x86_64, Linux aarch64, macOS aarch64 and Windows x86_64.
- A README written for people who are not developers.

### Changed

- Wall joins extend each wall by half of the *other* wall's thickness, so
  outside corners close for unequal walls.
- The app version is shown subtly in the toolbar; selecting it opens the
  release-notes window.

### Fixed

- PDF cross-reference parsing reads bytes directly, so sheets containing
  non-UTF-8 content open with their pages intact.

### Known limitations

- No installers yet (AppImage, MSI, macOS .app) — download the zipped binary
  from the Releases page and run it.
- The update check opens the Releases page; it does not replace the running
  binary in place.
- Accessibility work (keyboard navigation, screen-reader labels) is scheduled
  for the next milestone.

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip`, `lintel-windows-x86_64.zip`, and
`SHA256SUMS.txt` with one SHA-256 checksum per archive.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~2,200,000 tokens
- Cumulative since development began: ~4,900,000 tokens

### Roadmap

v0.2.0 completes the "Joint Resolution & Dimensions" milestone
(`ROADMAP.md`). Next up: installers, accessibility, and empty/error-state
polish.

---

## Lintel 0.1.0 — First functional test build

**Summary.** The first useful installable build: draw walls, place doors and
windows, save and reopen the plan, and export a PDF sheet. Walls, doors,
windows and levels are all real — backed by a headless domain model with
workflow tests and green CI.

### Added

- Walls, doors, windows and levels with real geometry and validation
- Snapping, pan and zoom on the plan canvas
- Undo/redo history (up to 128 edits)
- Save and reopen plans as JSON, and export a PDF sheet at true scale
- Metric and Imperial unit display
- Linux x86_64 CI artefact and release

### Known limitations

- Joint resolution assumed equal wall thicknesses
- No dimension lines, installers, accessibility features or GUI polish

### Artefacts

`lintel-linux-x86_64.zip` and `SHA256SUMS.txt`.
