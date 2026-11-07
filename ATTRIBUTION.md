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