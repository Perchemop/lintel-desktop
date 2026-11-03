# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelot.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## Unreleased

_(No entries yet — first unreleased version will be v0.1.0)_

## v0.1.0 — YYYY-MM-DD

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