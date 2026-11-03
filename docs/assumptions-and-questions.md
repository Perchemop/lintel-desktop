# Assumptions and Questions

## Assumptions

1. **Human is a non-developer client** — they speak in goals, priorities, and "does this work / feel right?" rather than technical specifications.

2. **Primary local platform** is the human's daily-use OS (Linux, macOS, or Windows). The app must produce a runnable binary for that platform via CI.

3. **Human has moderate disk space** — the app and its artefacts will be stored locally, but heavy builds run on GitHub Actions.

4. **Free models are used for coding** — the agent team starts with OpenCode free models and falls back to NVIDIA free models when exhausted.

5. **Suite pattern** — this is the first app in a suite that will share `crash-report` and `update-check` crates with future applications.

6. **Clean-room implementation** — all behaviour is original; no reverse-engineered proprietary binaries or copied UI elements.

7. **Semver releases** — frequent (`0.1.0`, `0.1.1`, `0.2.0`, ...) GitHub Releases with detailed notes after first functional build.

8. **Headless CI testing** — `cargo test` runs without a display, GPU, or window server. Core domain logic is fully testable without a GUI.

9. **MIT/Apache-2.0 dual licence** — the default licence policy per guide Part I §2.2 item 9 and Part II §19.2.

10. **User-supplied tokens only** — crash reports and update checks use the user's GitHub token (via device flow); no embedded bot tokens.

## Open Questions (decisions recorded in DECISIONS.md)

| Q-ID | Question | Current Lean | Decision Needed |
|------|----------|-------------|-----------------|
| Q-001 | Primary local platform for first build | Linux (developer's daily driver) | Confirm with human |
| Q-002 | Unit system default | Metric (mm) | Confirm with human |
| Q-003 | Release frequency after v0.1.0 | Monthly-ish, gated on evidence | Confirm with human |
| Q-004 | Crash reporting: GitHub Issues token required? | Optional, fallback to pre-filled URL | Confirm with human |
| Q-005 | Public research: any capabilities require external docs? | Only egui/eframe API reference | Confirm with human |
| Q-006 | "Joint resolution" limitation: document as known gap for 0.2.x? | Yes, documented in `join.rs` | Confirm with human |