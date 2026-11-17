# Capability inventory

Single source of truth for what Lintel is designed to do (guide II.27.3).
One row per capability; status values: not started / partial / implemented /
out of scope. Readiness % moves only from verified evidence (test name + CI
run, recorded in `docs/parity.md`).

## v0.1.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-001 | Create a project | New plan opens with one empty storey ("Level 0") | `Project::new` yields one level; `validate()` passes | — | implemented |
| CAP-002 | Add a wall | Two endpoints, thickness, height create a wall; band drawn at the joint | `Command::AddWall` accepted; `PlanGeometry` reports it | inspiration §wall addition | implemented |
| CAP-003 | Move a wall | Dragging a wall moves both endpoints; hosted openings move with it | `Command::MoveWall`; opening fit re-validated | inspiration §wall addition | implemented |
| CAP-004 | Set wall thickness | Inspector changes thickness within 20–2000 mm | `Command::SetWallThickness` rejects out-of-range | — | implemented |
| CAP-005 | Set wall height | Inspector changes height within 300–20000 mm | `Command::SetWallHeight` rejects out-of-range | — | implemented |
| CAP-006 | Delete a wall | Wall and its hosted openings disappear | `Command::DeleteWall` cascades to openings | — | implemented |
| CAP-007 | Place a door | Click on a wall → 900 × 2100 mm door centred at the click | `Command::PlaceOpening(Door)`; fit-on + no-overlap | inspiration §door placement | implemented |
| CAP-008 | Place a window | Click on a wall → 1200 × 1200 mm window at 900 mm sill | `Command::PlaceOpening(Window)`; fit-on + no-overlap | inspiration §window placement | implemented |
| CAP-009 | Move an opening | Drag an opening along its host wall; stops at jambs | `Command::MoveOpening` clamps to wall ends | — | implemented |
| CAP-010 | Set opening size | Inspector sets width/height/sill within bounds | `Command::SetOpeningSize`; fit-on re-checked | — | implemented |
| CAP-011 | Flip opening | Door hinge swaps jambs; window unaffected visually | `Command::FlipOpening` toggles `flipped` | inspiration §door placement | implemented |
| CAP-012 | Delete an opening | Opening removed from its wall | `Command::DeleteOpening` | — | implemented |
| CAP-013 | Add a level | New storey with name + elevation; becomes active | `Command::AddLevel` | — | implemented |
| CAP-014 | Delete a level | Cannot delete the last level | `Command::DeleteLevel` returns `LastLevel` | — | implemented |
| CAP-015 | Set active level | View switches to the chosen storey | `Command::SetActiveLevel` | — | implemented |
| CAP-016 | Rename the project | Project name editable; stored in file | `Command::RenameProject` rejects blank | — | implemented |
| CAP-017 | Save / load | JSON file written atomically; reopen yields identical state | `Document::save_as` / `Document::open` round-trip | inspiration §project JSON format | implemented |
| CAP-018 | Undo | Last command reverted | `Document::undo` restores prior snapshot | inspiration §undo/redo | implemented |
| CAP-019 | Redo | Undone command re-applied | `Document::redo` | inspiration §undo/redo | implemented |
| CAP-020 | PDF plan | One-page sheet: border, title block, plan at true scale | `render_plan` emits valid PDF 1.7 with plan geometry | inspiration §PDF rendering | implemented |
| CAP-021 | Joint extensions | Wall ends extend into corners so bands meet | `PlanGeometry` resolves extensions | inspiration §wall addition | implemented |
| CAP-022 | Hit testing | Pointer over a door selects the door, not the wall behind it | `PlanGeometry::hit_test` prefers openings | — | implemented |
| CAP-023 | Units | Metric (mm/m) or Imperial (ft-in) display | `UnitSystem::format` / `parse_length` round-trip | inspiration §unit display | implemented |
| CAP-024 | Validation | Invalid files and commands rejected with plain messages | `Project::validate`; command errors | — | implemented |
| CAP-025 | `Applied` result | Command reports which entity it created | `Applied { wall, opening, level }` | — | implemented |

## v0.2.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-026 | Dimension lines | Aligned/horizontal/vertical dimension between two points with an offset | `Command::PlaceDimension`; `PlanGeometry` reports it | — | implemented |
| CAP-027 | Dimension placement / move / delete | Two clicks place, dragging moves the line, Delete removes | `PlaceDimension`, `SetDimensionOffset` (drag), `DeleteDimension` | — | implemented |
| CAP-028 | Dimension offset editing | Inspector changes the offset; zero/negative rejected | `Command::SetDimensionOffset` validation | — | implemented |
| CAP-029 | Dimension hit testing | Clicking a dimension selects it; priority over the wall band | `PlanGeometry::hit_test` checks dimensions first | — | implemented |
| CAP-030 | Dimension validation | Zero-length and non-positive offset rejected with plain messages | Command validation strings | — | implemented |
| CAP-031 | Dimension undo/redo | Undo restores the previous state; redo re-applies | `Document::undo` / `redo` for dimension commands | — | implemented |
| CAP-032 | Dimension persistence | Save/load preserves dimensions | `Document::save_as` / `open` round-trip | — | implemented |
| CAP-033 | Dimension hit testing priority (over walls) | Clicking a dimension selects it; priority over the wall band | `PlanGeometry::hit_test` checks dimensions first | — | implemented |
| CAP-040 | Crash reporting | A panic writes a scrubbed JSON report; nothing is sent automatically | `crash-report` unit tests | guide §7.1 | implemented |
| CAP-041 | Update check | Queries the GitHub Releases API, compares versions, fails soft offline | `update-check` unit tests | guide §7.2 | implemented |
| CAP-042 | In-app release notes | Toolbar version opens a window with all notes + token metrics | `apps/lintel/src/about.rs` | guide §9 | implemented |
| CAP-043 | Cross-platform CI build matrix | Linux/macOS/Windows compile in CI | `build-matrix` job | guide §8 step 7 | implemented |
| CAP-044 | Multi-platform release artefacts | Release publishes 4 target archives + checksums | `release.yml` matrix | guide §6.2 | implemented |
| CAP-045 | User-facing README | Plain-language README for non-technical users | `README.md` | guide §9 | implemented |
| CAP-046 | Empty-state hint | A level with no walls shows an on-canvas hint | `apps/lintel/src/main.rs` | guide §6.2 | implemented |
| CAP-047 | Visible error feedback | Rejected edits and file failures show a toolbar status message | `LintelApp::note_status` | guide §9 | implemented |
| CAP-048 | macOS app bundle | The macOS archive contains `Lintel.app` with `Info.plist` | `release.yml` bundle step | guide §6.2 | implemented |
| CAP-049 | Keyboard shortcut hints | Toolbar line + hover tooltip on every control | `apps/lintel/src/main.rs`, `apps/lintel/src/inspector.rs` | guide §9 / II.21 | implemented |
| CAP-050 | Focus-ring contrast WCAG AA | COBALT focus rings meet AA on panel + canvas backgrounds | `apps/lintel/src/theme.rs` (comment) | guide II.21 | implemented |
| CAP-051 | Linux AppImage | Release publishes `lintel-appimage-x86_64.AppImage` (valid ELF) | `release.yml` AppImage step | guide §6.2 | implemented |
| CAP-052 | Windows installer | Release publishes `lintel-windows-x86_64-setup.exe` via Inno Setup | `release.yml` + `resources/windows/lintel.iss` | guide §6.2 | implemented |
| CAP-053 | `.iss` DefaultDirName fix | Installer suggests the correct Program Files dir | `resources/windows/lintel.iss` | — | implemented |
| CAP-054 | `.iss` AppId + Windows icon | AppId brace escaped; `SetupIconFile` uses a real `.ico` | `resources/windows/lintel.iss`, `resources/windows/lintel.ico` | — | implemented |
| CAP-055 | File keyboard shortcuts | Ctrl+N/O/S/Shift+S/E drive New/Open/Save/Save As/Export | `LintelApp` helpers + input block | guide II.21.2 | implemented |
| CAP-056 | Accessible canvas | Canvas is focusable and labelled with the level summary | `canvas.rs` `widget_info`, `plan_summary` | guide II.21.2 | implemented |
| CAP-057 | Toolbar level summary | Toolbar shows the active level's contents | `plan_summary` | guide II.21.2 | implemented |
| CAP-058 | Modifier-guarded tool keys | W/S/D/N/M/G/O/F do not fire with Ctrl/Cmd/Alt | input block `plain_keys` | guide II.21.2 | implemented |
| CAP-059 | PDF export reachable | File → Export sheet… opens the dialog (was unreachable) | `main.rs` File menu | guide §9 (no fake functionality) | implemented |
| CAP-060 | AppImage verification | Published AppImage checked (ELF + SquashFS + SHA-256) | `docs/qc/checklist.md` | guide §6.2 | implemented |
| CAP-061 | Add / delete a storey | Inspector → Storeys: Delete per row, and a name-elevation-Add row | `inspector.rs` storeys section | guide §9 (no fake functionality) | implemented |
| CAP-062 | Rename the project | Inspector → Project: name field + Apply | `inspector.rs` project section | guide §9 | implemented |
| CAP-063 | Switch units | Inspector → Project: Metric / Imperial radio | `inspector.rs` project section | guide §9 | implemented |
| CAP-064 | Drag a dimension | Dragging a selected dimension changes its offset | `main.rs` drag handler | guide §9 | implemented |
| CAP-065 | Reachability audit | Every `Command` variant is bound in the UI except the documented `MoveDimension` | `docs/qc/checklist.md` §7 | guide §9 | implemented |
| CAP-066 | Shortcuts with canvas focus | Tool/file/undo keys work while the canvas has focus, not while typing | `main.rs` keyboard scope + `CanvasResponse::widget_id` | guide II.21.2 | implemented |
| CAP-067 | Level summary unit-tested | Empty, singular and multi-storey summary cases covered | `main.rs` `#[cfg(test)]` | guide §9 | implemented |
| CAP-068 | Focus review recorded | Tab order, visible focus, Escape dismissal, no trapping | `ACCESSIBILITY.md` | guide II.21.4 | implemented |

## v0.9.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-069 | Auto-detected rooms | Planar face traversal lists enclosed rooms with area | `room::tests` | — | implemented |
| CAP-070 | Free text plan labels | Place, move, delete text labels on the plan | `command::tests::label::*` | — | implemented |
| CAP-071 | Opening schedule | Door/window schedule rows generated per level | `schedule::tests` | — | implemented |
| CAP-072 | Multi-sheet PDF drawing set | Cover, north arrow, scale bar, per-sheet schedule | `pdf::tests::render_set_*` | — | implemented |
| CAP-073 | Room area captions | Room area shown on canvas | `canvas.rs` | — | implemented |
| CAP-074 | Label tool + duplicate + export | L key, Ctrl+D duplicate, Ctrl+Shift+E export drawing set | `main.rs` | — | implemented |

## v0.10.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-075 | CI-managed token ledger | `docs/tokens.md` updated by CI on each tag | `release.yml` | guide §12.1 | implemented |
| CAP-076 | Release signing step | `release.yml` signs artefacts when key present | `release.yml` | guide §12.1 | implemented |
| CAP-077 | Development docs | `docs/development.md` added | — | guide §12.1 | implemented |
| CAP-078 | Property-based fuzz tests | Geometry and command-layer proptests | `properties.rs` | guide §12.1 | implemented |
| CAP-079 | Version UI bottom-right | Version in bottom-right; Release Notes popup + Update button | `about.rs` | guide §12.1 | implemented |
| CAP-080 | Window icon embedded | Icon embedded via `include_bytes!` + `image` crate | `main.rs` | guide §12.1 | implemented |

## v0.10.1 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-081 | Window icon via `with_icon()` | App window shows the Lintel icon | `main.rs` | — | implemented |
| CAP-082 | SVG icon source | `resources/icons/lintel.svg` canonical source | — | — | implemented |
| CAP-083 | Windows `.iss` version fix | Installer version corrected to 0.10.1 | `resources/windows/lintel.iss` | — | implemented |
| CAP-084 | Icon attribution | `ATTRIBUTION.md` records icon licence | `ATTRIBUTION.md` | — | implemented |
| CAP-085 | Command-layer fuzz (never panic/corrupt/undo) | Proptest cases in `properties.rs` | `properties.rs` | — | implemented |

## v0.11.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-086 | Area/perimeter query tool | Shift+M reports selected room area and perimeter | `measurements::tests` | — | implemented |
| CAP-087 | Wall length totals | Level and project totals in Inspector → Measurements | `measurements::tests` | — | implemented |
| CAP-088 | `format_area` honours units | m² / ft² / in² output | `units::tests` | — | implemented |

## Phase A foundation (v0.11.1/v0.12.1) capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-089 | Layers | `Layer`, protected default layer, per-entity assignment | `layer::tests` | — | implemented |
| CAP-090 | Construction geometry | Line/ray/circle/arc, snap-enabled, never printed | `construction::tests` | — | implemented |
| CAP-091 | Status bar: cursor coordinates | Cursor position shown | `status_bar.rs` | — | implemented |
| CAP-092 | Status bar: snap/ortho/grid chips | Mode chips shown | `status_bar.rs` | — | implemented |
| CAP-093 | Status bar: selection count + zoom % | Counts and zoom shown | `status_bar.rs` | — | implemented |

## v0.11.1 capabilities (published as 0.12.1)

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-094 | Enhanced entity snapping | Endpoint/midpoint/intersection/perpendicular/tangent/quadrant/grid/ortho with priority | `snap::tests` | — | implemented |
| CAP-095 | Window/crossing/fence selection | Selection modes owned by core | `selection::tests` | — | implemented |
| CAP-096 | Selection filters | By entity type and layer | `selection::tests` | — | implemented |
| CAP-097 | Command palette | Ctrl+K, fuzzy match, recent commands | `command_palette.rs` | — | implemented |
| CAP-098 | Snap visual indicators | On-canvas indicators per candidate kind | `canvas.rs` | — | implemented |

## v0.12.0–v0.13.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-099 | Drawing set / sheet layout | Title blocks and viewport linking | `pdf::tests::render_set_*` | — | implemented (CAP-113/CAP-114) |

## v0.14.0–v0.16.0 capabilities

| ID | Capability | Observable behaviour | Acceptance criteria | Source URL(s) | Status |
|---|---|---|---|---|---|
| CAP-111 | Constraint persistence | A level stores its constraints; they survive save/load and re-solve on load and after wall edits | `level::tests::constraints_survive_a_save_and_load`; `Project::validate` | — | implemented |
| CAP-112 | Constraint management | Inspector lists a level's constraints with Remove per row and an add form for perpendicular, fixed-angle and equal-length rules | `command::tests::adding_a_constraint_pushes_it_onto_the_active_level`, `removing_a_constraint_by_index_works`; `inspector.rs` | — | implemented |
| CAP-113 | Sheet title blocks | Every exported sheet carries a bottom-right title block (company, project, sheet title, level, scale, revision, drawn-by, date, number) | `pdf::tests::the_title_block_prints_the_project_fields`; `command::tests::setting_the_title_block_replaces_it_and_trims_whitespace` | — | implemented |
| CAP-114 | Sheet viewports | A level can be placed on a sheet several times through framed, scale-fit viewports with optional captions; invalid viewports are rejected | `pdf::tests::a_viewport_is_framed_and_captioned_on_its_sheet`; `command::tests::a_viewport_bound_to_a_missing_level_is_refused` | — | implemented |
| CAP-115 | Release QC checklist | `docs/qc/release-checklist.md` covers signed artefacts, AppImage smoke, screen-reader pass and ledger consistency with sign-off lines | `docs/qc/release-checklist.md` | guide §2.5 | implemented |
| CAP-116 | Block definitions | Named groups of entities stored on the project; add/remove definitions and instances are validated, undoable and persist | `command::tests::add_block_def_pushes_onto_the_active_level`; `level::tests::block_defs_and_instances_survive_a_save_and_load` | — | implemented |
| CAP-117 | Block instance placement UI | `Tool::Block` places an instance on the canvas from an inspector-selected definition; the inspector lists definitions and instances | `apps/lintel/src/main.rs`; `apps/lintel/src/inspector.rs` | — | implemented |
| CAP-118 | Viewport print polish | Real PDF clipping at the viewport region, viewport-anchored north arrow and scale bar, placement guards with fallback | `pdf::tests::a_viewport_geometry_is_cut_by_a_real_clipping_path`, `each_viewport_carries_its_own_north_arrow_and_scale_bar` | — | implemented |
| CAP-119 | Constraint palette and solver readout | Constraint entries in the command palette and a read-only inspector "Constraint solver" section with count and validation status | `command_palette.rs`; `inspector.rs` | — | implemented |
| CAP-120 | Move a block instance | Change an instance's X/Y position from the inspector or command; non-finite positions rejected and the instance left unchanged | `command::tests::a_block_instance_can_be_moved`, `a_block_instance_position_must_be_finite`; `level::tests::block_instances_can_be_moved_rotated_and_flipped` | — | implemented |
| CAP-121 | Rotate a block instance | Change an instance's rotation; non-finite angles rejected | `command::tests::a_block_instance_can_be_rotated`, `a_block_instance_rotation_must_be_finite` | — | implemented |
| CAP-122 | Flip a block instance | Mirror an instance across its local x-axis | `command::tests::a_block_instance_can_be_flipped`; `block::tests::toggle_flip_toggles_the_mirror` | — | implemented |
| CAP-123 | Canvas hit-testing and click-to-select | Clicking the Select tool selects the topmost instance under the pointer, outlined in red; Esc clears | `Level::hit_test_block_instance` rejects a non-finite point and tests transformed definition bounds; `apps/lintel/src/main.rs` `hit_block_instance` | — | implemented |
| CAP-124 | Editable instance fields | Inspector "Block instance" section edits X/Y position, rotation and flip with honest empty states | `apps/lintel/src/inspector.rs` `block_instance_fields` | — | implemented |
| CAP-125 | Auto-select after an edit | `Applied.block_instance` reports the affected instance so the UI selects it after add/move/rotate/flip | `command::tests::the_block_instance_edits_can_be_undone_and_redone`; `apps/lintel/src/main.rs` | — | implemented |
| CAP-126 | Dimension label precision | The Inspector "Precision:" control (0–4 dp) changes how many decimal places the drawn dimension label shows; old files load at 0 | `command::tests::a_dimension_label_precision_can_be_set`, `a_dimension_label_precision_is_bounded`, `a_dimension_saved_before_label_precision_still_loads`; `apps/lintel/src/dimension_ui.rs` | — | implemented |
| CAP-127 | Delete a single block instance | Delete removes the selected canvas instance; an already-gone instance is refused with `BlockInstanceNotFound` and the refusal is reported in the status bar | `command::tests::a_block_instance_can_be_deleted_by_id`, `deleting_a_missing_block_instance_reports_it_gone`; `delete_selection` in `apps/lintel/src/main.rs` | — | implemented |
| CAP-128 | WCAG AA theme contrast gate | Muted, hint, valid-input and palette-grey text meets 4.5:1 on every surface it sits on; muted/hint text is slightly darker than before | `theme::tests::theme_colours_meet_wcag_aa` (`relative_luminance`, `contrast_ratio`) | guide II.30.7.1 | implemented |

## Explicitly out of scope for v0.1.0

| ID | Capability | Reason |
|---|---|---|
| CAP-100 | Joint resolution for unequal wall thicknesses | Was out of scope for v0.1.0 — **implemented in v0.2.0** |
| CAP-200 | Dimension lines with numeric labels | Was out of scope for v0.1.0 — **implemented in v0.2.0** |
| CAP-300 | SVG / image export | PDF only for v0.1.0 |
| CAP-400 | Web / WASM target | Desktop-only for v0.1.0 |
| CAP-500 | Plugin architecture | No plugin boundary designed |
| CAP-600 | Real-time collaboration | Single-user, offline-first |
| CAP-700 | Cross-project search | Not a drafting need at v0.1.0 |
| CAP-800 | Custom paper sizes | A4/A3/A2/A1 only in PDF |
| CAP-900 | GUI polish (empty/error states, keyboard shortcuts, accessibility) | Phase 7 polish; keyboard shortcuts exist minimally |

Readiness % (v0.1.0): **100% — shipped** (CI `41793150063`).
Readiness % (v0.2.0): **100% — shipped** (CI `41907396871`; release `0.2.0`).
Readiness % (v0.3.0): **100% — shipped** (CI `41998649019`; release `0.3.0`).
Readiness % (v0.4.0): **100% — shipped** (CI `42013256872`; release `0.4.0`).
Readiness % (v0.5.0): **100% — shipped** (CI `42099183213`; release `0.5.0`).
Readiness % (v0.6.0): **100% — shipped** (CI `42192427679`; release `0.6.0`).
Readiness % (v0.7.0): **100% — shipped** (CI `42211261713`; release `0.7.0`).
Readiness % (v0.8.0): **100% — shipped** (CI `42294376007`; release `0.8.0`).
Readiness % (v0.9.0): **100% — shipped** (release `0.9.0`).
Readiness % (v0.10.0): **100% — shipped** (release `0.10.0`).
Readiness % (v0.10.1): **100% — shipped** (release `0.10.1`).
Readiness % (v0.11.0): **100% — shipped** (tag `v0.11.0`, commit `803b590`).
Readiness % (v0.12.1): **100% — shipped** (Phase A; tag `v0.12.1`, rebuilt 2026-11-10).
Readiness % (v0.13.0): **100% — shipped** (CAP-100–CAP-108; first signed release).
Readiness % (v0.14.0): **100% — shipped** (CAP-111–CAP-115; release `v0.14.0`).
Readiness % (v0.15.0): **100% — shipped** (CAP-116–CAP-119; release `v0.15.0`).
Readiness % (v0.15.1): **100% — shipped** (macOS `.dmg` installer; release `v0.15.1`).
Readiness % (v0.16.0): **100% of CAP-120–CAP-125 — merged; release tag pending.**
Readiness % (v0.17.0): **100% of CAP-126–CAP-128 — merged; release tag pending.**