<!-- This file is attached verbatim to every GitHub Release by
     .github/workflows/release.yml (--notes-file). The newest release belongs at
     the top. Everything above the first release heading stays invisible on
     GitHub because it is an HTML comment. -->

## Lintel 0.9.0 — Drawing Set

**Summary.** Lintel now issues a real drawing set: auto-detected rooms with area captions, free text plan labels, an opening schedule, north arrow and graphic scale bar on every sheet, multi-sheet PDF export with a cover sheet, and duplicate (Ctrl+D).

### Added
- Auto-detected rooms via planar face traversal; area captions rendered on canvas and PDF.
- Free text plan labels with CRUD, drag to move, Delete key.
- Opening schedule grouped by kind/size with D1/W1 marks; shown in Inspector and PDF.
- Multi-sheet drawing-set export: cover sheet + one sheet per storey via `render_set`.
- Sheet furniture: north arrow and graphic scale bar on plan sheets.
- Duplicate selection (Ctrl+D) for wall/dimension/label/opening.
- Export drawing set menu (Ctrl+Shift+E) and Label tool (L).

### Changed
- `PlanGeometry` now exposes `labels` and `rooms`; hit testing prioritises labels.
- PDF `escape_text` maps superscripts and dashes to ASCII for WinAnsi fidelity.

### Fixed
- Room detection orientation bug: disconnected wall groups previously produced duplicate rooms; now all non-positive walks are dropped.
- `LabelNotFound` error message no longer duplicates “label”.
- Room JSON round-trip uses approximate centroid comparison.

### Known limitations
- Rooms have no persistent identity; naming is done via plan labels anchored inside the room.
- `crates/update-check` is still in-repo; guide §12.1 requires a shared git submodule (deviation recorded).
- Version UI is still top-toolbar; guide §12.1 requires bottom-right version with Release Notes popup containing token metrics and Update button (planned for v0.10.0).

### Artefacts
`lintel-windows-x86_64-setup.exe`, `lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip`, `lintel-appimage-x86_64.AppImage`, and `SHA256SUMS.txt`.

### Token metrics
Approximate totals recorded at release time.

- This release: ~120,000 tokens
- Cumulative since development began: ~6,080,000 tokens

---
## Lintel 0.8.0 — Keyboard-focus review

**Summary.** A focus review found a regression: after tabbing to the drawing
canvas, every keyboard shortcut went dead. Shortcuts now work with the canvas
focused and are still suppressed while you type in a field.

### Added

- Unit tests for the level summary shown in the toolbar and announced to screen
  readers
- `ACCESSIBILITY.md` now records the focus-review outcome: tab order, visible
  focus, Escape dismissal, and that focus is never trapped

### Changed

- Keyboard shortcuts apply when nothing has focus **or** when the canvas has
  focus; they are still suppressed while typing in a text field

### Fixed

- **Shortcuts stopped working after tabbing to the canvas** (a regression from
  0.6.0, when the canvas became focusable)

### Known limitations

- Screen-reader output is verified manually, not in CI
- The Linux AppImage has not been executed on an x86_64 host

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~130,000 tokens
- Cumulative since development began: ~5,960,000 tokens

---

## Lintel 0.7.0 — Every documented capability is now reachable

**Summary.** An audit found four things Lintel could do but that had no way to
be triggered in the interface — add or delete a storey, rename the project, and
switch between metric and imperial units. All four are now in the Inspector,
and dragging a dimension moves its line.

### Added

- **Project panel:** rename the project, and switch between Metric (mm) and
  Imperial (ft-in) units
- **Storeys panel:** delete a storey, and add a new one with a name and
  elevation
- Dragging a selected dimension line moves it perpendicular to what it
  measures (its offset)

### Fixed

- Add storey, delete storey, rename project and switch units were implemented in
  the core but unreachable from the interface — the same class of defect as the
  unreachable export dialog fixed in 0.6.0

### Known limitations

- Moving a dimension's measured points (`MoveDimension`) is a core-only command
  covered by unit tests; dragging adjusts the offset instead, because moving the
  measured span would detach a dimension from what it measures
- Screen-reader output is verified manually, not in CI

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~160,000 tokens
- Cumulative since development began: ~5,830,000 tokens

---

## Lintel 0.6.0 — Keyboard-complete file flows and an accessible canvas

**Summary.** Everything you do with files is now one keyboard shortcut away,
and the canvas announces what is on the plan to a screen reader. This release
also fixes a real bug: the PDF export dialog existed but could not be opened.

### Added

- File keyboard shortcuts: **Ctrl+N** new, **Ctrl+O** open, **Ctrl+S** save,
  **Ctrl+Shift+S** save as, **Ctrl+E** export sheet
- The plan canvas is keyboard-focusable and carries a screen-reader label
  describing the active level, e.g. `Ground: 3 walls, 2 doors, 1 window, 1 dimension`
- The same level summary is shown in the toolbar, so you can see the model's
  contents at a glance

### Changed

- Plain tool shortcuts (W/S/D/N/M/G/O/F) no longer fire while Ctrl/Cmd or Alt is
  held — previously Ctrl+S would also switch to the Select tool

### Fixed

- **PDF export is reachable again.** The export dialog was wired up but nothing
  opened it, so the documented "Export sheet…" flow did not exist. It is now on
  the File menu and bound to Ctrl+E.

### Known limitations

- Canvas objects have no per-object labels (the canvas carries a level summary
  instead); documented in `ACCESSIBILITY.md`
- Screen-reader output is verified manually, not in CI
- The Linux AppImage is verified statically (ELF + SquashFS + checksum);
  running it requires an x86_64 Linux host

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~150,000 tokens
- Cumulative since development began: ~5,670,000 tokens

---

## Lintel 0.5.0 — Windows installer

**Summary.** Windows users get a real installer: the release now publishes
`lintel-windows-x86_64-setup.exe`, built on CI with Inno Setup, alongside the
Windows zip. Install to Program Files with Start-menu and optional desktop
shortcuts.

### Added

- Windows installer (`lintel-windows-x86_64-setup.exe`) published with the
  release, in addition to `lintel-windows-x86_64.zip`
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
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels;
  documented in `ACCESSIBILITY.md`

### Artefacts

`lintel-windows-x86_64-setup.exe` (Windows installer),
`lintel-windows-x86_64.zip`, `lintel-linux-x86_64.zip`,
`lintel-linux-aarch64.zip`, `lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage), and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~120,000 tokens
- Cumulative since development began: ~5,520,000 tokens

---

## Lintel 0.4.0 — Accessibility labels, keyboard shortcuts, AppImage

**Summary.** Keyboard shortcut hints and hover labels on every control,
WCAG AA focus-ring contrast, and a Linux AppImage release artefact. The
Windows installer is deferred (Inno Setup download hangs on CI).

### Added

- Keyboard shortcut line in the toolbar: W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete
- Hover tooltip on every menu button and Inspector control describing units and ranges
- Focus-ring colour COBALT verified above WCAG AA 4.5:1 on both panel and canvas backgrounds
- Escape dismissal confirmed on all dialogs (About, Release notes, Update, Export sheet)
- `ACCESSIBILITY.md` honesty statement and `docs/qc/checklist.md` accessibility smoke test
- Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, valid ELF)

### Changed

- (none)

### Fixed

- AppImage build: `appimagetool` is itself an AppImage needing FUSE; now extracted
  and the embedded `AppRun` ELF invoked directly so no FUSE is required on CI

### Known limitations

- No Windows installer yet — the Windows zip remains the deliverable; the Inno
  Setup installer step is deferred to a future release
- Screen-reader labels beyond egui's built-in AccessKit support still pending
- Canvas objects (walls/openings/dimensions) have no per-object ARIA labels;
  documented in `ACCESSIBILITY.md`

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-appimage-x86_64.AppImage` (Linux AppImage),
`lintel-windows-x86_64.zip`, and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~200,000 tokens
- Cumulative since development began: ~5,400,000 tokens

---

## Lintel 0.3.0 — Error feedback and macOS bundle

**Summary.** When something cannot be done — a rejected edit, a file that will
not open or save, a PDF that cannot be written — Lintel now says so in the
toolbar instead of staying silent. The macOS download is packaged as a real
`Lintel.app` application.

### Added

- A status message in the toolbar whenever an edit is rejected or a file
  operation fails (open, save, save as, PDF export). It clears itself after a
  few seconds, and **Help → Clear message** hides it immediately.
- The macOS release archive now contains `Lintel.app` with a real
  `Info.plist`, so it launches like a normal Mac application.

### Known limitations

- No Linux AppImage or Windows installer yet — unzip and run the binary.
- Screen-reader labelling beyond the toolkit's built-in support is pending.

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip` (contains `Lintel.app`),
`lintel-windows-x86_64.zip`, and `SHA256SUMS.txt`.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~300,000 tokens
- Cumulative since development began: ~5,200,000 tokens

---

## Lintel 0.2.0 — Joint resolution, dimension lines, suite infrastructure

**Summary.** Wall joins now close cleanly even when the two walls have
different thicknesses, and you can annotate the plan with dimension lines that
show real measurements. Under the hood this release adds the shared crash
reporting and update-checking infrastructure and builds on more platforms.

### Added

- Dimension lines (aligned, horizontal and vertical) with numeric labels and an
  editable offset. Place them with the Dimension tool, then move, re-offset or
  delete them from the canvas or the Inspector.
- Dimension kind selector and an extension-line (witness line) toggle in the
  Inspector.
- Rotated dimension labels that follow the line, so vertical and aligned
  measurements read correctly.
- Snapping now also applies when you *commit* a wall or dimension, so both ends
  land exactly on existing wall corners.
- An on-canvas hint when a level is empty.
- In-app release notes: the version shown in the toolbar opens a window with
  the notes for every release and token-usage metrics.
- Update checking: Lintel quietly checks the GitHub Releases API for a newer
  version and offers a one-click link to the download page. It fails soft when
  you are offline and never blocks the app.
- Crash reporting: a panic writes a scrubbed JSON log locally. Nothing is sent
  anywhere unless you open the pre-filled bug report yourself.
- Cross-platform CI build matrix (Linux, macOS, Windows) and release artefacts
  for Linux x86_64, Linux aarch64, macOS aarch64 and Windows x86_64.
- A README written for people who are not developers.

### Changed

- Wall joins extend each wall by half of the *other* wall's thickness, so
  outside corners close for unequal walls.
- The app version is shown subtly in the toolbar; selecting it opens the
  release-notes window.

### Fixed

- PDF cross-reference parsing reads bytes directly, so sheets containing
  non-UTF-8 content open with their pages intact.

### Known limitations

- No installers yet (AppImage, MSI, macOS .app) — download the zipped binary
  from the Releases page and run it.
- The update check opens the Releases page; it does not replace the running
  binary in place.
- Accessibility work (keyboard navigation, screen-reader labels) is scheduled
  for the next milestone.

### Artefacts

`lintel-linux-x86_64.zip`, `lintel-linux-aarch64.zip`,
`lintel-macos-aarch64.zip`, `lintel-windows-x86_64.zip`, and
`SHA256SUMS.txt` with one SHA-256 checksum per archive.

### Token metrics

Approximate totals recorded at release time, for transparency rather than as an
exact accounting.

- This release: ~2,200,000 tokens
- Cumulative since development began: ~4,900,000 tokens

### Roadmap

v0.2.0 completes the "Joint Resolution & Dimensions" milestone
(`ROADMAP.md`). Next up: installers, accessibility, and empty/error-state
polish.

---

## Lintel 0.1.0 — First functional test build

**Summary.** The first useful installable build: draw walls, place doors and
windows, save and reopen the plan, and export a PDF sheet. Walls, doors,
windows and levels are all real — backed by a headless domain model with
workflow tests and green CI.

### Added

- Walls, doors, windows and levels with real geometry and validation
- Snapping, pan and zoom on the plan canvas
- Undo/redo history (up to 128 edits)
- Save and reopen plans as JSON, and export a PDF sheet at true scale
- Metric and Imperial unit display
- Linux x86_64 CI artefact and release

### Known limitations

- Joint resolution assumed equal wall thicknesses
- No dimension lines, installers, accessibility features or GUI polish

### Artefacts

`lintel-linux-x86_64.zip` and `SHA256SUMS.txt`.
