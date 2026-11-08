# Elite Interop and Platform Capabilities for Lintel

**Date:** 2026-11-08  
**Scope:** Interop, automation, and platform capabilities to make Lintel a credible elite paid 2D architectural drafting desktop app.  
**Repo:** ~/lintel  
**Current state v0.10.1:** walls/openings/dimensions/labels/rooms/levels, headless core, hand-written PDF 1.7 export, egui/eframe UI, suites-standard release tooling. No DXF/DWG import/export, no SVG, no IFC, no scripting, no template registry, hand-written PDF.

Cross-references: ROADMAP.md v0.1.0–v0.10.1, DOMAIN_INVESTIGATION_ELITE_CAD.md, ELITE_CAD_RESEARCH_SUMMARY.md, docs/research/elite-architecture-audit.md, docs/research/elite-cad-competitive-analysis.md.

---

## 1. DXF/DWG import & export

### Current state
Lintel has no DXF/DWG support. Export is hand-written PDF only. Import is JSON only.

### Entity mapping
Lintel domain entities map naturally to DXF:

* Walls → `LWPOLYLINE` / `LINE` (centerline) with `THICKNESS` / layer `Walls`. Wall thickness is drawn as a filled polygon in PDF; in DXF a wall can be exported as a closed polyline with lineweight = thickness.
* Openings → `INSERT` block reference of a door/window symbol, or `LWPOLYLINE` rectangle with layer `Openings`. Host relationship is annotative; DXF does not enforce host, so import maps an opening to a wall by nearest wall segment and offset along wall.
* Dimensions → `DIMLINEAR`, `DIMROTATED`, `DIMANGULAR`. Anchor references are lost on export; DXF dimensions are associative to geometry but mapping back requires `DimensionAnchor` refactor (DimensionAnchor design in DOMAIN_INVESTIGATION_ELITE_CAD.md §3).
* Layers → DXF `LAYER` table entries. Lintel has no layer system yet; minimal mapping today is to a single `0` layer.
* Blocks → DXF `BLOCK` / `INSERT`. Lintel block system is planned v0.12.0 (DOMAIN_INVESTIGATION_ELITE_CAD.md §2).

DXF entities supported by permissive crates: `LINE`, `LWPOLYLINE`, `POLYLINE`, `INSERT`, `BLOCK`, `LAYER`, `DIMLINEAR`, `DIMROTATED`, `TEXT`, `MTEXT`, `HATCH`.

### Version targets
Target DXF R12 baseline for maximum compatibility, with R2000+ for dimension styles, `LWPOLYLINE`, `MTEXT`. DXF R12 through 2018+ is supported by modern crates.

DXF R12 is ASCII only and lacks `LWPOLYLINE`, so walls are exported as `POLYLINE` with vertices. R2000+ adds `LWPOLYLINE`, dimension styles, and `MTEXT`. Export should default to DXF R2000 (`AC1015`) and import should accept R12–2018+.

DWG is proprietary binary; acadrust provides native DWG read/write but is MPL-2.0 licensed, not MIT/Apache/BSD. Risk: licensing conflict with Lintel's MIT/Apache dual license. Recommendation: avoid DWG write for now; DXF is open and sufficient for 2D exchange.

### Permissive crates

* **dxf-tools-rs** – MIT licensed. Pure Rust DXF read/write ASCII/binary. Supports 30+ entity types, complete table system (Layers, LineTypes, Blocks, TextStyles, DimensionStyles), extended data, DXF versions R12 through 2018+. Docs.rs shows MIT license [docs.rs](https://docs.rs/dxf-tools-rs/latest/dxf_tools_rs/). Homepage https://github.com/hakanaktt/dxf-tools-rs.
  Effort: Medium. DXF-tools-rs API is `CadDocument`, `DxfReader`, `DxfWriter`. Need mapping layer from Lintel `Project`/`Level` to DXF tables/entities and back. Import requires heuristic wall reconstruction from polylines and opening detection.
  Risk: Low license risk, medium mapping risk. DXF dimension anchor loss.
  Recommendation: v0.12.0. DXF export can ship in v0.11.x as read-only walls/openings/lines; full import with layers/blocks/dimensions v0.12.0.

* **dxf** crate – crates.io 0.1.0, limited, community maintained. No active maintenance. Not recommended.

* **acadrust** – supports DXF/DWG R12–2018+ read/write, 48 EntityType variants, ACIS modeling, resilient reads. **License MPL-2.0** [GitHub](https://github.com/hakanaktt/acadrust). MPL-2.0 is weak copyleft and not compatible with Lintel's MIT/Apache dual license for a combined binary without file-level separation. Therefore not usable for core interop per licence check.

Alternative permissive path: use dxf-tools-rs for DXF, defer DWG to post-1.0 if a permissive DWG reader emerges.

### Decision
DXF import/export belongs in v0.12.0 with dxf-tools-rs. DWG import/export is post-1.0 due to licence and proprietary risk. DXF version target AC1015 / R2000 for export, import R12+.

---

## 2. PDF output upgrades

### Current state
Hand-written PDF 1.7 emitter in `crates/lintel-core/src/pdf.rs` ~1 477 LOC. Single `Vec<u8>` in memory, no streaming. No PDF/A, no layers, no embedded fonts subsetting. `escape_text` exists but no validation.

### Upgrade goals
PDF/A for archival, PDF layers (Optional Content Groups / OCG) for layer toggle in viewers, vector fidelity, embedded fonts.

### Permissive crates

* **krilla** – MIT/Apache-2.0 dual licensed [GitHub](https://github.com/LaurenzV/krilla). High-level PDF creation on top of `pdf-writer`. Supports PDF/A-1/2/3/4, PDF/UA-1, document outlines, annotations, metadata, tagged PDF, PDF versions 1.4–2.0. Zero `unsafe`. Current MSRV 1.92. Tested via snapshot + visual regression against 6 viewers.
  Effort: High. Migrating from hand-written emitter to krilla requires re-implementing `draw_sheet`, `draw_frame`, `draw_title_block`, schedule table, north arrow, scale bar. Krilla provides fills/strokes/paths/transforms/alpha/clip/gradients/text API.
  Risk: Low license, high effort. Hand-written PDF is already working; krilla reduces maintenance and adds PDF/A/UA. OCG layers are supported at pdf-writer level; krilla exposes blend modes/layer isolation but explicit OCG creation may need pdf-writer low-level access.

* **pdf-writer** – MIT/Apache-2.0, low-level, used by krilla. Direct OCG support likely present.

* **printpdf** – MIT/Apache-2.0, similar abstraction, based on lopdf. Alternative.

* **pdf-rs / pdf-oxide** – Apache-2.0 / MIT, more low-level.

Recommendation: adopt krilla for v0.12.0. It is permissive, actively maintained, and provides PDF/A support. PDF layers can be added once layer system exists in domain (v0.12.0). Hand-written PDF can be kept for v0.11.x.

### Effort / risk / timing
* v0.11.x: keep hand-written PDF, add PDF/A metadata fields via hand edit.
* v0.12.0: migrate to krilla, add PDF/A-2b output, embedded font subset.
* Post-1.0: PDF layers via OCG tied to domain Layer visibility.

---

## 3. SVG export

SVG is ideal for web preview, illustration handoff, and client review.

Permissive crates:

* **resvg** – MIT/Apache-2.0 [GitHub](https://github.com/linebender/resvg). SVG rendering library, pure Rust. Can be used for rendering SVG to raster, but for *export* we need to write SVG. resvg is mainly a renderer; SVG writing can be done with `svg` crate (MIT) or hand-write.

For export, `svg` crate (MIT) provides `Document` builder for paths/rects/text.

Mapping: walls → `<path>` with stroke/fill, openings → `<rect>`, dimensions → `<line>` + `<text>`, layers → `<g id="layer">`.

Effort: Low. SVG is simple vector, no font embedding complexity. Export can be headless and tested.

Recommendation: v0.11.x. SVG export is a nice-to-have for web preview and does not depend on layers/blocks.

---

## 4. IFC

### Relevance
IFC is BIM handoff. For a 2D drafting tool, IFC export is a differentiator for architectural credibility. Minimal scope: export walls, doors, windows as IfcWall, IfcDoor, IfcWindow with placement. No full BIM.

Permissive crates:
* **ifc-lite-processing** – part of ifc-lite project. Parsing, viewing, querying, validating, exporting IFC entirely client-side. Rust core. License typically MIT/Apache for crates in ecosystem. [ifclite.dev](https://ifclite.dev/docs/api/rust/).
* **bimifc-parser** – fast memory-efficient parser, likely MIT.

Risk: High complexity. IFC schema is large; minimal export is feasible but requires placement, units, GUIDs, property sets. Import is out of scope for 2D.

Recommendation: Post-1.0. IFC is a later milestone after layers/blocks and parametric dimensions stabilize. If added, start with export-only, IFC2x3.

---

## 5. Scripting/automation

### Goals
Macros, parametric scripting, headless CLI batch export.

Permissive crates:

* **mlua** – MIT licensed [GitHub](https://github.com/mlua-rs/mlua). Safe high-level bindings to Lua 5.4. Widely used. Good for embedding. Effort: medium. Requires exposing Lintel `Project` API to Lua with safe wrapper.

* **Rhai** – MIT/Apache-2.0 dual licensed [GitHub](https://github.com/rhaiscript/rhai). Embedded scripting language for Rust, safe, fast, no external dependency. `Engine::new()` + `register_fn`. Good for macros and simple automation.

Recommendation: Rhai for v0.12.0. Simpler to embed, permissive, Rust-native. mlua is viable but adds Lua dependency.

Headless CLI mode: `lintel export plan.lintel.json --format pdf --sheet A3` can be a thin `apps/cli` binary using `lintel-core` directly. No UI dependencies. This is a pure thin-shell and fits thick-core architecture. Recommend v0.11.x for CLI export, v0.12.0 for Rhai scripting.

Risk: Scripting surface area increases support burden. Keep scripting read-only for v0.12.0; no model mutation via script initially.

---

## 6. Template & block library

Paid CAD tools ship title blocks, detail libraries, symbol catalogs.

Packaging format: `.lintel-template` = JSON manifest + assets. Could reuse existing JSON project format with a `template: true` flag and a `TemplateManifest` referencing a title block SVG/PDF and default layers/blocks.

Implementation steps:
* Define `Template` struct: name, author, paper sizes, default layers, block definitions.
* Store user templates in `~/.config/lintel/templates/`.
* UI picker for new project from template.

Effort: Medium. Depends on block system and layer system.

Recommendation: v0.12.0 for template support, post-1.0 for block library marketplace.

---

## 7. Cloud/sync optionality

Lintel must remain offline-first. Opt-in sync could be:

* Local-first with optional encrypted sync via e.g., WebDAV, S3, or a self-hosted git repo. No cloud service dependency.

* Project files are JSON and small; they can be synced via any file sync tool.

* Collaboration: read-only share via PDF export; real-time collaboration is post-1.0 and high risk.

Recommendation: No cloud features in v0.11/v0.12. Keep offline-first. Document sync best practices. Post-1.0 consider an optional `lintel sync` CLI using rclone-style backends.

---

## 8. Cross-platform polish

Current state: macOS `.app` bundle shipped v0.3.0, Windows installer v0.5.0, Linux AppImage v0.4.0. Keyboard shortcuts work, canvas focusable, screen-reader level summary.

Gaps for elite paid feel:

* Native menus: macOS menu bar integration via `egui` native menu support. `eframe` provides `MenuBar`.
* File associations: register `.lintel.json` open with app on install.
* OS dark mode: egui theme adapts.
* HiDPI: egui handles, verify on 4K.
* Tablet/stylus: egui supports pointer events; need pressure and palm rejection testing.
* Trackpad gestures: pinch-to-zoom, two-finger pan.

Recommendation: v0.11.x polish items, v0.12.0 native file associations.

---

## 9. Licensing/packaging

Current: GitHub Actions release with AppImage, `.app`, Inno Setup installer. No code signing evident. No portable mode.

Permissive path:

* Windows: sign with Authenticode, Inno Setup or MSIX. Winget/choco distribution.
* macOS: notarize, sign with Apple Developer ID, distribute via Homebrew cask.
* Linux: AppImage, Flatpak, or deb/rpm.

Effort: Medium, largely CI.

Recommendation: v0.12.0 code signing and notarization. Portable mode post-1.0.

---

## 10. Accessibility platform features

Current: `ACCESSIBILITY.md` documents keyboard shortcuts, focus rings, AccessKit integration via egui. Canvas focusable with screen-reader level summary. No per-object labels.

Platform expectations:

* AccessKit for screen-reader tree. egui already uses AccessKit.
* High-contrast theme toggle.
* Reduced motion respect.
* Per-entity labels for walls/openings/dimensions via `accesskit::Node`.

Recommendation: v0.11.x add per-object labels for canvas objects, v0.12.0 high-contrast theme.

---

## Summary roadmap

**v0.11.x**
* SVG export (low effort)
* Headless CLI export
* Cross-platform polish: native menus, file associations
* Accessibility per-object labels
* PDF/A metadata via hand-written emitter

**v0.12.0**
* DXF import/export via dxf-tools-rs (MIT)
* Migrate PDF to krilla (MIT/Apache), PDF/A-2b, embedded fonts
* Rhai scripting engine (MIT/Apache)
* Minimal layers + template support
* Code signing/notarization
* Native file associations

**Post-1.0**
* IFC export
* DWG import/export via permissive crate if available
* PDF layers tied to domain layers
* Cloud sync optionality
* Block library marketplace

All crate recommendations respect MIT/Apache/BSD licence requirement. acadrust is excluded due to MPL-2.0. dxf-tools-rs MIT provides DXF coverage. krilla MIT/Apache provides PDF/A. Rhai MIT/Apache provides scripting. resvg MIT/Apache provides SVG rendering.

---

*Report generated by subagent research. No application code modified.*
