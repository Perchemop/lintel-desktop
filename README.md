# Lintel — Desktop Drafting Application

**Lintel** is a desktop application for creating, editing, and exporting architectural floor plans. Build walls, place doors and windows, add multiple levels, and render PDF plans — all from a native Rust GUI.

## Try it (without installing Rust)

1. **Download the CI artefact** from the GitHub Actions page (releases tab).
2. **Extract the archive** for your platform.
3. **Run the binary**:
   - Linux: `./lintel`
   - macOS: `./lintel` (or `open lintel.app`)
   - Windows: `lintel.exe`
4. **Try the workflow**:
   - Click "New Project" to start a fresh plan
   - Click "Add Wall" to draw a wall
   - Click "Place Door" or "Place Window" to add an opening
   - Use "Save" to save the project as JSON
   - Use "Reopen" to load a saved plan
   - Render a PDF plan export

## Build from source (requires Rust)

```bash
# Clone the repository
git clone https://github.com/perchemop/lintel-desktop.git
cd lintel-desktop

# Install dependencies
rustup default stable

# Build
cargo build --release -p lintel

# Run
./target/release/lintel
```

## Try headless tests

```bash
cargo test --workspace --lib -p lintel-core
```

All workflow tests run without a display, GPU, or window server.

## Documentation

- `docs/project-brief.md` — product overview and capabilities
- `docs/scope.md` — in-scope and out-of-scope capabilities for v0.1.0
- `docs/inspiration.md` — behavioural summaries and public sources
- `docs/product-name.md` — product name origin and handling
- `docs/assumptions-and-questions.md` — open decisions
- `ROADMAP.md` — staged milestones and future capabilities
- `CHANGELOG.md` — release history
- `ATTRIBUTION.md` — third-party licences
- `DECISIONS.md` — recorded project decisions
- `PROGRESS.md` — current progress tracking

## Licence

SPDX-License-Identifier: (MIT OR Apache-2.0)

See `LICENSE-MIT` and `LICENSE-APACHE` for details.

## Third-party licences

| Crate | Licence |
|-------|---------|
| `eframe` | MIT OR Apache-2.0 WITH LLVM-exception |
| `egui` | MIT OR Apache-2.0 WITH LLVM-exception |
| `serde` | MIT OR Apache-2.0 |
| `thiserror` | MIT |
| `tempfile` | MIT OR Apache-2.0 |

Font licences: OFL-1.1, Ubuntu Font License 1.0

## Contact

For questions, issues, or contribution inquiries, please use the GitHub repository.