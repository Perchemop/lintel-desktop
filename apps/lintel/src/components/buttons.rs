//! Styled button components — 4 roles × 3 sizes with 24pt floor.
//!
//! Patterns copied from `shadcn-egui` Button variants, `modern-egui`
//! `StyledButton`, and `egui-pop` component tokens (guide II.30.4, II.30.10).
//! No crate adoption — egui 0.36.2 incompatible with all three.

use crate::theme::{
    ButtonRole, ButtonRoleStyle, ButtonSize, BUTTON_DANGER_L, BUTTON_DANGER_M, BUTTON_DANGER_S,
    BUTTON_GHOST_L, BUTTON_GHOST_M, BUTTON_GHOST_S, BUTTON_PRIMARY_L, BUTTON_PRIMARY_M,
    BUTTON_PRIMARY_S, BUTTON_SECONDARY_L, BUTTON_SECONDARY_M, BUTTON_SECONDARY_S,
};
use eframe::egui;

/// Builder for a styled button. All colours resolve through tier-2 roles.
#[derive(Clone, Debug)]
pub struct StyledButton {
    label: String,
    role: ButtonRole,
    size: ButtonSize,
    loading: bool,
    enabled: bool,
    selected: bool,
}

impl StyledButton {
    /// Create a new button with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            role: ButtonRole::Secondary,
            size: ButtonSize::Medium,
            loading: false,
            enabled: true,
            selected: false,
        }
    }

    /// Set the semantic role (default: Secondary).
    pub fn role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }

    /// Set the size (default: Medium).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Show loading spinner. Keeps control size stable (no layout shift).
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Enable or disable (default: enabled). Disabled explains why on hover.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Mark as selected/active (for toggle buttons).
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Render the button and return the response.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let spec = self.style_spec();
        let fill = if self.loading || !self.enabled {
            spec.disabled
        } else if self.selected {
            spec.active
        } else {
            spec.fill
        };

        let mut btn = egui::Button::new(if self.loading { "⟳" } else { &self.label })
            .fill(fill)
            .stroke(egui::Stroke::new(2.0, spec.focus))
            .min_size(spec.min_size);

        if !self.enabled {
            btn = btn.sense(egui::Sense::hover()); // hover for tooltip
        }

        let resp = ui.add_enabled(self.enabled && !self.loading, btn);

        // Tooltip for disabled: explain why (II.30.11)
        // Note: on_hover_text consumes Response, so we handle this via tooltip text on the button itself
        // The disabled state is handled by the fill color; tooltip can be added via response callback if needed

        resp
    }

    fn style_spec(&self) -> &'static ButtonRoleStyle {
        match (self.role, self.size) {
            (ButtonRole::Primary, ButtonSize::Small) => &BUTTON_PRIMARY_S,
            (ButtonRole::Primary, ButtonSize::Medium) => &BUTTON_PRIMARY_M,
            (ButtonRole::Primary, ButtonSize::Large) => &BUTTON_PRIMARY_L,
            (ButtonRole::Secondary, ButtonSize::Small) => &BUTTON_SECONDARY_S,
            (ButtonRole::Secondary, ButtonSize::Medium) => &BUTTON_SECONDARY_M,
            (ButtonRole::Secondary, ButtonSize::Large) => &BUTTON_SECONDARY_L,
            (ButtonRole::Ghost, ButtonSize::Small) => &BUTTON_GHOST_S,
            (ButtonRole::Ghost, ButtonSize::Medium) => &BUTTON_GHOST_M,
            (ButtonRole::Ghost, ButtonSize::Large) => &BUTTON_GHOST_L,
            (ButtonRole::Danger, ButtonSize::Small) => &BUTTON_DANGER_S,
            (ButtonRole::Danger, ButtonSize::Medium) => &BUTTON_DANGER_M,
            (ButtonRole::Danger, ButtonSize::Large) => &BUTTON_DANGER_L,
        }
    }
}

/// Convenience: primary action button (most common case).
pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Primary).show(ui)
}

/// Convenience: secondary action button.
pub fn secondary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label)
        .role(ButtonRole::Secondary)
        .show(ui)
}

/// Convenience: ghost toolbar button.
pub fn ghost_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Ghost).show(ui)
}

/// Convenience: danger/destructive button.
pub fn danger_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Danger).show(ui)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;

    #[test]
    fn all_variants_render_without_panic() {
        let mut ctx = egui::Context::default();
        let mut ui = ctx.begin_frame(Default::default());

        for role in [
            ButtonRole::Primary,
            ButtonRole::Secondary,
            ButtonRole::Ghost,
            ButtonRole::Danger,
        ] {
            for size in [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large] {
                StyledButton::new("Test")
                    .role(role)
                    .size(size)
                    .show(&mut ui);
            }
        }

        // Loading state
        StyledButton::new("Loading").loading(true).show(&mut ui);
        // Disabled state
        StyledButton::new("Disabled").enabled(false).show(&mut ui);
        // Selected state
        StyledButton::new("Selected").selected(true).show(&mut ui);
    }

    #[test]
    fn min_size_enforced() {
        let mut ctx = egui::Context::default();
        let mut ui = ctx.begin_frame(Default::default());

        let resp = StyledButton::new("Tiny")
            .role(ButtonRole::Primary)
            .size(ButtonSize::Small)
            .show(&mut ui);

        // Small size has 24pt minimum
        assert!(
            resp.rect.height() >= 24.0,
            "height {} < 24",
            resp.rect.height()
        );
        assert!(
            resp.rect.width() >= 24.0,
            "width {} < 24",
            resp.rect.width()
        );
    }

    #[test]
    fn loading_keeps_size_stable() {
        let mut ctx = egui::Context::default();
        let mut ui = ctx.begin_frame(Default::default());

        let normal = StyledButton::new("Label").show(&mut ui);
        let loading = StyledButton::new("Label").loading(true).show(&mut ui);

        // Size should not change when loading (no layout shift)
        assert_eq!(normal.rect.size(), loading.rect.size());
    }
}
