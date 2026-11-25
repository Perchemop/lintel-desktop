# T-207 — Inspector Empty States for All Sections

**Phase:** 5 (Inspector Polish)

## Objective
Ensure every inspector section displays a helpful empty state with a primary action when it has no data.

## Context
- Inspector sections: Constraints, Viewports, Block Instances, Layers, Title Block, Blocks, Snap Settings
- Some sections already have empty states; others show nothing or a blank panel
- Pattern: description + primary action button (consistent with design system)

## Acceptance Criteria
- [ ] **Constraints section**: "No constraints — add one to pin a dimension to a wall" + [Add Constraint] button
- [ ] **Viewports section**: "No viewports — add one to frame a level on a sheet" + [Add Viewport] button
- [ ] **Block Instances section**: "No block instances — place one from a definition" + [Add Instance] button
- [ ] **Layers section**: "No custom layers — the default layer is always present" (no action needed)
- [ ] **Title Block section**: Shows current values or "Not set — click to configure" + [Edit] button
- [ ] **Blocks (definitions) section**: "No block definitions — create one to reuse groups of entities" + [Add Definition] button
- [ ] **Snap Settings section**: Always shows current settings (never empty)
- [ ] All empty states use consistent styling: muted text, primary button, proper spacing
- [ ] Empty states are accessible (labelled, keyboard reachable)

## Files to Modify
- `apps/lintel/src/inspector.rs` — each section's render function
- `apps/lintel/src/components/forms.rs` — if new form components needed

## Design Pattern
```rust
// Consistent empty state pattern
ui.vertical_centered(|ui| {
    ui.add_space(16.0);
    ui.label(RichText::new("No constraints").color(SPEC.text_muted()));
    ui.add_space(4.0);
    ui.label(RichText::new("Add one to pin a dimension to a wall").color(SPEC.text_muted()).size(SPEC.text_small()));
    ui.add_space(12.0);
    if ui.add(StyledButton::primary("Add Constraint")).clicked() {
        // Open add constraint form
    }
});
```

## Related
- T-002 (form components)
- T-006 (ThemeSpec tiers)
- Inspector sections in `apps/lintel/src/inspector.rs`

## Effort Estimate
Small (UI-only, pattern already established)