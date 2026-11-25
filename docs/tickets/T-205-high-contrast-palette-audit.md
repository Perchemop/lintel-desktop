# T-205 — High-Contrast Palette WCAG AA Audit

**Phase:** 3 (Theme & Accessibility)

## Objective
Audit and fix the high-contrast theme palette to meet WCAG AA contrast requirements (7:1 high-contrast text, 4.5:1 body text, 3:1 UI boundaries).

## Context
- High-contrast palette defined in `apps/lintel/src/theme.rs` as `ThemeSpec::HIGH_CONTRAST`
- Stricter requirements: 7:1 for high-contrast text per WCAG 2.2 §1.4.6
- Reference swatches defined in D-035 as asserted values
- Must work for users with low vision / system high-contrast mode

## Acceptance Criteria
- [ ] Run `cargo test -p lintel theme::contrast::contrast_test` — all assertions pass for high-contrast palette
- [ ] All high-contrast text pairings meet 7:1
- [ ] All body text pairings meet 4.5:1
- [ ] All UI boundary pairings meet 3:1
- [ ] Any intentional exemptions documented in `ACCESSIBILITY.md`
- [ ] Visual evidence generated via kittest shows high-contrast theme correctly

## Files to Modify
- `apps/lintel/src/theme.rs` — adjust high-contrast palette swatches
- `apps/lintel/src/theme.rs` — verify `ThemeSpec::HIGH_CONTRAST` construction
- `docs/qc/buttons-evidence/` — regenerate high-contrast evidence after fixes

## Key Pairings to Check (High-Contrast Specific)
- `TEXT_PRIMARY` on `SURFACE_BASE` ≥ 7:1
- `TEXT_SECONDARY` on `SURFACE_BASE` ≥ 4.5:1
- `ACCENT` on `SURFACE_BASE` ≥ 4.5:1
- `ACCENT_ON` on `ACCENT` ≥ 7:1
- `DANGER` on `SURFACE_BASE` ≥ 4.5:1
- `DANGER_ON` on `DANGER` ≥ 7:1
- `FOCUS_RING` on `SURFACE_BASE` ≥ 3:1
- Button states: all roles meet enhanced thresholds
- Form fields: error text meets 7:1 on base/error surfaces

## Related
- D-035 (contrast methodology)
- T-007 (contrast methodology ticket)
- WCAG 2.2 §1.4.6 (Enhanced Contrast)
- `theme::contrast::contrast_test` / `contrast_check`

## Effort Estimate
Small-Medium (palette tuning, visual verification)