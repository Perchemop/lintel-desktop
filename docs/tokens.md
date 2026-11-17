# Token-usage ledger (CI-managed, guide §24.1)

This file is the source of truth for Lintel's token-usage metrics
(guide §24.1; it supersedes the former `docs/tokens.json` ledger). The
release workflow (`release.yml`, step "Update docs/tokens.md ledger") reads
the cumulative figure from the last table row and appends one row per semver
tag; the commit is authored by `Lintel Dev <dev@example.com>`.

Every number is the **recorded value at release time**, quoted verbatim from
the release's "Token metrics" section in `RELEASE_NOTES.md`. Figures are not
re-derived or reconciled after the fact: early releases recorded approximate
figures (`~`) and there is one known accumulation hiccup around 0.10.0 that
is kept as recorded. Comma grouping and the `~` marker appear in the
historical rows; CI writes underscore-grouped integers (`1_234_567`) since
the §24.1 migration. The ledger parser in `release.yml` accepts both styles
and treats a `—` cell (no figure recorded) as 0 when computing the next
cumulative total.

| Version | Date | Tokens this release | Tokens cumulative | Notes |
|---------|------|---------------------|-------------------|-------|
| v0.1.0 | 2026-11-04 | — | — | First functional test build |
| v0.2.0 | 2026-11-05 | ~2,200,000 | ~4,900,000 | Joint resolution, dimension lines, suite infrastructure |
| v0.3.0 | 2026-11-05 | ~300,000 | ~5,200,000 | Error feedback and macOS bundle |
| v0.4.0 | 2026-11-05 | ~200,000 | ~5,400,000 | Accessibility labels, keyboard shortcuts, AppImage |
| v0.5.0 | 2026-11-06 | ~120,000 | ~5,520,000 | Windows installer |
| v0.6.0 | 2026-11-06 | ~150,000 | ~5,670,000 | Keyboard-complete file flows and an accessible canvas |
| v0.7.0 | 2026-11-06 | ~160,000 | ~5,830,000 | Every documented capability is now reachable |
| v0.8.0 | 2026-11-07 | ~130,000 | ~5,960,000 | Keyboard-focus review |
| v0.9.0 | 2026-11-07 | ~120,000 | ~6,080,000 | Drawing Set |
| v0.9.1 | 2026-11-07 | ~5,000 | ~6,085,000 | Version fix |
| v0.10.0 | 2026-11-07 | 5,000 | 5,000 | Suite-standard conformance |
| v0.10.1 | 2026-11-07 | 5,000 | 10,000 | Window icon and installer version |
| v0.11.0 | 2026-11-11 | 5,000 | 10,000 | Constraint-Driven Dimensions |
| v0.12.1 | 2026-11-11 | 5,000 | 15,000 | Constraint-Driven Dimensions |
| v0.13.0 | 2026-11-13 | 5,000 | 45,000 | Anchored Dimensions & Release Governance |
| v0.14.0 | 2026-11-14 | 5,000 | 50,000 | Constraint Persistence & Sheet Layout |
| v0.15.0 | 2026-11-15 | 5,000 | 55,000 | Blocks & Symbols, Print Polish |
| v0.15.1 | 2026-11-15 | 5,000 | 60,000 | macOS Installer Fix |
| v0.16.0 | 2026-11-16 | 5,000 | 65,000 | Block Instance Editing & Canvas Selection |
