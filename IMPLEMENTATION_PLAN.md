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

## Active Slice — v0.9.0 “Drawing Set”
**Status:** In progress → release pending CI green.

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

**Verification**
- `cargo test -p lintel-core` headless.
- CI `check` (fmt + clippy -D warnings) + `build-matrix` (Linux/macOS/Windows release).
- Manual try-steps after CI green.

## Next Slices
- v0.10.0 Suite Standards Conformance: `docs/tokens.json` ledger, bottom-right version UI, Release Notes popup with token metrics + Update button, README screenshots, guide Appendix references cleanup.
- v0.11.0 Measurements: area/perimeter query tool, wall length totals.
- v1.0.0 Pavement: polish, README screenshots, accessibility spot-checks, legal checklist.

## Dependencies
- `crates/update-check` and `crates/crash-report` are in-repo for now; guide §12.1 requires them as a shared git submodule. Tracked in DECISIONS.md D-012.
- Branch protection is documented in guide §II.25; repo currently has no protection. Follow PR flow pending protection being enabled.
