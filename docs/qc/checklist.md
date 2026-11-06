# Accessibility smoke test checklist

Run this checklist whenever the UI is touched (guide II.21.4). Record each run
in the run-log table below. Screen-reader checks are manual (Narrator /
NVDA / VoiceOver) — there is no automated CI check for screen-reader output
(see ACCESSIBILITY.md).

## 1. Keyboard-only walk-through

Goal: complete the full create → edit → undo → save → export flow without a
mouse.

| Step | Action | Pass? | Notes |
|------|--------|-------|-------|
| 1 | Press **W** to activate the Wall tool | | |
| 2 | Click twice (keyboard-driven click or tap) to draw a wall | | |
| 3 | Press **D**, then click a wall to place a door | | |
| 4 | Press **M**, then click twice to place a dimension | | |
| 5 | Tab to the Inspector; edit thickness, height, kind, offset | | |
| 6 | Press **Z** (Undo) to remove the dimension | | |
| 7 | Press **Y** (Redo) to restore it | | |
| 8 | Select an item and press **Delete** to remove it | | |
| 9 | Press **Ctrl+S**; confirm the plan saves | | |
| 10 | Press **Ctrl+E**; set paper/scale/sheet/title; export | | |
| 11 | Press **Ctrl+N** for a new plan, then **Ctrl+O** to reopen the saved one | | |
| 12 | Press **F** to zoom to fit | | |
| 13 | Press **Escape** to cancel any active draft / clear selection | | |
| 14 | Tab to the canvas; confirm the level summary is announced | | |
| 15 | Inspector → Storeys: add a storey, then delete it | | |
| 16 | Inspector → Project: rename the plan, then switch Metric ↔ Imperial | | |
| 17 | Select a dimension and drag it; confirm the dimension line moves | | |

## 2. Dialog Escape dismissal

| Dialog | Closes on Escape? | Notes |
|--------|-------------------|-------|
| About Lintel | | |
| Release notes | | |
| Update available | | |
| Export sheet | | |

## 3. Screen-reader spot check (manual)

| Screen reader / platform | Tool names announced? | Inspector labels announced? | Focus visible? | Notes |
|--------------------------|-----------------------|-----------------------------|----------------|-------|
| Narrator (Windows) | | | | |
| NVDA (Windows) | | | | |
| VoiceOver (macOS) | | | | |

## 4. Reflow at 200% / narrow window

| Check | Pass? | Notes |
|-------|-------|-------|
| UI readable at 200% system font / DPI scaling | | |
| Narrow window (~700 px wide) — no clipped controls | | |
| Shortcuts line still visible and on one line | | |

## 5. High-contrast check (both themes)

Lintel has one theme; check against both light panel backgrounds and the
canvas PAPER background.

| Check | Pass? | Notes |
|-------|-------|-------|
| Focus ring visible and contrasting on canvas background | | |
| Focus ring visible and contrasting on panel backgrounds | | |
| Shortcuts hint legible (grey on background) | | |
| All text meets at least WCAG AA contrast | | |

## 6. Linux AppImage end-user check

Download `lintel-appimage-x86_64.AppImage` from the latest release and, on an
x86_64 Linux machine:

| Check | Pass? | Notes |
|-------|-------|-------|
| `chmod +x lintel-appimage-x86_64.AppImage` then run it — window opens | | |
| The embedded desktop entry and icon are present (`--appimage-extract`) | | |
| `sha256sum` matches the release `SHA256SUMS.txt` | | |

Static verification (no x86_64 host required): the published AppImage is a
valid ELF (`7f 45 4c 46`), contains a SquashFS payload (`hsqs` magic), and its
SHA-256 matches `SHA256SUMS.txt`. Execution was verified on an x86_64 host only
where available; the ARM development machine cannot execute the x86_64 binary.

## 7. Command-reachability audit (no fake functionality)

Every `lintel_core::Command` variant must be bound somewhere in `apps/lintel`,
or be explicitly documented as core-only. Run this from the repository root:

```bash
for v in AddWall MoveWall SetWallThickness SetWallHeight DeleteWall \
         PlaceOpening MoveOpening SetOpeningKind SetOpeningSize FlipOpening \
         DeleteOpening PlaceDimension MoveDimension SetDimensionOffset \
         SetDimensionKind SetDimensionExtensionLines DeleteDimension \
         AddLevel DeleteLevel SetActiveLevel SetUnits RenameProject; do
  n=$(grep -rn "Command::$v" apps/lintel/src/ | wc -l)
  [ "$n" -eq 0 ] && echo "UNREACHABLE: $v"
done
```

| Check | Pass? | Notes |
|-------|-------|-------|
| No variant reports `UNREACHABLE` except `MoveDimension` | | `MoveDimension` is deliberately core-only (DECISIONS.md D-026) |

## Run log

| Date | Tester | Result | Notes |
|------|--------|--------|-------|
| 2026-11-06 | agent (static) | partial | AppImage ELF + SquashFS + SHA-256 verified; GUI execution needs an x86_64 host |
