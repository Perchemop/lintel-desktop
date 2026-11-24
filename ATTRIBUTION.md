# Attribution

MIT OR Apache-2.0

Third-party licences used in this project:

| Crate | Version | Licence |
|-------|---------|---------|
| `eframe` | 0.36.2 | MIT OR Apache-2.0 WITH LLVM-exception |
| `egui` | 0.36.2 | MIT OR Apache-2.0 WITH LLVM-exception |
| `serde` | 1.0 | MIT OR Apache-2.0 |
| `serde_json` | 1.0 | MIT OR Apache-2.0 |
| `thiserror` | 2.0 | MIT OR Apache-2.0 |
| `tempfile` | 3.x | MIT OR Apache-2.0 |
| `ureq` | 3.x | MIT OR Apache-2.0 |
| `uuid` | 1.x | Apache-2.0 OR MIT |
| `resvg` | 0.45 | MIT OR Apache-2.0 |
| `usvg` | 0.45 | MIT OR Apache-2.0 |
| `tiny-skia` | 0.11 | MIT OR Apache-2.0 |
| `tiny-skia-path` | 0.11 | MIT OR Apache-2.0 |

The first-party shared crates `update-check` and `crash-report` are part of this
repository and carry the same dual licence as the rest of Lintel.

Font licences bundled inside egui/epaint_default_fonts:
- OFL-1.1 (Ubuntu Font License)
- Ubuntu Font License 1.0

All other dependencies are either MIT/Apache-2.0 or otherwise permissive licences allowed by the `deny.toml` licence policy.

No GPL/AGPL/MPL-2.0 crates are used without explicit maintainer approval.

## Icons

| Asset | Path | Licence | Notes |
|-------|------|---------|-------|
| Lintel icon (PNG/ICO) | `resources/linux/lintel.png`, `resources/windows/lintel.png`, `resources/windows/lintel.ico` | Original (MIT OR Apache-2.0) | Designed for Lintel; 256×256 PNG source |
| Lintel icon (SVG) | `resources/icons/lintel.svg` | Original (MIT OR Apache-2.0) | Scalable source for all platform icons |
| Tool & UI icons (SVG) | `apps/lintel/assets/icons/*.svg` | Original (MIT OR Apache-2.0) | 24×24 viewBox, stroke=#1A1816 (BASE_INK), 2px; 12 tool icons + 12 snap/layer/dim/block icons |
| CAD workflow icons (12) | `apps/lintel/assets/icons/*.svg` | Original (MIT OR Apache-2.0) | `snap_endpoint`, `snap_midpoint`, `snap_center`, `snap_intersection`, `snap_grid`, `snap_ortho`, `layer_visible`, `layer_lock`, `layer_print`, `dimension_aligned`, `dimension_radial`, `block_insert` |
## Agent-skill patterns (development-time only; no code shipped in the app)

- **scope-creep-detector, dependency-doctor, first-reader, commit-archaeologist** —
  patterns adapted from shubhamsaboo/awesome-llm-apps (agent_skills), Apache-2.0.
  Original wording; no upstream code copied. Used at development workflow gates.
- **pstack playbooks** (verify-and-stop, surgical-patch, safe-refactor,
  investigate-first, lean-build, migration) — concepts from
  cursor/plugins/pstack (Apache-2.0), consumed as native OpenCode skills.
- **Native DSH skills** (verify-and-stop, surgical-patch, safe-refactor,
  investigate-first, lean-build, migration, scope-creep-check, dependency-doctor,
  first-reader, commit-archaeologist) — native DSH skill packages installed at
  `~/.config/dsh/skill/`, Apache-2.0, invoked automatically at workflow gates
  per `~/.config/opencode/instructions/skill-triggers.md`.
- **caveman** — terse response style (short sentences, minimal words), MIT licence,
  `~/.config/opencode/skill/caveman/`, invoked when concise output is needed.
