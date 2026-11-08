# Elite Paid 2D Architectural Drafting Tools — Competitive Analysis for Lintel

**Date:** 2026-11-08  
**Target:** Lintel v0.10.1 clean-room Rust+egui 2D CAD  
**Scope:** Feature sets, differentiators, ecosystem expectations, UX conventions, commercial models, and gap analysis for credibility as a paid-desktop CAD alternative.  
**Sources:** Public 2026 product pages, pricing pages, and documentation. Repo state read from `~/lintel/ROADMAP.md`, `PROGRESS.md`, `crates/lintel-core/src/`.

---

## 1. Executive summary

Lintel v0.10.1 is a headless-command core with walls, openings, dimensions, labels, rooms, levels, and multi-sheet PDF export. It ships with suite-standard tokens, crash/update infrastructure, and a release pipeline. It is architecturally mature but functionally sparse compared to the table-stakes expected by architectural reviewers: no layers, no blocks/symbols, no construction geometry, limited snapping, static dimensions, no paperspace/sheet sets beyond PDF export, no DXF/DWG import/export, no scripting/automation, and no standards compliance beyond JSON persistence.

Professional 2D drafting tools are judged on precision tooling, file interop, and workflow reliability — not render quality. The must-have set for credibility is narrow but non-negotiable. The paid differentiators are parametric relationships, automation, and ecosystem depth.

---

## 2. Market landscape 2026 — feature snapshot

### AutoCAD / AutoCAD LT

*Pricing 2026:* AutoCAD $2,095/year subscription; AutoCAD LT $540–$540–$560/year depending on region. Both annual subscriptions; 3-year term available with price protection. No perpetual option [search web results:0].
Features: Precision 2D drafting, DWG native, 7 industry toolsets, compare drawings, counting, tables, AutoLISP customization, web/mobile access, multi-user markup via Forma Data Management Essentials [search web results:0].
2026 new in LT: cross-platform subscription benefits, customization/AutoLISP, installer refinements [search web results:0].
Table stakes: command line, osnaps, grips, layers, xrefs, paperspace/modelspace, blocks, attributes, dimensions with associative anchors, linetypes/lineweights, field updates, PDF underlay, DWG/DXF import/export.

### Revit 2026 (BIM)

Standalone $2,915/year; AEC Collection $3,375/year. Revit LT $560/year [search web results:1].
Key differentiators: parametric BIM, auto-updated plans/sections/schedules, worksharing, MEP/structural toolsets. 2026 additions: ReCap Pro Mesh import, accelerated graphics, toposolid enhancements [search web results:1].
While BIM not Lintel's target, architectural reviewers expect 2D export and IFC round-tripping.

### Vectorworks Architect 2026

Architect $1,530/year; Fundamentals $1,080/year; Design Suite $1,980/year [search web results:2].
Parasolid kernel, parametric BIM walls/doors/windows with data, OPEN BIM IFC/BCF, Python/API scripting, native macOS. New 2026: Depth Cueing, Door/Window Assemblies, Sustainability Dashboard [search web results:2].
Ecosystem expectations: IFC import/export, BCF issue tracking, sheet layers, viewports.

### SketchUp Pro / LayOut 2026

Pro $399/year; Studio $899/year [search web results:3].
Pro includes desktop modeler, LayOut 2D documentation, extensions, CAD exchange, PreDesign, IFC workflows. Not a precision 2D drafting tool, but LayOut is the de-facto 2D documentation path for SketchUp. UX conventions: inference engine, scenes, tags/layers.

### Rhino 8 / Grasshopper 2026

Rhino 8 one-time $995; upgrades $395–$595 [search web results:4].
8 additions: ShrinkWrap, SubD Creases, Mac Metal speed boost, PushPull workflows, better drafting tools, customizable UI, faster renderer, new Grasshopper data types [search web results:4].
Rhino is used for 2D drafting via viewports and layout sheets. Differentiators: NURBS, parametric scripting, extensive file interop.

### ArchiCAD 2026

ArchiCAD Studio $2,414 upfront for 1-year term ($201/mo billed), Collaborate $2,840, Max $5,700 [search web results:5].
Graphisoft unveiled 2026 portfolio with AI-enabled design intelligence, Dark Mode for Windows, multi-story wall visibility, Project Info management, Navigator organization, expanded drawing-label options [search web results:5].
OPEN BIM focus, IFC native, BIMcloud collaboration.

### Affinity Designer 2 2026

Now free for everyone; historically $49.99 one-time [search web results:6].
Vector precision, typographic controls. Not CAD-focused: no osnaps, no layers with CAD semantics, no DWG/DXF native. Useful for graphic annotation but lacks table-stakes CAD infrastructure.

### BrisCAD / progeCAD / nanoCAD

progeCAD Professional $540 one-time perpetual; DWG-native, AutoCAD compatibility [search web results:7].
BricsCAD tiered Lite/Pro/BIM/Mechanical, perpetual available, mid-range pricing ~€314–€1,120 [search web results:7].
nanoCAD base $249/year; free edition available [search web results:7].
All are AutoCAD workalikes: DWG/DXF native, layers, xrefs, blocks, command line, osnaps, paperspace.

### Chief Architect Premier 2026

$1,995/year; $166.25/mo billed annually [search web results:8].
Residential construction docs, automatic roofs/foundations/framing, dimensions, schedules, materials lists, 3D/2D sync. Differentiators: smart cabinets, manufacturer catalogs, automated construction documents.

---

## 3. Must-have table-stakes for 2D architectural drafting

These are the capabilities reviewers label "not a real CAD tool" without.

### Core geometry & editing

* Robust line/polygon/arc creation with endpoint-midpoint-intersection-perpendicular snapping
* Entity selection by window/crossing/fence, with additive/subtractive Ctrl/Shift selection
* Move/copy/rotate/mirror/scale with grips at entity handles
* Undo/redo with command history, journaling
* Object properties: layer, color, linetype, lineweight, transparency
* Construction geometry: infinite lines/rays, non-printing reference

### Drafting precision

* Osnaps set: Endpoint, Midpoint, Center, Intersection, Perpendicular, Tangent, Quadrant, Node
* Ortho, polar, grid snapping with tolerance scaling
* Dynamic input and command line with aliases
* Dimensioning: aligned/horizontal/vertical/radial/arc; associative anchors to entities; extension lines with gap/overrun; witness line style control
* Text/annotations with style, justification, leader lines
* Hatch/fill patterns with scale/angle

### Organization & documentation

* Layers with visibility/lock/freeze/printability, layer filters, per-layer properties
* Blocks/symbols: named definitions, instances with scale/rotation/mirror, block editor, block library
* Xrefs/external references with overlay/attach, path binding
* Paperspace/modelspace or sheet layers: multiple layouts, viewports with scale
* Sheet sets: cover, north arrow, scale bar, title block, multi-sheet PDF export
* Dimension styles, text styles, linetype definitions

### File interop & standards

* DWG/DXF import/export with entity fidelity
* PDF import as underlay, PDF export with layers and vector accuracy
* IFC export for BIM handoff, at minimum IFC2x3/IFC4 geometry with walls/doors/windows
* SVG for web/preview

### Reliability

* File validation on load, with user-facing errors
* Save/load round-trip, versioning, autosave
* Performance on typical domestic plans <500 walls with snapping pre-compute

Source: Lintel current `Level` holds walls/openings/dimensions/labels only, no layers/blocks/construction [repo files: ROADMAP.md CAP-001..CAP-074].

---

## 4. Differentiators that justify paid pricing

* **Parametric relationships**: geometric constraints, associative dimensions, block parameters. AutoCAD has constraints; Vectorworks and ArchiCAD have parametric BIM objects.
* **Automation**: scripting/LISP/Python/API, macros, batch processing. AutoCAD AutoLISP, Vectorworks Python, BricsCAD .NET API.
* **Standards compliance**: IFC certification, PDF/A, DWG version compatibility, BCF issue tracking.
* **Templates & libraries**: architectural title blocks, detail libraries, symbol catalogs, manufacturer content.
* **Collaboration**: worksharing, cloud sync, xrefs with binding, markup workflows.
* **Specialized toolsets**: MEP, structural, landscape, rendering, analysis.
* **Support & training**: 8x5 support, SSO, update cadence, education licensing.

Lintel currently has none of these except basic PDF export.

---

## 5. Ecosystem expectations

### File formats

* **DWG/DXF**: De facto exchange. Open format specs exist but native DWG is proprietary. Permissive Rust crates: `acadrust` — pure Rust DXF/DWG read/write, MIT/Apache permissive [search web results:10].
* **PDF**: Professional documentation. Rust permissive options: `krilla` supports PDF/A-1/2/3/4, PDF/UA-1, MIT/Apache [search web results:9]; `pdfrs` Apache-2.0 [search web results:9]; `pdf_oxide` MIT/Apache [search web results:9]. Lintel already hand-writes PDF 1.7; migration to a permissive library would reduce maintenance.
* **IFC**: buildingSMART ISO 16739. Rust parsers: `ifc-lite-processing` — parse, view, query, validate, export; MIT/Apache typical for Rust crates [search web results:9]. Pure-Rust reader `oxideav-ifc` clean-room ISO 10303-21 [search web results:9].
* **SVG**: web preview. Rust `resvg` permissive.

### Libraries permissive license check

* `acadrust` MIT/Apache [search web results:10].
* `krilla` high-level PDF, PDF/A support, MIT/Apache [search web results:9].
* `ifc-lite-processing` permissive; `bimifc-parser` fast parser [search web results:9].
* No GPL dependencies acceptable for Lintel's Apache/MIT dual license.

### Plugins & scripting

Paid tools expose APIs: AutoCAD .NET/LISP, Rhino Grasshopper, Vectorworks Python. Lintel clean-room policy prohibits copying, but an API surface for extensions should be planned.

### Templates & blocks

Paid tools ship with architectural title blocks, detail libraries. Lintel's drawing set v0.9.0 adds cover/north arrow/scale bar but no user-editable templates [repo ROADMAP v0.9.0].

---

## 6. UX conventions professionals expect

* Command line with aliases, command history, transparent input
* Grips for move/rotate/scale on selection
* Osnap aperture with priority order, snap markers
* Layers palette with filters, lock/freeze
* Blocks palette with insertion point, scale, rotation
* Paperspace: model space creation, viewport scale, annotative scaling
* Quick properties panel, contextual inspector
* Status bar with coordinates, snap mode toggles, current layer
* Keyboard shortcuts: copy/paste, undo/redo, select all, zoom extents

Lintel's current UI: egui with toolbar, inspector, canvas. Keyboard shortcuts exist, canvas is focusable, but no grips, no command line, no osnap priority beyond endpoint/ortho/grid [repo crates/lintel-core/src/snap.rs].

---

## 7. Commercial model patterns

* **Subscription only**: AutoCAD $2,095/yr, LT $540/yr, Revit $2,915/yr [search web results:0,1].
* **Subscription with 3-year lock**: Autodesk, Graphisoft price protection [search web results:0,5].
* **Perpetual with maintenance**: Rhino $995 one-time + support, BricsCAD perpetual tiered [search web results:4,7].
* **Freemium**: SketchUp Free browser, Affinity now free [search web results:3,6].
* **Education**: heavily discounted, often free for students.
* **Updater & telemetry**: opt-in telemetry, signed updates, SHA256SUMS verification, release notes in-app. Lintel already has update-check and crash-report crates, token ledger, version UI bottom-right [repo PROGRESS.md v0.10.0].

---

## 8. Gap analysis for Lintel

### What exists

From `ROADMAP.md` and `PROGRESS.md` v0.1.0–v0.10.1:

* Walls with thickness/height, joint extensions for unequal thickness
* Doors/windows hosted on walls with move/size/flip
* Levels/storeys, unit system metric/imperial
* Dimensions: aligned/horizontal/vertical with offset, undo/redo, hit testing priority
* Labels, auto-detected rooms, opening schedule
* Multi-sheet PDF drawing set with cover, north arrow, scale bar
* Command-driven core, JSON persistence, headless tests
* Accessibility labels, keyboard shortcuts, release infrastructure

### Critical gaps vs table-stakes

1. **Layers** — No layer system. Entities cannot be organized visually or for plotting. Professional reviewers expect freeze/lock/visibility per layer.
2. **Blocks/symbols** — No reusable symbol definitions. Doors/windows are placed as individual openings, not library items.
3. **Construction geometry** — No infinite lines/rays for reference drafting.
4. **Enhanced snapping** — Snap limited to Endpoint, Ortho, Grid. Missing Midpoint, Intersection, Perpendicular, Tangent, Center. Snap priority system absent.
5. **Associative dimensions** — Dimensions store free points, not anchors to entities. Moving a wall leaves dimensions stale. No witness line gap/overrun control.
6. **Selection filtering** — No window/crossing/fence selection; single selection via hit test only.
7. **File interop** — JSON only. No DWG/DXF import/export, no IFC export, no PDF import as underlay.
8. **Paperspace/sheet sets** — PDF export exists but no editable sheet layout, viewports, annotative scaling.
9. **Command line & grips** — No command line, no grips for direct manipulation.
10. **Entity properties** — No per-entity style overrides, linetype, lineweight.

Source: `DOMAIN_INVESTIGATION_ELITE_CAD.md` identifies these as v0.11.1/v0.12.0 targets [repo DOMAIN_INVESTIGATION_ELITE_CAD.md].

### What would move Lintel to "credible paid desktop CAD"

Minimum viable professional credibility:

* **v0.11.x**: Construction geometry, enhanced snapping (midpoint/intersection/perpendicular), constraint-driven dimension anchors, window/crossing selection. These are low-risk domain additions, pure Rust, maintain thick-core.
* **v0.12.x**: Minimal block/symbol system with named definitions and instances; minimal layer system with default layer and per-entity layer_id migration; entity style struct.
* **v0.13.x**: DWG/DXF import/export via `acadrust`; IFC export via `ifc-lite-processing`; PDF/A export via `krilla`.
* **v0.14.x**: Paperspace/sheet layout with viewports, annotative scaling, title block templates.

The `ELITE_CAD_RESEARCH_SUMMARY.md` prioritizes construction geometry, enhanced snapping, constraint-driven dimensions, selection filtering for v0.11.1, and blocks/layers/grouping for v0.12.0 [repo ELITE_CAD_RESEARCH_SUMMARY.md].

---

## 9. Standards and interop specifics

### DXF/DWG

* DXF is ASCII/binary open spec; DWG is proprietary but reverse-engineered.
* `acadrust` provides pure Rust read/write for DXF and native binary DWG, MIT/Apache [search web results:10].
* `libdxfrw` C++ open source for DXF, limited DWG [search web results:11].
* Permissive license is required to stay MIT/Apache. No GPL.

### PDF/A

* `krilla` supports PDF/A-1/2/3/4 and PDF/UA-1 [search web results:9].
* Lintel's hand-written PDF 1.7 works but lacks PDF/A metadata, tagging, and accessibility compliance.

### IFC

* IFC is STEP-based. Rust parsers exist with permissive licenses.
* `ifc-lite-processing` Rust core compiled to WASM for parse/view/query/validate/export [search web results:9].
* Pure-Rust IFC reader `oxideav-ifc` clean-room from ISO 10303-21 [search web results:9].
* buildingSMART validation service exists for conformance [search web results:9].

### SVG

* `resvg` permissive; used for web preview.

### Licensing notes

All recommended libraries are MIT/Apache-2.0 compatible, matching Lintel's dual license.

---

## 10. Recommendations

* Prioritize table-stakes before differentiators: layers, blocks, construction geometry, enhanced snapping, associative dimensions.
* Keep thick-core discipline: all features are pure domain types, commands, validation, serde-compatible.
* Adopt permissive libraries for interop rather than hand-rolled parsers.
* Maintain clean-room: no proprietary source text or pixel-accurate UI copy. Use public docs for workflows.
* Track token metrics and release parity as per suite standards.

---

## 11. Sources

* AutoCAD pricing and features 2026: Autodesk product pages [search web results:0].
* AutoCAD LT pricing 2026: Autodesk LT overview [search web results:0].
* Revit pricing 2026: MyArchitectAI, Autodesk [search web results:1].
* Revit LT pricing: Autodesk Revit LT [search web results:1].
* Vectorworks 2026 features: Vectorworks newsroom [search web results:2].
* Vectorworks pricing 2026: Render a House [search web results:2].
* SketchUp pricing 2026: SketchUp plans, vagon.io [search web results:3].
* Rhino 8 new features: Rhino 3D website [search web results:4].
* Rhino pricing 2026: Render a House [search web results:4].
* ArchiCAD 2026 pricing: Graphisoft pricing page [search web results:5].
* ArchiCAD 2026 portfolio: Graphisoft newsroom [search web results:5].
* Affinity Designer pricing: Affinity Studio, Canva newsroom [search web results:6].
* progeCAD comparison: progeCAD site, SmartCADSoft [search web results:7].
* BricsCAD/nanoCAD pricing: Alcads, Kulman Lab [search web results:7].
* Chief Architect pricing 2026: Chief Architect site, Render a House [search web results:8].
* IFC Rust libraries: crates.io ifc-lite-processing, OxideAV GitHub [search web results:9].
* PDF Rust libraries: krilla GitHub, pdfrs crates.io, pdf_oxide [search web results:9].
* DWG/DXF Rust library: acadrust crates.io [search web results:10].
* LibDXF/DWG: codelibs libdxfrw GitHub [search web results:11].

Repo evidence: `~/lintel/ROADMAP.md`, `PROGRESS.md`, `crates/lintel-core/src/`, `ELITE_CAD_RESEARCH_SUMMARY.md`, `DOMAIN_INVESTIGATION_ELITE_CAD.md`, `DOMAIN_DESIGN_SKETCHES.md`.

---

*Report generated for Lintel competitive positioning. No application code modified.*
