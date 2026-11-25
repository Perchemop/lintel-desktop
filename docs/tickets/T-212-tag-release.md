# T-212 — Tag v0.21.0 and Verify Release

**Phase:** 6 (Release Prep)

## Objective
Tag v0.21.0 on main after all CI checks pass, triggering the release workflow.

## Prerequisites (must be complete)
- [ ] T-200 through T-208: All fixes implemented and tested
- [ ] T-209: Version bumps committed
- [ ] T-210: CHANGELOG.md updated
- [ ] T-211: RELEASE_NOTES.md updated
- [ ] All required CI checks green on main:
  - `check` (format, clippy, tests)
  - `contrast-test`
  - `kittest`
  - `build-matrix` (ubuntu, macos, windows)
  - `pr-author-is-dev`

## Steps
1. Ensure on `main` branch with latest commits
2. Verify `cargo check --workspace` passes locally (or via CI)
3. Create annotated tag: `git tag -a v0.21.0 -m "v0.21.0"`
4. Push tag: `git push origin v0.21.0`
5. Monitor release workflow: `gh run list --workflow release.yml --limit 1`
6. Verify release publishes:
   - 8 artefacts (3 platforms × installers + checksums + signature)
   - `SHA256SUMS.txt` + `SHA256SUMS.txt.asc`
   - Release notes populated from RELEASE_NOTES.md
7. Verify in-app update check shows v0.21.0

## Rollback Plan
If release workflow fails:
- Delete tag: `git tag -d v0.21.0 && git push origin :refs/tags/v0.21.0`
- Fix issue
- Re-tag

## Files to Modify
- None (git tag only)

## Related
- Guide §6.1 (semver), §6.2 (v0.1.0 gate), §12.1 (release gate)
- `release.yml` workflow
- D-031 (Dev account authorship)

## Effort Estimate
Trivial (git operations, monitoring)