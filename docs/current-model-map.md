# Current model map

Per guide Appendix D.4 and guide Section 5.2: which free model is assigned
to which role in this session. Updated when the provider order fails over.

| Role | Model | Why |
|---|---|---|
| Orchestrator / research | OpenCode free (best instruction-following, web-capable) | Plan, discovery, session flow |
| Coder | OpenCode free (best free coding model) | Rust implementation |
| Reviewer | Second free model if available | PR / CI-fix review |

Provider order: OpenCode free → NVIDIA free → Groq/Google fallback. Record
failovers here when they occur.

## Guide.md sync (guide §12.3)

Guide read in full at this session: all sections 0–18, Part II II.18–II.30,
Appendix A–O. Guide revision: 2026-11-09 (added II.30 Rust GUI/UX). This
ruleset was already synchronized to guide.md.

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