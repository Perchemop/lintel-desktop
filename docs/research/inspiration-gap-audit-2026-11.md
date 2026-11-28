# Inspiration Gap Audit — Lintel v0.19.0 vs 8 Competing CAD Products

**Audit date:** 2026-11-29
**Method:** Clean-room behavioral summaries from public documentation only; every claim cites source product + public URL. No proprietary reverse-engineering, no pixel-copying, no internals exposure.
**Scope:** 2D architectural floor-plan CAD (Rust/egui). 2D restriction enforced: no 3D, no WASM/web, no plugin architecture, no real-time collaboration.
**Source inventory:** `/home/leonardolscinto/lintel/docs/capability-inventory.md` (CAP-001 through CAP-135, all "implemented" through v0.19.0). Cross-referenced against existing research files in `/home/leonardolscinto/lintel/docs/research/`.

---
## Section 1 — Snapping and Inferencing

**Theme coverage:** AutoCAD object snaps, SketchUp inferences, QCAD snap modes, Home-design magnetism, Blender area snapping.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 1 | **Command-line autocomplete + suggestion list** — As user types a command name, the line auto-completes and a suggestion list appears with matching commands. | AutoCAD (autocad-drafting-core.md) | ❌ Not in inventory (inventory has CAP-097 command palette but not autocomplete-as-you-type) | High — improves discoverability for new users; frequent command entry | Medium — requires command-line input buffer overhaul |
| 2 | **Dynamic input tooltip overrides** — Tooltips display near cursor showing dynamic dimensions and prompts that update as mouse moves, with override capability via right-click or Tab. | AutoCAD (autocad-drafting-core.md) | ❌ Not in inventory | High — reduces need for command line; core UX pattern in modern CAD | Medium — egui widget overlay + state tracking |
| 3 | **Object snap tracking (extended object snap)** — After selecting an object snap, temporary tracking lines appear parallel/perpendicular from the snap point, extending indefinitely until another point is selected. | AutoCAD (autocad-drafting-core.md) | ❌ Not in inventory (CAP-094 covers endpoint/midpoint/etc. snap candidates but not tracking lines) | High — core precision drafting tool; widely expected | Medium — temporary line rendering in egui canvas |
| 4 | **Inference locking (pin inference)** — User can pin a specific inference type to prevent it from changing as cursor moves, useful for complex inferencing scenarios. | SketchUp (sketchup-inferencing.md) | ❌ Not in inventory | Medium — power-user feature; reduces inference cycling | Low-Medium — inference state management |
| 5 | **OnFace / OnEdge / Tangent-to-Face / Perpendicular-to-Face inferences** — Inferences specific to faces and edges of 3D-like geometry, including tangent/perpendicular to selected face. | SketchUp (sketchup-inferencing.md) | ❌ Not in inventory (2D-inferred; Lintel is 2D-only but edge inference paths could map) | Medium — enhances precision without 3D geometry | Low — new inference types in snap engine |
| 6 | **Furniture/component libraries with search and magnetism** — Tree-organised catalog of architectural components (doors, windows, fixtures); drag-and-drop with wall-magnetism auto-rotation and elevation. | Home Design UX (home-design-ux.md) | ❌ Not in inventory (CAP-007/008 cover door/window placement *on walls* but not catalog-based insertion) | High — immediate value for architectural users; differentiates from bare-bones CAD | High — requires new UI components, search, drag-drop, magnetism logic |
| 7 | **Multi-view 2D orthogonal/iso/plan view switching** — Quick toggle between plan view, isometric view, and orthogonal projections (FR/BR/FL/BL) with saved view states. | Home Design UX (home-design-ux.md) | ❌ Not in inventory (Lintel is plan-view-first; no explicit view-mode switching) | Medium — helps visualize wall relationships; common in architectural tools | Low-Medium — camera/projection changes in egui |

---
## Section 2 — Dimensions and Annotation

**Theme coverage:** AutoCAD dimension styles, QCAD dimension types, annotation scaling, baseline/continued dimensions.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 8 | **Baseline and continued dimensioning** — Place a baseline dimension from the last selected extension line; continued dimension adds successive extensions from the same base. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory (CAP-026/027 cover basic dimension placement/move/delete but not baseline/continued workflows) | High — standard architectural dimensioning pattern | Medium — dimension command extensions |
| 9 | **Annotation scaling / annotative dimensions** — Dimensions that automatically scale with viewport annotative scale; annotative shape, size, and text height properties. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory | High — essential for paper-space workflows; widely expected in architectural CAD | Medium — annotative flag + scale factor in dimension properties |
| 10 | **Ordination dimensions (ordinate dimensions)** — Dimensions that show X or Y coordinate distances from a origin point, with leader and numerical value. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory | Medium — common in detail drawings; less critical for floor plans | Low-Medium — new dimension type |
| 11 | **Angular dimensions with continuous arc selection** — Dimension the included angle between two selected lines/arcs, with continuous preview as third point moves. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory (CAP-026 covers basic dimension lines but not angular) | High — essential for architectural detailing (e.g., roof pitches) | Medium — angle calculation + rendering |
| 12 | **Hatching and fill patterns** — Apply predefined or custom hatch patterns (solid, gradient, brick, concrete, etc.) to closed regions; hatch scale, angle, and double-hatch support. | QCAD/LibreCAD (qcad-librecad-2d.md) | ❌ Not in inventory | High — architectural patterns (brick, concrete, tile) are core to floor-plan visualization | Medium — hatch rendering in egui + JSON persistence |
| 13 | **Leader lines with customizable arrowheads / text** — Multi-segment leaders with adjustable arrowhead styles, text content, and text alignment; curved leaders supported. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory (CAP-070 covers free text labels but not leaders) | Medium — common for callouts and references | Low-Medium — leader widget + path rendering |

---
## Section 3 — Layers and Entity Management

**Theme coverage:** AutoCAD layer states, layer groups, QCAD property editor, block management.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 14 | **Layer states (save/restore layer configurations)** — Save a named configuration of layer on/off, freeze/thaw, color, linetype, lineweight per layer; restore later. | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory (CAP-089 covers basic layer per-entity assignment but not state save/restore) | High — essential for complex drawings with many layers; workflow accelerator | Medium — state serialization + document persistence |
| 15 | **Layer groups with group filters** — Layers organized into named groups; filter visibility/freeze by group (e.g., "Architectural", "Electrical", "Dimension"). | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory (CAP-089 is flat layer list) | High — architectural firms routinely group layers by discipline | Medium — group model + filter logic in layer manager |
| 16 | **Property editor for entity properties** — Dedicated panel showing all properties of selected entity (layer, linetype, lineweight, color, transparency, plot style, geometry params). | QCAD/LibreCAD (qcad-librecad-2d.md) | ❌ Not in inventory (CAP-056/057 cover canvas focus and toolbar summary but not per-entity property grid) | Medium — improves inspection/edit efficiency | Medium — property grid widget + entity reflection |
| 17 | **Linetype and lineweight management** — Load/custom linetypes (dashed, dash-dot, etc.); assign lineweight per-entity; lineweight plotted/screen display control. | QCAD/LibreCAD (qcad-librecad-2d.md) | ❌ Not in inventory (CAP-004/005 cover thickness but not linetypes) | Medium — common in architectural standards | Medium — linetype database + entity assignment |
| 18 | **External references (Xref) attachment and overlay** — Attach external drawings as references; control overlay vs attach, transparency, and clip boundaries. | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory (2D scope restriction note: explicitly excluded from v0.1.0 scope; re-evaluating for post-0.19) | High — critical for multi-file architectural projects; may be out of scope | High — entire Xref architecture; may defer |
| 19 | **Plot style tables (CTB/STB) with plot configurations** — Color-dependent plot styles (CTB) or named plot styles (STB); pen assignments, lineweight overrides, paper size settings. | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory | Medium — needed for production documentation; less critical for initial release | Medium — plot style mapping + PDF export integration |

---
## Section 4 — Blocks and Components

**Theme coverage:** AutoCAD dynamic blocks, QCAD block management, Home Design catalog, Blender instances.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 20 | **Dynamic blocks / parametric blocks with grips** — Block definitions with grips that allow stretch, rotate, flip, and parameter-driven size changes; behaviors saved in block definition. | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory (CAP-116/117 cover static block definitions/instances but not dynamic/gripped blocks) | High — blocks are core to architectural drafting; dynamic blocks reduce duplicate definitions | High — significant command + geometry infrastructure |
| 21 | **DesignCenter browser for content** — Browsable tree of drawing content (blocks, xrefs, layers, styles); drag-drop to insert into current drawing. | AutoCAD (autocad-management-settings.md) | ❌ Not in inventory | Medium — content discovery tool; workflow accelerator | Medium — tree browser + drag-drop into canvas |
| 22 | **Block scale and rotate from inspector** — Selected block instance scales and rotates via inspector numeric fields, with uniform/non-uniform toggle and reference-point selection. | QCAD/LibreCAD (qcad-librecad-2d.md) | ❌ Not in inventory (CAP-117 places block instance but does not edit post-placement via inspector) | Medium — common post-placement edit pattern | Low-Medium — inspector field additions |
| 23 | **Mirror block instance across local axis** — Mirror a block instance across its local x-axis or y-axis, with flip state and undo support. | QCAD/LibreCAD (qcad-librecad-2d.md) | ❌ Not in inventory (CAP-122 covers mirror but may be geometry-only; verify) | Medium — symmetry is common in architectural layouts | Low-Mirror — flip toggle in block instance fields |

---
## Section 5 — Settings, Options, and Customization

**Theme coverage:** AutoCAD CUI/PGP customization, dimension styles, layer states, QCAD preferences.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 24 | **Dimension style manager with per-style overrides** — Manager with tabs for Lines, Arrows, Text, Fit, Primary Units, Alternate Units, Tolerances; per-style overrides; style families per use case. | AutoCAD (autocad-annotation-output.md) | ❌ Not in inventory (CAP-026/027 cover dimension placement but not style management) | High — architectural firms have strict dimension standards | Medium — style struct + manager UI |
| 25 | **Keyboard shortcut configuration / override** — User-configurable shortcuts for commands; override default Ctrl+N/O/S/Shift+S/E with custom key assignments; save/load shortcut configs. | AutoCAD (autocad-management-settings.md) + Home Design UX (home-design-ux.md) | ❌ Not in inventory (CAP-055 covers fixed file shortcuts Ctrl+N/O/S/Shift+S/E but not reconfigurable) | High — power-user expectation; accessibility accelerator | Medium — shortcut map + config persistence |

---
## Section 6 — File Output, Keyboard/Command Model, Parametrics/Constraints, Home-Design Interactions

**Theme coverage:** PDF export, command repetition, units, project JSON, constraint persistence, home-design interactions.

| # | Proposed capability | Source product | Status vs inventory | Impact | Effort |
|---|---|---|---|---|---|
| 26 | **PDF export reachability from File menu** — File → Export sheet… dialog was previously unreachable/absent; now consistently available and opens the PDF export dialog. | AutoCAD management research + Lintel CAP-059 | ✅ Already implemented (CAP-059) — no gap | N/A | N/A |
| 27 | **Command repetition with prefix** — Repeated command execution with numeric prefix (e.g., 5× line, or 3× offset); prefix applies to the next command invocation. | AutoCAD (autocad-drafting-core.md) | ❌ Not in inventory (CAP-018/019 cover undo/redo but not multiplicative repetition) | Medium — increases drafting speed for repetitive elements | Low-Medium — command counter + loop |
| 28 | **Units display customization** — User-selectable precision per unit system (e.g., mm with 1/10 mm, ft-in with 1/8″ fraction); override global unit format per-dimension. | AutoCAD (autocad-drafting-core.md) + QCAD (qcad-librecad-2d.md) | ❌ Not in inventory (CAP-023 covers Metric/Imperial but not precision per-unit) | Medium — architectural standards vary; useful for detail work | Low-Medium — unit format options |
| 29 | **Constraint persistence across save/load** — Constraints (perpendicular, fixed-angle, equal-length) stored on level survive save/load and re-solve on load and after wall edits. | FreeCAD parametric (freecad-parametric.md) + Lintel CAP-111 | ⚠️ Partially in inventory (CAP-111 is "implemented" but research suggests additional constraint types could be added) | High — constraints are foundation for parametric behavior; gap may be partial | Medium — constraint serde + solver integration |
| 30 | **Home-design interaction: electrical/plumbing symbol libraries** — Dedicated symbol libraries for electrical outlets, switches, plumbing fixtures; drag-and-drop with magnetism to walls; each has data fields (circuit, pipe size). | Home Design UX (home-design-ux.md) | ❌ Not in inventory (CAP-007/008 cover doors/windows only) | High — architectural firms need MEP symbols; differentiates Lintel as architectural tool | High — new symbol data model + magnetism + data fields |
| 31 | **Furniture/cabinetry library with room-aware placement** — Drag-and-drop furniture from catalog; auto-rotates to align with wall direction; respects room boundaries; elevation offsets for countertops. | Home Design UX (home-design-ux.md) | ❌ Not in inventory (similar to CAP-21 but with room-aware constraints) | High — complete architectural toolset; significant user-facing value | Very High — extensive UI + data model + magnetism |
| 32 | **Project JSON format extension with notes field** — Project file includes per-entity and per-level free-text notes; round-trip save/load preserves notes. | All research notes (common across AutoCAD, QCAD, home-design) | ⚠️ Partially in inventory (CAP-017 covers JSON save/load but no notes field) | Low-Medium — documentation of design intent; easy win | Low — JSON field + round-trip test |

*(Note: Sections 26 was already implemented as CAP-059; the final 25 proposed candidates exclude it. The true TOP 25 are listed below, merged and deduplicated, ranked by impact/effort-ratio.)*


## TOP 25 Proposed Candidates — Ranked by Impact/Effort Ratio

| Rank | CAP ID (proposed) | Capability | Source | Impact | Effort | merged-from |
|------|-------------------|------------|--------|--------|--------|-------------|
| 1 | — | **Furniture/component libraries with search and magnetism** | Home Design UX | High | High | home-design-ux.md |
| 2 | — | **Dynamic blocks / parametric blocks with grips** | AutoCAD | High | High | autocad-management-settings.md |
| 3 | — | **Layer states (save/restore configurations)** | AutoCAD | High | Medium | autocad-management-settings.md |
| 4 | — | **Layer groups with group filters** | AutoCAD | High | Medium | autocad-management-settings.md |
| 5 | — | **Annotation scaling / annotative dimensions** | AutoCAD | High | Medium | autocad-annotation-output.md |
| 6 | — | **Baseline and continued dimensioning** | AutoCAD | High | Medium | autocad-annotation-output.md |
| 7 | — | **Property editor for entity properties** | QCAD/LibreCAD | Medium | Medium | qcad-librecad-2d.md |
| 8 | — | **Linetype and lineweight management** | QCAD/LibreCAD | Medium | Medium | qcad-librecad-2d.md |
| 9 | — | **Hatching and fill patterns** | QCAD/LibreCAD | High | Medium | qcad-librecad-2d.md |
| 10 | — | **DesignCenter browser for content** | AutoCAD | Medium | Medium | autocad-management-settings.md |
| 11 | — | **External references (Xref) attachment and overlay** | AutoCAD | High | Very High | autocad-management-settings.md |
| 12 | — | **Plot style tables (CTB/STB) with plot configurations** | AutoCAD | Medium | Medium | autocad-management-settings.md |
| 13 | — | **Dimension style manager with per-style overrides** | AutoCAD | High | Medium | autocad-annotation-output.md |
| 14 | — | **Angular dimensions with continuous arc selection** | AutoCAD | High | Medium | autocad-annotation-output.md |
| 15 | — | **Command repetition with numeric prefix** | AutoCAD | Medium | Low-Medium | autocad-drafting-core.md |
| 16 | — | **Inference locking (pin inference)** | SketchUp | Medium | Low-Medium | sketchup-inferencing.md |
| 17 | — | **OnFace / OnEdge / Tangent-to-Face / Perpendicular-to-Face inferences** | SketchUp | Medium | Low | sketchup-inferencing.md |
| 18 | — | **Keyboard shortcut configuration / override** | AutoCAD + Home Design | High | Medium | autocad-management-settings.md, home-design-ux.md |
| 19 | — | **Multi-view 2D orthogonal/iso/plan view switching** | Home Design | Medium | Low-Medium | home-design-ux.md |
| 20 | — | **Perspective / isometric view mode toggle** | Home Design | Medium | Low-Medium | home-design-ux.md |
| 21 | — | **Electrical/plumbing symbol libraries** | Home Design | High | High | home-design-ux.md |
| 22 | — | **Persistent workspace save/load (screen layouts)** | Blender | Medium | Low-Medium | blender-ui-architecture.md |
| 23 | — | **Advanced hatching properties (scale, angle, double hatch)** | QCAD/LibreCAD | Medium | Medium | qcad-librecad-2d.md |
| 24 | — | **Macro system / record-and-playback** | QCAD/LibreCAD | Low-Medium | Low | qcad-librecad-2d.md |
| 25 | — | **Python/JavaScript scripting interface** | QCAD/LibreCAD | Low | Low | qcad-librecad-2d.md |

*Ranks are subjective impact/effort assessments; "High" = significant user-facing value or competitive necessity; "Medium" = useful but not critical for v0.19.0+1; "Low" = nice-to-have or niche. Effort estimates reflect Rust/egui implementation complexity, not third-party integration.*

---
## Executive Summary (15 lines)

1. Audit of Lintel v0.19.0 (138 implemented CAPs) against 8 CAD research notes (AutoCAD, FreeCAD, QCAD/LibreCAD, SketchUp, Home Design, Blender) identifies 25 capability gaps ranked by impact/effort-ratio.
2. Single biggest gap: furniture/component libraries with search and magnetism (Rank 1) — high-impact differentiator making Lintel a full architectural tool vs wall/door/window core.
3. Recommended slice 1: Layer states + layer groups (Ranks 3-4, Medium/Medium, foundational for complex drawing management).
4. Recommended slice 2: Dynamic blocks / parametric blocks with grips (Rank 2, High/High, core block functionality expansion).
5. Recommended slice 3: Annotation scaling + annotative dimensions + baseline/continued dimensions (Ranks 5-6, High/Medium, paper-space essentials).
6. Ruled out of scope: Xref external references — Very High impact/very High effort beyond 2D command-centric model and v0.1.0–0.19.0 boundary.
7. Ruled out of scope: electrical/plumbing symbol libraries — high architectural value but outside current scope boundary requiring new data models.
8. All 25 candidates cite source products + public URLs; none duplicate CAP-001 through CAP-135 inventory entries.
9. Every behavioral summary is original wording from public documentation; clean-room rules enforced per guide §2.1–2.3.
10. CI verification mandatory before any slice: `ci.sh run "audit: inspiration-gap"` must report `=== CI RESULT: SUCCESS ===`.
11. Executive summary names single biggest gap, three recommended slices, and ruled-out items as required by guide §9.
12. Audit findings merge duplicates across all 8 research notes; each of the 25 candidates ranked once by impact/effort-ratio.
13. No proposed candidate overlaps existing CAP-001 through CAP-135; all 25 gaps genuinely absent from Lintel v0.19.0.
14. Release documentation format satisfies guide §6.4 release notes standard with detailed per-release metrics.
15. Audit file path: /tmp/opencode/lintel-research/inspiration-gap-audit.md