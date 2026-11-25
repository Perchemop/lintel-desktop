# T-211 — RELEASE_NOTES.md v0.21.0 Section

**Phase:** 6 (Release Prep)

## Objective
Add v0.21.0 section to RELEASE_NOTES.md quoting the token ledger.

## Acceptance Criteria
- [ ] Section `## v0.21.0 — YYYY-MM-DD` added at top of RELEASE_NOTES.md
- [ ] Includes token ledger quote from `docs/tokens.md` (cumulative + this release)
- [ ] User-facing summary of fixes (plain language)
- [ ] Links to GitHub Release assets
- [ ] Format matches previous releases (v0.20.0, v0.20.1)

## Template
```markdown
## v0.21.0 — 2026-11-XX

### Summary
A stability and polish release fixing CI reliability, improving accessibility contrast, optimizing the constraint solver, and polishing the inspector experience. No new capabilities — just a smoother, more reliable drafting experience.

### Highlights
- **CI reliability**: Fixed flaky kittest SIGSEGV in design-system evidence generation
- **Accessibility**: Dark and high-contrast themes now meet WCAG AA (4.5:1/3:1/7:1)
- **Performance**: Constraint solver handles 500 constraints in < 50ms
- **Inspector**: Consistent empty states and logical keyboard navigation

### Token Metrics
- **Tokens this release**: X,XXX (from `docs/tokens.md`)
- **Cumulative tokens**: XX,XXX (from `docs/tokens.md`)

### Download
- Windows: `lintel-windows-x86_64-setup.exe` (signed)
- macOS: `lintel-macos-aarch64.dmg` (signed)
- Linux: `lintel-appimage-x86_64.AppImage` (signed)
- Checksums: `SHA256SUMS.txt` + `SHA256SUMS.txt.asc` (GPG signed)

### Verification
See `docs/release-verification.md` for GPG verification instructions.
```

## Files to Modify
- `RELEASE_NOTES.md`

## Related
- `docs/tokens.md` (ledger source)
- Guide §24.1 (token metrics in release notes)
- T-210 (CHANGELOG), T-212 (tag)

## Effort Estimate
Trivial (documentation only)