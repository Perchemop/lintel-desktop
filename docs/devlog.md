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

**In flight (background agent):** v0.17.0 release docs (RELEASE_NOTES, CHANGELOG,
PROGRESS, inventory, parity, traceability). Version bumps already staged
(Cargo.toml ×2 + Cargo.lock → 0.17.0).

**A11y Slice B landed (PR #53, merged):** `relative_luminance`/`contrast_ratio` +
`theme_colours_meet_wcag_aa` merge gate in theme.rs; five palette roles darkened
to pass AA (TEXT_MUTED 110→102, HINT 128→100, STATUS_OK→26,112,62,
PALETTE_DISABLED 130→105, RECENT 140→95, CATEGORY 150→105 — measured with Python
sRGB maths before editing); raw Color32 in status_bar/command_palette/toolbar
moved into named theme roles; fixed the agent's `colour.r` field-vs-method
compile error (`Color32` in egui 0.36 uses `.r()`), four rustfmt diffs, and
clippy dead-code allows on the test-consumed helpers.

**Queued next (on docs report):**
1. One CI run for the release-docs commit → merge
2. Tag v0.17.0 → release workflow → verify installers + signed checksums + body
   (token metrics must cite docs/tokens.md, 5,000 / 70,000)
3. Local install-and-run of v0.17.0

**Known loose ends:**
- `lintel-appimage-x86_64.AppImage` untracked in repo root — never commit
- Screenshot tooling unavailable on this compositor — install evidence is
  checksum + signature + stable-run + version string
- Block-instance selection remains pointer-only (recorded honestly in
  ACCESSIBILITY.md; a keyboard path is a future slice)

