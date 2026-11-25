# T-204 — Dark Palette WCAG AA Audit

**Phase:** 3 (Theme & Accessibility)

## Objective
Audit and fix the dark theme palette to meet WCAG AA contrast requirements (4.5:1 body text, 3:1 UI boundaries, 7:1 high-contrast text).

## Context
- v0.20.0 established three-tier theme system (BASE → semantic roles → component tokens)
- Dark palette defined in `apps/lintel/src/theme.rs` as `ThemeSpec::DARK`
- Contrast test (`theme::contrast::contrast_test`) asserts all pairings
- Some dark palette pairings may fall below thresholds

## Acceptance Criteria
- [ ] Run `cargo test -p lintel theme::contrast::contrast_test` — all assertions pass for dark palette
- [ ] All semantic role pairings meet 4.5:1 (body text) / 3:1 (UI boundaries)
- [ ] All component token pairings meet 3:1 minimum
- [ ] Any intentional exemptions documented in `ACCESSIBILITY.md`
- [ ] Visual evidence generated via kittest shows dark theme correctly

## Files to Modify
- `apps/lintel/src/theme.rs` — adjust dark palette swatches
- `apps/lintel/src/theme.rs` — verify `ThemeSpec::DARK` construction
- `docs/qc/buttons-evidence/` — regenerate dark evidence after fixes

## Key Pairings to Check
- `TEXT_PRIMARY` on `SURFACE_BASE` (body text)
- `TEXT_SECONDARY` on `SURFACE_BASE` (muted text)
- `TEXT_MUTED` on `SURFACE_BASE` (hints)
- `BORDER_STRONG` on `SURFACE_RAISED` (component boundaries)
- `FOCUS_RING` on `ACCENT` (focus indicator)
- `SELECTION_EDGE` on `ACCENT` (selection marker)
- Button states: primary/secondary/ghost/danger on all surfaces
- Form fields: base fill, text, border, placeholder, error

## Related
- D-035 (contrast methodology)
- T-007 (contrast methodology ticket)
- `theme::contrast::contrast_test` / `contrast_check`

## Effort Estimate
Small-Medium (palette tuning, visual verification)