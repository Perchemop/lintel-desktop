# T-208 — Inspector Tab Order Audit

**Phase:** 5 (Inspector Polish)

## Objective
Ensure the inspector panel has a logical, trap-free Tab order that follows reading order (top-to-bottom, left-to-right).

## Context
- Inspector is a scrollable panel with multiple sections
- Each section has multiple interactive controls (buttons, fields, checkboxes)
- Current Tab order may jump between sections or trap focus
- Must work with keyboard-only navigation

## Acceptance Criteria
- [ ] Tab enters inspector at first focusable control (top of first visible section)
- [ ] Tab traverses all controls in reading order within a section
- [ ] Tab moves to next section's first control after last control of current section
- [ ] Tab wraps from last control to first control (no trap)
- [ ] Shift+Tab reverses order correctly
- [ ] Focus is visible on all controls (COBALT focus ring per T-001)
- [ ] No focus trap in any subsection (forms, dropdowns, lists)

## Files to Modify
- `apps/lintel/src/inspector.rs` — control order in each section
- `apps/lintel/src/components/forms.rs` — Field/FieldSet Tab behavior
- `apps/lintel/src/theme.rs` — verify `FOCUS_RING` token used

## Audit Checklist
| Section | Controls (in order) | Notes |
|---------|---------------------|-------|
| Project | Name, Units, Rename/Save buttons | Top level |
| Storeys | Level list (select), Add/Delete/Rename | List keyboard nav |
| Title Block | Company, Project, Sheet, Revision, Date, Drawn By, Sheet Number | Form fields |
| Viewports | List (select), Add/Remove, Level/Scale/Caption | Form in add form |
| Layers | List (select), Visible/Locked toggles, Add/Delete | List + toggles |
| Constraints | List (select), Remove, Add form (kind/dim/wall/param/degrees) | Complex form |
| Blocks (defs) | List (select), Add/Delete | Simple list |
| Block Instances | List (select), X/Y/Rot/Flip fields, Remove | Form fields |
| Snap Settings | Grid/Ortho toggles, Grid size, Tolerance | Checkboxes + fields |

## Related
- T-001 (button tokens + focus ring)
- T-002 (form components)
- T-006 (ThemeSpec tiers)
- D-032 (command bindings in inspector)

## Effort Estimate
Small (ordering fixes, verification)