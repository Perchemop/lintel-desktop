# T-203 — Anchor + Constraint Round-Trip Tests

**Phase:** 2 (Core Fixes)

## Objective
Ensure anchored dimensions and constraints survive save/load identically, with the solver re-applying on load.

## Context
- T-104 (backward-compat load) established basic round-trip for anchors
- Constraints are now persisted on `Level` (CAP-111)
- Need comprehensive tests covering all constraint types with anchors

## Acceptance Criteria
- [ ] Test saves project with PerpendicularDistance + anchored dimension → loads identically
- [ ] Test saves project with FixedAngle + two anchored dimensions → loads identically
- [ ] Test saves project with EqualLength + two anchored dimensions → loads identically
- [ ] Test saves project with mixed constraints + anchors → loads identically
- [ ] Verify anchors deserialize with correct `wall_id`, `param`, `perpendicular`
- [ ] Verify constraints deserialize with correct `dimension`, `wall_id`, `param` / `degrees`
- [ ] Verify solver re-applies on load (dimension geometry matches constraint)

## Files to Modify
- `crates/lintel-core/src/persistence.rs` — add tests in `#[cfg(test)]` module
- `crates/lintel-core/src/constraint.rs` — if solver behavior needs verification

## Test Cases
1. **Single PerpendicularDistance**: One dimension anchored at both ends to same wall, constraint at param 0.5
2. **FixedAngle pair**: Two dimensions anchored to different walls, FixedAngle between them
3. **EqualLength pair**: Two dimensions anchored to different walls, EqualLength between them
4. **Mixed**: PerpendicularDistance on dim A + EqualLength(A, B) + FixedAngle(A, C)
5. **Multiple walls**: Dimensions anchored to 3+ different walls with various constraints

## Related
- CAP-111 (constraint persistence)
- T-104 (backward-compat load)
- `from_json` → `solve_constraints()` flow

## Effort Estimate
Small (test-only, persistence logic already implemented)