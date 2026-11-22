# T-003: Flexbox Layout Helpers

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-001 (tokens for spacing/sizes)
**Estimated scope:** Single PR

---

## Objective

Implement layout helpers solving egui single-pass layout limitations: `vstack`, `hstack`, `wrap_row`, `card`.

---

## Acceptance Criteria

- [ ] `layout/helpers.rs` exports:
  - `vstack(ui, spacing, |ui| { ... })` — vertical stack with equal spacing
  - `hstack(ui, spacing, |ui| { ... })` — horizontal stack with equal spacing
  - `wrap_row(ui, spacing, |ui| { ... })` — wraps to next row on overflow
  - `card(ui, |ui| { ... })` — elevated surface with padding, border, radius
- [ ] All spacing/radius/colours from `theme::tokens()` — no raw `Color32`
- [ ] `wrap_row` handles narrow window without clipping (II.30.7.6)
- [ ] `card` uses `SEMANTIC_SURFACE_RAISED`, `SEMANTIC_BORDER_SUBTLE`, `RADIUS_MD`
- [ ] Integration test: resize window 400→1200px → no overlap/clipping

---

## Files to Create/Modify

- `apps/lintel/src/layout/helpers.rs` — new file
- `apps/lintel/src/layout/mod.rs` — export `helpers`
- `apps/lintel/src/tests/layout.rs` — integration tests

---

## Notes

- Copy pattern from `shadcn-egui` `layout::vstack`/`hstack`/`wrap_row`/`card`
- egui single-pass means `wrap_row` must measure first — use `ui.available_width()`
- `card` elevation via `theme::shadows()` (new token)
- Reflow at narrow window: canvas shrinks last (II.30.7.6)