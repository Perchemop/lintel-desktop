# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## v0.8.0 — 2026-11-07

### Summary

A keyboard-focus review. It found a regression introduced when the canvas
became focusable: tabbing to the canvas silently disabled every keyboard
shortcut. Shortcuts now work with the canvas focused, and the level summary is
unit-tested.

### Added

- Unit tests for the level summary shown in the toolbar and announced to screen
  readers (empty, singular and multi-storey cases)
- `ACCESSIBILITY.md` records the focus-review outcome: tab order, visible focus,
  Escape dismissal and that focus is never trapped

### Changed

- Keyboard shortcuts now apply when nothing has focus **or** when the canvas has
  focus; they are still suppressed while typing in a text field

### Fixed

- **Shortcuts stopped working after tabbing to the canvas.** Making the canvas
  focusable (0.6.0) meant `focused().is_none()` became false, so W/S/D/N/M,
  Ctrl+N/O/S/E and undo/redo all went dead until focus moved elsewhere

### Known limitations

- Screen-reader output is verified manually, not in CI
- The Linux AppImage has not been executed on an x86_64 host

### Token metrics

- Approx. tokens this release: 130,000
- Approx. cumulative tokens since development began: 5,960,000

(Figures are approximate totals recorded at release time.)

## v0.7.0 — 2026-11-06

### Summary

Closes the gap between what the core can do and what the interface exposes. An
audit found four documented capabilities — add/delete a storey, rename the
project and switch units — that existed in the core but had no way to be
triggered, like the export dialog before them. All four are now reachable.

### Added

- **Inspector → Project:** an editable project name (with Apply) and a Metric /
  Imperial unit selector
- **Inspector → Storeys:** a Delete button per storey (disabled for the last
  one) and a name-elevation-Add row to create a storey
- Dragging a selected dimension line moves it perpendicular to the measurement,
  changing its offset
- A documented command-reachability audit in the QC checklist that greps the UI
  for every `Command` variant

### Changed

- The Inspector takes a small `InspectorDraft` for the new-storey fields so the
  text being typed survives between frames

### Fixed

- Four capabilities were unreachable from the UI despite being listed as
  implemented: `AddLevel`, `DeleteLevel`, `SetUnits` and `RenameProject`

### Known limitations

- `MoveDimension` (moving a dimension's measured points) remains a core-only
  command covered by unit tests; the canvas binds dragging to the offset
  instead, because moving the measured span detaches a dimension from what it
  measures (see DECISIONS.md D-026)
- Screen-reader output is verified manually, not in CI

### Token metrics

- Approx. tokens this release: 160,000
- Approx. cumulative tokens since development began: 5,830,000

(Figures are approximate totals recorded at release time.)

## v0.6.0 — 2026-11-06

### Summary

Keyboard-complete file flows and an accessible canvas. A real bug is also
fixed: the PDF export dialog existed but had no way to open it.

### Added

- File keyboard shortcuts: Ctrl+N (new), Ctrl+O (open), Ctrl+S (save),
  Ctrl+Shift+S (save as), Ctrl+E (export sheet)
- The plan canvas is keyboard-focusable and carries a screen-reader label
  naming the active level and its contents, e.g.
  `Ground: 3 walls, 2 doors, 1 window, 1 dimension`
- The same level summary is shown in the toolbar
- `ACCESSIBILITY.md` and the QC checklist document the new keyboard flows

### Changed

- Plain tool shortcuts (W/S/D/N/M/G/O/F) no longer fire while a Ctrl/Cmd or Alt
  modifier is held, so Ctrl+S cannot also switch to the Select tool

### Fixed

- **The PDF export dialog was unreachable**: nothing set `export_open`, so the
  documented "Export sheet…" flow did not exist. It is now on the File menu and
  bound to Ctrl+E.

### Known limitations

- Screen-reader labels beyond egui's built-in AccessKit support (and the canvas
  level summary) still pending
- Canvas objects have no per-object labels — see `ACCESSIBILITY.md`
- Linux AppImage verified statically only (execution needs an x86_64 host)

### Token metrics

- Approx. tokens this release: 150,000
- Approx. cumulative tokens since development began: 5,670,000

(Figures are approximate totals recorded at release time.)

## v0.5.0 — 2026-11-06

### Summary

Windows users finally get a real installer: the release now publishes
`lintel-windows-x86_64-setup.exe`, built on CI with Inno Setup via Chocolatey.

### Added

- Windows installer (`lintel-windows-x86_64-setup.exe`) published alongside the
  Windows zip; installs to Program Files with Start-menu and optional desktop
  shortcuts
- Inno Setup script checked into the repo (`resources/windows/lintel.iss`) so a
  human can rebuild the installer offline

### Changed

- App and core version bumped to 0.5.0

### Fixed

- `.iss` `DefaultDirName` typo (`{autpf}` → `{autopf}`), so the installer
  suggests the correct Program Files directory

### Known limitations

- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Linux AppImage not yet verified on end-user systems
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels

### Token metrics

- Approx. tokens this release: 120,000
- Approx. cumulative tokens since development began: 5,520,000

(Figures are approximate totals recorded at release time.)

## v0.4.0 — 2026-11-05

### Summary

Accessibility polish and packaging: keyboard shortcut hints and hover labels on
every control, WCAG AA focus-ring contrast, and a Linux AppImage release
artefact. Windows installer deferred (Inno Setup download hangs on CI).

### Added

- Keyboard shortcut line in the toolbar: W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete
- Hover tooltip on every menu button and Inspector control describing units and ranges
- Focus-ring colour COBALT verified above WCAG AA 4.5:1 on both panel and canvas backgrounds
- Escape dismissal confirmed on all dialogs (About, Release notes, Update, Export sheet)
- `ACCESSIBILITY.md` honesty statement and `docs/qc/checklist.md` accessibility smoke test
- Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, valid ELF)
- Release notes now come from `RELEASE_NOTES.md` per tag

### Changed

- (none)

### Fixed

- AppImage build: `appimagetool` is itself an AppImage needing FUSE; now extracted
  and the embedded `AppRun` ELF invoked directly so no FUSE is required on CI

### Known limitations

- No Windows installer yet (zip remains the Windows deliverable; Inno Setup step deferred)
- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels; documented in `ACCESSIBILITY.md`

### Token metrics

- Approx. tokens this release: 200,000
- Approx. cumulative tokens since development began: 5,400,000

(Figures are approximate totals recorded at release time.)

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