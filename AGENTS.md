# Lintel — agent rules

This repository is governed by the suite guide and the global ruleset.

- **Guide:** `~/guide.md` (local, never committed) — single source of truth.
  Read every section before work that touches a capability area it governs.
- **Global ruleset:** `~/.config/opencode/AGENTS.md` — the binding rule set
  (Authority, §5.6 OpenCode protocol, §5.7 multi-agent, house rules, II.18–30).
- **Skill invocation:** `~/.config/opencode/instructions/skill-triggers.md` —
  user-installed skills are invoked automatically at their workflow gates
  (pre-PR → scope-creep-check; pre-tag → dependency-doctor; user-facing docs →
  first-reader; refactor → commit-archaeologist; plus the native
  verify-and-stop / surgical-patch / safe-refactor / investigate-first /
  lean-build / migration skills).
- **Repo-specific decisions:** `DECISIONS.md` (D-001–D-033).

## Repo conventions (quick card)

- Rust + egui/eframe 0.36 (pinned `=`). Command-centric: the UI sends
  `Command` values to `lintel-core`; it never mutates the model directly.
- Every `Command` variant is bound in `apps/lintel` or documented core-only
  (D-027, current state D-032).
- Tokens: one source, `apps/lintel/src/theme.rs`, three tiers (BASE → semantic
  roles → component). No raw `Color32` outside theme.rs.
- Verification only via `ci.sh` (HARD BUILD RULE, §4 item 5). The required
  checks are in `.ci-required.txt`.
- Git identity: `Lintel Dev <dev@example.com>`; GitHub actions from the Dev
  account `Perchemop` (II.26.7, D-031).
