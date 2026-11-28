# FreeCAD Parametric Workflow — Clean-Room Research Note for Lintel

**Date:** 2026-11-28 · **Project:** Lintel (Rust/egui 2D architectural floor-plan CAD)
**Method:** Public sources only. Primary evidence: FreeCAD source code files on GitHub
(fetched and read directly). Each claim carries the URL of the source file it was verified
against. Where a claim describes a user-facing feature, the canonical public documentation
page (wiki.freecad.org) is given as a supplementary pointer; the wiki is bot-gated to
automated fetching, so all wording below is Lintel's own original behavioural summary
derived from the public source code, never reproduced text.

---

## 1) Behaviours (grouped, each with source URL)

### 1.1 Sketcher — geometry, constraints and the solver loop

- **Geometry-first, constraints-second model.** A sketch stores a list of geometry
  elements (lines, arcs, circles, B-splines, etc., including "construction" geometry that
  participates in solving but not in the resulting shape) plus a separate list of
  constraints that reference geometry by id and, where relevant, by a sub-element
  position (start / end / midpoint / curve itself). Sketch axes (a horizontal and a
  vertical axis) are always present and addressable by negative ids.
  Source: `src/Mod/Sketcher/App/Constraint.h` (GeoElementId list, PointPos, type enum)
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/Constraint.h —
  documentation pointer: https://wiki.freecad.org/Sketcher_Workbench

- **Constraints are objects with a value, a driving flag, a name, visibility, an active
  flag and a "virtual space" flag.** A dimensional constraint stores a numeric `Value`
  and an `isDriving` boolean; a constraint can be named (free text), hidden, deactivated
  (`isActive`), or parked "in virtual space" (displayed on the mirrored side of the
  sketch plane without changing what it enforces).
  Source: same file (fields `Value`, `isDriving`, `Name`, `isVisible`, `isActive`,
  `isInVirtualSpace`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/Constraint.h

- **Solver is a non-linear least-squares system with three selectable algorithms** (BFGS,
  Levenberg–Marquardt, DogLeg; SparseQR/DenseQR used internally), run per sketch; a
  separate "redundant solving" pass re-solves with a different algorithm and tolerance set
  to identify which constraints are redundant.
  Source: `src/Mod/Sketcher/App/planegcs/GCS.cpp` (`System::solve`, algorithm switch)
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/planegcs/GCS.cpp

- **Solve status taxonomy.** The solver reports, in a fixed priority order:
  `Success`, `Overconstrained`, `ConflictingConstraints`, `MalformedConstraints`,
  `SolverError`, `RedundantConstraints` (plus an "InvalidGeometry" case). Over-constrained
  means DoF count went negative; "conflicting" means mutually contradictory constraints
  (the solver lists them); "malformed" means a constraint applied in a way that has no
  geometric meaning (e.g. distance to a self-intersecting edge); "redundant" means the
  constraint removes no remaining freedom; "partially redundant" means it overlaps another
  constraint over only part of the geometry.
  Source: `src/Mod/Sketcher/App/SketchObject.h` (`SketchSolveStatus` enum, priority order
  documented in the `solve()` comment) and `Sketch.h` (per-sketch lists
  `Conflicting`, `Redundant`, `PartiallyRedundant`, `MalformedConstraints`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/SketchObject.h
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/Sketch.h

- **Degrees of freedom are computed and displayed.** The sketch exposes a DoF count and a
  boolean "fully constrained"; the UI status area shows one of: "Empty sketch",
  "Over-constrained: (conflicting list)", "Malformed constraints: (list)",
  "Redundant constraints: (list)", "Partially redundant: (list)",
  "Solver failed to converge", "Under-constrained: N Degrees of Freedom", or
  "Fully constrained". Long lists are truncated ("first 3 … and N more").
  Source: `src/Mod/Sketcher/Gui/ViewProviderSketch.cpp` (`UpdateSolverInformation`,
  `intListHelper`, `appendConstraintMsg`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/ViewProviderSketch.cpp —
  documentation pointer: https://wiki.freecad.org/Sketcher_requirement

- **Per-element solver diagnosis.** After a solve, every geometry element carries a
  solver extension with per-parameter status: each point (start/mid/end) is classified
  Independent/Dependent per axis, plus radius/weight/focus parameter status for curved
  types, and an overall FullyConstraint/NotFullyConstraint verdict per element. This is
  what allows the UI to know *which* element still has freedom (and enables dragging the
  free parts).
  Source: `src/Mod/Sketcher/App/SolverGeometryExtension.h` (`SolverStatus`,
  `ParameterStatus::Dependent/Independent`, `PointParameterStatus`, `getDoFs`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/SolverGeometryExtension.h

- **Solver feedback colours are tokenised parameters**, one colour role per meaning:
  normal curve colour, construction colour, internal-alignment colour, fully-constrained
  element colour (plus construction and internal-alignment variants),
  invalid-sketch colour, fully-constrained axis colour, driving dimensional-constraint
  colour, constraint-icon colour, non-driving (reference) dimension colour,
  expression-driven dimension colour, deactivated constraint colour, external-geometry
  colour, external-defining colour, preselection highlight, selection, and an
  information overlay colour.
  Source: `src/Mod/Sketcher/Gui/EditModeCoinManager.cpp` (parameter→colour table).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/EditModeCoinManager.cpp —
  documentation pointer: https://wiki.freecad.org/Sketcher_reference

- **Auto-constraints while drawing.** During interactive drawing the handlers ask the
  system for "suggested" constraints (`AutoConstraint` = constraint type + target geoId +
  target position; targets are VERTEX, CURVE, VERTEX_NO_TANGENCY). On finishing the
  element the suggestions are added automatically; a preference
  ("AvoidRedundantAutoconstraints") makes the system skip suggestions that would be
  redundant given constraints it is about to add; dragging existing geometry can also
  create auto-constraints (dedicated drag-auto-constraint handler).
  Source: `src/Mod/Sketcher/Gui/AutoConstraint.h`,
  `src/Mod/Sketcher/Gui/DrawSketchHandler.h` (`seekAutoConstraint`, `createAutoConstraints`,
  `filterRedundantAutoConstraints`, `DrawSketchHandlerDragAutoConstraint.h`),
  `src/Mod/Sketcher/Gui/ViewProviderSketch.cpp` (AvoidRedundant parameter).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/AutoConstraint.h
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/DrawSketchHandler.h —
  documentation pointer: https://wiki.freecad.org/Sketcher_Constraint_settings

- **Dimensional-constraint dialog accepts numbers and expressions.** Editing a datum opens
  a quantity spin box with a formula ("f(x)") button (`showFormulaDialog`); acceptance
  calls `setDatum(id, quantity)` and then executes the sketch's expression engine. A
  dimensional constraint can therefore be a plain number or an expression referencing
  other objects.
  Source: `src/Mod/Sketcher/Gui/EditDatumDialog.cpp`.
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/EditDatumDialog.cpp —
  documentation pointer: https://wiki.freecad.org/Sketcher_Workbench#Dimensions

- **Internal alignment geometry.** The `InternalAlignment` constraint type ties the
  *internal* geometry of composite elements (ellipse major/minor diameters, foci,
  hyperbola/parabola axes and foci, B-spline control points and knot points) to separate
  construction elements, with an index into which internal piece is aligned. FreeCAD
  includes analysis helpers that detect "missing" coincidences and equalities by geometric
  proximity (tolerance-based) and can apply them in stages (vertical/horizontal first,
  then point-on-point classified into coincident/tangent/perpendicular by tangent-vector
  analysis, then equality), solving after each stage and aborting with a plain-language
  error if the sketch becomes unsolvable.
  Source: `src/Mod/Sketcher/App/Constraint.h` (InternalAlignmentType enum);
  `src/Mod/Sketcher/App/SketchAnalysis.cpp` (`autoconstraint`, staged detection,
  `analyseMissingPointOnPointCoincident`, per-stage error strings,
  `autoRemoveRedundants`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/SketchAnalysis.cpp

### 1.2 Recompute / dependency model

- **Document-wide DAG recompute.** A document recompute collects the objects to process,
  topologically sorts them by dependency, and re-executes each in order; a cyclic
  dependency is detected and falls back to a *partial* topological sort so the user can
  still see which objects are in a cycle. Objects are marked `PendingRecompute` during the
  pass; a recompute log records per-object errors.
  Source: `src/App/Document.cpp` (recompute, `partialTopologicalSort`, "cyclic dependency
  detected", recompute log).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/App/Document.cpp

- **Touched / error markers per object.** Every document object has status bits: `Touch`
  (marked as changed, needs recompute), `Error` (last recompute failed), plus flags like
  "touched on property change". Property changes call the object's `onChanged`; whether
  the object participates in the next recompute is decided by `mustExecute`/
  `mustRecompute`. `purgeTouched` clears the marker after a successful recompute.
  Source: `src/App/DocumentObject.h` (`ObjectStatus::Touch`, `ObjectStatus::Error`,
  `enforceRecompute`, `purgeTouched`, `isTouched`, `mustExecute`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/App/DocumentObject.h —
  documentation pointer: https://wiki.freecad.org/Property

- **Solver failure blocks the feature chain with actionable text.** When a sketch fails to
  solve (over-constrained / conflicting / redundant / malformed), its `execute()` returns
  an error result containing a plain-language message ("Over-constrained sketch" /
  "Sketch with conflicting constraints" + "Remove at least one of the following
  constraints: 1, 4, 5"). Dependent features therefore fail too, and the message names the
  constraint numbers to remove. Redundancy is likewise treated as an error at recompute
  time (with an option elsewhere to auto-remove redundant constraints).
  Source: `src/Mod/Sketcher/App/SketchObject.cpp` (`execute()` switch over
  `SketchSolveStatus`, `appendConflictMsg` etc.); see also `SketchAnalysis.cpp`
  (`autoRemoveRedundants`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/SketchObject.cpp

- **Constraint renames propagate to expressions.** Because named constraints are address
  targets for the expression engine, renaming a constraint emits a rename map
  (old object identifier → new) so expressions bound to the old name are rewritten.
  Source: `src/Mod/Sketcher/App/PropertyConstraintList.cpp` (`signalConstraintsRenamed`,
  rename detection in `setValues`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/PropertyConstraintList.cpp

### 1.3 Expression engine

- **Bindings are (object-identifier → expression) maps per object.** The engine stores, on
  each object, a mapping from a target property path (e.g. a named constraint path such as
  `Sketch.Constraints.Length`) to a parsed expression; evaluating the engine writes
  results back into the bound properties and registers the referenced objects as
  dependencies so a change upstream marks the downstream object touched.
  Source: `src/App/PropertyExpressionEngine.h` (getExpressions/setExpressions contract,
  dependency handling, dynamic-property rename hooks).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/App/PropertyExpressionEngine.h —
  documentation pointer: https://wiki.freecad.org/Expressions

- **Unit-aware quantities and a large function library.** Expressions evaluate to
  quantities with units (length, angle, etc.); dimensional constraint values present
  themselves with the unit appropriate to their type (Distance/DistanceX/DistanceY/Radius/
  Diameter → Length, Angle → degrees-angle unit). The grammar supports conditionals,
  arithmetic operators, trigonometry and vector/matrix functions, string/parse helpers,
  aggregates (SUM, AVERAGE, COUNT, MAX, MIN, STDDEV, AND, OR), object creation
  (matrix/placement/rotation/vector/list), and an explicit "hidden reference" function
  that reads another object *without* creating a dependency edge.
  Source: `src/App/ExpressionParser.h` (`FunctionExpression::Function` enum incl.
  HIDDENREF), `src/Mod/Sketcher/App/Constraint.cpp` (`getPresentationValue` unit mapping).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/App/ExpressionParser.h
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/Constraint.cpp

- **Spreadsheet cells as named parameters.** A spreadsheet sheet is a document object;
  cells hold content and can be given aliases; the sheet exposes cell values as
  quantity-typed properties, so any expression (including a sketch constraint) can bind to
  `Spreadsheet.alias`. Aliases have validity rules and reserved tokens.
  Source: `src/Mod/Spreadsheet/App/Sheet.h` (`setAlias`, `getAddressFromAlias`,
  `isValidAlias`, `PropertySpreadsheetQuantity`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Spreadsheet/App/Sheet.h —
  documentation pointer: https://wiki.freecad.org/Spreadsheet_Workbench

### 1.4 Part Design (only conceptually transferable)

- **A sketch becomes a solid feature by parameters.** The Pad feature extrudes a sketch by
  a `Length`, with modes (`Length`, `UpToLast`, `UpToFirst`, `UpToFace`), plus `Midplane`
  and `Reversed` toggles; Pocket is the subtractive counterpart. The pattern of interest
  for Lintel is: a 2D profile + a handful of named parameters produce/modify dependent
  geometry, and editing the sketch or the parameters recomputes the feature chain.
  Source: `src/Mod/PartDesign/App/FeaturePad.h` (Type enums and documented behaviour).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/PartDesign/App/FeaturePad.h —
  documentation pointer: https://wiki.freecad.org/PartDesign_Workbench

### 1.5 TechDraw (2D drawing production)

- **Pages, templates, views, dimensions.** A drawing page is a document object holding a
  list of views plus a template link and a per-page "keep updated" flag. Dimensions are
  their own document objects placed on views; types are Distance, DistanceX, DistanceY,
  DistanceZ, Radius, Diameter, Angle, Angle3Pt. A dimension references either the 2D view
  geometry (References2D) or the 3D model (References3D), with a measure type
  True/Projected; there is a reference-type concept (vertex-vertex, edge, edge-edge), a
  broken-reference detection path, and optional "theoretical exact" and equal-tolerance
  presentation flags.
  Source: `src/Mod/TechDraw/App/DrawPage.h`; `src/Mod/TechDraw/App/DrawViewDimension.h`.
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/TechDraw/App/DrawPage.h
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/TechDraw/App/DrawViewDimension.h —
  documentation pointer: https://wiki.freecad.org/TechDraw_Workbench

### 1.6 BIM/Arch workbench — the most relevant part for a floor-plan tool

- **Walls are built from a baseline.** A Wall takes a `Base` (a Draft line/wire, a
  sketch, an "ArchSketch", or a face/solid) and parameters `Length`, `Width`, `Height`,
  `Align` (Left/Right/Center), `Offset`, and a normal; the documented contract states the
  Length is *editable only if* the baseline is a Draft line, an unconstrained
  single-line sketch, or absent — i.e. if the baseline is a constrained sketch, geometry
  drives the wall, not the other way round. Recompute extrudes the thickened baseline
  vertically ("typically obtained by giving a thickness to a base line, then extruding it
  vertically"). Per-segment overrides exist for width/align/offset (`OverrideWidth`,
  `OverrideAlign`, `OverrideOffset`) when a sketch has multiple edges.
  Source: `src/Mod/BIM/ArchWall.py` (property docstrings, `execute()`, `getExtrusionData()`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWall.py —
  documentation pointer: https://wiki.freecad.org/Arch_Wall

- **Height inheritance from levels.** `Height` documents "Keep 0 to automatically match
  the height of the enclosing Level or Building"; the level (a BuildingPart with IFC type
  "Building Storey") has `Height` and a `HeightPropagate` flag ("the height value
  propagates to contained objects if the height of those objects is set to 0"); the wall
  walks up its parents to find a height if its own is zero.
  Source: `src/Mod/BIM/ArchWall.py` (Height docstring); `src/Mod/BIM/ArchBuildingPart.py`
  (HeightPropagate, touchChildren); `src/Mod/BIM/ArchComponent.py` (getParentHeight).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchBuildingPart.py

- **Hosted openings (windows/doors) cut voids in hosts.** A Window (doors are windows
  with a door IFC type and door presets) holds a `Hosts` link list. The host wall's
  recompute scans its *in-list* for objects whose `Host`/`Hosts` contains it and cuts
  their "subvolume" from its shape (additions are fused first, then subtractions cut).
  The void volume is the window's base profile extruded through the wall: a custom
  `Subvolume` object wins if set; otherwise depth is `HoleDepth` if non-zero, else
  auto-computed as the host wall's maximum thickness (including multi-layer materials)
  plus 100 mm (for non-wall hosts: the largest bounding-box dimension of the base).
  Which wire of the window profile makes the hole is `HoleWire` if non-zero, else the
  biggest wire automatically.
  Source: `src/Mod/BIM/ArchComponent.py` (`processSubShapes`, in-list scan of
  `Hosts`/`Host`, `getSubVolume` call); `src/Mod/BIM/ArchWindow.py` (`getSubVolume`
  auto-depth +100 mm logic, `HoleWire` biggest-wire fallback).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchComponent.py
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py —
  documentation pointer: https://wiki.freecad.org/Arch_Window

- **Hosts are touched when the opening changes.** When a window's Base, WindowParts,
  Placement, HoleDepth, Height, Width, or Shape change, each host is `touch()`ed "to mark
  host to recompute so it can detect this object" (anti-recursive-loop guards included).
  This is the dependency plumbing that keeps holes in sync when a window is dragged or
  resized.
  Source: `src/Mod/BIM/ArchWindow.py` (`onBeforeChange`/`onChanged` host-touch logic).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py

- **Opening parameters can drive sketch constraints.** If the window's base sketch has a
  named constraint equal to "Width" or "Height", changing the window's Width/Height
  property calls `setDatum(name, value)` on that sketch — so the window's own parameters
  are propagated *into* the constrained sketch, keeping the frame geometry consistent.
  This is the reverse-direction parametric bridge (property → sketch datum).
  Source: `src/Mod/BIM/ArchWindow.py` (`onChanged`, Width/Height/Frame →
  `obj.Base.setDatum(prop, val)`).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py

- **Doors: swing geometry is data, not bespoke code.** Opening behaviour is a per-part
  enumeration ("None", "Arc 90", "Arc 90 inv", "Arc 45", "Arc 45 inv", "Arc 180",
  "Arc 180 inv", "Triangle", "Triangle inv", "Sliding", "Sliding inv") plus a hinge edge
  (an edge index of the base sketch) and a global `Opening` percentage. The panel shape
  rotates about the hinge edge by mode × percentage; sliding panels translate along the
  hinge (clamped so the handle stays exposed at 100%). Plan symbols are generated from the
  same data: quarter/half/third-circle arcs + lines drawn from the hinge point (the
  classic door-swing notation), with `SymbolPlan`/`SymbolElevation` toggles; the arc
  side is derived from the hinge edge geometry.
  Source: `src/Mod/BIM/ArchWindow.py` (WindowOpeningModes list, `buildShapes` symbol and
  rotation logic).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py

- **Levels/storeys organisation.** BuildingPart objects carry an IFC type ("Building
  Storey" = level, "Building", "Annotation"); they group children, expose Level/Height/
  LevelOffset, can display level information, and provide "Clone Level Up" tooling.
  Children with `MoveWithHost` (windows default to True) move when the host/level moves.
  Source: `src/Mod/BIM/ArchBuildingPart.py`; `src/Mod/BIM/ArchComponent.py` (MoveWithHost,
  getMovableChildren).
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchBuildingPart.py

- **IFC as the exchange concept.** Arch components carry `IfcType` and IFC property
  mapping modules exist to translate objects to/from the IFC schema; a Wall defaults to
  IfcType "Wall", a Window to "Window". Lintel need not implement IFC, but the *concept*
  — every object self-describes its building semantics — is directly transferable.
  Source: `src/Mod/BIM/ArchWall.py` (`obj.IfcType = "Wall"`); `src/Mod/BIM/ArchWindow.py`
  (`obj.IfcType = "Window"`); `src/Mod/BIM/ArchIFC.py`.
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchIFC.py

- **Measuring/annotation in Arch.** Arch objects compute and expose derived quantities as
  properties (Wall `Area` = Height × Length, auto-updated `Length` from the solved
  baseline edges, block counts); a Schedule object can aggregate object properties into a
  table; measurement of arbitrary geometry is available through Draft dimensions and the
  Part measuring utilities that Arch commands wrap.
  Source: `src/Mod/BIM/ArchWall.py` (Length/Area recalculation in `execute()`);
  `src/Mod/BIM/ArchSchedule.py`.
  https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchSchedule.py

---

## 2) Sketcher constraint inventory table

Constraint set verified against the type enum in
`src/Mod/Sketcher/App/Constraint.h` (https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/Constraint.h);
UI-feedback wording summarised from the solver-status and colour-role sources above.

| Constraint | What it enforces | UI feedback |
|---|---|---|
| Coincident | Two points (by element+position) share one location | Constraint icon; joins coincidence groups (drag moves both) |
| Horizontal | Edge (or two points) horizontal | Auto-suggested while drawing; icon; element becomes DoF-diagnosed |
| Vertical | Edge (or two points) vertical | Same as horizontal |
| Parallel | Two edges parallel | Icon on both edges |
| Perpendicular | Two edges at 90° (edge-edge, or endpoint tangency variant) | Icon; endpoint variant also joins points |
| Tangent | Edge-edge tangency (with optional endpoint contact) or edge-point tangency | Endpoint tangency counts as a coincidence for analysis; icon |
| Equal | Two edges equal (length for lines; radius for arcs/circles) | Icon; feeds equality auto-detection |
| Symmetric | Two points symmetric about a line or a third point | Icon at construction reference |
| PointOnObject | A point lies on an edge/curve | Icon; target = CURVE in auto-constraint detection |
| Block | Element(s) frozen in place (no solving of that element) | Icon; "has block constraint" changes drag behaviour |
| Distance | Length between two points or length of an edge | Driving dimension shown with value; colour: driving-dim colour; editable datum dialog w/ expression |
| DistanceX | Signed horizontal distance | Same as Distance (value shows X component) |
| DistanceY | Signed vertical distance | Same as Distance |
| Angle | Angle between two edges (or of an arc); has orientation (CW/CCW) and supplementary-flip helpers | Driving dimension in angle units; can be converted to supplementary |
| Radius | Circle/arc radius | Driving dimension |
| Diameter | Circle/arc diameter | Driving dimension |
| Weight | B-spline weight (rarely used directly) | Driving datum on rational splines |
| SnellsLaw | Refraction law between two rays at an interface (niche) | Driving datum with refractive index |
| InternalAlignment | Ties construction geometry to internal structure of ellipse/hyperbola/parabola/B-spline (with an index per internal piece) | Internal-alignment colour role; auto-managed by "expose internal geometry" tools |
| Group | Constraint grouping construct (solver-level) | Not surfaced as a user constraint icon |
| Text | Text annotation element inside a sketch (with font, size flag) | Text drawn in sketch; datum dialog variant for text |

Cross-cutting states applying to any constraint: **driving** (enforces its value) vs
**reference/non-driving** (reports measured value; different colour), **active** vs
**deactivated** (deactivated drawn in its own colour and ignored by solver), **named** (its
name becomes an expression address `Sketch.Constraints.<Name>`), and **virtual space**
(visual side only). Sources: `Constraint.h` (`isDriving`, `isActive`, `Name`,
`isInVirtualSpace`), `EditModeCoinManager.cpp` colour roles, `ViewProviderSketch.cpp`
solver messages. Dimensional set is enumerated by `Constraint::isDimensional()`
(Distance, DistanceX, DistanceY, Radius, Diameter, Angle, SnellsLaw, Weight).

---

## 3) Edge cases & nuances

1. **Over-constrained vs conflicting.** Both mean "cannot be solved as stated", but they
   are reported separately: over-constrained is detected by a negative DoF count;
   conflicting constraints are named explicitly. The UI message for conflicts is
   actionable: "Remove at least one of the following constraints: …" (list capped at
   3 shown + "and N more"). Source: `ViewProviderSketch.cpp` (`UpdateSolverInformation`,
   `appendConstraintMsg`, `intListHelper`).

2. **Redundant ≠ conflicting, and redundancy is an error, not a warning, at recompute.**
   A redundant constraint still solves, but FreeCAD's sketch `execute()` treats it as a
   failed recompute with a "remove the following redundant constraints" message; a
   separate API (`autoRemoveRedundants`) can delete them programmatically; "partially
   redundant" (overlapping only part of another constraint's effect) is a distinct,
   separately-listed state. Source: `SketchObject.cpp` execute() switch;
   `SketchAnalysis.cpp` solvesketch/autoRemoveRedundants; `Sketch.h` (PartiallyRedundant).

3. **Malformed constraints** — a constraint that is syntactically valid but geometrically
   meaningless for its inputs — form yet another failure bucket with its own message and
   list. Source: `SketchObject.cpp` (MalformedConstraints case).

4. **Solver convergence can fail even when the constraint set is formally correct**;
   FreeCAD reports this as "Solver failed to converge" rather than blaming any constraint.
   The advanced solver panel exposes algorithm and tolerance choices (BFGS/LM/DogLeg,
   iteration and convergence settings) for such cases. Sources:
   `ViewProviderSketch.cpp`; `TaskSketcherSolverAdvanced.*`; `GCS.cpp`.

5. **Editing an old feature re-triggers the whole downstream chain** (document DAG
   recompute), and any failure anywhere marks that object `Error` and stops dependents
   receiving a valid shape; the recompute log retains per-object messages. Cycles are
   detected and reported via partial topological sort rather than hanging. Sources:
   `Document.cpp`, `DocumentObject.h`.

6. **Constrained baselines are read-only through the wall.** A wall whose Base is a
   constrained sketch does not accept a Length edit from the wall side (the Length
   property is documented as editable only for a Draft line, an unconstrained
   single-line sketch, or no base); conversely, editing the wall Length *when allowed*
   moves the baseline's endpoint. Source: `ArchWall.py` property docstrings + `onChanged`.

7. **Host deletion leaves the opening orphaned but intact.** Hosts are discovered by
   scanning the wall's in-list at recompute time; if the wall is deleted the window keeps
   its own base sketch and its own shape (frame/panel parts), so it survives as a
   standalone object — but the void obviously disappears with the host. The `MoveWithHost`
   flag (windows default to True) only governs movement while the host exists. Sources:
   `ArchComponent.py` (processSubShapes in-list scan, getMovableChildren);
   `ArchWindow.py`.

8. **Hole depth is a guess unless stated.** The automatic hole depth is "host thickness +
   100 mm" (or the bounding-box diagonal for non-wall hosts); users can set `HoleDepth`
   or a custom `Subvolume`. The hole wire is auto-picked as the largest bounding-box wire
   unless `HoleWire` says otherwise. Source: `ArchWindow.py` getSubVolume().

9. **Anti-loop guards.** Window↔host updates are deliberately one-directional with
   recursion guards: host-touching on window change is suppressed for properties that
   would re-trigger the base sketch's Placement changes. Source: `ArchWindow.py`
   onChanged().

10. **Auto-constraints can be avoided when they would be redundant** (preference
    `AvoidRedundantAutoconstraints`), because naive auto-constraints are a classic way to
    push a sketch into redundancy. Source: `ViewProviderSketch.cpp` parameter observer;
    `DrawSketchHandler.h` filterRedundantAutoConstraints.

11. **Staged auto-constraining aborts safely.** The bulk "fully constrain" analysis
    applies stages one at a time (V/H, then point-on-point, then equality), re-solving
    after each stage and rolling back with a plain-language error if a stage makes the
    sketch unsolvable; it even deletes *all* constraints first when running the full
    autoconstraint pass. Source: `SketchAnalysis.cpp` (autoconstraint stages,
    makeConstraintsOneByOne error strings, autoDeleteAllConstraints).

12. **Reference (non-driving) dimensions never fight the solver** — they are excluded from
    the constraint effect but still display the measured value; expression-driven and
    deactivated dimensions each have their own colour so the user can tell at a glance
    which dimensions are "live". Source: `EditModeCoinManager.cpp` colour roles;
    `SketchObject.h` (setDriving/setActive).

---

## 4) Candidate capabilities for Lintel

Each item: behavioural summary (from public sources above) → acceptance-criteria idea →
evidence URL. Priority order matches Lintel's floor-plan focus.

### 4.1 Hosted-opening semantics (highest value)
**Behaviour:** An opening (door/window) records which wall hosts it; the wall's rebuild
fuses additions then cuts each hosted opening's void; the void depth defaults to "wall
thickness + margin" and is overridable; editing the opening (size/position) re-touches the
host so the hole follows; deleting the host leaves the opening as an intact standalone
object.
**Acceptance-criteria ideas:**
- Moving a hosted door updates the cut in the wall with no user action (single drag →
  recompute → hole follows).
- Setting opening width to a new value updates the void width in the wall after recompute.
- Deleting the host wall keeps the door object rendering its panel/swing symbol, with no
  dangling references in the solver.
- Void depth = host thickness + configurable margin unless user sets an explicit depth.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchComponent.py
(processSubShapes); https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py
(getSubVolume, host-touch).

### 4.2 Wall-from-baseline parametrics
**Behaviour:** A wall is baseline (chain of segments) + width + alignment (left/center/
right) + offset + height; height 0 inherits from the enclosing storey; per-segment width/
alignment overrides; editing a constrained baseline drives the wall, while editing the
wall's own length (when the baseline is unconstrained) drives the baseline.
**Acceptance-criteria ideas:**
- A wall drawn on a 3-segment baseline with width 200 and Align=Left renders an offset
  outline on the correct side of each segment; switching Align to Center re-renders
  symmetric around the baseline without user geometry edits.
- Two walls whose baselines share an endpoint with alignment Center produce a clean
  corner junction after one recompute.
- A room's wall with Height=0 adopts its storey's height; changing the storey height
  updates the wall on next recompute.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWall.py
(getExtrusionData, Align/Offset/Override*); https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchBuildingPart.py
(HeightPropagate).

### 4.3 Constraint-status feedback (DoF-style reporting)
**Behaviour:** After every solve, a status line reports exactly one of: empty /
over-constrained (with conflicting list) / malformed (list) / redundant (list) /
partially-redundant (list) / solver-failed / under-constrained (N DoF) / fully constrained;
list rendering truncates to 3 + "and N more"; each failure also names the constraint ids
to remove.
**Acceptance-criteria ideas:**
- Adding a duplicate FixedAngle that contradicts an existing one switches the status line
  to over-constrained/conflicting and lists both ids.
- Removing the conflicting constraint returns the status to under-constrained with a DoF
  count, and the count decreases by 1 when a dimensional constraint is added.
- The status line text never shows more than 3 ids without the "and N more" truncation.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/ViewProviderSketch.cpp
(UpdateSolverInformation).

### 4.4 Solver colour tokens (semantic roles, token-driven)
**Behaviour:** Element and constraint rendering uses distinct colour roles per meaning:
normal geometry, construction, fully-constrained element, invalid-sketch element,
driving dimension, reference dimension, expression-driven dimension, deactivated
constraint, external/defining reference, preselection, selection. Colours are user
preferences (token overrides), not scattered literals.
**Acceptance-criteria ideas:**
- A reference (non-driving) dimension renders in the non-driving colour role; toggling it
  to driving switches colour to the driving role in the same frame.
- A sketch in a failed state renders its elements in the invalid colour role; fixing the
  constraint conflict restores normal colours.
- All colours resolve from Lintel's theme tokens (no hard-coded hex in widget code).
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/EditModeCoinManager.cpp
(colour-role parameter table).

### 4.5 Expression-driven parameters (named datums + aliases)
**Behaviour:** Dimensional constraints can be named; a name becomes an address
(`<element>.<constraint name>`); a dimension's value can be an expression referencing
other named datums or spreadsheet-like parameter cells; renaming propagates to
expressions; the datum edit dialog accepts both a raw quantity and an expression; units
are respected (length vs angle).
**Acceptance-criteria ideas:**
- Naming a wall-length dimension "RoomWidth" and binding a door-width dimension to
  `RoomWidth/2` recomputes the door width when RoomWidth changes.
- Renaming the constraint updates the dependent expression without leaving a dangling
  reference.
- An expression referencing a non-existent name produces a per-object error state, not a
  silent wrong value.
- Angle datums display and parse in degrees even though internally stored as radians.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/App/PropertyExpressionEngine.h;
https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/PropertyConstraintList.cpp
(rename propagation); https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/EditDatumDialog.cpp
(formula dialog).

### 4.6 Auto-constraints while drawing (with redundancy avoidance)
**Behaviour:** While the user draws, the system suggests constraints (endpoint snapping →
coincident; axis-aligned direction → horizontal/vertical; proximity to an existing curve →
point-on-object; equal lengths/radii → equal) and applies them on completion, unless the
suggestion would be redundant given the other constraints about to be added.
**Acceptance-criteria ideas:**
- Drawing a line snapped to another line's endpoint ends with an automatic Coincident
  constraint attached (visible in the constraint list) without extra clicks.
- Drawing an axis-aligned line attaches Horizontal/Vertical automatically; drawing a
  second axis-aligned line in a chain where that constraint would be redundant attaches
  nothing.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/AutoConstraint.h;
https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/Gui/DrawSketchHandler.h.

### 4.7 Door swing as data
**Behaviour:** Swing behaviour = mode enumeration (quarter/half/third arc, either side;
triangle; sliding) + hinge edge + percentage; plan symbols (arc + perpendicular line from
the hinge) derive from the same data; 100% opening clamps sliding so the panel never fully
disappears into the pocket.
**Acceptance-criteria ideas:**
- A door with mode Arc-90 and hinge at its left jamb renders the classic quarter-circle
  plan symbol opening from that jamb; mirroring the mode renders the mirrored symbol.
- Opening=50 rotates the panel symbol half way; Opening is display-only in plan symbols
  (static hinge symbol) unless animated.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWindow.py
(WindowOpeningModes, buildShapes symbols).

### 4.8 Storey/level organisation with height inheritance
**Behaviour:** Levels group floor-plan objects; a level's height propagates to members
whose height is zero; members can be marked to move with the host; level offset positions
the group's origin.
**Acceptance-criteria ideas:**
- Walls with height 0 inside "Level 1" (height 2700) render 2700 tall; setting Level 1 to
  3000 updates them.
- Hosted openings move when the level (or wall) is moved.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchBuildingPart.py.

### 4.9 Measure properties on objects (derived, read-only)
**Behaviour:** Objects expose derived read-only quantities (wall area, length from solved
baseline) recomputed automatically; they are shown read-only in the property editor and
usable in schedules/exports.
**Acceptance-criteria ideas:**
- Wall area equals width × height after any geometry edit, updated without manual refresh.
- Wall length equals half the perimeter of its (closed) baseline, matching recompute after
  dragging a corner.
**URL:** https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/BIM/ArchWall.py
(execute() length/area).

---

## 5) Not-applicable to Lintel (with reasons)

- **3D features and B-rep solids (Pad/Pocket booleans, mesh handling).** Lintel is a 2D
  floor-plan tool; the value is the *parameter pattern*, not the solid modelling
  (`src/Mod/PartDesign/App/FeaturePad.h` — https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/PartDesign/App/FeaturePad.h).
- **B-spline/ellipse internal-alignment constraints, SnellsLaw, Weight.** Floor plans use
  lines/arcs; Lintel's constraint set (PerpendicularDistance, FixedAngle, EqualLength,
  anchored dimensions) covers the relevant subset; the analysis machinery for splines
  adds no user value now
  (`src/Mod/Sketcher/App/Constraint.h` InternalAlignmentType — URL above).
- **External-geometry projection between sketches.** FreeCAD lets a sketch reference
  projected edges of other objects; Lintel's single-plan model does not need cross-sketch
  projection (`SketchObject.h` ExternalGeometry — https://github.com/FreeCAD/FreeCAD/blob/main/src/Mod/Sketcher/App/SketchObject.h).
- **TechDraw page templates / multi-view projection / 3D-referenced dimensions.** Lintel
  already *is* the 2D drawing; its PDF output corresponds to the page concept only.
  Measure types True/Projected and 3D references have no counterpart
  (`src/Mod/TechDraw/App/DrawViewDimension.h` — URL above).
- **IFC import/export.** Valuable for interoperability but out of scope for Lintel's
  current milestone; the transferable piece is only the "objects self-describe building
  semantics" idea (`src/Mod/BIM/ArchIFC.py` — URL above).
- **Multi-layer (multi-material) wall thickness auto-depth in hole depth.** Depends on
  FreeCAD's material layer model, which Lintel does not have; Lintel's hosted-opening
  margin should be a simple explicit setting (`ArchWindow.py` getSubVolume widths logic —
  URL above).
- **Solver algorithm switching UI (BFGS/LM/DogLeg advanced panel).** Lintel's planar
  constraint problems are small; one robust algorithm plus good failure reporting
  suffices (`TaskSketcherSolverAdvanced` / `GCS.cpp` — URL above).
- **"Hidden reference" expression function.** Reads values without creating dependency
  edges — a power-user escape hatch that risks silently stale recomputes; Lintel should
  keep all expression references dependency-creating
  (`src/App/ExpressionParser.h` HIDDENREF — URL above).
- **Carbon copy / clone / copy-paste of constrained geometry.** Workflow features around
  sketch reuse; not needed until Lintel has block templates solidified
  (`SketchObject.cpp` carbonCopy — URL above).

---

## 10-line summary of highest-value findings

1. FreeCAD separates *geometry*, *constraints*, and *parameters* cleanly; every
   dimensional constraint is a named, addressable datum that expressions can bind to.
2. The solver reports one unambiguous status at all times (empty / over-constrained /
   malformed / redundant / partially-redundant / failed / under-constrained with N DoF /
   fully constrained) and names the offending constraint ids.
3. Conflicting constraints produce an actionable message ("remove at least one of: 1, 4")
   with list truncation — directly portable to Lintel's status bar.
4. Solver feedback is colour-role based (driving vs reference vs expression-driven vs
   deactivated dims; normal vs construction vs invalid elements) — matches Lintel's
   token discipline.
5. Hosted openings are the key architectural pattern: opening records hosts; wall rebuild
   cuts the opening's void; editing the opening re-touches the host; void depth = host
   thickness + margin, overridable.
6. Deleting a host wall leaves the opening intact as a standalone object — Lintel should
   decide and document the same orphan semantics explicitly.
7. Walls are baseline + width + alignment (left/center/right) + offset; a *constrained*
   baseline drives the wall (Length read-only), an unconstrained one lets the wall drive
   the baseline — a crisp two-way parametric contract worth copying.
8. Level/storey objects propagate height to members with height 0 (`HeightPropagate`) and
   openings move with hosts (`MoveWithHost`).
9. Door swings are pure data (mode enum + hinge edge + percentage) from which plan
   symbols are derived — no bespoke swing code per door type.
10. Auto-constraints while drawing (with a "don't add redundant ones" rule) plus a datum
    dialog accepting either a quantity or an expression round out the UX loop that keeps
    FreeCAD's parametric editing fast.
