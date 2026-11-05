# Lintel accessibility statement

Lintel targets **WCAG 2.2 Level AA where feasible** for a native desktop
application. egui is an immediate-mode toolkit, so a few web-oriented criteria
cannot be met exactly; where a criterion cannot be met, the limitation is
recorded here honestly rather than glossed over (guide II.21.1).

## What works

- **Keyboard tool switching:** W = Wall, S = Select, D = Door, N = Window,
  M = Dimension — from anywhere a control is not focused.
- **Undo/redo:** Z / Y (Ctrl+Z / Ctrl+Y on the Edit menu too).
- **Delete:** Delete or Backspace removes the selected wall, opening or
  dimension.
- **Zoom:** F = zoom to fit; scroll-wheel zooms; middle-drag pans.
- **Inspector via Tab:** the Inspector panel (wall thickness/height, opening
  kind/width/height/sill/position, dimension kind/offset/extension lines) is
  reachable with Tab and each field carries its name as a label plus a
  hover text explaining units and ranges.
- **Escape dismissal:** the About, Release notes, Export sheet and Update
  windows all close on Escape (egui `Window` with `.open()` handles Escape by
  default; none override it).
- **Screen-reader semantics:** egui's AccessKit integration exposes a semantic
  tree (roles, names, values) on supported platforms — Windows and macOS
  today. Interactive widgets carry meaningful names.
- **Focus rings:** egui draws focus rings around keyboard-focused widgets by
  default. The ring colour is the theme's COBALT accent, which meets WCAG AA
  contrast (~8.3:1 on the canvas PAPER background, ~7.8:1 on panel
  backgrounds). Focus indication is never removed.
- **Shortcut hints:** a single grey "Shortcuts:" line under the tool selector
  lists the keyboard bindings.

## Limitations (WCAG 2.2 AA, "where feasible")

- **Canvas objects have no per-object ARIA-equivalent labels.** Walls,
  openings and dimensions are drawn by a custom painter on the canvas, so a
  screen reader cannot announce individual plan objects by name or position.
  Selection state is conveyed through the Inspector instead.
- **No high-contrast theme toggle.** There is one warm "paper and ink" theme;
  system high-contrast colours are not independently honoured.
- **Reduced motion is not applicable.** Lintel has no animations, so there is
  nothing to disable for the reduced-motion preference.
- **Screen-reader verification is manual only.** There is no automated CI
  check for Narrator/NVDA/VoiceOver output; spot checks are recorded in the
  QC run log (docs/qc/checklist.md).

## Keyboard shortcut table

| Action | Shortcut | Notes |
|--------|----------|-------|
| Wall tool | W | Click twice to draw a wall |
| Select tool | S | Click to select walls, openings, dimensions |
| Door tool | D | Click a wall to place a door |
| Window tool | N | Click a wall to place a window |
| Dimension tool | M | Click twice to place a dimension |
| Undo | Z / Ctrl+Z | From the Edit menu too |
| Redo | Y / Ctrl+Y | From the Edit menu too |
| Delete selection | Delete / Backspace | Removes the selected item |
| Zoom to fit | F | From the View menu too |
| Cancel draft / clear selection | Escape | Also dismisses open dialogs |
| Toggle grid | G | Canvas background grid |
| Toggle orthogonal drawing | O | Constrain walls to horizontal/vertical |

## Getting help / reporting an accessibility issue

Accessibility issues are tracked with the same seriousness as other bugs.
Use the accessibility issue template
(`.github/ISSUE_TEMPLATE/accessibility.yml`) or open a GitHub Issue and label
it `accessibility`. Include your platform, screen reader (if any), and the
keyboard flow that is blocked.

Lintel is free and offline-first; your plan stays on your disk. Licensed
MIT OR Apache-2.0.
