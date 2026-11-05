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
| 9 | Open File → Save; confirm the plan saves | | |
| 10 | Open File → Export PDF; set paper/scale/sheet/title; export | | |
| 11 | Press **F** to zoom to fit | | |
| 12 | Press **Escape** to cancel any active draft / clear selection | | |

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

## Run log

| Date | Tester | Result | Notes |
|------|--------|--------|-------|
| | | | |
