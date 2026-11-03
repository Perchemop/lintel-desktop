# Inspiration & References

This project was inspired by publicly documented architectural drafting applications. The following public sources were consulted to understand typical workflows, file formats, and user interaction patterns. All behavioural summaries are original — no proprietary source text or pixel-accurate UI elements have been copied.

## Public Sources Consulted

1. **egui documentation** — immediate-mode GUI toolkit for Rust native apps
   - URL: <https://github.com/emilk/egui>
   - Key behaviours: command-centric model, `&mut Ui` rendering, no retained mode

2. **eframe API reference** — native Rust application framework
   - URL: <https://github.com/emilk/eframe>
   - Key behaviours: `App::update()`, `CreationContext`, `Frame` management

3. **PDF specification (RFC 3779)** — PDF 1.7 structure
   - URL: <https://www.adobe.com/content/dam/acom/en/publications/pdfs/pdf32000_2008.pdf>
   - Key behaviours: content stream operators (`m`, `l`, `f`, `S`, ``), page tree, catalog

4. **Keep a Changelog** format convention
   - URL: <https://keepachangelog.com/>
   - Key behaviours: `Unchanged / Added / Changed / Fixed / Known Limitations` per release

5. **GitHub Releases API** — update check and crash reporting
   - URL: <https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository>
   - Key behaviours: `GET /repos/:owner/:repo/releases`, version comparison, asset upload

6. **SQLite file format** — persistence reference (though this project uses JSON)
   - URL: <https://www.sqlite.org/fileformat.html>

## Behavioural Summaries (original wording)

- **Wall addition:** User specifies two endpoints (start, end), thickness, and height. The wall is stored as a line segment with axis-aligned bounding box computation available. Joints are resolved by extending each wall end by half the joining wall's thickness.

- **Door placement:** User selects a host wall and an offset along it. The door has a fixed default width (900 mm) and height (2100 mm), with a sill offset. The door leaf swings open perpendicular to the wall direction.

- **Window placement:** Same pattern as door but with default width 1200 mm and height 1200 mm. No swing animation.

- **Undo/redo:** Every command mutates the project state. The document maintains a command history stack (up to 128 entries). Undo reapplies the previous command with a different epoch timestamp; redo re-applies the undone command.

- **PDF rendering:** The plan is placed on a sheet with margins, a title block containing the project name, sheet title, sheet number, scale, and date. Walls are filled rectangles; openings are cleared gaps with revealed jambs.

- **Unit display:** Lengths are shown either as metric (mm, m) or imperial (feet, inches with sixteenth-inch fractions). The unit system is stored in the project file so re-opened files display in the author's units.

- **Project JSON format:** A plain JSON file with `format_version`, `name`, `units`, `levels` (each with `id`, `name`, `elevation`, `walls`, `openings`), `active_level`, `created_epoch_s`, `modified_epoch_s`, `notes`, and `next_id`. Version 1 is the current format; newer files are refused.

- **Command pattern:** Every user edit is a `Command` enum variant. The UI never mutates the model directly — it sends a command and renders the result. This makes workflow tests exercise the same path the app uses.