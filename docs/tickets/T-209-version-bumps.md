# T-209 — Version Bumps 0.20.1 → 0.21.0

**Phase:** 6 (Release Prep)

## Objective
Bump version numbers in both crates and Cargo.lock for v0.21.0 release.

## Acceptance Criteria
- [ ] `crates/lintel-core/Cargo.toml`: `version = "0.21.0"`
- [ ] `apps/lintel/Cargo.toml`: `version = "0.21.0"`
- [ ] `Cargo.lock` updated (run `cargo generate-lockfile` or let CI update)
- [ ] No other version references need updating (check `RELEASE_NOTES.md`, `CHANGELOG.md`)

## Files to Modify
- `crates/lintel-core/Cargo.toml`
- `apps/lintel/Cargo.toml`
- `Cargo.lock` (auto-generated)

## Verification
```bash
# After changes, verify:
cargo check --workspace
grep -r "0.20.1" --include="*.toml" --include="*.md" | grep -v "CHANGELOG" | grep -v "0.21.0"
```

## Related
- Guide §6.1 (semver policy)
- T-210 (CHANGELOG), T-211 (RELEASE_NOTES), T-212 (tag)

## Effort Estimate
Trivial (version bump only)