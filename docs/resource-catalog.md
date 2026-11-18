# Resource catalog

Adopted resources from the guide §25 catalogs, per §25.7 rule 2: every entry
records the catalog, the source, the licence, the version/date pinned, and the
decision reference.

## Agent skills (from shubhamsaboo/awesome-llm-apps, §25.4)

| Skill | Source | Licence | Pinned | Decision | Deployment |
|-------|--------|---------|--------|----------|------------|
| scope-creep-detector | https://github.com/shubhamsaboo/awesome-llm-apps/tree/main/agent_skills/scope-creep-detector | Apache-2.0 | 2026-11-17 | D-033 | Adapted clean-room into `~/.config/opencode/skill/scope-creep-check/` (invoked before every PR) |
| dependency-doctor | https://github.com/shubhamsaboo/awesome-llm-apps/tree/main/agent_skills/dependency-doctor | Apache-2.0 | 2026-11-17 | D-033 | Adapted into `~/.config/opencode/skill/dependency-doctor/` (invoked before every release tag) |
| first-reader | https://github.com/shubhamsaboo/awesome-llm-apps/tree/main/agent_skills/first-reader | Apache-2.0 | 2026-11-17 | D-033 | Adapted into `~/.config/opencode/skill/first-reader/` (invoked on user-facing docs) |
| commit-archaeologist | https://github.com/shubhamsaboo/awesome-llm-apps/tree/main/agent_skills/commit-archaeologist | Apache-2.0 | 2026-11-17 | D-033 | Adapted into `~/.config/opencode/skill/commit-archaeologist/` (invoked before refactors / why-questions) |

All four are clean-room adaptations: original wording, no upstream code, tuned
to this suite's workflow (`ci.sh` verification, decision-log conventions).

## pstack-derived skills (guide §22; registered natively in OpenCode)

| Skill | Provenance | Use in this repo |
|-------|-----------|------------------|
| verify-and-stop | pstack posture (guide §22) | Validation-only tasks, completion checks, last-mile proof before a tag |
| surgical-patch | pstack posture | Bug fixes at the narrowest responsible layer with regression proof (§5.5 item 4) |
| safe-refactor | pstack posture | Behaviour-preserving restructures (e.g. the ThemeSpec 3-tier move) |
| investigate-first | pstack posture | Ambiguous failures, intermittent behaviour (e.g. the v0.17.0 "app death" false alarm) |
| lean-build | pstack posture | New behaviour with strict scope + explicit stop condition |
| migration | pstack posture | Reversible transitions (e.g. the tokens.json → tokens.md ledger move) |

Adapted per guide §22.2 (read as reference; concepts used, not the plugin).
Licence Apache-2.0, recorded in `ATTRIBUTION.md`.

## MCP servers (from punkpeye/awesome-mcp-servers, §25.2)

| Server | Source | Licence | Status |
|--------|--------|---------|--------|
| Playwright MCP | https://github.com/microsoft/playwright-mcp | Apache-2.0 | Evaluated, deferred — no web/WASM target yet; egui_kittest covers AccessKit evidence natively |
| GitHub MCP | https://github.com/github/github-mcp-server | MIT | Superseded by `gh` CLI + `execute` (equivalent capability, fewer moving parts) |

## Free-tier services (from ripienaar/free-for-dev, §25.6)

| Service | Use | Re-verify before |
|--------|-----|------------------|
| GitHub Actions (ubuntu/macos/windows + arm) | CI + release artefacts | Every release (rate/minutes limits) |
| GitHub Releases | Distribution + update-check API | Every release |

No third-party free-tier service is currently load-bearing for Lintel's
pipeline; the update-check and crash-report paths read only GitHub APIs.
