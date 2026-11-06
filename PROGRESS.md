# Progress

## v0.8.0 Status — IN PROGRESS (implemented; CI/tag pending)

- [x] Focus review found a regression: tabbing to the canvas disabled every shortcut
- [x] Shortcuts now apply when nothing is focused **or** the canvas is focused; still suppressed in text fields
- [x] Unit tests for `plan_summary` (empty, singular, multi-storey) — verified in CI
- [x] `ACCESSIBILITY.md` records the focus-review outcome
- [ ] CI green on `main` for the v0.8.0 slice
- [ ] v0.8.0 tag and GitHub Release published

Readiness % (v0.8.0): **~85% — implemented; verification and release pending.**

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

- [x] No large-scale research notes needed; public egui/eframe/PDF/ureq sources only
- [x] `docs/research/` reserved for gated research before larger Tasks

## Process feedback

- No friction logged this session.