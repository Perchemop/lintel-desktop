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
