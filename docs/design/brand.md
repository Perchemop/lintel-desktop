# Brand — Lintel

## Meaning

A lintel is the beam that spans an opening in a wall and carries the load
above it. The name says what the app does: walls with openings, drawn honestly.
It is one word, easy to say and spell (LIN-tel), culture-neutral, and it does
not imply affiliation with any commercial CAD product.

## Wordmark

Text only, set in the UI font: "Lintel". No competitor logomarks are copied or
referenced.

## Colour tokens

All original, drafted for warm paper on screen:

| Token | Use | Value |
|---|---|---|
| `paper` | Canvas background | #F8F8F2 (warm off-white) |
| `ink` | Text, lines, door leaves | #1A1816 (near-black graphite) |
| `ink-dim` | Secondary text | #706A62 |
| `cobalt` | Selection, active tool | #124E78 |
| `amber` | Ortho / grid hints | #D38600 |
| `green` | Endpoint snap | #229E60 |
| `wall-fill` | Wall band fill | #E2DED6 |
| `wall-stroke` | Wall band outline | #544E46 |
| `grid-minor` / `grid-major` | Drawing grid | #E9E2D8 / #C8BEB0 |
| `panel-bg` | Panels | #F5F0E8 |
| `panel-border` | Panel borders | #D4CCC0 |
| `status-bg` | Status bar | #EFE8DC |

## Shapes

- Rectangular sheet with subtle margins; wall bands are filled rectangles
  with hairline outlines.
- Doors: jamb lines + leaf line + open swing arc.
- Windows: glass lines across the reveal.
- Snap markers: small filled circles at wall ends.

## Motion

- Pan (middle drag), zoom at cursor (scroll), snap preview while drawing.
- No decorative animation; reduced motion is honoured by keeping transitions
  instant.

## Attribution

All marks are agent-authored originals. Third-party typefaces: none (egui
default fonts). Record any future icon/font in `ATTRIBUTION.md`.
