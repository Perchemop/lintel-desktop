# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## v0.3.0 — 2026-11-05

### Summary

Packaging and feedback polish: rejected edits and file errors are now shown in
the app instead of only on the console, and the macOS download ships as a real
`Lintel.app` bundle.

### Added

- A status message in the toolbar whenever an edit is rejected or a file
  operation fails (open, save, save as, PDF export), with a
  "Help → Clear message" item
- The macOS release archive now contains `Lintel.app` with a real
  `Info.plist`, instead of a bare binary

### Changed

- (none)

### Fixed

- (none)

### Known limitations

- No Linux AppImage or Windows installer yet
- Screen-reader labelling beyond egui's built-in accessibility is still pending

### Token metrics

- Approx. tokens this release: 300,000
- Approx. cumulative tokens since development began: 5,200,000

(Figures are approximate totals recorded at release time.)

## v0.2.0 — 2026-11-05

### Summary

Joint resolution for walls of unequal thickness, plus linear dimension lines
with numeric labels. This release also brings the shared suite infrastructure
(crash reporting and update checking), a cross-platform CI build matrix, and a
user-facing README.

### Added

- Dimension lines (aligned, horizontal and vertical) with numeric labels and an
  editable offset; place, move and delete them from the canvas and the Inspector
- Dimension hit testing that takes priority over the wall beneath it, plus
  dimension save/load and full undo/redo
- Dimension kind selector and extension-line toggle in the Inspector, labels
  that rotate to follow the line, and snapping applied when a wall or dimension
  is committed (not just previewed)
- An on-canvas hint when a level has no walls yet
- Shared `update-check` crate (GitHub Releases API, semantic version comparison,
  fail-soft offline) and the in-app "check for updates" flow
- Shared `crash-report` crate (panic hook, scrubbed diagnostics, optional
  user-confirmed issue URL)
- In-app release notes window, opened from the version shown in the toolbar,
  with per-release and cumulative token metrics
- Cross-platform CI build matrix (Linux, macOS, Windows) and a multi-platform
  release matrix (Linux x86_64 + aarch64, macOS aarch64, Windows x86_64)
- User-facing README written for non-technical users

### Changed

- Wall joins now extend each wall by half of the *other* wall's thickness, so
  outside corners close cleanly for unequal walls
- The app version is displayed subtly in the toolbar; selecting it opens the
  release-notes window
- `deny.toml` licence policy extended for the new dependency graph

### Fixed

- PDF cross-reference parsing reads bytes directly, so sheets with non-UTF-8
  content load with their pages intact

### Known limitations

- Dimension labels are drawn horizontally; rotated labels for vertical/aligned
  dimensions are planned for v0.2.x
- No Windows/macOS/Linux installers; download the zipped binary from Releases
- The update check opens the Releases page rather than replacing the running
  binary in place

### Token metrics

- Approx. tokens this release: 2,200,000
- Approx. cumulative tokens since development began: 4,900,000

(Figures are approximate totals recorded at release time.)

## v0.1.0 — 2026-11-04

### Summary

First functional test build. Supports walls, doors, windows, levels, save/load round-trip, and PDF plan export. Joint resolution for unequal thicknesses is a known limitation scheduled for 0.2.x.

### Added

- Core domain: walls, openings (doors/windows), levels, project persistence (JSON)
- Command pattern: AddWall, MoveWall, SetWallThickness, SetWallHeight, DeleteWall, PlaceOpening, MoveOpening, SetOpeningSize, FlipOpening, DeleteOpening, AddLevel, DeleteLevel, SetActiveLevel, SetUnits, RenameProject
- Undo/redo history (up to 128 entries)
- Project validation (duplicate ids, missing hosts, overlapping openings, stale identity counter)
- PDF rendering with title block, scale label, sheet number
- Unit system: Metric (mm) and Imperial (feet/inches)
- Hit testing: identify which wall or opening is under the pointer
- Plan geometry: resolved wall bands, joint extensions, bounds computation

### Changed

- (none for v0.1.0 — first release)

### Fixed

- (none for v0.1.0 — first release)

### Known limitations

- Joint resolution assumes equal wall thicknesses; unequal thicknesses may show a small step at outside corners (documented in `join.rs`)
- PDF rendering does not include dimension lines or numeric labels
- No Windows/macOS/Linux installers (binary built via GitHub Actions only)
- No web/WASM target
- No accessibility features (keyboard navigation, screen reader support)
- No GUI polish: empty states, error dialogs, keyboard shortcuts

### Try it

Download the CI artefact for your platform and run:
```bash
./lintel
```
Then try: New Project → Add Wall → Place Door → Save → Reopen → Render PDF.