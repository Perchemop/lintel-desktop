# T-005: Shell Restructure — Regions Pattern

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-003 (layout helpers for rail/columns)
**Estimated scope:** Single PR

---

## Objective

Restructure `main.rs` app shell to return `Regions { title_bar, rail, canvas }` from `Shell::regions(&ctx)` following `spaceui` pattern.

---

## Acceptance Criteria

- [ ] `app/shell.rs` new module with:
  ```rust
  pub struct Regions {
      pub title_bar: Response,
      pub rail: Response,
      pub canvas: Response,
  }
  pub fn regions(ctx: &egui::Context) -> Regions { ... }
  ```
- [ ] Three zones visually distinct: title bar (top), rail (left), canvas (center)
- [ ] Rail uses `vstack` from T-003 for tool groups
- [ ] Canvas receives remaining space, handles scroll/zoom
- [ ] Title bar: app name, window controls, breadcrumb
- [ ] All colours/spacing from `theme::tokens()` — no raw `Color32`
- [ ] Narrow window: rail collapses before canvas shrinks (II.30.7.6)
- [ ] Integration test: window 800×600 → three zones rendered

---

## Files to Create/Modify

- `apps/lintel/src/app/shell.rs` — new file
- `apps/lintel/src/app/mod.rs` — export `shell`
- `apps/lintel/src/main.rs` — use `Shell::regions()` instead of inline layout
- `apps/lintel/src/tests/shell.rs` — integration tests

---

## Notes

- Copy pattern from `spaceui` `Theme::install()` returning `Regions`
- Rail = tool palette (draw, modify, snap, layers)
- Canvas = drawing area + rulers + status
- Title bar = custom (not OS native) for cross-platform consistency
- Reduced motion: animations off by default (II.30.7.7)