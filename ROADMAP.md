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

**Readiness:** 100% — released as v0.11.1 (tag `v0.11.1` pending). All measurements slice capabilities delivered.

## v0.11.1 — Phase A continued: Snapping + Selection + Command Palette

**Status:** **Complete** (PR #21 merged, CI green)

| ID | Capability | Done |
|---|-----------|------|
| CAP-094 | Enhanced entity snapping (endpoint, midpoint, intersection, perpendicular, tangent, quadrant, grid, ortho) with priority ordering | ✅ |
| CAP-095 | Window/crossing/fence selection modes | ✅ |
| CAP-096 | Selection filter by entity type and layer | ✅ |
| CAP-097 | Command palette (Ctrl+K, fuzzy match, recent commands) | ✅ |
| CAP-098 | Snap visual indicators on canvas | ✅ |

**Foundation already on the branch:** the core layer model and construction
geometry (`crates/lintel-core/src/layer.rs`, `construction.rs`) and the
persistent status bar (`apps/lintel/src/status_bar.rs`) landed earlier under the
Phase A plan (CAP-089–CAP-093); their UI wiring (layers palette, construction
tools) still follows.

**Release notes (intended):** Completes the Phase A working environment of a
professional drafting tool. Snapping grows from endpoints to midpoint,
intersection, perpendicular, tangent and quadrant candidates — plus grid and
ortho — resolved through an explicit priority order and shown with on-canvas
indicators; selection gains window, crossing and fence modes owned by the core
and filtered by entity type and layer; and a searchable command palette
(Ctrl+K) exposes every action with fuzzy matching and a recent-commands list.
Earlier project files load unchanged. Release gate follows guide §12.1 (signed
artefacts, `SHA256SUMS.txt.asc`, `docs/tokens.json` ledger).

**Readiness:** 0% — starting the Phase A continued work (CAP-094–CAP-098).

## v0.12.0 — Phase A continued (Snapping + Selection + Command Palette)

**Status:** Published as **0.12.1** — the Phase A content (enhanced snapping,
selection modes, command palette, layers, construction geometry, status bar)
was released under tag `v0.12.1` (commit `803b590`, rebuilt 2026-11-10 with
correct version stamps). The constraint-dimension rows below (CAP-109, CAP-110)
are **not in that release**; they move to v0.13.0 as CAP-100/CAP-103. This
section is retained for history; the collision where it reused CAP-097/098
(already consumed by v0.11.1's command palette and snap indicators) is resolved
by renumbering.

| ID | Capability | Done |
|---|-----------|------|
| CAP-109 | Constraint-driven dimensions foundation: `DimensionAnchor`, witness-line gap/overrun, backward-compatible load | — |
| CAP-110 | Dimension anchor UI: click-to-place witness lines, grip editing | — |
| CAP-099 | Drawing set / sheet layout with title blocks and viewport linking — scope moved to v0.14.0 as CAP-113/114 (D-030 numbering) | — |

**Release notes (intended):** Completes Phase A. Snapping gains midpoint,
intersection and perpendicular candidates; selection gains window and crossing
modes owned by the core; a searchable command palette exposes every action; and
dimensions become anchored to the geometry they measure instead of detached
points. Earlier files load through `DimensionAnchor::Free`. Release gate follows
guide §12.1 (signed artefacts, `SHA256SUMS.txt`, `docs/tokens.json` ledger).

**Readiness:** 0% — planned; depends on v0.11.1 layers, construction geometry and
the snapping foundation.

## v0.13.0 — Anchored Dimensions & Release Governance

**Status:** Planned — v0.12.1 shipped Phase A (enhanced snapping, selection modes, command
palette, layers, construction geometry, status bar). v0.13.0 lands Phase B
(constraint-driven dimensions, currently on `opencode/ci`, unmerged), closes the
release-governance defects found in the 2026-11-11 audit (unsigned artefacts,
in-app update without checksum verification, stale embedded release notes), and
restores documentation parity.

| ID | Capability | Done |
|---|-----------|------|
| CAP-100 | Constraint-driven dimensions core: `DimensionAnchor` (`wall_id`, `param`, `perpendicular`), `start_anchor`/`end_anchor`/`witness_gap`/`witness_overrun` on `Dimension`, anchor resolution (`effective_start`/`effective_end`), backward-compatible JSON load | — |
| CAP-101 | Constraint solver: `Constraint` enum (`PerpendicularDistance`, `FixedAngle`, `EqualLength`), `solve_constraints`, `apply_constraints_to_anchors`; dimensions follow wall moves | — |
| CAP-102 | Anchor commands: `SetDimensionStartAnchor`, `SetDimensionEndAnchor`, `SetDimensionWitnessGap`, `SetDimensionWitnessOverrun` with undo, validation and persistence | — |
| CAP-103 | Anchor UI: inspector fields for anchors and witness gap/overrun, click-to-place witness lines on canvas, grip editing | — |
| CAP-104 | Update verification: `update-check` verifies `SHA256SUMS` (and `.asc` signature when present) before surfacing an update; cache TTL honoured instead of `check_uncached` | — |
| CAP-105 | In-app release notes generated from `RELEASE_NOTES.md` at build time (or CI-synced), so the popup can never drift from the published notes; reconcile legacy pre-0.10.0 token totals | — |
| CAP-106 | Release governance: `release.yml` fails when the tag does not match `CARGO_PKG_VERSION`, and when `GPG_PRIVATE_KEY` is unset (no more silent unsigned publishes); one tag convention (`v`-prefixed) | — |
| CAP-107 | Documentation parity: `docs/capability-inventory.md` extended through CAP-099 with status + evidence, `docs/parity.md` rows for 0.9.0–0.13.0, CHANGELOG/ROADMAP/PROGRESS reconciled to published tags | — |
| CAP-108 | Phase A UI completion: layers palette and construction tools fully wired and reachable from the UI (palette, menu, shortcuts) | — |

**Release notes (intended):** Dimensions become anchored to the geometry they
measure: an anchor ties a measured point to a wall at a parametric position, so
moving a wall drags the dimension with it; witness lines gain editable gap and
overrun; a constraint solver keeps anchored dimensions correct as geometry
changes. The in-app update flow now verifies published checksums before
surfacing a new release, and the release-notes popup is generated from the same
source that feeds GitHub releases, ending version drift. Releases are signed
and version-guarded by CI. Earlier project files load unchanged.

**Release gate (guide §12.1):** signed artefacts (`SHA256SUMS.txt.asc`),
`docs/tokens.json` ledger updated by CI and quoted in the release notes,
version bump landed before the tag, `RELEASE_TOKENS` set for a credible ledger
figure.

**Audit traceability:** CAP-100–103 close audit findings F1/F4 (versioned,
tagged releases containing the promised code); CAP-104–105 close F3/F7 (update
verification, notes drift); CAP-106 closes F3/F8 (signing, ledger defaults);
CAP-107 closes the doc-parity gap (audit §4/§8 rec 1); CAP-108 closes audit §8
rec 3.

**Readiness:** 0% — planned; depends on landing the `opencode/ci`
constraint-dimension slice and the v0.12.1 rebuild confirming version stamps.

## v0.14.0 — Constraint Persistence & Sheet Layout

**Status:** Planned — v0.13.0 shipped anchored dimensions, a constraint solver
that takes an explicit list, and a governed signed release pipeline. v0.14.0
makes constraints first-class persisted data (so they survive save/load and
fire on wall moves), completes the sheet story with title blocks and viewports
(CAP-099), and codifies the manual QC that the 2026-11-11 audit left open.

| ID | Capability | Done |
|---|-----------|------|
| CAP-111 | Constraint persistence: `constraints: Vec<Constraint>` on `Level` with serde defaults, `Project::validate` rules (dangling walls, duplicate pins, param ranges), constraints solved on load | — |
| CAP-112 | Constraint management UI: list, add and remove a level's constraints from the inspector; the solver runs after wall edits so anchored dimensions stay correct without manual re-solve | — |
| CAP-113 | Sheet title blocks: every exported sheet carries a title block (project name, sheet number/title, level, scale, date, revision) drawn in the PDF and reflected in the drawing-set export | — |
| CAP-114 | Sheet viewports: a sheet may hold multiple viewports, each bound to a level region, with per-viewport scale; the drawing set lays them out on the sheet | — |
| CAP-115 | QC codified: `docs/qc/release-checklist.md` — signed-artefact verification steps, AppImage execution smoke test, screen-reader pass — each with an executable command or a named manual procedure and a sign-off line | — |

**Release notes (intended):** Constraints become part of the drawing: they are
saved with the project, re-solved on load and after every wall move, and
managed from the Inspector. Sheets gain proper title blocks and multiple
level-bound viewports, so a drawing set reads like a issued document rather
than a collection of pages. Earlier project files load unchanged.

**Release gate (guide §12.1):** signed artefacts, `docs/tokens.json` ledger
updated by CI and quoted in the release notes, version bump before the tag,
`RELEASE_TOKENS` set for the release.

**Readiness:** 0% — planned; CAP-111 is the foundation slice (core first, UI
second, sheets third, QC last).

## v0.15.0 — Blocks & Symbols, Print Polish

**Status:** Planned — v0.14.0 shipped constraint persistence (CAP-111/112),
sheet title blocks and viewports (CAP-113/114), and the QC checklist (CAP-115).
v0.15.0 closes the two biggest ergonomic gaps that remain: reusable geometry
(blocks/symbols) and true viewport clipping, and brings constraint management
into the command palette.

| ID | Capability | Done |
|---|-----------|------|
| CAP-116 | Block definitions: named groups of entities (wall, opening, dimension, label) saved on the project; insert/create/update/delete blocks; instances reference the definition | — |
| CAP-117 | Block instance UI: place a block from a defined list onto a level (click-to-place with preview), instance identity (id, block ref, param, flip), instance fields in the inspector, a block instance schedule listing every instance with its wall | — |
| CAP-118 | Viewport print polish: real PDF clipping at the viewport region (frames cut instead of drawing-whole), north arrow and scale bar anchored to the viewport instead of page corners; viewport placement guards | — |
| CAP-119 | Constraint palette & solver readout: add/remove/list constraints from the command palette; a readout of the solved count vs active constraints shown in the inspector next to each dimension | — |

**Release notes (intended):** Blocks and symbols give you repeatable drafting:
define a wall-opening-detail once, place it everywhere, change it once. Sheets
are properly framed (clipped viewports, north arrows that travel with their
viewports), and constraints live in the command palette with a solver
readout — pinned geometry is visible, not invisible. Earlier project files
load unchanged.

**Release gate (guide §12.1):** signed artefacts, `docs/tokens.json` ledger
updated by CI and quoted in the release notes, version bump before the tag,
`RELEASE_TOKENS` set for the release, QC checklist executed and filed.

**Readiness:** 0% — planned; depends on v0.14.0 being shipped (it is) and on
the constraint core being stable (it is, since CAP-111/112).

**Unscheduled / future:** Web/WASM port, plugin architecture, collaborative
editing, custom paper sizes, on-canvas hints, 3D/structural/IFC
track (separate roadmap).