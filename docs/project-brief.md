# Project Brief

**Working title:** Lintel — a desktop drafting application for architectural floor plans

**Primary user:** Architects, draftspeople, and home designers who need to create, edit, and export 2D architectural plans with walls, openings (doors/windows), levels, and PDF rendering.

**Problem it solves:** Replaces manual CAD drafting with a modern, native desktop application that stores plans as portable JSON files, provides undo/redo history, crash reporting, and update checking — all while running headless in CI for automated QA.

**Part of a suite:** Yes — this is the first app in a suite that will share `crash-report` and `update-check` crates with future applications.

**Key capabilities (v0.1.0 scope):**
- Create and edit walls with configurable thickness and height
- Place doors and windows on walls with offset positioning
- Multiple levels (storeys) with independent wall sets
- Save/load projects to JSON format
- Render PDF plans with title block, scale, and sheet numbers
- Undo/redo history (up to 128 steps)
- Unit display in Metric or Imperial
- Basic geometry validation and intersection detection

**Inspiration products:** Open-source architectural drafting tools; public documentation of similar CAD/ BIM applications

**Out of scope (v0.1.0):** Multi-storey complex joint resolution, dimension line rendering, full SVG export, web/WASM target, plugin architecture, commercial licensing

**Technical preferences:** Pure Rust + egui/eframe + wgpu; primary platform: human's local OS (Linux/macOS/Windows); offline-first with optional GitHub-backed crash reporting and update checks; MIT/Apache-2.0 dual licence