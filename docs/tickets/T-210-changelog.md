# T-210 — CHANGELOG.md v0.21.0 Section

**Phase:** 6 (Release Prep)

## Objective
Add v0.21.0 section to CHANGELOG.md following Keep a Changelog format.

## Acceptance Criteria
- [ ] Section `## [0.21.0] — YYYY-MM-DD` added at top of CHANGELOG.md
- [ ] Categories: Added, Changed, Fixed, Security (if applicable)
- [ ] Entries reference ticket IDs (T-200 through T-208)
- [ ] No "Internal" or "Refactor" categories — user-visible changes only
- [ ] Date matches release date

## Template
```markdown
## [0.21.0] — 2026-11-XX

### Fixed
- **T-200**: Fix kittest SIGSEGV in evidence generation on CI runners
- **T-201**: Door/window placement regression tests verify centreline projection
- **T-202**: Constraint solver 2-node EqualLength cycle stress tests
- **T-203**: Anchor + constraint round-trip persistence tests
- **T-204**: Dark palette WCAG AA contrast compliance
- **T-205**: High-contrast palette WCAG AA (7:1) contrast compliance
- **T-206**: Constraint solver performance optimization (< 50ms for 500 constraints)
- **T-207**: Inspector empty states for all sections
- **T-208**: Inspector Tab order audit for keyboard accessibility

### Changed
- Constraint solver: optimized graph traversal for 500 constraints
- Inspector: consistent empty states and keyboard navigation

### Security
- None
```

## Files to Modify
- `CHANGELOG.md`

## Related
- Guide §6.4 (release notes standard)
- T-211 (RELEASE_NOTES.md), T-212 (tag)

## Effort Estimate
Trivial (documentation only)