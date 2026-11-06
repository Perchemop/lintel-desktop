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
| CAP-021 | Joint resolution for unequal wall thicknesses | Walls of different thicknesses close cleanly at outside corners | `resolve_wall` extends each end by half the *other* wall's thickness; tests pass | — | implemented |
| CAP-026 | Dimension lines | Aligned/horizontal/vertical dimension between two points with an offset | `Command::PlaceDimension`; `PlanGeometry` reports it | — | implemented |
| CAP-027 | Dimension placement / move / delete | Two clicks place, dragging moves the line, Delete removes | `PlaceDimension`, `SetDimensionOffset` (drag), `DeleteDimension` | — | implemented |
| CAP-028 | Dimension offset editing | Inspector changes the offset; zero/negative rejected | `Command::SetDimensionOffset` validation | — | implemented |
| CAP-029 | Dimension hit testing | Clicking a dimension selects it; priority over the wall band | `PlanGeometry::hit_test` checks dimensions first | — | implemented |
| CAP-030 | Dimension validation | Zero-length and non-positive offset rejected with plain messages | Command validation strings | — | implemented |
| CAP-031 | Dimension undo/redo | Undo restores the previous state; redo re-applies | `Document::undo` / `redo` for dimension commands | — | implemented |
| CAP-032 | Dimension persistence | Save/load preserves dimensions | `Document::save_as` / `open` round-trip | — | implemented |
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
Readiness % (v0.8.0): **~85% — implemented; CI verification and tag pending.**

Still planned for v0.7.0: screen-reader output spot-checks on Windows/macOS,
Linux AppImage execution on an x86_64 host, keyboard-focus management review.
