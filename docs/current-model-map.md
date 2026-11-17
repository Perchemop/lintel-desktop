# Current model map

Per guide Section 5.2: which free model is assigned to which role in this session. Updated when the provider order fails over.

| Role | Model | Why |
|---|---|---|
| Orchestrator / research | OpenCode free (best instruction-following, web-capable) | Plan, discovery, session flow |
| Coder | OpenCode free (best free coding model) | Rust implementation |
| Reviewer | Second free model if available | PR / CI-fix review |

Provider order: OpenCode free → NVIDIA free → Groq/Google fallback. Record failovers here when they occur.

## Guide.md sync (guide §12.3)

Guide read in full at this session: all sections 0–18, Part II II.18–II.30, Section 19 (Claude Code / Codex Skills), Section 20 (Design Anti-Patterns — 30 vibecoded signals). Guide revision: 2026-11-09. This ruleset was synchronized to the current guide.md.

Key changes from previous version:
- guide.md shortened to 1406 lines (appendices A–O removed)
- Section 19 added: Claude Code / Codex Skills catalog (Top 10 by stars)
- Section 20 added: Design Anti-Patterns — 30 vibecoded design signals to avoid
- II.30.14 Non-goals simplified to 8 items
- II.30.17 Quick card retained
- No Appendix A–O; merge content now inline in sections

## Design skills (guide II.27.11)

Three design skills cloned into `~/tools/` per guide II.27.12:

| Skill | Path | Role (per guide II.27.11) |
|---|---|---|
| `hallmark` | `~/tools/hallmark` | Page composition, type scale, spacing rhythm (CSS/HTML — method, not mechanics in egui apps) |
| `impeccable` | `~/tools/impeccable` | Critique vocabulary: hierarchy, density, states, copy, motion, a11y |
| `security-audit` | `~/tools/security-audit-skill` | Threat-model review of anything the design exposes (guide II.20) |

A theme dealt by a skill is a starting point, never the product identity (guide II.27.11). One theming/design path per app; `hallmark` + `impeccable` + `security-audit` must not be mashed together in one binary (guide II.30.4 hard limit).

## OpenCode skills (local config)

Available skill directories under `~/.config/opencode/skills/` include: `rust-engineer`, `security-reviewer`, `secure-code-guardian`, `typescript-pro`, `vue-expert`, `nextjs-developer`, and 48 others. Skill selection is per-agent via the `skill` tool invocation; no session-level skill default is set.

## Non-goals (guide II.30.14)

- No Astryx, React, StyleX or any CSS design system inside a native Rust GUI
- No two competing egui design systems in one binary
- No rebuilding the app shell without explicit migration task + client approval
- No copying inspiration product's pixels, icons, fonts, string tables or presets
- No emoji set standing in for an icon system
- No toolkit/design-crate version bump hidden inside a UX slice
- No "design improved" claimed without contrast, keyboard or rendered evidence
- No heavy build, test or cargo command on the client machine

## Non-negotiables enforced every session

- `guide.md` and `addendum.md` stay local forever — never pushed
- No cargo locally; all heavy work via `ci.sh` → GitHub Actions
- All changes land on `main` via merged PR as `Lintel Dev <dev@example.com>`
- Secret scan + personal-data scan before every commit/push/release (II.29)
- CI green before asking client to try a build
- Release ships installable artefacts per platform (not just zips) plus `SHA256SUMS.txt` + `.asc`
- Release tag is ancestor of `main` before declaring completion

## Guide synchronization record (§12.3)

- **2026-11-17 — guide.md dated 2026-11-17 (1,677 lines) read in full** and
  diffed against the agent ruleset. Ruleset now carries: §5.2 three-tier
  free-model rotation (opencode → nvidia → openrouter `:free`, owner-directed
  tier 3; Groq never rotated; failover.json is the only control surface),
  §5.5 fourteen house rules (Dev-identity authorship per II.26.7), §6.1
  versioning policy, §6.5 security releases, §7.1 mandatory crash scrub +
  II.29 fifth scan surface (crash payload), §7.2 signing-key compromise
  procedure, §9.1–9.4 QC restructure (feature gates, GitHub hygiene, branch
  protection checklist, release/in-app gates), §11 tracking files incl.
  `docs/tokens.md` (supersedes `docs/tokens.json`), `docs/dev.md`,
  `docs/update-signing.md`, `docs/resource-catalog.md`,
  `contributors/people.toml`, §19 token-efficiency skills (measured-only
  rule), §20 product-facing-copy scope note, §23 config appendices, §24
  token/contributor schemas, §25 six external catalogs with binding use
  rules, §22 pstack posture.
- **Repo consequence queued:** the release workflow still writes
  `docs/tokens.json`; migrating the ledger to `docs/tokens.md` (§24.1) is a
  release-infra slice for the next release cycle.
- **2026-11-16 — guide.md dated 2026-11-16 (1,557 lines) read in full** (§0–18,
  II.18–II.30, §12, §19–22) and diffed against the agent ruleset
  (`~/.config/opencode/AGENTS.md`). Ruleset updated with: §6.1 Versioning
  Policy (semver, pre-releases, tag-after-green), §6.3 readiness-% formulas +
  CHANGELOG-as-source-of-truth, §6.5 Emergency/Security Releases, §7.2
  updater token persistence (keychain, minimal scopes) and required signature
  verification with `docs/update-signing.md` pointer, and the §22 pstack
  rigor posture (already carried from the same-day sync). Design guidelines
  re-pointed to guide §20/§21; `~/design-guidelines.md` deprecated.