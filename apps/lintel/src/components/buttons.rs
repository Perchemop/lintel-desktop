//! `StyledButton` — the one button component (4 roles × 3 sizes, six states).
//!
//! Patterns copied from `shadcn-egui` Button variants, `modern-egui`
//! `StyledButton` and `egui-pop` component tokens (guide II.30.4, II.30.10).
//! No crate adoption — all three target newer egui than the pinned 0.36.2.
//!
//! Every colour, radius and size comes from `crate::theme` — no raw `Color32`
//! here (guide II.30.6). The states are driven by the live interaction state
//! each frame: default, hover, pressed, focus (2px ring, only while focused),
//! disabled (grey fill, explains why on hover) and loading (rest fill, spinner
//! in place of the label, interaction off — the control size never changes).
//!
//! At most one `Primary` button per view (guide II.30.10).

pub use crate::theme::{ButtonRole, ButtonSize};

use crate::theme::{
    ButtonRoleStyle, BUTTON_CORNER_RADIUS, BUTTON_DANGER_L, BUTTON_DANGER_M, BUTTON_DANGER_S,
    BUTTON_GHOST_L, BUTTON_GHOST_M, BUTTON_GHOST_S, BUTTON_PRIMARY_L, BUTTON_PRIMARY_M,
    BUTTON_PRIMARY_S, BUTTON_SECONDARY_L, BUTTON_SECONDARY_M, BUTTON_SECONDARY_S,
};
use eframe::egui;

/// Builder for a styled button. Colours resolve through the tier-2 roles.
#[derive(Clone, Debug)]
pub struct StyledButton {
    label: String,
    role: ButtonRole,
    size: ButtonSize,
    loading: bool,
    enabled: bool,
    disabled_reason: Option<String>,
    selected: bool,
}

impl StyledButton {
    /// Create a new button with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            role: ButtonRole::Secondary,
            size: ButtonSize::Md,
            loading: false,
            enabled: true,
            disabled_reason: None,
            selected: false,
        }
    }

    /// Set the semantic role (default: [`ButtonRole::Secondary`]).
    pub fn role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }

    /// Set the size (default: [`ButtonSize::Md`]).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Show a loading spinner instead of the label. Interaction is disabled
    /// and the control keeps its size — no layout shift (guide II.30.10).
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Enable or disable the button (default: enabled).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Mark as selected/active (for toggle buttons).
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Explain on hover/focus why the button is disabled (guide II.30.10).
    /// Without a reason, a neutral "Unavailable right now" is shown.
    pub fn disabled_reason(mut self, reason: impl Into<String>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    /// Render the button and return the response.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled(self.enabled && !self.loading, self)
    }

    /// The component token for this role × size combination.
    fn spec(&self) -> &'static ButtonRoleStyle {
        match (self.role, self.size) {
            (ButtonRole::Primary, ButtonSize::Sm) => &BUTTON_PRIMARY_S,
            (ButtonRole::Primary, ButtonSize::Md) => &BUTTON_PRIMARY_M,
            (ButtonRole::Primary, ButtonSize::Lg) => &BUTTON_PRIMARY_L,
            (ButtonRole::Secondary, ButtonSize::Sm) => &BUTTON_SECONDARY_S,
            (ButtonRole::Secondary, ButtonSize::Md) => &BUTTON_SECONDARY_M,
            (ButtonRole::Secondary, ButtonSize::Lg) => &BUTTON_SECONDARY_L,
            (ButtonRole::Ghost, ButtonSize::Sm) => &BUTTON_GHOST_S,
            (ButtonRole::Ghost, ButtonSize::Md) => &BUTTON_GHOST_M,
            (ButtonRole::Ghost, ButtonSize::Lg) => &BUTTON_GHOST_L,
            (ButtonRole::Danger, ButtonSize::Sm) => &BUTTON_DANGER_S,
            (ButtonRole::Danger, ButtonSize::Md) => &BUTTON_DANGER_M,
            (ButtonRole::Danger, ButtonSize::Lg) => &BUTTON_DANGER_L,
        }
    }

    /// Why the button cannot be used right now (hover/focus tooltip).
    fn unavailable_reason(&self) -> String {
        if self.loading {
            "Working…".to_owned()
        } else {
            self.disabled_reason
                .clone()
                .unwrap_or_else(|| "Unavailable right now".to_owned())
        }
    }
}

impl egui::Widget for StyledButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let spec = self.spec();
        let (rect, response) = ui.allocate_exact_size(spec.min_size, egui::Sense::click());
        let enabled = response.enabled();
        let pressed = enabled && response.is_pointer_button_down_on();
        let hovered = enabled && response.hovered();

        // State → token fields (guide II.30.10). Loading keeps the rest fill
        // and swaps the label for a spinner; the size never changes.
        // Selected state shows active fill (guide II.30.10).
        let fill = if self.loading {
            spec.fill
        } else if !enabled {
            spec.disabled
        } else if self.selected {
            spec.active
        } else if pressed {
            spec.active
        } else if hovered {
            spec.hover
        } else {
            spec.fill
        };
        let label_colour = if self.loading {
            spec.text
        } else if !enabled {
            spec.text_disabled
        } else if pressed {
            spec.text_active
        } else {
            spec.text
        };

        ui.painter().rect_filled(rect, BUTTON_CORNER_RADIUS, fill);

        // Focus ring: 2px, only while the widget has keyboard focus
        // (guide II.30.7.4). `spec.focus` is checked ≥3:1 in the theme tests.
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(2.0),
                BUTTON_CORNER_RADIUS,
                egui::Stroke::new(2.0, spec.focus),
                egui::StrokeKind::Outside,
            );
        }

        if self.loading {
            let diameter = (spec.min_size.y * 0.5).min(16.0);
            let spinner =
                egui::Rect::from_center_size(rect.center(), egui::vec2(diameter, diameter));
            egui::Spinner::new()
                .size(diameter)
                .color(spec.text)
                .paint_at(ui, spinner);
        } else {
            let font = egui::TextStyle::Button.resolve(ui.style());
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &self.label,
                font,
                label_colour,
            );
        }

        // The AccessKit name is always the human label — never the spinner —
        // so a screen reader keeps announcing the action while it runs.
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, self.label.clone())
        });

        // Disabled and loading both explain why on hover/focus (II.30.10).
        if enabled {
            response
        } else {
            response.on_disabled_hover_text(self.unavailable_reason())
        }
    }
}
