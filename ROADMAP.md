# Roadmap — Lintel

## v0.1.0 — First Functional Test Build (Hard Gate)

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.1.0

| ID | Capability | Done |
|---|-----------|------|
| CAP-001 | Create a new project with a name | ✓ |
| CAP-002 | Add a wall | ✓ |
| CAP-003 | Move a wall | ✓ |
| CAP-004 | Set wall thickness | ✓ |
| CAP-005 | Set wall height | ✓ |
| CAP-006 | Delete a wall | ✓ |
| CAP-007 | Place a door opening | ✓ |
| CAP-008 | Place a window opening | ✓ |
| CAP-009 | Move an opening offset | ✓ |
| CAP-010 | Set opening size | ✓ |
| CAP-011 | Flip opening handedness | ✓ |
| CAP-012 | Delete an opening | ✓ |
| CAP-013 | Add a level | ✓ |
| CAP-014 | Delete a level | ✓ |
| CAP-015 | Set active level | ✓ |
| CAP-016 | Rename the project | ✓ |
| CAP-017 | Save project to JSON round-trip | ✓ |
| CAP-018 | Undo history | ✓ |
| CAP-019 | Redo history | ✓ |
| CAP-020 | PDF plan rendering | ✓ |
| CAP-021 | Wall geometry with joint extensions | ✓ |
| CAP-022 | Door/window hit testing | ✓ |
| CAP-023 | Unit system (Metric/Imperial) | ✓ |
| CAP-024 | Project validation | ✓ |
| CAP-025 | Command apply returns `Applied` | ✓ |

**Release notes:** First functional test build (v0.1.0); limited scope: walls, doors, windows, levels, save/load, PDF export. Joint resolution for unequal thicknesses is a known limitation scheduled for 0.2.x.

## v0.2.0 — Joint Resolution & Dimensions

**Planned capabilities:**
- Joint resolution for unequal wall thicknesses
- Dimension lines with numeric labels
- Full undo/redo persistence across saves

## v0.3.0 — Multi-Platform & Polish

**Planned capabilities:**
- Windows installer
- macOS .app bundle
- Linux AppImage
- Full accessibility (keyboard navigation, screen reader support)
- Empty states, error handling, keyboard focus management

---

**Unscheduled / future:** Web/WASM port, plugin architecture, collaborative editing, custom paper sizes, advanced rendering (dimensions, hatching).