# Progress

## v0.11.0 Status — RELEASED

- [x] `crates/lintel-core/src/measurements.rs` added: `Measurements`,
      `LevelTotals`, `ProjectTotals` (room area/perimeter, level and project
      wall/opening totals)
- [x] `format_area` honours `UnitSystem` (m² / ft² / in²) in
      `crates/lintel-core/src/units.rs`
- [x] Unit tests in `measurements::tests` (area, perimeter, totals, missing
      level, out-of-range room, JSON round-trip)
- [x] UI: `Tool::Measurements` with Shift+M; Inspector → Measurements shows the
      selected room, per-level totals and per-room list, and project totals
- [x] Release prep: version bumps landed in commit `803b590`
      (2026-11-11 rebuild; the original tag shipped with a stale 0.10.1 stamp —
      audit F1)
- [x] CI green, tag `v0.11.0` on `main` (commit `803b590`), GitHub Release
      published with all assets and `SHA256SUMS.txt`
- [x] `docs/tokens.json` ledger updated by CI (CI-owned; hand commits discarded)
- [ ] Guide §12.1 release gate: signed artefacts — `SHA256SUMS.txt.asc` still
      absent (audit F3: GPG step silently skips when `GPG_PRIVATE_KEY` unset);
      release notes for 0.11.0 added in `803b590`

Readiness % (v0.11.0): **95% — released; signing gate open (audit F3).**

## v0.11.1 Status — RELEASED AS 0.12.1

**Phase A continued: Snapping + Selection + Command Palette**, continuing the
2026-11-08 elite CAD reports. No `v0.11.1` tag ever existed; this content was
published under tag `v0.12.1`.

- [x] CAP-094 — Enhanced entity snapping (endpoint, midpoint, intersection,
      perpendicular, tangent, quadrant, grid, ortho) with priority ordering
- [x] CAP-095 — Window / crossing / fence selection modes
- [x] CAP-096 — Selection filter by entity type and layer
- [x] CAP-097 — Command palette (Ctrl+K, fuzzy match, recent commands)
- [x] CAP-098 — Snap visual indicators on canvas
- [x] Tests: snap candidate generation and priority ordering in `snap.rs`;
      window/crossing/fence, add/remove sets and type/layer filters in
      `selection.rs`
- [x] Release prep: both `Cargo.toml` files bumped to 0.12.1 in commit
      `803b590` (tag originally shipped with a stale 0.10.1 stamp — audit F1)
- [x] CI green, tag `v0.12.1` on `main` (commit `803b590`), GitHub Release
      published with all assets and `SHA256SUMS.txt`
- [x] Guide §12.1 gate: `docs/tokens.json` ledger updated by CI (30,000
      cumulative after the 2026-11-10 rebuild runs)
- [x] Signing gate (audit F3) — capability in place 2026-11-11: repo secret
      `GPG_PRIVATE_KEY` set (project key `09960C90…E442D0FE`, `Lintel Release
      Signing <dev@example.com>`); public key + verification guide committed in
      `6fdcc7d` (`resources/lintel-release-signing-public.asc`,
      `docs/release-verification.md`). Applies to releases built from now on;
      backfilling signatures onto the already-published 0.11.0/0.12.1 artefacts
      was declined — re-running the workflow would inflate the CI token ledger
      further (audit F8). First signed release: v0.13.0.

Phase A foundation already on the branch (core):

- [x] `layer.rs`: `Layer`, protected default layer and per-entity assignment
      with serde-default migration
- [x] `construction.rs`: line/ray/circle/arc construction geometry, snap-enabled
      and never printed
- [x] `status_bar.rs`: cursor coordinates, snap/ortho/grid chips, selection
      count and zoom %
- [ ] UI wiring for the foundation: layers palette and construction tools
      (→ v0.13.0 CAP-108)
- [x] `snap.rs` candidate foundation (midpoint/intersection/perpendicular source
      and priority order) — folded into the CAP-094 enhanced-snapping work

Readiness % (v0.11.1): **100% of the Phase A continued slice (CAP-094–CAP-098)
— published as 0.12.1, rebuilt 2026-11-10 (runs `42902103302` / `42902103890`).**

## v0.13.0 Status — IN PROGRESS

**Anchored Dimensions & Release Governance** (CAP-100–CAP-108). Full scope in
`ROADMAP.md`.

- [x] CAP-100 — Constraint-driven dimensions core: `DimensionAnchor`,
      `start_anchor`/`end_anchor`/`witness_gap`/`witness_overrun`, anchor
      resolution, backward-compatible JSON load — PR #22 merged (`2b07f58`),
      CI run green (check + 3-platform build-matrix), 24 new tests
- [x] CAP-101 — Constraint solver: `Constraint` enum, `solve_constraints`,
      `apply_constraints_to_anchors` (now also updates the stored fallback
      point) — PR #22
- [x] CAP-102 — Anchor commands: `SetDimensionStartAnchor`/`EndAnchor`,
      `SetDimensionWitnessGap`/`Overrun` with validation
      (`DimensionNotFound`, param-in-[0,1], margins ≥ 0) — PR #22
- [x] CAP-103 — Anchor UI: inspector fields, click-to-place witness lines,
      grip editing (renderer adopts `dimension_line_at`/`witness_line` +
      active level plumbing) — PR #23 merged (`7d88830`), CI green; inspector
      anchor rows + witness margin controls, canvas witness lines + cobalt
      anchor grips; canvas click-to-place deferred to CAP-108 as noted
- [x] CAP-104 — Update verification: `update-check` gains asset/digest
      metadata (`ReleaseAsset`, `checksum_asset`, `signature_asset`,
      `sha256`), app honours the cache TTL (single hoisted `UpdateChecker`),
      and the update copy states plainly that Lintel never auto-installs and
      the user verifies against `SHA256SUMS.txt`/`.asc` — PR #24 merged
      (`61cf97a`); `update-check` is a git submodule (`lintel-shared-crates`
      @ `4f7045f`)
- [ ] CAP-105 — In-app release notes generated from `RELEASE_NOTES.md`
- [x] CAP-106 — Release governance: `release.yml` now fails when the tag does
      not match both crate versions, refuses to publish unsigned (and
      self-verifies the signature), anchors releases to the real git tag with
      `--verify-tag` (single `v`-convention), and warns on the default
      `RELEASE_TOKENS`; `ci.yml` gains a version-consistency gate — PR #25
      merged (`d248b8b`)
- [ ] CAP-107 — Documentation parity: capability-inventory through CAP-099,
      parity rows, CHANGELOG/ROADMAP/PROGRESS reconciled
- [ ] CAP-108 — Phase A UI completion: layers palette + construction tools
      fully wired

**Landed:** the core slice merged to `main` at `b4a7df1` (merge of PR #22,
commit `2b07f58`, 2026-11-11); the anchor UI merged at `7d88830` (merge of
PR #23, 2026-11-12). Constraint persistence on `Level` is still an open design
point (solver takes an explicit `&[Constraint]`). Next slices: CAP-104 (update
verification), CAP-106 (release governance), CAP-105 (in-app release notes),
CAP-107 (docs parity), CAP-108 (Phase A UI completion).

Readiness % (v0.13.0): **75% — core (CAP-100–102), anchor UI (CAP-103),
update verification (CAP-104) and release governance (CAP-106) landed;
release-notes sync (CAP-105), docs parity (CAP-107) and Phase A UI completion
(CAP-108) remain.**

## v0.10.1 Status — RELEASED

- [x] Window icon embedded via `with_icon()` + `image` crate decode
- [x] `include_bytes!` path fixed to three levels to repo root
- [x] `resources/icons/lintel.svg` created as canonical source
- [x] Windows `.iss` version bumped 0.8.0 → 0.10.1
- [x] `ATTRIBUTION.md` icon attribution added
- [x] Command-layer fuzz tests added (3 proptest cases in `crates/lintel-core/tests/properties.rs`)
- [x] CI green (PR #16 audit fixes merged, PR #17 v0.10.1 bump)
- [x] v0.10.1 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.1 (release run `42372245530`; all seven assets present, `SHA256SUMS.txt` verified)
- [x] Token ledger `docs/tokens.json` updated by CI; cumulative 10,000 tokens (ledger accumulation bug fixed)

Readiness % (v0.10.1): **100% — shipped.**

## v0.10.0 Status — RELEASED

- [x] Suite-standard conformance: `docs/tokens.json` CI-managed ledger, release signing step added to `release.yml`, `docs/development.md` added
- [x] Property-based geometry and command-layer fuzz tests added (`crates/lintel-core/tests/properties.rs`)
- [x] Version UI moved to bottom-right; Release Notes popup gains Update button
- [x] Window icon embedded via `include_bytes!` + `image` crate
- [x] CI green, PR merged, tag `v0.10.0` pushed, release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.0

Readiness % (v0.10.0): **100% — shipped.**

## v0.9.0 Status — RELEASED

- [x] Drawing Set shipped: auto-detected rooms, free text plan labels, opening schedule, multi-sheet PDF with cover, north arrow + scale bar
- [x] Room detection orientation bug fixed; non-positive walks dropped
- [x] `LabelNotFound` error message normalized to `"{0} does not exist"`
- [x] Room area captions, label rendering, `Tool::Label` L key, drag-move, Delete, Ctrl+D duplicate, Ctrl+Shift+E Export drawing set
- [x] `PlanGeometry.labels/rooms` wired, IDs/error/command/join plumbing complete
- [x] CI green, PR `opencode/v0.9.0-rooms` merged, tag `v0.9.0` pushed, release published

Readiness % (v0.9.0): **100% — shipped.**

## v0.8.0 Status — RELEASED

- [x] Focus review found a regression: tabbing to the canvas disabled every shortcut
- [x] Shortcuts now apply when nothing is focused **or** the canvas is focused; still suppressed in text fields
- [x] Unit tests for `plan_summary` (empty, singular, multi-storey) — verified in CI
- [x] `ACCESSIBILITY.md` records the focus-review outcome
- [x] CI green (PR #11 run `42294376007`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.8.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.8.0
      (release run `42295310359`; all six assets present)

Readiness % (v0.8.0): **100% — shipped.**

### Still planned (v0.9.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host

## v0.7.0 Status — RELEASED

- [x] Audit: grepped the UI for every `Command` variant; found 4 unreachable capabilities
- [x] Inspector → Project: rename the project (`RenameProject`) and switch Metric/Imperial (`SetUnits`)
- [x] Inspector → Storeys: delete a storey (`DeleteLevel`, disabled for the last) and add one (`AddLevel`)
- [x] Dragging a selected dimension changes its offset (`SetDimensionOffset`)
- [x] Fmt/clippy-safe: removed the now-unused `LevelId` import; rustfmt clean
- [x] CI green (PR #10 run `42211261713`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.7.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.7.0
      (release run `42212698044`; all six assets present)

Readiness % (v0.7.0): **100% — shipped.**

### Known, deliberate exception

`MoveDimension` (move a dimension's measured points) stays core-only: dragging
binds to the offset instead, so a dimension never detaches from what it
measures. Recorded in DECISIONS.md D-026.

### Still planned (v0.8.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host
- [ ] Keyboard-focus management review

## v0.6.0 Status — RELEASED

- [x] File keyboard shortcuts: Ctrl+N/O/S/Shift+S/E
- [x] Focusable canvas with a screen-reader label naming the active level and its contents
- [x] Level summary shown in the toolbar
- [x] Plain tool shortcuts guarded so Ctrl+S cannot also switch tools
- [x] **Fixed:** PDF export dialog was unreachable (nothing set `export_open`); now File → Export sheet… and Ctrl+E
- [x] Linux AppImage statically verified (ELF + SquashFS magic + SHA-256); execution needs an x86_64 host
- [x] `ACCESSIBILITY.md` and `docs/qc/checklist.md` updated
- [x] CI green (PR #9 run `42192427679`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.6.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.6.0
      (release run `42193835276`; all six assets present)

Readiness % (v0.6.0): **100% — shipped.**

### Still planned (v0.7.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host
- [ ] Keyboard-focus management review

## v0.5.0 Status — RELEASED

- [x] Windows installer step restored using Chocolatey (`choco install innosetup`)
- [x] `.iss` `DefaultDirName` typo fixed (`{autpf}` → `{autopf}`)
- [x] `.iss` AppId literal brace escaped; `SetupIconFile` uses a real Windows `.ico`
- [x] `.iss` version bumped to 0.5.0; app + core bumped to 0.5.0
- [x] `*.exe` re-added to the release collect pattern so the installer ships with checksums
- [x] CI green (PR #8 run `42099183213`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.5.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.5.0
      (release run `42103705669`; `lintel-windows-x86_64-setup.exe` verified as a valid PE,
      SHA256SUMS recomputed and matching)

Readiness % (v0.5.0): **100% — shipped.**

## v0.4.0 Status — RELEASED

- [x] Keyboard shortcut hints + hover labels on all controls (W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete)
- [x] Focus-ring contrast WCAG AA (COBALT verified ~8.3:1 on PAPER, ~7.8:1 on PANEL_BG)
- [x] Escape dismissal on all dialogs (About, Release notes, Update, Export sheet)
- [x] Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, verified ELF)
- [x] ACCESSIBILITY.md (honest statement incl. canvas limitation) and docs/qc/checklist.md
- [x] CI green (PR #7 run `42013256872`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.4.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.4.0
      (release run `42024620827`; AppImage + SHA256SUMS verified)

Readiness % (v0.4.0): **100% — shipped.**

### Still planned (v0.5.0)

- [ ] Windows installer (MSI or setup .exe) — Inno Setup download hangs on CI; deferred
- [ ] Screen-reader labels beyond egui's built-in accessibility
- [ ] Keyboard-focus management review

## v0.3.0 Status — RELEASED

- [x] Rejected edits and file errors surfaced in the toolbar (with a Clear message item)
- [x] macOS release archive packages `Lintel.app` with a real `Info.plist`
- [x] Empty-state hint when a level has no walls (landed with the v0.2.0 polish)
- [x] CI green (PR #6 run `41998649019`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.3.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.3.0
      (release run `41999766800`; `Lintel.app` verified inside the published asset)

Readiness % (v0.3.0): **100% — shipped.**

## v0.2.0 Status — RELEASED

- [x] Dimension lines (aligned/horizontal/vertical) with numeric labels
- [x] Dimension place / move / offset / delete commands and Inspector panel
- [x] Dimension hit testing ahead of walls; save/load and undo/redo
- [x] Joint resolution for unequal wall thicknesses
- [x] Shared `update-check` crate (GitHub Releases API, fail-soft offline)
- [x] Shared `crash-report` crate (panic hook, scrubbed diagnostics, issue URL)
- [x] In-app release-notes window with per-release and cumulative token metrics
- [x] In-app update check with a non-intrusive "update available" prompt
- [x] Cross-platform CI build matrix (Linux, macOS, Windows)
- [x] Multi-platform release matrix (Linux x86_64/aarch64, macOS aarch64, Windows x86_64)
- [x] README rewritten for non-technical users
- [x] CI green (PR #4 run `41907396871`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.2.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.2.0

Readiness % (v0.2.0): **100% — shipped.**

### Still planned (v0.2.x / v0.3.0)

- [ ] Dimension kind selector + extension-line toggle + label precision
- [ ] Rotated dimensions labels for vertical/aligned dimensions
- [ ] Installers (AppImage, MSI, macOS .app)
- [ ] Accessibility, empty states and keyboard-focus management

## v0.1.0 Status — RELEASED

- [x] Core domain implemented (`crates/lintel-core`) with headless workflow tests
- [x] GUI shell (`apps/lintel`) with eframe/egui UI wired to core commands
- [x] Documentation set (project-brief, scope, inspiration, architecture, capability-inventory, parity, …)
- [x] ROADMAP / CHANGELOG / DECISIONS / ATTRIBUTION maintained
- [x] `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- [x] CI green on push (`check` job)
- [x] v0.1.0 tag and release — https://github.com/Perchemop/lintel-desktop/releases/tag/0.1.0

Readiness % (v0.1.0): **100% — shipped.**

## Research

- [x] No large-scale research notes needed for v0.1.0–v0.10.x; public
      egui/eframe/PDF/ureq sources only
- [x] Elite CAD feasibility reports filed 2026-11-08:
      `ELITE_CAD_RESEARCH_SUMMARY.md`, `DOMAIN_INVESTIGATION_ELITE_CAD.md`,
      `DOMAIN_DESIGN_SKETCHES.md`, and `docs/research/elite-*.md`
- [x] `docs/research/` reserved for gated research before larger Tasks

## Process feedback

- No friction logged this session.