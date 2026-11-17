# Devlog — session continuity (local-only, gitignored)

## 2026-11-17

**Landed (all merged, CI green):**
- PR #48 — CAP-126 dimension label precision (domain + UI): `Dimension.label_precision`
  0–4 dp, `SetDimensionPrecision`, Inspector "Precision:" control, canvas label honours it
- PR #49 — CAP-126 tracking records + honest stale-checklist fix in PROGRESS.md
- PR #50 — CAP-127 single block-instance delete (`DeleteBlockInstance`), keyboard truth
  (plain Z/Y undo-redo, R release notes, Ctrl+U update check), palette honesty fixes
  (dead "View Constraints" removed, N-of-N footer fixed, `palette_shortcuts_are_unique`
  test), `docs/qc/shortcuts.md`, ACCESSIBILITY.md corrections
- PR #51 — guide 2026-11-17 sync record in docs/current-model-map.md

**Ruleset:** synchronized with guide.md 2026-11-17 (1,677 lines) — every section
verified; zero gaps. Notable: three-tier model rotation, 14 house rules, §9.1–9.4
QC restructure, §24 schemas, §25 catalogs.

**Installed + verified locally:** v0.16.0 AppImage (aarch64): SHA-256 + GPG signature
verified, stale v0.15.1 removed, .desktop updated, app runs stably.

**In flight (background agent):** docs/tokens.json → docs/tokens.md ledger migration
(guide §24.1) — release.yml rewrite + reconstructed table + tokens.json removal.

**Queued next:**
1. A11y Slice B — contrast unit test in theme.rs (II.30.7.1 merge gate) + move
   status_bar/command_palette raw colours into theme tokens + CI raw-colour grep gate
2. Release v0.17.0 — version bumps 0.16.0 → 0.17.0, RELEASE_NOTES.md + CHANGELOG
   (CAP-126, CAP-127, a11y slice), tag, verify installers + signed checksums
3. Open a11y limitation recorded honestly: block-instance selection is pointer-only

**Known loose ends:**
- `lintel-appimage-x86_64.AppImage` untracked in repo root — never commit
- Screenshot tooling unavailable on this compositor (grim lacks wlr-screencopy,
  not GNOME) — install evidence is checksum + signature + stable-run + version string
