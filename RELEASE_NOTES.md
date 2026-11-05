<!-- This file is attached verbatim to every GitHub Release by
     .github/workflows/release.yml (--notes-file). The newest release belongs at
     the top. Everything above the first release heading stays invisible on
     GitHub because it is an HTML comment. -->

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
