# Devlog — session continuity (local-only, gitignored)

## 2026-11-17

**Landed (all merged, CI green):**
- PR #48 — CAP-126 dimension label precision (domain + UI)
- PR #49 — CAP-126 tracking records + honest stale-checklist fix
- PR #50 — CAP-127 block-instance delete + keyboard truth + palette honesty + a11y docs
- PR #51 — guide 2026-11-17 sync record
- PR #52 — tokens.md ledger migration (guide §24.1; 19-row reconstructed table;
  release.yml appends rows; tokens.json deleted; prose refs updated) **+ II.26.7
  enforcement**: `pr-author-is-dev` required check (ci.yml job + .ci-required.txt),
  D-031 in DECISIONS.md, ci.sh asserts `gh api user` against
  `~/.config/opencode/dev-identity` (= Perchemop) — verified live in PR #52's run

**Ruleset:** synchronized with guide.md 2026-11-17 (1,677 lines) — zero gaps.
II.26.1 also resolved (CONTRIBUTING.md wording); II.26.7 in force.

**In flight (background agent):** A11y Slice B — `relative_luminance`/`contrast_ratio`
pure functions + `theme_colours_meet_wcag_aa` test in theme.rs (II.30.7.1 merge gate);
raw Color32 in status_bar/command_palette/main.rs toolbar moved into named theme roles.

**Queued next:**
1. On Slice B report: verify diff, one CI run, merge (watch for failing-pair findings)
2. Release v0.17.0 — version bumps 0.16.0 → 0.17.0, RELEASE_NOTES.md + CHANGELOG
   (CAP-126, CAP-127, a11y slices), ledger note now cites docs/tokens.md, tag,
   verify installers + signed checksums + release body
3. Local install-and-run of v0.17.0 (same gates as v0.16.0)

**Known loose ends:**
- `lintel-appimage-x86_64.AppImage` untracked in repo root — never commit
- Screenshot tooling unavailable on this compositor — install evidence is
  checksum + signature + stable-run + version string
- Block-instance selection remains pointer-only (recorded honestly in
  ACCESSIBILITY.md; a keyboard path is a future slice)

