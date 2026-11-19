# T-001: Button Component Tokens + StyledButton Widget

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** None
**Estimated scope:** Single PR, fits Smart Zone (<150k tokens)

---

## Objective

Add 12 button variant tokens (4 roles × 3 sizes) to `theme.rs` and implement `StyledButton` widget in `components/buttons.rs`.

---

## Acceptance Criteria

- [ ] `theme.rs` defines `COMPONENT_BUTTON_*` tokens for:
  - Roles: `primary`, `secondary`, `ghost`, `danger`
  - Sizes: `sm` (24pt min), `md` (32pt), `lg` (40pt)
  - Each variant: default, hover, active, focus, disabled, loading states
- [ ] `components/buttons.rs` exports `StyledButton` with builder API:
  ```rust
  StyledButton::new("Label")
      .role(ButtonRole::Primary)
      .size(ButtonSize::Md)
      .loading(false)
      .show(ui) -> Response
  ```
- [ ] No raw `Color32` in button code — all colours from `theme::tokens()`
- [ ] 24pt minimum target size enforced (AC-02)
- [ ] `egui_kittest` assertions: role=button, accessible name, focus ring visible, size ≥ 24pt
- [ ] Unit test: all 12 variants render without panic

---

## Files to Create/Modify

- `apps/lintel/src/theme.rs` — add `COMPONENT_BUTTON_*` tokens
- `apps/lintel/src/components/buttons.rs` — new file, `StyledButton` implementation
- `apps/lintel/src/components/mod.rs` — export `buttons`
- `apps/lintel/src/tests/buttons.rs` — unit tests (headless)
- `apps/lintel/src/tests/kittest_buttons.rs` — AccessKit assertions

---

## Notes

- Copy pattern from `shadcn-egui` Button variants + `modern-egui` `StyledButton` + `egui-pop` component tokens
- Loading state keeps control size stable (no layout shift)
- Disabled state explains why on hover/focus (II.30.10)
- At most one Primary per view (II.30.10)