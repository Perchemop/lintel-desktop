# T-202 — Constraint Solver 2-Node Cycle Stress Test

**Phase:** 2 (Core Fixes)

## Objective
Validate that the constraint solver correctly handles 2-node EqualLength cycles (A ↔ B) without false-positive circular reference errors.

## Context
- EqualLength constraints create bidirectional dependencies (A depends on B, B depends on A)
- This is a valid 2-node cycle that should be allowed
- The cycle detection in `crates/lintel-core/src/constraint.rs` explicitly allows 2-node cycles
- Need stress tests with multiple interconnected 2-node cycles

## Acceptance Criteria
- [ ] Test creates 5+ pairs of dimensions with mutual EqualLength constraints
- [ ] Solver resolves all without "circular reference" error
- [ ] Test verifies solved values are consistent (A.length == B.length)
- [ ] Test mixes 2-node cycles with other constraint types (PerpendicularDistance, FixedAngle)
- [ ] Test with 10+ dimensions in various cycle topologies

## Files to Modify
- `crates/lintel-core/src/constraint.rs` — add test in `#[cfg(test)]` module
- `crates/lintel-core/src/constraint.rs` — verify `has_circular_reference` logic

## Test Topologies
1. **Simple pair**: A ↔ B (2 constraints)
2. **Chain of pairs**: A ↔ B, C ↔ D, E ↔ F
3. **Star**: A ↔ B, A ↔ C, A ↔ D (all equal to A)
4. **Mixed**: A ↔ B + PerpendicularDistance on A + FixedAngle on B
5. **Large**: 10 dimensions, 5 independent 2-node cycles

## Related
- `has_circular_reference` / `has_cycle_dfs` in constraint.rs
- CAP-101, CAP-111
- T-106 (core tests)

## Effort Estimate
Small (test-only, logic already implemented)