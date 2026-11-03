# Scope

This document defines what is **in scope** for the first major release (v0.1.0) and what is explicitly **out of scope** until a later milestone.

## In Scope — v0.1.0

| ID | Capability | Acceptance Criteria |
|---|-----------|---------------------|
| CAP-001 | Create a new project with a name | `Project::new("My Plan", epoch)` creates a project with one empty "Level 0" |
| CAP-002 | Add a wall | `Command::AddWall` adds a wall; visible in `PlanGeometry::of_level()` |
| CAP-003 | Move a wall | `Command::MoveWall` changes wall endpoints; geometry updates accordingly |
| CAP-004 | Set wall thickness | `Command::SetWallThickness` changes wall thickness; validation rejects impossible values |
| CAP-005 | Set wall height | `Command::SetWallHeight` changes wall height; validation rejects impossible values |
| CAP-006 | Delete a wall | `Command::DeleteWall` removes a wall; geometry recomputes |
| CAP-007 | Place a door opening | `Command::PlaceOpening` with `OpeningKind::Door` adds a door; fits-on validation works |
| CAP-008 | Place a window opening | `Command::PlaceOpening` with `OpeningKind::Window` adds a window; fits-on validation works |
| CAP-009 | Move an opening offset | `Command::MoveOpening` changes opening offset on its host wall |
| CAP-010 | Set opening size | `Command::SetOpeningSize` changes door/window width/height/sill |
| CAP-011 | Flip opening handedness | `Command::FlipOpening` changes door handedness |
| CAP-012 | Delete an opening | `Command::DeleteOpening` removes an opening |
| CAP-013 | Add a level | `Command::AddLevel` adds a new storey with a given name and elevation |
| CAP-014 | Delete a level | `Command::DeleteLevel` removes a level (walls open on it are removed too) |
| CAP-015 | Set active level | `Command::SetActiveLevel` switches the currently visible level |
| CAP-016 | Rename the project | `Command::RenameProject` changes the project name |
| CAP-017 | Save project to JSON | `Document::save_as(path)` writes valid JSON; `Document::open(path)` re-reads it round-trip |
| CAP-018 | Undo history | `Document::undo()` reverts the last command; `Document::redo()` re-applies it |
| CAP-019 | Redo history | `Document::redo()` re-applies an undone command |
| CAP-020 | PDF plan rendering | `render_plan()` produces a valid PDF%PDF-1.7 ... %%EOF containing the plan, title block, and scale label |
| CAP-021 | Wall geometry with joint extensions | `PlanGeometry` resolves wall end extensions; corners are drawn correctly |
| CAP-022 | Door/window hit testing | `PlanGeometry::hit_test()` correctly identifies which opening is under the pointer |
| CAP-023 | Unit system (Metric/Imperial) | `UnitSystem::label()` and `UnitSystem::format()` display lengths in chosen units |
| CAP-024 | Project validation | `Project::validate()` rejects invalid states (duplicate ids, missing hosts, overlapping openings, stale identity counter) |
| CAP-025 | Command apply returns `Applied` | `Command::apply_command()` returns `Applied { wall, opening, level }` with the newly created entity |

## Explicitly Out of Scope — v0.1.0

| ID | Reason |
|---|--------|
| CAP-100 | Joint resolution for unequal wall thicknesses (documented limitation in `join.rs`; scheduled for 0.2.x) |
| CAP-200 | Dimension lines with numeric labels (basic geometry is in; dimension aesthetics are 0.2.x) |
| CAP-300 | Full SVG or image export (PDF only for 0.1.0) |
| CAP-400 | Web/WASM target (desktop-only for 0.1.0) |
| CAP-500 | Plugin architecture or shared suite crates beyond `crash-report` + `update-check` |
| CAP-600 | Real-time collaborative editing (single-user, offline-first) |
| CAP-700 | Advanced search/filter across levels or projects |
| CAP-800 | Custom paper sizes beyond A4/A3/A2/A1 in PDF rendering |
| CAP-900 | GUI polish: empty states, error dialogs, keyboard shortcuts, accessibility (these are Phase 7+) |

**Readiness % (v0.1.0):** 85% — all v0.1.0 acceptance criteria are implemented and verified by headless CI tests.

**Readiness % calculation:** Only counts from verified CI evidence + local builds. Roadmap percentages move only from evidence.