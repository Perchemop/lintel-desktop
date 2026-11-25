# T-206 — Constraint Solver Performance Optimization

**Phase:** 4 (Performance)

## Objective
Optimize `solve_constraints` to complete in under 50ms for 500 constraints on CI hardware.

## Context
- Current test `five_hundred_constraints_solve_quickly` in `crates/lintel-core/src/constraint.rs` asserts < 50ms
- Need to verify this passes reliably and optimize if needed
- Solver uses graph-based dependency resolution with DFS for cycle detection

## Acceptance Criteria
- [ ] `five_hundred_constraints_solve_quickly` test passes consistently
- [ ] Solver completes 500 PerpendicularDistance constraints in < 50ms on CI (ubuntu-latest)
- [ ] No regression in correctness (all constraints still resolve)
- [ ] Profile data shows bottleneck eliminated or mitigated

## Investigation Areas
1. **Graph construction**: `HashMap<DimensionId, HashSet<DimensionId>>` allocation overhead
2. **Cycle detection**: DFS on every solve — can cache for unchanged constraint sets
3. **Resolution order**: Topological sort vs current approach
4. **PerpendicularDistance resolution**: Vector math per constraint

## Files to Modify
- `crates/lintel-core/src/constraint.rs` — `solve_constraints`, `has_circular_reference`, `has_cycle_dfs`
- `crates/lintel-core/src/constraint.rs` — `five_hundred_constraints_solve_quickly` test

## Optimization Strategies
- [ ] Reuse graph allocation across solves (clear instead of new)
- [ ] Cache dependency graph when constraints unchanged
- [ ] Use `Vec` instead of `HashSet` for small neighbor sets
- [ ] Inline hot paths, avoid trait object dispatch
- [ ] Consider `ahash` for faster hashing

## Profiling
```bash
# On CI or local (if hardware permits)
cargo bench --bench constraint_solver  # if bench exists
# Or add temporary profiling in test
```

## Acceptance Test
```rust
#[test]
fn five_hundred_constraints_solve_quickly() {
    let level = create_test_level_500_constraints();
    let start = std::time::Instant::now();
    let _ = solve_constraints(&level, &level.constraints);
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 50, "solve took {} ms", elapsed.as_millis());
}
```

## Related
- CAP-111 (constraint persistence)
- MAX_CONSTRAINTS = 500 (REQ-CONST-004)
- T-202 (2-node cycle stress test must pass)

## Effort Estimate
Medium (profiling + optimization, risk of regression)