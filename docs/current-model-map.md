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