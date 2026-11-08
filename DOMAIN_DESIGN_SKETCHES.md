# Domain Design Sketches — Elite CAD Features
## Detailed Rust Type Proposals for crates/lintel-core

This document provides concrete Rust sketches for each feature area. These are *design proposals*, not production code.

---

## 1. Geometric Constraints — Detailed Types

```rust
// crates/lintel-core/src/constraint.rs

use serde::{Deserialize, Serialize};
use crate::ids::{ConstraintId, EntityId, WallId};
use crate::geom::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Endpoint { Start, End }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointRef {
    Wall { wall_id: WallId, endpoint: Endpoint },
    WallPoint { wall_id: WallId, offset_mm: f64 },
    BlockRefPoint { block_ref_id: EntityId, local_point: Point2 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityRef {
    Wall(WallId),
    Arc(EntityId),
    Circle(EntityId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintKind {
    Coincident { a: PointRef, b: PointRef },
    Parallel { a: EntityRef, b: EntityRef },
    Perpendicular { a: EntityRef, b: EntityRef },
    Horizontal { entity: EntityRef },
    Vertical { entity: EntityRef },
    Tangent { a: EntityRef, b: EntityRef },
    Concentric { a: EntityRef, b: EntityRef },
    EqualLength { a: EntityRef, b: EntityRef },
    Distance { a: PointRef, b: PointRef, value: f64 },
    Angle { a: EntityRef, b: EntityRef, value_deg: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub id: ConstraintId,
    pub kind: ConstraintKind,
    pub strength: ConstraintStrength,
    pub status: ConstraintStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintStrength { Required, Soft }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintStatus { Satisfied, Overconstrained, Conflicting, NeedsResolve }

impl Level {
    pub fn constraints(&self) -> &[Constraint] { &self.constraints }
}
```

**Solver API (pure, headless):**
```rust
pub struct ConstraintSolver {
    max_iterations: usize,
    tolerance_mm: f64,
}

impl ConstraintSolver {
    pub fn solve(&self, level: &Level) -> Result<SolvedLevel, ConstraintError>;
}
```

For v0.12.0, store constraints but solver returns `Unimplemented` — UI shows "Constraints not yet solved".

---

## 2. Block/Symbol System — Detailed Types

```rust
// crates/lintel-core/src/block.rs

use serde::{Deserialize, Serialize};
use crate::geom::Point2;
use crate::ids::{BlockId, BlockRefId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDef {
    pub id: BlockId,
    pub name: String,
    pub description: String,
    pub origin: Point2, // insertion reference point
    pub entities: BlockEntityData, // stored as serialized JSON of Entity
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRef {
    pub id: BlockRefId,
    pub block_def_id: BlockId,
    pub insert_point: Point2,
    pub scale: f64, // uniform for v0.12
    pub rotation_deg: f64,
    pub mirrored: bool,
}

// BlockRef is part of Level alongside walls
pub struct Level {
    // ...
    block_refs: Vec<BlockRef>,
    blocks: Vec<BlockDef>, // per-document blocks
}
```

**Commands:**
```rust
Command::CreateBlock {
    name: String,
    entities: Vec<EntityId>, // selection
}
Command::InsertBlock {
    block_def_id: BlockId,
    insert_point: Point2,
    scale: f64,
    rotation_deg: f64,
}
Command::EditBlockDefinition {
    block_def_id: BlockId,
    entity_updates: Vec<EntityPatch>,
}
```

**Rendering:**
```rust
impl PlanGeometry {
    pub fn resolve_blocks(&self, level: &Level) -> Vec<ResolvedEntity> {
        // Expand BlockRefs into resolved entities with transform applied
    }
}
```

---

## 3. Constraint-Driven Dimensioning

```rust
// crates/lintel-core/src/dimension.rs (evolved)

pub enum DimensionAnchor {
    EntityPoint { entity_id: EntityId, point_ref: PointRef },
    Free(Point2),
}

pub struct Dimension {
    pub id: DimensionId,
    pub kind: DimensionKind,
    pub start_anchor: DimensionAnchor,
    pub end_anchor: DimensionAnchor,
    pub offset_mm: f64,
    pub extension_gap_mm: f64,
    pub extension_overrun_mm: f64,
    pub text_override: Option<String>,
}

impl Dimension {
    pub fn resolved_start(&self, level: &Level) -> Option<Point2> {
        match &self.start_anchor {
            DimensionAnchor::EntityPoint { entity_id, point_ref } => {
                // Resolve point_ref against entity
                Some(resolve_point_ref(entity_id, point_ref, level)?)
            }
            DimensionAnchor::Free(p) => Some(*p),
        }
    }
}
```

**Backward compatibility:** On deserialization of old files, free points become `DimensionAnchor::Free`. On save, upgrade anchors where possible via heuristic.

---

## 4. Construction Geometry

```rust
// crates/lintel-core/src/construction.rs

pub enum ConstructionKind {
    Line { start: Point2, end: Point2 },
    InfiniteLine { point: Point2, direction: Vec2 },
    Ray { origin: Point2, direction: Vec2 },
    Circle { center: Point2, radius: f64 },
    Arc { center: Point2, radius: f64, start_deg: f64, end_deg: f64 },
}

pub struct ConstructionEntity {
    pub id: ConstructionId,
    pub kind: ConstructionKind,
    pub snap_enabled: bool,
    pub visible: bool,
    pub print_enabled: bool, // always false for v0.12
}

pub struct Level {
    // ...
    construction: Vec<ConstructionEntity>,
}
```

**PDF export:** `if entity.is_construction() { skip }`

---

## 5. Entity Snapping — Enhanced

```rust
// crates/lintel-core/src/snap.rs (evolved)

pub enum SnapCandidateKind {
    Endpoint,
    Midpoint,
    Center,
    Intersection,
    Perpendicular,
    Tangent,
    Quadrant,
    Grid,
    Ortho,
    Free,
}

pub struct SnapCandidate {
    pub point: Point2,
    pub kind: SnapCandidateKind,
    pub priority: u8,
    pub source: SnapSource,
}

pub enum SnapSource {
    WallEndpoint(WallId),
    WallMidpoint(WallId),
    Intersection(WallId, WallId),
    // etc
}

pub fn generate_snap_candidates(
    level: &Level,
    settings: &SnapSettings,
) -> Vec<SnapCandidate> {
    // Pre-compute intersections, midpoints, etc.
}
```

**Snap priority order:**
1. Endpoint (100)
2. Intersection (90)
3. Perpendicular (80)
4. Tangent (70)
5. Midpoint (60)
6. Center (50)
7. Quadrant (40)
8. Grid (20)
9. Ortho (10)
10. Free (0)

---

## 6. Layer System — Detailed Types

```rust
// crates/lintel-core/src/layer.rs

pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub color: Option<Rgb>,
    pub linetype: Linetype,
    pub lineweight_mm: f64,
    pub visible: bool,
    pub locked: bool,
    pub printable: bool,
}

pub enum Linetype {
    Continuous,
    Dashed { dash: f64, gap: f64 },
    Hidden { dash: f64, gap: f64 },
    Center { dash: f64, gap: f64 },
}

// All entities get layer_id
pub struct Wall {
    pub id: WallId,
    pub layer_id: LayerId,
    // ...
}

pub struct Level {
    pub layers: Vec<Layer>,
    // ...
}
```

**Default layer:** `LayerId(0)` = "0", always exists, can't be deleted.

**Commands:**
```rust
Command::CreateLayer { name, color, linetype, lineweight }
Command::DeleteLayer { layer_id }
Command::SetLayerProperty { layer_id, property }
Command::AssignEntityToLayer { entity_id, layer_id }
```

---

## 7. Entity Properties

```rust
// crates/lintel-core/src/style.rs

pub struct EntityStyle {
    pub layer_id: LayerId,
    pub color_override: Option<Rgb>,
    pub linetype_override: Option<Linetype>,
    pub lineweight_override: Option<f64>,
    pub transparency: f32, // 0.0 = opaque, 1.0 = invisible
}

impl Default for EntityStyle {
    fn default() -> Self {
        Self {
            layer_id: LayerId(0),
            color_override: None,
            linetype_override: None,
            lineweight_override: None,
            transparency: 0.0,
        }
    }
}

// Add to all entities
pub struct Wall {
    pub id: WallId,
    pub style: EntityStyle,
    // ...
}
```

---

## 8. Selection Filtering

```rust
// crates/lintel-core/src/selection.rs

pub enum SelectionMode {
    Window { bounds: Bounds, crossing: bool },
    Fence { points: Vec<Point2> },
    Lasso { points: Vec<Point2> },
}

pub enum SelectionFilter {
    All,
    EntityType(EntityType),
}

pub fn select(geometry: &PlanGeometry, mode: SelectionMode, filter: SelectionFilter) -> HashSet<EntityId>;
```

---

## 9. Object Grouping

```rust
// crates/lintel-core/src/group.rs

pub struct Group {
    pub id: GroupId,
    pub name: Option<String>,
    pub entity_ids: Vec<EntityId>,
}

pub struct Level {
    // ...
    groups: Vec<Group>,
}
```

**Atomic commands for groups:**
```rust
Command::MoveGroup {
    group_id: GroupId,
    delta: Vec2,
}
```

Implementation detail: `move_group` expands to `MoveWall`/`MoveOpening` for each entity in group, but does it in one command application to keep undo atomic.

---

## 10. Entity Selection Sets

```rust
// crates/lintel-core/src/selection_set.rs

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

---

## Integration Summary

All features fit the `thick-core/thin-shell` model:

- Core owns types, commands, validation, derived geometry
- UI only renders and builds commands
- Persistence is JSON via serde
- Tests are headless

**Module layout for v0.12.0:**
```
crates/lintel-core/src/
  block.rs
  constraint.rs
  construction.rs
  group.rs
  layer.rs
  selection.rs
  selection_set.rs
  style.rs
  snap.rs (enhanced)
```

No changes to `Document`, `Command` enum is extended, not replaced.

**Migration strategy:**
- Add new fields with `#[serde(default)]` for backward compatibility
- `Project::validate` checks layers exist for entities referencing them
- On load of old files, create default layer and assign all entities to it
