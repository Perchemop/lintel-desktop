# Lintel — structured reference

Generated for team pre-loading (guide §5.7.2): teammates query this instead of
re-reading raw source. Regenerate when architecture changes.

## Architecture

- **`crates/lintel-core`** — pure domain: commands, project/level model,
  geometry, constraints, dimensions, blocks, PDF export. Headless-tested; must
  never depend on the GUI toolkit (guide §17).
- **`apps/lintel`** — egui/eframe shell: canvas, inspector, command palette,
  status bar, theme. Sends `Command` values to core; never mutates the model.
- **`crates/crash-report`**, **`crates/update-check`** — shared suite crates,
  git submodules (D-029).

## Key modules

| Path | Role |
|------|------|
| `crates/lintel-core/src/command.rs` | Every user edit as a `Command` variant; exhaustive `apply_command` |
| `crates/lintel-core/src/level.rs` | Storey model: walls, openings, dimensions, blocks, constraints |
| `crates/lintel-core/src/dimension.rs` | Anchored dimensions, kinds, label precision |
| `crates/lintel-core/src/theme.rs` → `apps/lintel/src/theme.rs` | Three-tier tokens (BASE → semantic roles → component); WCAG AA gate test |
| `apps/lintel/src/canvas.rs` | Custom painter; hit-testing; snap markers via `theme::snap_marker_colour` |
| `apps/lintel/src/inspector.rs` | Property panels; every `Command` variant bound here or in palette (D-027) |
| `apps/lintel/src/command_palette.rs` | Ctrl+K palette: 49 actions, forms, fuzzy match, recents |

## Workflows

- **Draw:** wall → door/window → dimension → constraint (two-click flows)
- **Blocks:** define → place → select (click or Ctrl+K) → move/rotate/flip/delete
- **Export:** sheet PDF (Ctrl+E) or drawing set (Ctrl+Shift+E), title block + viewports
- **Persist:** JSON at a fixed workflow path (D-025), atomic write → rename

## Conventions

- Commands: `ci.sh run "<type>(vX.Y.Z): <summary>"` — Conventional Commits.
- Required checks: `.ci-required.txt` (check, build-matrix ×3, kittest,
  pr-author-is-dev).
- Every capability: CAP id in `docs/capability-inventory.md`, evidence in
  `docs/parity.md`, trace in `REQUIREMENTS_TRACEABILITY.md`.
