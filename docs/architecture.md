# Architecture

Lintel is split into a **thick, headless core** and a **thin UI shell** (guide Appendix K.3):

```
apps/lintel/          UI shell (egui/eframe) — paint + input only
crates/lintel-core/   domain, commands, persistence, PDF export — fully tested
crates/update-check/  shared: GitHub Releases check, fail-soft offline   (guide §7)
crates/crash-report/  shared: panic hook, scrubbed diagnostics           (guide §7)
```

## Layering rule

- The UI never mutates the model. It builds `lintel_core::Command` values and
  hands them to `Document::apply(command, now_epoch_s)`.
- `Document` applies the command to a clone of the project and commits the
  clone only when the command succeeds, so a rejected edit cannot leave the
  project half-changed. Every change is recorded as a project snapshot on an
  undo stack (128 deep).
- `crates/lintel-core` does **not** depend on `eframe`, `egui`, `rfd`, or any
  GUI crate, so `cargo test -p lintel-core` runs without a display or GPU.

## Module map (`lintel-core`)

| Module | Responsibility |
|---|---|
| `ids` | Typed identifiers (`WallId`, `OpeningId`, `LevelId`) — allocation-ordered u64 |
| `units` | Millimetre model; Metric/Imperial display and forgiving length parsing |
| `geom` | `Point2`, `Vec2`, `Bounds`, segment projections, arc polylines |
| `wall` | Wall centreline + thickness + height; band footprint; validity bounds |
| `opening` | Door/window hosted on a wall: offset, size, sill, flip; fit-on checks |
| `dimension` | Linear dimension line between two points: kind, offset, label length |
| `level` | One storey: walls, openings, dimensions, extent |
| `project` | Versioned project metadata + levels; structural validation on load/save |
| `command` | All edits (`AddWall`, `PlaceOpening`, …); returns `Applied` |
| `document` | Project + undo/redo + dirty tracking + atomic save/load |
| `join` | Derived geometry: wall-band extensions at joints, door swings, hit test |
| `snap` | Snap-to-endpoint / ortho / grid for drawing input |
| `persistence` | Pretty JSON (`<name>.lintel.json`), atomic write via temp + rename |
| `pdf` | Hand-written PDF 1.7 sheet: border, title block, walls, openings, dimensions |

## Suite infrastructure (guide §7)

- `update-check` is UI-free and blocking; the shell runs a check on a worker
  thread and picks the result up through a channel, so the UI never stalls. Any
  network, HTTP or parse failure becomes a quiet "could not check" state.
- `crash-report` installs a panic hook that writes a scrubbed JSON log and
  restores the previous hook. Nothing is ever sent automatically; the only
  submission path is a pre-filled GitHub "New Issue" URL the user opens.

## Joint resolution (v0.2.0)

Joints fill by extending each wall end by half the **joining** wall's thickness,
so outside corners close cleanly even when the two walls differ in thickness.
Covered by the unequal-thickness tests in `join.rs`.
