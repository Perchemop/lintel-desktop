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

## Explicitly out of scope for v0.1.0

| ID | Capability | Reason |
|---|---|---|
| CAP-100 | Joint resolution for unequal wall thicknesses | Half-thickness fill is exact only for equal walls; documented in `join.rs` |
| CAP-200 | Dimension lines with numeric labels | Geometry exists; dimension aesthetics deferred |
| CAP-300 | SVG / image export | PDF only for v0.1.0 |
| CAP-400 | Web / WASM target | Desktop-only for v0.1.0 |
| CAP-500 | Plugin architecture | No plugin boundary designed |
| CAP-600 | Real-time collaboration | Single-user, offline-first |
| CAP-700 | Cross-project search | Not a drafting need at v0.1.0 |
| CAP-800 | Custom paper sizes | A4/A3/A2/A1 only in PDF |
| CAP-900 | GUI polish (empty/error states, keyboard shortcuts, accessibility) | Phase 7 polish; keyboard shortcuts exist minimally |

Readiness % (v0.1.0): **implemented, awaiting CI evidence**. Update after the
v0.1.0 CI run is green.
