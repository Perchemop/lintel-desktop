# Lintel keyboard shortcuts

Generated from code on 2026-11-16 (`apps/lintel/src/main.rs` keyboard block and
`apps/lintel/src/command_palette.rs` `PaletteAction::ALL`).

## Wired in main.rs (keyboard block)

| Key | Action | Where |
|-----|--------|-------|
| Ctrl+N | New plan | main.rs keyboard block |
| Ctrl+O | Open plan | main.rs keyboard block |
| Ctrl+S | Save | main.rs keyboard block |
| Ctrl+Shift+S | Save As | main.rs keyboard block |
| Ctrl+E | Export sheet | main.rs keyboard block |
| Ctrl+Shift+E | Export drawing set | main.rs keyboard block |
| Ctrl+A | Select all | main.rs keyboard block |
| Z / Ctrl+Z | Undo | main.rs keyboard block |
| Y / Ctrl+Y | Redo | main.rs keyboard block |
| Delete / Backspace | Delete selection (or selected block instance) | main.rs keyboard block |
| Escape | Cancel draft / clear selection | main.rs keyboard block |
| Ctrl+D | Duplicate selection | main.rs keyboard block |
| W | Wall tool | main.rs keyboard block |
| S | Select tool | main.rs keyboard block |
| D | Door tool | main.rs keyboard block |
| N | Window tool | main.rs keyboard block |
| M | Dimension tool | main.rs keyboard block |
| Shift+M | Measurements tool | main.rs keyboard block |
| L | Label tool | main.rs keyboard block |
| C | Construction line tool | main.rs keyboard block |
| Shift+C | Construction ray tool | main.rs keyboard block |
| Q | Construction circle tool | main.rs keyboard block |
| X | Construction arc tool | main.rs keyboard block |
| Shift+V | Toggle construction visibility | main.rs keyboard block |
| F | Zoom to fit | main.rs keyboard block |
| Ctrl+K | Command palette | main.rs ui() (global) |

## Palette navigation

| Key | Action | Where |
|-----|--------|-------|
| ↑ / ↓ | Move highlight | command_palette.rs search_ui |
| Enter | Run highlighted command | command_palette.rs search_ui |
| Escape | Close palette / cancel form | command_palette.rs show |

## Palette shortcut hints (PaletteAction::ALL, non-empty)

| Key | Action |
|-----|--------|
| S | Select tool |
| W | Wall tool |
| D | Door tool |
| N | Window tool |
| M | Dimension tool |
| L | Label tool |
| Shift+M | Measurements tool |
| Ctrl+N | New plan |
| Ctrl+O | Open plan |
| Ctrl+S | Save plan |
| Ctrl+Shift+S | Save plan as |
| Ctrl+E | Export sheet |
| Ctrl+Shift+E | Export drawing set |
| Ctrl+Z | Undo |
| Ctrl+Y | Redo |
| Del | Delete selection / Delete construction / Remove constraint |
| Ctrl+D | Duplicate selection |
| Ctrl+A | Select all |
| Esc | Clear selection |
| F | Zoom to fit |
| E | Snap: endpoint |
| V | Snap: midpoint |
| I | Snap: intersection |
| P | Snap: perpendicular |
| T | Snap: tangent |
| G | Toggle grid |
| O | Toggle orthogonal drawing |
| U | Switch Metric / Imperial |
| C | Construction line tool |
| Shift+C | Construction ray tool |
| Q | Construction circle tool |
| X | Construction arc tool |
| Shift+V | Construction visibility |
| Ctrl+Shift+P | Add Perpendicular Distance Constraint |
Why empty (removed): ShowAbout (?), ShowReleaseNotes (R), CheckForUpdates (U), ViewConstraints (Shift+K) — no wired handler.