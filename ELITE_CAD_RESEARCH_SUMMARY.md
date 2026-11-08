# Elite CAD Feature Research — Executive Summary
## Lintel Domain Investigation for v0.11.1 / v0.12.0

**Investigator:** Domain Dev Subagent  
**Date:** 2026-11-08  
**Repo:** ~/lintel / crates/lintel-core  
**Objective:** Assess 10 professional CAD features for feasibility

---

## Current State Assessment

Lintel v0.10.1 is a clean, well-architected thick-core drafting tool with:
- Command-driven model (100% reproducible)
- Headless testable core (no UI dependencies)
- Walls, openings, dimensions, labels, rooms, levels
- Snapping: endpoint, ortho, grid
- PDF export

**Gaps vs. elite paid tools:** No constraints, no blocks, no layers, limited snapping, static dimensions, no construction geometry, no selection tools.

---

## Feature-by-Feature Feasibility

| # | Feature | Complexity | v0.11.1 | v0.12.0 | Key Risk |
|---|---------|-----------|---------|---------|----------|
| 1 | Geometric constraints | High | No | Partial (storage only) | Solver research needed |
| 2 | Block/symbol system | Medium | Yes (minimal) | Yes (full) | Transform math, rendering |
| 3 | Constraint-driven dimensioning | Medium | Yes (anchors) | Yes (driving) | Backward compat |
| 4 | Construction geometry | Low | Yes | Yes | PDF filter |
| 5 | Entity snapping (midpoint/intersection/perpendicular) | Low | Yes | Yes | Intersection O(n²) |
| 6 | Layer system | Medium | Minimal | Yes (full) | Schema migration |
| 7 | Entity properties | Low | Yes (with layers) | Yes | Style inheritance |
| 8 | Selection filtering (window/fence) | Low | Yes | Yes | Pure functions |
| 9 | Object grouping | Medium | Partial | Yes | Atomic commands |
| 10 | Entity selection sets | Low | No | Yes | Data structure only |

---

## Recommended v0.11.1 Scope

**Must-have for professional credibility:**

1. **Construction geometry** (1 week)
   - New `ConstructionEntity` type in `Level`
   - Commands: Add/Delete construction line/ray
   - PDF export skips construction
   - Snap candidates include construction entities

2. **Enhanced snapping** (1 week)
   - Add `Midpoint`, `Intersection`, `Perpendicular` to `SnapCandidateKind`
   - Pre-compute intersections for wall centrelines
   - Snap priority system

3. **Constraint-driven dimensions** (2 weeks)
   - Replace `Dimension.start/end: Point2` with `DimensionAnchor`
   - Anchor can be `EntityPoint` or `Free`
   - Witness lines with gap/overrun
   - Backward compatibility via deserialization upgrade

4. **Selection filtering** (1 week)
   - `select::select_by_window` and `select_by_fence` pure functions
   - UI consumes, core owns logic

**Optional if time permits:**
- Minimal layers: add `Layer` struct, default layer, entity `layer_id` with migration

---

## Recommended v0.12.0 Scope

1. **Block/symbol system**
   - `BlockDef` and `BlockRef` types
   - Commands: CreateBlock, InsertBlock, EditBlock
   - Rendering expands blocks at draw time
   - Library import/export

2. **Full layer system**
   - Freeze/thaw, lock/unlock
   - Layer filters
   - Color/linetype/lineweight per layer
   - Entity style overrides

3. **Object grouping**
   - `Group` entity with `entity_ids`
   - Atomic MoveGroup command
   - Group explode

4. **Entity selection sets**
   - Named persistent selections
   - Project-level storage

---

## Architectural Principles Maintained

- **Thick-core/thin-shell:** All features are pure domain logic
- **Commands:** All edits via `Command` enum → `Document::apply`
- **Serde:** All types derive `Serialize/Deserialize` with `#[serde(default)]` for backward compat
- **Validation:** `Project::validate` checks new invariants
- **Headless tests:** Every new type gets unit tests in `#[cfg(test)]`
- **Derived geometry:** `PlanGeometry` expands blocks/groups/construction for rendering

---

## Domain Types to Add

**v0.11.1:**
- `construction.rs` — `ConstructionEntity`, `ConstructionKind`
- `dimension.rs` — `DimensionAnchor` enum, evolve `Dimension`
- `snap.rs` — `SnapCandidate`, `SnapSource`, intersection pre-compute
- `selection.rs` — `select_by_window`, `select_by_fence`

**v0.12.0:**
- `block.rs` — `BlockDef`, `BlockRef`, `BlockEntityData`
- `layer.rs` — `Layer`, `Linetype`
- `group.rs` — `Group`
- `selection_set.rs` — `SelectionSet`
- `style.rs` — `EntityStyle`

---

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Constraint solver complexity | Defer solving to v0.13.0; store constraints as documentation |
| Block transform math | Use uniform scale only for v0.12.0; non-uniform later |
| Layer migration breaks old files | `#[serde(default)]` + default layer assignment on load |
| Intersection O(n²) performance | Domestic plans <500 walls; compute once per snap operation |
| Dimension anchor migration | On load, convert free points to anchors via heuristic search |

---

## Next Steps

1. Review this report with orchestrator
2. Approve v0.11.1 scope
3. Domain dev creates `construction.rs`, enhanced `snap.rs`, `dimension.rs` anchor refactor
4. CI runs headless tests
5. UI dev wires selection filtering and construction drawing tools

---

## Files Created

- `DOMAIN_INVESTIGATION_ELITE_CAD.md` — full design report
- `DOMAIN_DESIGN_SKETCHES.md` — concrete Rust type proposals
- `ELITE_CAD_RESEARCH_SUMMARY.md` — this executive summary

All files are documentation only. No code was written per investigation mandate.
