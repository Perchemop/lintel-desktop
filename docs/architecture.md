# Architecture

Lintel is split into a **thick, headless core** and a **thin UI shell** (addendum K.3):

```
apps/lintel/          UI shell (egui/eframe) — paint + input only
crates/lintel-core/   domain, commands, persistence, PDF export — fully tested
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
| `level` | One storey: walls, openings, extent |
| `project` | Versioned project metadata + levels; structural validation on load/save |
| `command` | All edits (`AddWall`, `PlaceOpening`, …); returns `Applied` |
| `document` | Project + undo/redo + dirty tracking + atomic save/load |
| `join` | Derived geometry: wall-band extensions at joints, door swings, hit test |
| `snap` | Snap-to-endpoint / ortho / grid for drawing input |
| `persistence` | Pretty JSON (`<name>.lintel.json`), atomic write via temp + rename |
| `pdf` | Hand-written PDF 1.7 sheet: border, title block, walls, openings, dimensions |

## Known v0.1.0 limitation (documented in `join.rs`)

Joints fill by extending each wall end by half the **joining** wall's thickness.
Exact for equal thicknesses; unequal walls can show a small step at the corner.
Scheduled for 0.2.0 (CAP-100 in `docs/scope.md`).
