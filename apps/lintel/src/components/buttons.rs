//! `StyledButton` — the one button component (4 roles × 3 sizes, six states).
//!
//! Patterns copied from `shadcn-egui` Button variants, `modern-egui`
//! `StyledButton` and `egui-pop` component tokens (guide II.30.4, II.30.10).
//! No crate adoption — all three target newer egui than the pinned 0.36.2.
//!
//! Every colour, radius and size comes from `crate::theme` — no raw `Color32`
//! here (guide II.30.6). The widget is hand-painted on egui primitives because
//! `egui::Button::fill` overrides any hover effect; the six visual states
//! (default, hover, active, selected, disabled, loading) resolve through the
//! pure [`ButtonVisualState`] state machine, which is unit-tested headlessly.
//!
//! Interaction details (guide II.30.7, II.30.10):
//! - the button grows to fit its label; `spec.min_size` is a floor, never a
//!   cap, and the 24pt small size honours the target-size minimum;
//! - loading swaps the label for a spinner inside the space the label
//!   reserved — the control size never changes;
//! - disabled and loading buttons sense hover only: they stay hoverable for
//!   the explanation tooltip, are skipped by keyboard navigation (egui only
//!   makes click/drag widgets focusable) and are never faded by egui's
//!   `disable()` — the token table owns the disabled colours;
//! - focus paints a 2px ring outside the button, only while focused;
//! - selection paints the pressed fill plus a 2px inner marker, and reports
//!   toggle semantics to assistive technology.
//!
//! At most one `Primary` button per view (guide II.30.10).

pub use crate::theme::{ButtonRole, ButtonSize};

use crate::theme::{
    BUTTON_CORNER_RADIUS, BUTTON_DANGER_L, BUTTON_DANGER_M, BUTTON_DANGER_S, BUTTON_GHOST_L,
    BUTTON_GHOST_M, BUTTON_GHOST_S, BUTTON_PRIMARY_L, BUTTON_PRIMARY_M, BUTTON_PRIMARY_S,
    BUTTON_SECONDARY_L, BUTTON_SECONDARY_M, BUTTON_SECONDARY_S, ButtonRoleStyle,
};
use eframe::egui;

/// Tooltip shown on a disabled button that was given no explicit reason
/// (guide II.30.10: disabled explains why; pass [`StyledButton::disabled_reason`]
/// for a precondition-specific message).
pub const DISABLED_REASON: &str = "Disabled — precondition not met";

/// The six visual states of a [`StyledButton`].
///
/// Pure and token-driven: resolve the state from the flags, then read the
/// fill/label colour from the role × size component tokens. Headlessly
/// unit-tested in `tests::buttons`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVisualState {
    /// At rest.
    Default,
    /// Pointer hovering the interactive button.
    Hover,
    /// Pointer pressed on the button.
    Active,
    /// Disabled — interaction off, grey token fill, explains why on hover.
    Disabled,
    /// Working — interaction off, rest fill, spinner in the label's space.
    Loading,
    /// Toggled on — pressed fill plus a 2px inner marker.
    Selected,
}

impl ButtonVisualState {
    /// Resolve the state from the widget flags.
    ///
    /// Precedence: [`Disabled`](Self::Disabled) > [`Loading`](Self::Loading) >
    /// [`Selected`](Self::Selected) > [`Active`](Self::Active) >
    /// [`Hover`](Self::Hover) > [`Default`](Self::Default).
    #[must_use]
    pub fn resolve(
        enabled: bool,
        loading: bool,
        selected: bool,
        hovered: bool,
        pointer_down: bool,
    ) -> Self {
        if !enabled {
            Self::Disabled
        } else if loading {
            Self::Loading
        } else if selected {
            Self::Selected
        } else if pointer_down {
            Self::Active
        } else if hovered {
            Self::Hover
        } else {
            Self::Default
        }
    }

    /// Fill colour for this state, from the role × size component tokens.
    /// Loading deliberately keeps the *rest* fill (the theme contract), so the
    /// only moving part is the spinner.
    #[must_use]
    pub fn fill(self, spec: &ButtonRoleStyle) -> egui::Color32 {
        match self {
            Self::Default | Self::Loading => spec.fill,
            Self::Hover => spec.hover,
            Self::Active | Self::Selected => spec.active,
            Self::Disabled => spec.disabled,
        }
    }

    /// Label (and spinner) colour for this state.
    #[must_use]
    pub fn text(self, spec: &ButtonRoleStyle) -> egui::Color32 {
        match self {
            Self::Default | Self::Hover | Self::Loading => spec.text,
            Self::Active | Self::Selected => spec.text_active,
            Self::Disabled => spec.text_disabled,
        }
    }

    /// Whether the state accepts pointer and keyboard interaction.
    #[must_use]
    pub fn interactive(self) -> bool {
        !matches!(self, Self::Disabled | Self::Loading)
    }
}

/// Builder for a styled button. Colours resolve through the tier-2 roles.
#[derive(Clone, Debug)]
pub struct StyledButton {
    label: String,
    role: ButtonRole,
    size: ButtonSize,
    loading: bool,
    enabled: bool,
    selected: bool,
    disabled_reason: Option<String>,
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
            selected: false,
            disabled_reason: None,
        }
    }

    /// Set the semantic role (default: [`ButtonRole::Secondary`]).
    #[must_use]
    pub fn role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }

    /// Set the size (default: [`ButtonSize::Md`]).
    #[must_use]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Show a loading spinner instead of the label. Interaction is disabled
    /// and the control keeps its size — no layout shift (guide II.30.10).
    // Part of the ticket's builder API; no view drives a busy action yet
    // (wired with the later slices), exercised by `tests::buttons` meanwhile.
    #[allow(dead_code)]
    #[must_use]
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Enable or disable the button (default: enabled).
    // Part of the ticket's builder API; views enable/disable buttons as the
    // later slices land, exercised by `tests::buttons` meanwhile.
    #[allow(dead_code)]
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Mark the button as toggled on (default: off). A selected button paints
    /// the pressed fill plus a 2px inner marker (guide II.30.7.2) and reports
    /// toggle state to assistive technology.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Explain on hover why the button is disabled (guide II.30.10). Without a
    /// reason, [`DISABLED_REASON`] is shown.
    // Part of the ticket's builder API; views pass specific preconditions as
    // the later slices land, exercised by `tests::buttons` meanwhile.
    #[allow(dead_code)]
    #[must_use]
    pub fn disabled_reason(mut self, reason: impl Into<String>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    /// Render the button and return the response.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let spec = self.spec();

        // Sizing galley: the label reserves its space even while loading, so
        // the control size never changes (guide II.30.10, no layout shift).
        // `min_size` is a floor, never a cap — long labels grow the button.
        let sizing = ui
            .painter()
            .layout_no_wrap(self.label.clone(), button_font(ui), spec.text);
        let padding = ui.spacing().button_padding;
        let desired = egui::vec2(
            (sizing.size().x + 2.0 * padding.x).max(spec.min_size.x),
            (sizing.size().y + 2.0 * padding.y).max(spec.min_size.y),
        );

        // Interaction: disabled and loading buttons sense hover only. They
        // stay hoverable for the explanation tooltip and are skipped by
        // keyboard navigation; `Sense::hover` also keeps egui's `disable()`
        // fade out of the paint path so the token colours render exactly.
        let interactive = ButtonVisualState::resolve(
            self.enabled && ui.is_enabled(),
            self.loading,
            self.selected,
            false,
            false,
        )
        .interactive();
        let sense = if interactive {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        // AccessKit: role = button, name = the human label — never the
        // spinner — so a screen reader keeps announcing the action while it
        // runs. Selected buttons report toggle semantics.
        response.widget_info(|| {
            if self.selected {
                egui::WidgetInfo::selected(
                    egui::WidgetType::Button,
                    interactive,
                    true,
                    self.label.clone(),
                )
            } else {
                egui::WidgetInfo::labeled(egui::WidgetType::Button, interactive, self.label.clone())
            }
        });

        let state = ButtonVisualState::resolve(
            self.enabled && ui.is_enabled(),
            self.loading,
            self.selected,
            response.hovered(),
            response.is_pointer_button_down_on(),
        );

        ui.painter()
            .rect_filled(rect, BUTTON_CORNER_RADIUS, state.fill(spec));

        // Selection carries a fill AND a marker (guide II.30.7.2): 2px inner
        // outline in the label colour, so it reads on every role's fill.
        if state == ButtonVisualState::Selected {
            ui.painter().rect_stroke(
                rect,
                BUTTON_CORNER_RADIUS,
                egui::Stroke::new(2.0, state.text(spec)),
                egui::StrokeKind::Inside,
            );
        }

        // Focus ring: 2px, only while the widget has keyboard focus
        // (guide II.30.7.4). egui does not paint rings for hand-built widgets.
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(2.0),
                BUTTON_CORNER_RADIUS + 2.0,
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
                .color(state.text(spec))
                .paint_at(ui, spinner);
        } else {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &self.label,
                button_font(ui),
                state.text(spec),
            );
        }

        // Disabled and loading both explain why on hover (guide II.30.10).
        // Gated here rather than via `on_disabled_hover_text`, because that
        // helper only fires for responses egui considers disabled — a
        // hover-only response in an enabled `Ui` does not qualify.
        if !interactive && response.hovered() {
            response.show_tooltip_ui(|tip| {
                tip.set_max_width(ui.spacing().tooltip_width);
                tip.label(self.unavailable_reason());
            });
        }

        response
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
                .unwrap_or_else(|| DISABLED_REASON.to_owned())
        }
    }
}

impl egui::Widget for StyledButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.show(ui)
    }
}

/// The font the button label is laid out in. Safe lookup: falls back to the
/// proportional face if a customised style removed the `Button` text style —
/// `TextStyle::resolve` panics in that case, and the never-crash rule (guide
/// II.26.6) forbids panicking in production paint paths.
fn button_font(ui: &egui::Ui) -> egui::FontId {
    ui.style()
        .text_styles
        .get(&egui::TextStyle::Button)
        .cloned()
        .unwrap_or_else(|| egui::FontId::proportional(14.0))
}

/// Convenience: primary action button — the one action a view exists for.
/// At most one per view (guide II.30.10).
// Not wired into a view yet; the design-system tickets (T-002/T-005) adopt
// these one-call helpers. Exercised by `tests::buttons` meanwhile.
#[allow(dead_code)]
pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Primary).show(ui)
}

/// Convenience: secondary action button.
#[allow(dead_code)]
pub fn secondary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label)
        .role(ButtonRole::Secondary)
        .show(ui)
}

/// Convenience: ghost toolbar button.
#[allow(dead_code)]
pub fn ghost_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Ghost).show(ui)
}

/// Convenience: danger/destructive button.
#[allow(dead_code)]
pub fn danger_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    StyledButton::new(label).role(ButtonRole::Danger).show(ui)
}
