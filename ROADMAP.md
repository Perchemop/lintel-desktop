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

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.7.0

CI evidence: PR #10 run `42211261713`; release workflow run `42212698044`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-061 | Add / delete a storey from the Inspector | ✓ |
| CAP-062 | Rename the project from the Inspector | ✓ |
| CAP-063 | Switch Metric / Imperial units from the Inspector | ✓ |
| CAP-064 | Drag a dimension to change its offset | ✓ |
| CAP-065 | Command-reachability audit documented and clean | ✓ |

## v0.8.0 — Keyboard-focus review

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.8.0

CI evidence: PR #11 run `42294376007`; release workflow run `42295310359`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-066 | Shortcuts work while the canvas has keyboard focus | ✓ |
| CAP-067 | Level summary unit-tested (empty / singular / multi-storey) | ✓ |
| CAP-068 | Focus-review outcome recorded (tab order, visible focus, no trapping) | ✓ |

**Planned for v0.9.0:**
- Screen-reader output spot-check on Windows/macOS (manual, needs a host)
- Linux AppImage execution check on an x86_64 host

---

## v0.9.0 — Drawing Set

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.9.0

CI evidence: PR `opencode/v0.9.0-rooms` merged; release workflow run.

| ID | Capability | Done |
|---|-----------|------|
| CAP-069 | Auto-detected rooms via planar face traversal | ✓ |
| CAP-070 | Free text plan labels with placement, move, delete | ✓ |
| CAP-071 | Opening schedule generation | ✓ |
| CAP-072 | Multi-sheet PDF drawing set with cover, north arrow, scale bar | ✓ |
| CAP-073 | Room area captions in canvas | ✓ |
| CAP-074 | Label tool (L key), Ctrl+D duplicate, Ctrl+Shift+E Export | ✓ |

## v0.10.0 — Suite Standards Conformance

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.0

CI evidence: release workflow run `42365552699`.

| ID | Capability | Done |
|---|-----------|------|
| CAP-075 | `docs/tokens.json` CI-managed ledger | ✓ |
| CAP-076 | Release signing step in `release.yml` | ✓ |
| CAP-077 | `docs/development.md` added | ✓ |
| CAP-078 | Property-based geometry and command-layer fuzz tests | ✓ |
| CAP-079 | Version UI bottom-right with Release Notes popup + Update button | ✓ |
| CAP-080 | Window icon embedded | ✓ |

## v0.10.1 — Window Icon & Installer Version Fix

**Status:** Released — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.1

CI evidence: PR #16 audit fixes merged; PR #17 version bump.

| ID | Capability | Done |
|---|-----------|------|
| CAP-081 | Window icon via `with_icon()` + `image` crate | ✓ |
| CAP-082 | `resources/icons/lintel.svg` SVG source | ✓ |
| CAP-083 | Windows `.iss` version corrected 0.8.0 → 0.10.1 | ✓ |
| CAP-084 | Icon attribution in `ATTRIBUTION.md` | ✓ |
| CAP-085 | Command-layer fuzz tests (never panic/corrupt/undo) | ✓ |

## v0.11.0 — Measurements

**Status:** In Progress — measurement code is on `main`; release pending.

CI evidence: to be recorded when the 0.11.0 release workflow runs.

| ID | Capability | Done |
|---|-----------|------|
| CAP-086 | Area/perimeter query tool (Shift+M) with a selected-room readout | ✓ |
| CAP-087 | Wall length totals per level and for the whole project (Inspector → Measurements) | ✓ |
| CAP-088 | `format_area` honours `UnitSystem` (m² / ft² / in²) | ✓ |

**Release notes (pending publication):** Adds a Measurements tool that reports a
selected room's area and perimeter and swaps the Inspector to live level and
project totals. Wall lengths, opening widths, room counts, areas and perimeters
are shown in the project's Metric or Imperial units, and `format_area` now
formats square millimetres consistently as m², ft² or in². The release follows
guide §12.1: signed artefacts with `SHA256SUMS.txt`, and the CI-managed
`docs/tokens.json` ledger updated for 0.11.0 and quoted in the published notes.

**Readiness:** 90% — code and tests complete; release gate pending. The version
bump is not in this checkout (`apps/lintel/Cargo.toml` still reads 0.10.1).

## v0.11.1 — Phase A: Table Stakes (Layers + Construction Geometry)

**Status:** Planned

| ID | Capability | Done |
|---|-----------|------|
| CAP-089 | `Layer` model, protected default layer, per-entity layer assignment with serde migration | — |
| CAP-090 | Layers palette: create/delete, visibility, lock, assign selection | — |
| CAP-091 | Construction geometry (line, ray, circle): snap-enabled, never printed | — |
| CAP-092 | Snapping foundation: `SnapCandidateKind` midpoint/intersection/perpendicular, source and priority order | — |
| CAP-093 | Status bar: cursor coordinates, snap/ortho/grid chips, zoom percentage | — |

**Release notes (intended):** Introduces the table-stakes working environment of
a professional drafting tool: a layer system with a protected default layer and
per-entity assignment, construction geometry that guides drawing but never
prints, the candidate-and-priority foundation for enhanced snapping, and a
persistent status bar. Existing project files load unchanged through serde
defaults and a default-layer migration. Release gate follows guide §12.1
(signed artefacts, `SHA256SUMS.txt`, `docs/tokens.json` ledger).

**Readiness:** 0% — planned; design recorded in the 2026-11-08 elite CAD reports.

## v0.12.0 — Phase A continued (Snapping + Selection + Command Palette)

**Status:** Planned

| ID | Capability | Done |
|---|-----------|------|
| CAP-094 | Enhanced entity snapping: midpoint, intersection and perpendicular candidates with on-canvas markers | — |
| CAP-095 | Window and crossing selection in the core (`select_by_window`) with add/remove sets | — |
| CAP-096 | Command palette (Ctrl+K): command registry, fuzzy search, keyboard navigation | — |
| CAP-097 | Constraint-driven dimensions foundation: `DimensionAnchor`, witness-line gap/overrun, backward-compatible load | — |

**Release notes (intended):** Completes Phase A. Snapping gains midpoint,
intersection and perpendicular candidates; selection gains window and crossing
modes owned by the core; a searchable command palette exposes every action; and
dimensions become anchored to the geometry they measure instead of detached
points. Earlier files load through `DimensionAnchor::Free`. Release gate follows
guide §12.1 (signed artefacts, `SHA256SUMS.txt`, `docs/tokens.json` ledger).

**Readiness:** 0% — planned; depends on v0.11.1 layers, construction geometry and
the snapping foundation.

**Unscheduled / future:** Web/WASM port, plugin architecture, collaborative editing, custom paper sizes, better on-canvas hints.