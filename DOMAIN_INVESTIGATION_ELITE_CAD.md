# Domain-Level Design Investigation: Elite 2D CAD Features for Lintel
## Report: Crates/lintel-core Domain Expansion Assessment

**Date:** 2026-11-08  
**Scope:** Domain-level types, traits, and architecture for making Lintel function like an elite paid 2D architectural drafting tool  
**Source:** Investigates `crates/lintel-core` current architecture and identifies gaps for professional CAD parity

---

## Current Architecture Summary

Lintel v0.10.1 follows **thick-core / thin-shell** (guide Appendix K.3):

- **`crates/lintel-core`** — headless domain, commands, persistence, PDF export, unit-tested  
  Modules: `ids`, `units`, `geom`, `wall`, `opening`, `dimension`, `label`, `level`, `project`, `command`, `document`, `join`, `snap`, `persistence`, `pdf`, `room`, `schedule`, `measurements`
- **`apps/lintel`** — egui/eframe UI shell only. Builds `Command` values, renders state.

**Core principles:**
- Commands are the only mutation path. UI never mutates model directly.
- All domain types derive `serde::Serialize/Deserialize` for JSON persistence.
- Geometry uses millimetre coordinates, y-up. UI flips y for screen.
- Derived geometry (`join::PlanGeometry`) is recomputed on demand, never stored.
- Validation on load and after every command prevents corruption.

Current entity model per `Level`:
```rust
Level {
  walls: Vec<Wall>,
  openings: Vec<Opening>,  // hosted on WallId
  dimensions: Vec<Dimension>, // free points
  labels: Vec<PlanLabel>
}
```

**Key constraints:** No construction geometry, no layers, no blocks, no constraints, no entity properties beyond basic geometry, snapping limited to endpoint/ortho/grid.

---

## 1. Geometric Constraints

### Conceptual Design
Professional CAD uses **parametric sketch constraints**: entities store both geometry and relationships. Constraints are solved when geometry changes (e.g., moving a wall end dragged a constrained parallel wall).

Constraint types needed for architectural drafting:
- **Geometric:** coincident, parallel, perpendicular, horizontal, vertical, tangent, concentric, equal length/radius
- **Dimensional:** distance between two points/lines, angle between lines, radius/diameter, length along entity
- **Symmetry:** mirrored pairs, pattern spacing
- **Construction:** construction entities (see §4)

### Rust Type Design
```rust
// crates/lintel-core/src/constraint.rs

/// A constraint ties entities together. It is *driven* (geometry follows) or *driving* (geometry is fixed).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub id: ConstraintId,
    pub kind: ConstraintKind,
    pub entities: Vec<EntityRef>, // references to walls, arcs, circles
    pub strength: ConstraintStrength, // Required | Soft (elastic)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintKind {
    Coincident { a: PointRef, b: PointRef },
    Parallel { e1: EntityRef, e2: EntityRef },
    Perpendicular { e1: EntityRef, e2: EntityRef },
    Horizontal { e: EntityRef },
    Vertical { e: EntityRef },
    Tangent { e1: EntityRef, e2: EntityRef },
    Concentric { e1: EntityRef, e2: EntityRef },
    EqualLength { e1: EntityRef, e2: EntityRef },
    Distance { e1: PointRef, e2: PointRef, value: f64 },
    Angle { e1: EntityRef, e2: EntityRef, value: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConstraintStrength {
    Required,
    Soft,
}

/// Reference to a point on an entity, not just an entity id.
/// Needed for midpoint, endpoint, quadrant of arcs.
pub enum PointRef {
    WallEndpoint { wall: WallId, end: Endpoint },
    WallPointAt { wall: WallId, offset: f64 },
    CircleCenter { circle: EntityId },
    ArcQuadrant { arc: EntityId, quadrant: u8 },
}

/// Entity reference that can be constrained.
pub enum EntityRef {
    Wall(WallId),
    Arc(EntityId),
    Circle(EntityId),
    Line(EntityId),
}
```

**Solver integration:** Constraints can't be solved incrementally with a command stack. Options:
1. **Full solver** (e.g., `nalgebra` + custom Jacobian / use `cad_solve`-style) — heavy, for v1.0+
2. **Deferred constraints** — constraints are stored but only enforced on explicit "re-solve" command. Suitable for v0.12.0.

**Fit:** Thick-core. `Constraint` becomes a first-class entity stored in `Level`. `Command::AddConstraint/RemoveConstraint/ToggleConstraint`. Solver is pure function `solve_constraints(level: &Level) -> Result<Level, Error>`.

**Feasibility:**
- **v0.11.1: Not feasible.** Requires solver research and entity graph. Storing constraints without solving is misleading.
- **v0.12.0: Partially feasible.** Store constraints as documentation (no solving). UI shows constraint symbols. Becomes feasible with a simple constraint manager library in v0.13.0+.

---

## 2. Block/Symbol System

### Conceptual Design
Blocks = named reusable symbol definitions consisting of entities. Instances (`BlockReference`) store transform (insertion point, scale, rotation, mirror). Editing the definition updates all instances.

Professional CAD distinction:
- **Anonymous blocks** (AutoCAD): arbitrary selection becomes a block
- **Named blocks**: library of doors/windows/standard details
- **Nested blocks**: blocks within blocks
- **Dynamic blocks**: parameters (not needed v1.0)

### Rust Type Design
```rust
// crates/lintel-core/src/block.rs

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDef {
    pub id: BlockId,
    pub name: String,
    pub description: String,
    pub entities: BlockEntities, // walls/openings/... but transformed relative to origin
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRef {
    pub id: BlockRefId,
    pub block_def_id: BlockId,
    pub insert_point: Point2,
    pub scale: Vec2, // non-uniform allowed in CAD
    pub rotation_deg: f64,
    pub mirrored: bool,
}

pub struct BlockEntities {
    // Can't reuse Wall directly because block has its own coordinate space.
    // Options:
    // A) Store entity data duplicated (simpler, serde-friendly)
    // B) Store entity ids referencing a shared block-local id space
}
```

**Block storage location:**
- **Per-document blocks**: `Project.blocks: Vec<BlockDef>`
- **Library blocks**: separate `.lintel-block` files (future)

**Commands:**
```rust
Command::CreateBlock { name, entity_selection: Vec<EntityId> }
Command::InsertBlock { block_def_id, insert_point, scale, rotation }
Command::EditBlockDef { block_def_id, transform_entities }
Command::ExplodeBlock { block_ref_id }
```

**Thick-core fit:** Very good. Blocks are pure domain. All transforms are applied at render time via `PlanGeometry`. Editing a block def requires command that mutates block and re-renders instances.

**Feasibility:**
- **v0.11.1: Feasible in minimal form.** Per-document named blocks with simple entities (walls + openings). No nesting, no scale distortion, only uniform scale. Roughly 600 LOC.
- **v0.12.0: v0.11.1 + block library loading.** Allow import/export of block definitions as JSON. Block editor UI (thin-shell).

**Migration impact:** Current `Level` entities need a discriminator: `Entity::Wall(Wall) | Entity::BlockRef(BlockRef)`. Alternative: keep blocks as first-class alongside walls, render them via expansion.

---

## 3. Constraint-Driven Dimensioning

### Conceptual Design
Current dimensions in Lintel are **associative but not parametric**: `Dimension` stores two `Point2` values exactly as placed. Moving a wall doesn't move the dimension points; they become stale.

Professional CAD dimensions:
- **Driven dimensions**: report current geometry, update automatically when entities move. Points reference entities (e.g., wall endpoint). Dimension line is derived, not stored.
- **Driving dimensions**: changing the dimension value modifies entity geometry (parametric).
- **Witness lines**: extension lines that connect dimension line to measured geometry, with gap and extension offsets.

### Current State
```rust
Dimension {
    id: DimensionId,
    kind: DimensionKind,
    start: Point2,  // raw point, not entity reference
    end: Point2,
    offset: f64,
    extension_lines: bool,
}
```
Length recomputed via `length()` but points are static.

### Domain Evolution
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionAnchor {
    Point { entity_id: EntityId, point_index: u8 },
    Free(Point2),
}

/// Dimension becomes entity-referenced.
pub struct Dimension {
    id: DimensionId,
    kind: DimensionKind,
    start_anchor: DimensionAnchor,
    end_anchor: DimensionAnchor,
    offset: f64,
    extension_gap: f64,      // space between witness line and geometry
    extension_overrun: f64,  // how far witness line extends beyond anchor
    text_override: Option<String>, // manual override vs auto-computed
}
```

**Command flow:**
- `PlaceDimension` now collects entity references via snap, not raw points.
- `MoveDimension` moves anchors to adjacent snap points.

**Feasibility:**
- **v0.11.1: Very feasible.** Change dimension storage to use anchors with fallback to free points for backward compatibility. Implement witness lines in PDF rendering (`join.rs`). This is a domain-first change with no solver needed.
- **v0.12.0:** Add driving dimensions (dimension change modifies geometry). Requires constraint solver.

---

## 4. Construction Geometry

### Conceptual Design
Construction lines are **infinite lines, rays, and construction circles/arcs** that guide drawing but are not printed/exported. They are always visible on canvas but flagged `construction: true`.

Professional use cases:
- Centerlines for wall grids
- Alignment guides for openings
- Temporary reference for snapping

### Rust Type Design
```rust
// New entity type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Entity {
    Wall(Wall),
    Opening(Opening),
    Dimension(Dimension),
    Construction(ConstructionEntity),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructionEntity {
    pub id: ConstructionId,
    pub kind: ConstructionKind,
    pub color: Option<Color>, // override from layer
    pub visible: bool,
    pub snap_allowed: bool, // can user snap to it?
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstructionKind {
    InfiniteLine { point: Point2, direction: Vec2 },
    Ray { origin: Point2, direction: Vec2 },
    ConstructionArc { centre: Point2, radius: f64, start_angle: f64, end_angle: f64 },
    ConstructionCircle { centre: Point2, radius: f64 },
}
```

**PDF/export filtering:** `pdf::render_plan` skips entities where `is_construction()` is true.

**Thick-core fit:** Excellent. Pure addition.

**Feasibility:**
- **v0.11.1: Feasible.** Add `ConstructionEntity` to `Level`. Commands `AddConstructionLine`, `DeleteConstruction`. Snap system already works with points; construction entities just provide more snap anchors. No rendering changes needed beyond filter.
- **v0.12.0:** Construction geometry editor, visibility toggles per construction type.

---

## 5. Entity Snapping

### Current State
`crate::snap` provides:
- `SnapKind::Endpoint` — wall ends
- `SnapKind::Ortho` — horizontal/vertical from origin
- `SnapKind::Grid` — grid round
- `SnapKind::Free`

### Elite CAD Snapping Requirements
Add snap types:
- **Midpoint** — center of wall segment
- **Center** — center of circle/arc
- **Intersection** — intersection of two lines (wall centreline with wall centreline)
- **Perpendicular** — perpendicular projection onto wall centreline
- **Tangent** — tangent point to arc/circle
- **Quadrant** — 45° points on arc
- **Node** — all existing points (wall ends + midpoints + intersections)

Snap priority: endpoint > intersection > perpendicular > midpoint > center > grid > free.

### Rust Type Design
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapKind {
    Endpoint,
    Midpoint,
    Center,
    Intersection,
    Perpendicular,
    Tangent,
    Quadrant,
    Node,
    Ortho,
    Grid,
    Free,
}

pub struct SnapCandidate {
    pub point: Point2,
    pub kind: SnapKind,
    pub priority: u8,
    pub source_id: Option<EntityId>, // for display
}
```

**Snap engine changes:**
- `snap_point` now takes a `SnapContext` that pre-computes all intersection points from wall centrelines (and later arcs).
- Intersection computation is O(n²) for walls; for a domestic plan (<500 walls) this is trivial.

**Feasibility:**
- **v0.11.1: Feasible.** Add midpoint, center (for future arcs), intersection, perpendicular. Keep existing snap settings. ~400 LOC.
- **v0.12.0:** Quadrant, tangent snaps once arcs are domain entities.

---

## 6. Layer System

### Conceptual Design
Layers in CAD are organizational containers with visual properties:
- Each entity assigned a layer (`layer_id: LayerId`)
- Layer has: name, color, linetype, lineweight, visibility, printability, locked state
- Layers supersede per-entity properties

Lintel currently has `Level` (storey). Layers are next orthogonal concern: one level can have many layers.

### Rust Type Design
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub color: Option<Color>, // None = by layer (use default)
    pub linetype: Linetype,
    pub lineweight: f64,
    pub visible: bool,
    pub locked: bool,
    pub printable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Linetype {
    Continuous,
    Dashed,
    DashDot,
    Hidden,
    Center,
}

pub struct Level {
    id: LevelId,
    name: String,
    elevation: f64,
    layers: Vec<Layer>, // per-level layers
    walls: Vec<Wall>,
    openings: Vec<Opening>,
    // ... others
}

pub struct Wall {
    id: WallId,
    layer_id: LayerId, // replace per-entity color with layer reference
    start: Point2,
    end: Point2,
    thickness: f64,
    height: f64,
}
```

**Backward compatibility:** On load, if file has no `layer_id`, assign to default layer.

**Feasibility:**
- **v0.11.1: Feasible but disruptive.** Requires schema migration for all entities. Could add layers without migrating entities (entities keep color, layers provide defaults). Minimal change: add layers + per-entity `layer_id` with default.
- **v0.12.0: Recommended.** Full layer system with freeze/thaw, lock/unlock, layer filters.

---

## 7. Entity Properties

### Conceptual Design
Selecting an entity should expose editable properties:
- Layer, color, linetype, lineweight, transparency, plot style
- For walls: thickness, height
- For openings: width, height, sill, flip
- For dimensions: offset, extension lines, text override

Current inspector already edits thickness/height/size. Need to generalize.

### Rust Type Design
```rust
pub trait EntityProperties {
    fn layer_id(&self) -> Option<LayerId>;
    fn set_layer_id(&mut self, layer_id: LayerId);
    fn color(&self) -> Option<Color>;
    fn set_color(&mut self, color: Option<Color>);
    // etc.
}
```

Better: use a central `EntityStyle` struct stored per-entity or inherited from layer:
```rust
pub struct EntityStyle {
    pub layer_id: LayerId,
    pub color_override: Option<Color>,
    pub linetype_override: Option<Linetype>,
    pub lineweight_override: Option<f64>,
    pub transparency: f32,
}
```

**Commands:**
```rust
Command::SetEntityStyle { entity_id: EntityId, style_patch: StylePatch }
```

**Feasibility:**
- **v0.11.1: Feasible in conjunction with layers.** Add style struct to entities. Inspector shows layer dropdown + property overrides.
- **v0.12.0:** Full plot style support (PDF export respects linetypes/lineweights).

---

## 8. Selection Filtering

### Conceptual Design
Professional CAD selection modes:
- **Rectangle window** — select entities fully inside rectangle
- **Crossing window** — select entities intersecting rectangle
- **Fence** — polyline selects entities crossed
- **Lasso** — freeform selection
- **Ctrl+click** — additive selection toggle
- **Shift+click** — subtractive selection
- **Selection filter** — only select walls, only dimensions, etc.

Current selection is implied by hit test. No multi-select.

### Rust Type Design
Selection is UI state, but selection queries belong in core:
```rust
// crates/lintel-core/src/selection.rs
pub enum SelectionFilter {
    All,
    WallsOnly,
    OpeningsOnly,
    DimensionsOnly,
    LabelsOnly,
}

pub fn select_by_window(geometry: &PlanGeometry, window: Bounds, crossing: bool, filter: SelectionFilter) -> Vec<EntityId>;

pub fn select_by_fence(geometry: &PlanGeometry, fence: &[Point2], filter: SelectionFilter) -> Vec<EntityId>;
```

**Thick-core fit:** Selection queries are pure functions. UI maintains `selected: HashSet<EntityId>`.

**Feasibility:**
- **v0.11.1: Feasible.** Add `selection` module with `select_by_window` and `select_by_fence`. UI consumes it.
- **v0.12.0:** Selection sets (named selections).

---

## 9. Object Grouping

### Conceptual Design
Group entities into a logical unit that moves/selects/resizes together. Groups can be nested. Explode group → entities become independent.

### Rust Type Design
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: Option<String>,
    pub entity_ids: Vec<EntityId>, // flat list, no nesting for v0.12
    pub bounding_box: Bounds, // cached for hit test
}

pub struct Level {
    // ...
    groups: Vec<Group>,
}
```

**Commands:**
```rust
Command::GroupEntities { name, entity_ids: Vec<EntityId> }
Command::Ungroup { group_id: GroupId }
Command::ExplodeGroup { group_id: GroupId }
Command::MoveGroup { group_id: GroupId, delta: Vec2 }
```

**Feasibility:**
- **v0.11.1: Partially feasible.** Groups without nesting. Moving a group requires moving each entity individually (which fires validation). Complex.
- **v0.12.0: Feasible.** Group entities are treated as a unit for commands; `MoveGroup` translates all contained entities atomically.

---

## 10. Entity Selection Sets

### Conceptual Design
Named collections of entity IDs that persist across sessions. Used for "all windows", "exterior walls", etc.

### Rust Type Design
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionSet {
    pub id: SelectionSetId,
    pub name: String,
    pub entity_ids: HashSet<EntityId>,
}

pub struct Project {
    // ...
    selection_sets: Vec<SelectionSet>,
}
```

**Commands:**
```rust
Command::CreateSelectionSet { name, entity_ids }
Command::AddToSelectionSet { set_id, entity_ids }
Command::RemoveFromSelectionSet { set_id, entity_ids }
Command::DeleteSelectionSet { set_id }
Command::SelectSelectionSet { set_id } // UI-only, selection is UI state
```

**Feasibility:**
- **v0.12.0: Feasible.** Simple data structure, no geometry changes.

---

## Summary: Feasibility Roadmap

### v0.11.1 (Next release, conservative)
**High feasibility, high value, low risk:**
1. **Construction geometry** — infinite lines, rays. Pure addition. Enables reference drawing.
2. **Enhanced snapping** — midpoint, intersection, perpendicular. Core of precision drafting.
3. **Constraint-driven dimensioning (anchors)** — dimensions reference entities, not free points. Major usability win.
4. **Entity selection filtering** — window/crossing/fence selection queries in core. Enables multi-select.

**Partial:**
- **Layer system (minimal)** — add layers to project, per-entity layer_id, default layer. No freeze/thaw yet.

### v0.12.0
**Requires more design, but achievable:**
1. **Block/symbol system** — named blocks, insert, scale/rotate, edit definition. Essential for doors/windows libraries.
2. **Layer system (full)** — freeze/thaw, lock/unlock, layer filters, layer properties.
3. **Entity properties** — style overrides, transparency, plot style.
4. **Object grouping** — groups with move/explode.
5. **Entity selection sets** — named selections.
6. **Construction geometry editor** — UI controls for construction entities.

### v0.13.0+
**Major complexity:**
1. **Geometric constraints** — requires solver research, incremental solving, UI for constraint inference.
2. **Driving dimensions** — parametric editing.
3. **Dynamic blocks** — parameters.

---

## Architectural Notes for Thick-Core/Thin-Shell

All 10 areas fit thick-core:
- Types are pure, serde-compatible, no egui dependencies.
- Commands remain the mutation path.
- Derived geometry (`PlanGeometry`) expands to render blocks, layers, groups.
- Validation can enforce layer existence, block definition integrity, group entity validity.

**Clean-room compliance:** All types are original. No proprietary CAD logic is copied; only generic drafting concepts.

---

## Recommendation

For elite paid tool parity, prioritize in this order:
1. **Enhanced snapping + construction geometry** — immediate precision boost, foundational for everything else.
2. **Constraint-driven dimensions** — professional dimensioning is table stakes.
3. **Blocks/symbols** — essential for architectural reuse.
4. **Layers** — organizational maturity.
5. **Geometric constraints** — long-term differentiator, but requires solver investment.

This investigation provides the domain types, traits, and command patterns needed. UI can be built after core domain is stable and tested headless.
