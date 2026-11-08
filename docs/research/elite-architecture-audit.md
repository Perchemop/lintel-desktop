# Elite Architecture Audit — Lintel

Findings are derived from a comprehensive read of the Lintel workspace at ~/lintel (crates/lintel-core/src, apps/lintel/src). Dates are relative to the current checkout (2026-11-08). No code was modified.

## 1. Module structure

**crates/lintel-core/src** (≈ 7 168 LOC, 20 modules)

| file | LOC | role |
|---|---|---|
| command.rs | 1 269 | Command enum + apply_command; 35+ variants; 1 269 LOC is a god-module for edits |
| pdf.rs | 1 477 | Hand-written PDF 1.7 emitter; single large file for all export logic |
| room.rs | 553 | Planar face traversal for rooms; no caching |
| join.rs | 463 | ResolvedWall/PlanGeometry generation; recomputed per paint |
| project.rs | 378 | Project struct, validate, counts |
| measurements.rs | 360 | LevelTotals / ProjectTotals |
| geom.rs | 316 | Point2/Vec2/Bounds; the only geometry primitive |
| … | … | wall, opening, dimension, level, label, schedule, snap, units, document, error, ids, persistence |

*Module sizes* from `wc -l` in the exploration show a concentration in command.rs and pdf.rs.

**apps/lintel/src** (≈ 2 615 LOC)

| file | LOC | role |
|---|---|---|
| main.rs | 1 222 | App state, event loop, shortcuts, canvas orchestration |
| inspector.rs | 654 | Property editor, many `.expect` on active_level / wall id |
| canvas.rs | 398 | egui painting, hit testing, draft handling |
| release_notes.rs | 142 | Suite-standard release notes popup |
| about.rs | 142 | About dialog |
| dimension_ui.rs | 66 | Dimension widget helpers |
| theme.rs | 49 | Colour palette |

**Coupling / seams**

* Good: `lintel-core` is headless, tested without window/GPU (lib.rs:1-10).
* Problem: `Command` is both *definition* and *application* (command.rs:143-297). Validation lives inline with mutation; there is no separate validator or transaction log.
* Problem: `PlanGeometry::of_level` (join.rs:105-123) clones dimensions/labels, resolves walls, detects rooms, runs `detect_rooms` every paint.
* Problem: `room::detect_rooms` (room.rs:77-97) builds canonical nodes by iterating walls × walls (room.rs:114-133) and splits walls by scanning nodes (room.rs:203-223). Complexity is roughly O(n²) in wall count. No memoisation; moving one wall recomputes all rooms.
* God-module risk: command.rs is 1 269 LOC with 35+ variants and a match with >30 arms. Adding new edits requires editing the same file and grows it further.
* Tight coupling to Vec lookups: Level::wall(id) etc. use `iter().find` (level.rs:45-47). All hit tests in `PlanGeometry::hit_test` (join.rs:157-... ) loop linearly over walls/openings each frame (main.rs:378, 460, etc. use `active_level().expect`).

## 2. Command layer

* Commands are an enum `Command` (command.rs:26-131). Applied via `apply_command(project, command)` (command.rs:143-297).
* Validation occurs before mutation but inside the same match arms. The pattern is “clone candidate, validate, swap” (document.rs:44-58). This is good for atomicity.
* No command registry, no metadata (name, shortcut, description). The UI maps tools to commands ad-hoc in main.rs. There is no central palette source, so a command palette/CLI would need to duplicate the enum.
* Missing commands: no batch transaction, no rename wall, no copy/paste, no duplicate level, no undo/redo grouping. `Applied` only returns IDs, no change description.
* Undo/redo is snapshot-based via `Document` (document.rs:16-23). `MAX_UNDO = 128` (document.rs:14). Each step clones entire `Project` (≈ kilobytes). For large drawings the memory cost and clone cost grows linearly with project size.
* No command log for replay / macro recording.
* Tests show command validation is solid (command.rs:679-1269), but `apply_command` is `pub(crate)`, not exposed for external scripting.

## 3. Persistence

* Format: pretty-printed JSON, suffix `lintel.json` (persistence.rs:16), `format_version = 1` (project.rs:17). `Document::save` uses `save_atomic` (document.rs:109-117).
* Atomic write: write to sibling `.tmp` then rename (persistence.rs:44-56). No fsync, no crash-recovery journal.
* Validation on load: `from_json` parses then `project.validate()` (persistence.rs:24-27). The validator checks duplicate IDs, impossible dimensions, dangling hosts, overlapping openings (project.rs:101-217). Good.
* Schema versioning: `UnsupportedVersion` error if file version > supported (project.rs:103-106, persistence test). No migration path; older → newer is not handled.
* Autosave: none. `Document::dirty` flag exists (document.rs:22, 57, 68) but the app never auto-saves on timer or on `dirty` change.
* Backup: none. No rotating backups, no file locking. Concurrent edits could corrupt.
* File locking: not implemented. `save_atomic` writes without advisory lock.
* Crash recovery: not implemented. In-flight edits are lost if the process exits before save. Undo stack lives only in memory.

## 4. Performance hot spots

* **Room detection** — `room::detect_rooms` (room.rs:77-97). Steps:
  - `canonical_nodes` does O(w²) endpoint checks (room.rs:113-124) and O(w²) proper crossings (room.rs:127-133).
  - `split_walls` scans all nodes per wall (room.rs:204-210).
  - `trace_faces` walks half-edges linearly but the node construction dominates.
  For 10k walls this is several hundred million distance checks per frame. `PlanGeometry::of_level` calls `detect_rooms` every paint (join.rs:121).

* **PDF export** — hand-written PDF (pdf.rs:1-1477). `render_plan` builds the entire PDF in memory as `Vec<u8>` (pdf.rs:156-184). `render_set` iterates over all levels, builds content for each, so memory scales with page count × drawing complexity. No streaming to file.

* **Hit testing** — `PlanGeometry::hit_test` (join.rs:157-... ) loops over labels, openings, walls, dimensions each call. Called per mouse move in canvas. Linear in entity count.

* **Geometry recomputation** — `PlanGeometry::of_level` recomputes resolved walls and joins for every frame (join.rs:105-123). `resolve_wall` (join.rs:190-210) scans all walls to find joins: O(w²).

* **Level lookups** — `Project::level`/`level_mut` use `iter().find` (project.rs:75-81). `Level::wall` etc. use `iter().find` (level.rs:45-... ). No hash maps. For large drawings, repeated lookups become O(n) each.

* **Document clone on edit** — `Document::apply` clones whole project (document.rs:45). For 10k walls, clone cost dominates.

Potential O(n²) in UI rendering: canvas paints every ResolvedWall and opening each frame. No spatial index.

## 5. Error handling

* Core errors are typed in `CoreError` (error.rs:7-30). Messages are plain language (error.rs:32-36).
* Commands return `Result<Applied, CoreError>`; rejected edits leave project unchanged (document.rs:45-58, tests).
* `unwrap`/`expect` in production code:
  - `apps/lintel/src/main.rs` uses `active_level().expect("level")` repeatedly (lines 378, 460, 581, 616, 623, 643, 667, 797, 818, 839, 858, 1021). These are unsafe if a level is deleted while UI holds a reference.
  - `inspector.rs` mirrors the same pattern (lines 40-41, 53-54, 68-69, 82-83, 259, 351, 370, 451-452).
  - `crates/update-check/src/lib.rs` uses `expect` in tests only? line 346-359 are tests.
  - No panic on user input; but UI code can panic on invariant violation.

* User-facing messages: rejected commands surface via `CoreError` string (main.rs status note). No structured error codes for i18n.

## 6. Testing

* Unit tests exist in every core module (e.g. command.rs:679-1269, document.rs:136-225, persistence.rs:74-253, project.rs:220-378).
* Integration tests: `tests/workflows.rs`, `tests/dimensions.rs`, `tests/properties.rs`.
* Property-based tests via `proptest` (Cargo.toml). Fuzz tests for command layer exist per ROADMAP v0.10.1 CAP-085.
* Missing: benchmarks (`cargo bench`) – none found. No performance regression tests for large drawings.
* Missing: E2E UI tests (egui automation). No visual regression tests.
* Missing: fuzzing for JSON parser (serde is robust but no corpus).
* Test coverage by module is uneven: `pdf.rs` has extensive tests (lines 1120-1477) but `join.rs` and `room.rs` rely on unit tests only.

## 7. Accessibility

Current state per ACCESSIBILITY.md and code:
* Keyboard shortcuts exist (main.rs:716-762). Canvas is focusable (ACCESSIBILITY.md:27-31).
* egui AccessKit integration is relied upon for screen-reader semantics (ACCESSIBILITY.md:24-26).
* Focus rings use COBALT accent, WCAG AA contrast claimed.
* Limitations documented: canvas objects have no per-object labels (ACCESSIBILITY.md:62-67), no high-contrast theme toggle.

Gaps for elite paid tool:
* No ARIA-equivalent for individual walls/openings/dimensions.
* No keyboard navigation *within* the canvas (tab between objects).
* No voice control or custom shortcuts.
* No accessibility audit in CI.

## 8. Localization / i18n

* No i18n framework. All user-visible strings are hard-coded in Rust (`format!`, literals). Example: error messages in `CoreError` (error.rs:8-21), UI labels in `main.rs` and `inspector.rs`.
* Units are supported (Metric/Imperial) but numbers are formatted with `format!` – no locale-aware decimal separators.
* Adding i18n would require extracting all strings, adding a localisation crate (e.g. `fluent`), and externalising messages. Current error messages are plain English.

## 9. Plugin / extensibility

* No extension points. `Command` is the only entry point, `pub(crate)`.
* No scripting, no plugin API, no import/export beyond PDF/JSON.
* `lintel-core` is headless, which is a good foundation, but it exports no public API for third-party extensions. Adding a scripting system would need:
  - Public command builder API
  - Stable plugin ABI / WASM sandbox
  - Event bus for model changes

## 10. Crash reporting / telemetry

* `crates/crash-report` provides panic hook, scrubbed diagnostics, manual report building (lib.rs:1-17). Uses `ureq` for optional submission (Cargo.toml).
* No opt-in telemetry, no performance metrics, no usage analytics.
* Update check exists (`crates/update-check`) with GitHub Releases API, fail-soft offline.
* No in-app crash reporter UI beyond pre-filled issue URL.

## 11. Packaging / distribution

* Releases via GitHub Actions. Artefacts: AppImage (Linux), `.app` bundle (macOS), Inno Setup installer (Windows) per ROADMAP.
* Auto-update: update-check crate checks GitHub Releases but no automatic download/install; user must open release notes and click Update.
* Code signing not evident in repo. `resources/windows/lintel.iss` exists.
* Portable mode not implemented; paths are OS-specific.
* No installer wizard for first-run settings, no telemetry opt-in at install.

## 12. Security

* Input surface: JSON project files parsed with serde_json (persistence.rs:24). No size limit, no streaming parser. Maliciously large JSON can cause OOM.
* PDF export writes raw bytes; no validation of user-supplied text for PDF injection (e.g., `(`, `)`, backslash) – `escape_text` exists (pdf.rs:953) but not proven complete.
* File parsing: `from_json` validates structure via `project.validate()` (persistence.rs:24-27). Good.
* Resource limits: none. Room detection, PDF rendering, and document clone can all be forced into CPU/memory exhaustion with large drawings.
* No sandboxing for plugins (none exist).
* Secrets scanning is in scripts (`scripts/scan-secrets.sh`) but runtime protection is minimal.

---

## Prioritised architectural changes

### P0 – Must for elite paid feel
1. **Replace snapshot undo with command log + reversible commands**  
   Remove `Document::undo` cloning whole project (document.rs:61-70). Store a reversible command stack. Reduces memory and clone cost for large drawings. Prerequisites for stable autosave and crash recovery.
2. **Spatial index and incremental geometry**  
   Add R-tree / grid index for walls/openings. Cache `PlanGeometry` and invalidate on touched entities only. Eliminates O(n²) room detection and per-frame hit testing on 10k+ entities. Requires refactor of `join.rs` `of_level` and `room::detect_rooms`.
3. **File locking, autosave, and backup**  
   Implement advisory file lock on open, periodic autosave to `.lintel.json.autosave`, keep last N backups. Add `save_atomic` fsync. Prevent data loss – a paid tool cannot lose work.
4. **Schema migration pipeline**  
   Versioned JSON with explicit migrations (e.g., `migrate::v1_to_v2`). Current `UnsupportedVersion` (project.rs:103) only rejects newer files. Needed for long-term file compatibility.
5. **Error handling hardening in UI**  
   Remove `expect` on `active_level` in `apps/lintel/src/main.rs` and `inspector.rs`. Use `Result`/`Option` propagation and show user-friendly error toasts instead of panic. Non-negotiable for production.

### P1 – Should
6. **Command registry and palette**  
   Extract `Command` metadata (name, id, shortcut, description) into a registry. Drive command palette, CLI, and future scripting. Decouples UI from core.
7. **Streaming PDF export**  
   Stream PDF to file instead of `Vec<u8>` in memory. Add pagination streaming for multi-page sets. Prevents OOM on huge drawings.
8. **Performance tests and benchmarks**  
   Add `criterion` benchmarks for room detection, hit testing, PDF render with 1k/10k/100k entities. Gate CI on regression.
9. **i18n foundation**  
   Centralise strings via `fluent` or similar; extract all UI and error messages. Prepare for paid international market.
10. **Telemetry opt-in**  
    Add opt-in, privacy-preserving metrics (crash-free rate, feature use). Use existing `crash-report` and `update-check` crates as foundation.

### P2 – Nice
11. **Plugin API**  
    Expose safe, versioned core API for extensions (exporters, custom tools). WASM sandbox for scripting.
12. **High-contrast / theme accessibility**  
    Add system high-contrast theme toggle, per-object canvas labels for screen readers, keyboard navigation inside canvas.
13. **Resource limits and DoS protection**  
    Cap JSON parse size, limit entity counts, add progress callbacks for long operations.
14. **Code signing and auto-update installer**  
    Sign binaries, implement in-app silent update with rollback.

**Prerequisites for elite paid tool feel:** P0 items 1-5. Without stable undo/autosave, performant large-drawings handling, and safe file I/O, the app will feel brittle compared to paid CAD tools.

Report generated by subagent exploration of ~/lintel.
