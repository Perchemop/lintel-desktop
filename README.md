# Lintel

**Lintel** is a free desktop app for drawing simple 2D architectural floor plans — walls, doors, windows, and levels — made for anyone who wants to sketch out a space without learning professional CAD software.

## Why Lintel?

- **Free and open source.** Lintel costs nothing, and its source code is openly available under the MIT OR Apache-2.0 licence. No trial limits, no locked features.
- **Completely offline.** Lintel never needs an internet connection. Your plans stay on your own computer.
- **Fast and native.** A small native application that starts quickly and responds instantly — no browser, no web app.
- **No account, no subscription.** Download it, open it, and start drawing.

## Get started

1. **Download.** Visit the [Releases](https://github.com/Perchemop/lintel-desktop/releases) page of the Lintel repository and download the file for your computer (Linux, macOS, or Windows). The files are built automatically by GitHub Actions, so every release matches the published source code.
2. **Unzip.** Extract the downloaded archive to any folder.
3. **Run.** Double-click the Lintel icon:
   - **Linux:** open the `lintel` file (or run `./lintel` in a terminal).
   - **macOS:** open the Lintel app (or run `lintel` in a terminal).
   - **Windows:** double-click `lintel.exe`.

That's it — Lintel opens with a blank sheet and a toolbar on the side.

## What you can do today

- **Draw walls.** Pick the Wall tool and click twice on the sheet to draw a wall, then move it where you like. Wall thickness and height are adjustable.
- **Place doors and windows.** Pick the Door or Window tool and click on a wall. Openings sit inside the wall and can be moved, resized, or flipped.
- **Add levels.** Organise your building floor by floor and switch between them.
- **Add dimension lines.** Annotate the plan with measurements, in metric (mm) or imperial (feet/inches).
- **Undo and redo.** Every change can be stepped back.
- **Save and reopen plans.** Plans are saved as plain JSON files — a simple, readable format you can back up or copy anywhere.
- **Export a PDF sheet.** Turn the current level into a PDF plan sheet with a border, title block, scale label, and sheet number.

*Screenshots are coming.*

## Your data and privacy

- Everything happens on your computer. Lintel works fully offline and nothing you draw is ever uploaded anywhere.
- Plan files are saved wherever you choose, in a plain JSON format you can open with any text editor.
- There is no account, no sign-in, and no telemetry.
- Lintel checks GitHub for a newer release when it starts, and only reads public release information — never your plans or any personal data. If you are offline, the check simply says so and everything keeps working. You can also check on demand from the **Help** menu.
- If something ever goes wrong, a crash log is written locally on your computer. Nothing is sent anywhere unless you choose to open a pre-filled bug report yourself.

## Project status and roadmap

Lintel is young and actively developed. Version 0.2.0 adds joint resolution for unequal wall thicknesses and dimension lines. Installers, accessibility work, and more are on the way. See:

- [`ROADMAP.md`](ROADMAP.md) — what is done and what is planned
- [`CHANGELOG.md`](CHANGELOG.md) — release-by-release history

Found a bug or have an idea? Please open an issue on the [project's GitHub page](https://github.com/Perchemop/lintel-desktop).

## Documentation

- [`docs/capability-inventory.md`](docs/capability-inventory.md) — every capability and its status
- [`docs/architecture.md`](docs/architecture.md) — how the app is structured
- [`ROADMAP.md`](ROADMAP.md) — milestones and future capabilities
- [`CHANGELOG.md`](CHANGELOG.md) — release history
- [`ATTRIBUTION.md`](ATTRIBUTION.md) — third-party licences
- [`DECISIONS.md`](DECISIONS.md) — recorded project decisions

## Licence

SPDX-License-Identifier: (MIT OR Apache-2.0)

See `LICENSE-MIT` and `LICENSE-APACHE` for details.

## Third-party licences

| Crate | Licence |
|-------|---------|
| `eframe` | MIT OR Apache-2.0 WITH LLVM-exception |
| `egui` | MIT OR Apache-2.0 WITH LLVM-exception |
| `serde` | MIT OR Apache-2.0 |
| `serde_json` | MIT OR Apache-2.0 |
| `thiserror` | MIT OR Apache-2.0 |
| `tempfile` | MIT OR Apache-2.0 |
| `ureq` | MIT OR Apache-2.0 |
| `uuid` | Apache-2.0 OR MIT |

Font licences: OFL-1.1, Ubuntu Font License 1.0.

## For developers

Want to build Lintel from source or contribute? See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full guide. In short:

```bash
git clone https://github.com/Perchemop/lintel-desktop.git
cd lintel-desktop
cargo build --release -p lintel
cargo test --workspace
```
