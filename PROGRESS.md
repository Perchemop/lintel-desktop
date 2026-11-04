# Progress

## v0.2.0 Status — IN PROGRESS (features implemented; CI/tag pending)

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
