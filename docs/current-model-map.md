# Current model map

Per addendum Section D.4 and guide Section 5.2: which free model is assigned
to which role in this session. Updated when the provider order fails over.

| Role | Model | Why |
|---|---|---|
| Orchestrator / research | OpenCode free (best instruction-following, web-capable) | Plan, discovery, session flow |
| Coder | OpenCode free (best free coding model) | Rust implementation |
| Reviewer | Second free model if available | PR / CI-fix review |

Provider order: OpenCode free → NVIDIA free → Groq/Google fallback. Record
failovers here when they occur.
