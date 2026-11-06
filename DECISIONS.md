# Decisions

## Recorded decisions for this project.

| Decision ID | Area | Decision | Rationale | Date |
|---|---|---|---|---|
| D-001 | Product name | Chose "Lintel" over alternatives | Structural clarity; no trademark conflicts; handle `@perchemop/lintel-desktop` available | 2026-11-03 |
| D-002 | License | Dual licence MIT OR Apache-2.0 | Permissive; allows commercial and open-source use; no copyleft viral risk | 2026-11-03 |
| D-003 | Persistence format | JSON over SQLite | Human-readable; no native dependency; round-trip editable; part of clean-room rules | 2026-11-03 |
| D-004 | Unit system default | Metric (mm) | Developer's local platform; metric is universal for architectural drafting | 2026-11-03 |
| D-005 | CLI/gate strategy | Headless `cargo test` as primary QA; client visual pass on green CI | Matches addendum K/M rules; no display required in CI | 2026-11-03 |
| D-006 | v0.1.0 gate | All Section 6.1 criteria must be satisfied before tagging | Hard gate prevents feature creep; ensures shipable foundation | 2026-11-03 |
| D-007 | Shared suite crates | `crash-report` + `update-check` from day one | Suite readiness; consistent crash/update behaviour across apps | 2026-11-03 |
| D-008 | CI from day one | `.github/workflows/ci.yml` and `release.yml` from foundation stage | Learn release path early; avoid broken tags/artefacts at the worst time | 2026-11-03 |
| D-009 | Core/UI split | `crates/lintel-core` (domain, commands, persistence, PDF) + `apps/lintel` (egui shell); core has zero GUI dependencies | addendum K.3: headless workflow tests without a display | 2026-11-04 |
| D-010 | PDF export | Hand-written PDF 1.7 writer in `lintel-core::pdf`; no PDF crate dependency | Zero-licence-question, fully byte-tested export | 2026-11-04 |
| D-011 | GUI toolkit | eframe/egui 0.36.2 | Suite-consistent version; native desktop | 2026-11-04 |
| D-012 | Shared suite crates | Deferred until after v0.1.0 | addendum Phase 5: crash-report/update-check optional before the hard gate | 2026-11-04 |
| D-013 | Release matrix | v0.1.0 builds Linux x86_64 only | macOS/Windows runners lack linkers without cross tooling; expand after v0.1.0 | 2026-11-04 |
| D-014 | Dimensions | Command-driven dimension lines stored on the `Level`, hit-tested ahead of walls | Keeps geometry in the core, every edit undoable and persisted like walls | 2026-11-04 |
| D-015 | Shared suite crates | Adopt `update-check` + `crash-report` as workspace crates (supersedes D-012's deferral) | guide section 7 / Phase 4; the app now ships update checks and crash capture | 2026-11-05 |
| D-016 | HTTP client | `ureq` 3 with its default `rustls` (ring) provider | Ring builds without CMake/NASM or system OpenSSL, so all three CI platforms compile cleanly | 2026-11-05 |
| D-017 | Update delivery | The in-app check opens the GitHub Releases download page rather than replacing the running binary | Self-replacement needs signing and platform-specific installers; a one-click link is honest and safe | 2026-11-05 |
| D-018 | Multi-platform | CI builds a Linux/macOS/Windows matrix; Releases publish Linux x86_64 + aarch64, macOS aarch64, Windows x86_64 | guide section 6.2 / section 8 step 7 | 2026-11-05 |
| D-019 | Accessibility labels | Keyboard shortcut hints + hover labels on every control; COBALT focus rings meet WCAG AA | guide II.21; every interactive widget gets a name/label | 2026-11-05 |
| D-020 | Linux AppImage | Release publishes `lintel-appimage-x86_64.AppImage`; Windows installer deferred (Inno Setup download hangs on CI) | AppImage built without FUSE by extracting `appimagetool` and running `AppRun` directly | 2026-11-05 |
| D-021 | Windows installer | Built on CI with Inno Setup installed via Chocolatey, not a direct `.exe` download | The direct download hung on windows-latest; `choco install innosetup` is reliable and preinstalled | 2026-11-06 |
| D-022 | Windows icon | `SetupIconFile` uses a real `.ico` (`resources/windows/lintel.ico`) wrapping the 256x256 PNG | Inno Setup rejects PNG for the setup icon; ICO supports a PNG-compressed 256x256 entry | 2026-11-06 |
| D-023 | Canvas accessibility | The canvas is focusable and labelled with a level summary; individual plan objects are not labelled | A custom painter cannot expose per-object semantic nodes; a region label plus the Inspector covers the model | 2026-11-06 |
| D-024 | File shortcuts | Ctrl+N/O/S/Shift+S/E drive New/Open/Save/Save As/Export; plain tool keys require no modifier | Completes keyboard flows (guide II.21.2) and prevents Ctrl+S from switching tools | 2026-11-06 |
| D-025 | File dialogs | Open/Save/Save As use one fixed path (`workflow.lintel.json`); no native file picker | The `rfd` picker dragged GPL/LGPL dependencies; a fixed path keeps the build clean (recorded since v0.1.0) | 2026-11-06 |
| D-026 | Dimension drag | Dragging a dimension changes its **offset**; `MoveDimension` stays core-only | Moving the measured span would detach a dimension from what it measures; offset is the professional gesture | 2026-11-06 |
| D-027 | Reachability audit | Every `Command` variant must be bound in `apps/lintel` (or explicitly documented as core-only) | Two releases in a row shipped a documented-but-unreachable capability (export dialog in 0.6.0, storeys/units/rename in 0.7.0) | 2026-11-06 |