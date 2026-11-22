# T-002: Field / FieldSet / FieldGroup Form Components

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-001 (button tokens for submit buttons)
**Estimated scope:** Single PR

---

## Objective

Implement form composition components following `shadcn-egui` pattern: `Field` (label + input + error/help), `FieldSet` (group of fields with legend), `FieldGroup` (inline fields).

---

## Acceptance Criteria

- [ ] `components/forms.rs` exports:
  - `Field::new(label).input(widget).error_text("msg").help_text("hint").show(ui)`
  - `FieldSet::new(legend).fields(vec![...]).show(ui)`
  - `FieldGroup::new().fields(vec![...]).direction(Direction::Horizontal).show(ui)`
- [ ] All colours/spacing from `theme::tokens()` — no raw `Color32`
- [ ] Error state: red border + error text + icon (II.30.11)
- [ ] Disabled state: explains why on hover (II.30.11)
- [ ] Keyboard: Tab order follows visual order; Enter submits form
- [ ] Unit tests: render field with error/help/disabled states

---

## Files to Create/Modify

- `apps/lintel/src/components/forms.rs` — new file
- `apps/lintel/src/components/mod.rs` — export `forms`
- `apps/lintel/src/tests/forms.rs` — unit tests

---

## Notes

- Copy pattern from `shadcn-egui` Field/FieldSet/FieldGroup
- Label associated via `labelled_by` for AccessKit (II.30.7.4)
- Error never colour-only — text + icon required (II.30.7.2)