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
| CAP-024 | Project validation | ✓ |
| CAP-025 | Command apply returns `Applied` | ✓ |

**Release notes:** First functional test build (v0.1.0); limited scope: walls, doors, windows, levels, save/load, PDF export. Joint resolution for unequal thicknesses is a known limitation scheduled for 0.2.x.

## v0.2.0 — Joint Resolution & Dimensions

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.2.0

CI evidence: PR #4 run `41907396871` (`check` + `build-matrix` on Linux/macOS/Windows).

| ID | Capability | Done |
|---|-----------|------|
| CAP-021 | Wall geometry with joint extensions (unequal thickness) | ✓ |
| CAP-022 | Door/window hit testing | ✓ |
| CAP-026 | Dimension lines (aligned, horizontal, vertical) | ✓ |
| CAP-027 | Dimension placement, move, delete | ✓ |
| CAP-028 | Dimension offset editing | ✓ |
| CAP-029 | Dimension hit testing (priority over walls) | ✓ |
| CAP-030 | Dimension validation (zero-length, negative offset) | ✓ |
| CAP-031 | Dimension undo/redo | ✓ |
| CAP-032 | Dimension persistence (save/load) | ✓ |
| CAP-033 | Dimension hit testing priority (over walls) | ✓ |
| CAP-023 | Unit system (Metric/Imperial) for dimensions | ✓ |
| CAP-040 | Crash reporting (panic hook, scrubbed diagnostics) | ✓ |
| CAP-041 | Update check (GitHub Releases, fail-soft offline) | ✓ |
| CAP-042 | In-app release notes with token metrics | ✓ |
| CAP-043 | Cross-platform CI build matrix | ✓ |
| CAP-044 | Multi-platform release artefacts | ✓ |
| CAP-045 | User-facing README for non-technical users | ✓ |

**Planned for v0.2.x:**
- Dimension kind selection (aligned / horizontal / vertical) in the UI
- Dimension extension-line toggle
- Dimension label precision and unit formatting
- Dimension snap to wall endpoints and openings
- Rotated labels for vertical/aligned dimensions

## v0.3.0 — Error Feedback & macOS Bundle

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.3.0

CI evidence: PR #6 run `41998649019`; release workflow run `41999766800`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-046 | Empty-state hint on a level with no walls | ✓ |
| CAP-047 | Rejected edits and file errors shown in the app | ✓ |
| CAP-048 | macOS release ships a `Lintel.app` bundle | ✓ |

## v0.4.0 — Accessibility labels, keyboard shortcuts, AppImage

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.4.0

CI evidence: PR #7 run `42013256872`; release workflow run `42024620827`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-049 | Keyboard shortcut hints + hover labels on all controls | ✓ |
| CAP-050 | Focus-ring contrast WCAG AA (COBALT) | ✓ |
| CAP-051 | Linux AppImage release artefact | ✓ |

## v0.5.0 — Windows installer

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.5.0

CI evidence: PR #8 run `42099183213`; release workflow run `42103705669`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-052 | Windows installer (Inno Setup via Chocolatey on CI) | ✓ |
| CAP-053 | `.iss` DefaultDirName typo fixed (`{autpf}` → `{autopf}`) | ✓ |
| CAP-054 | `.iss` AppId brace escaped + Windows `.ico` icon | ✓ |

## v0.6.0 — Keyboard-complete flows, accessible canvas

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.6.0

CI evidence: PR #9 run `42192427679`; release workflow run `42193835276`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-055 | File keyboard shortcuts (Ctrl+N/O/S/Shift+S/E) | ✓ |
| CAP-056 | Focusable canvas with a screen-reader level summary | ✓ |
| CAP-057 | Level summary shown in the toolbar | ✓ |
| CAP-058 | Tool shortcuts guarded against Ctrl/Cmd/Alt | ✓ |
| CAP-059 | PDF export dialog reachable from the UI (bug fix) | ✓ |
| CAP-060 | Linux AppImage static verification recorded | ✓ |

## v0.7.0 — Reachable capabilities

**Status:** In progress — implemented; CI verification and tag pending.

| ID | Capability | Done |
|---|-----------|------|
| CAP-061 | Add / delete a storey from the Inspector | ✓ |
| CAP-062 | Rename the project from the Inspector | ✓ |
| CAP-063 | Switch Metric / Imperial units from the Inspector | ✓ |
| CAP-064 | Drag a dimension to change its offset | ✓ |
| CAP-065 | Command-reachability audit documented and clean | ✓ |

**Planned for v0.8.0:**
- Screen-reader output spot-check on Windows/macOS (manual, needs a host)
- Linux AppImage execution check on an x86_64 host
- Keyboard-focus management review

---

**Unscheduled / future:** Web/WASM port, plugin architecture, collaborative editing, custom paper sizes, better on-canvas hints.