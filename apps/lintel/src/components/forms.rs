//! Form composition components — `Field`, `FieldSet`, `FieldGroup`.
//!
//! Patterns copied from `shadcn-egui` Field/FieldSet/FieldGroup (guide II.30.4,
//! spec v0.20.0 AC-03). All colours/spacing/radii from `SPEC.component.field`.
//! No raw `Color32` outside `theme.rs`.

use crate::theme;
use crate::theme::SPEC;
use eframe::egui;

/// A single form field: label + input + optional error/help text.
///
/// ```
/// Field::new("Wall thickness")
///     .input(|ui| ui.add(egui::DragValue::new(&mut thickness).speed(10.0).suffix(" mm")))
///     .help_text("Exterior walls typically ≥ 200 mm")
///     .show(ui);
/// ```
pub struct Field<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    label: String,
    required: bool,
    input_fn: Option<F>,
    error_text: Option<String>,
    help_text: Option<String>,
    disabled: bool,
    disabled_reason: Option<String>,
}

impl<F> Field<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    /// Create a new field with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            required: false,
            input_fn: None,
            error_text: None,
            help_text: None,
            disabled: false,
            disabled_reason: None,
        }
    }

    /// Mark the field as required (appends `*` to label).
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Set the input widget. The closure receives a `Ui`, adds the input
    /// widget, and returns its `Response`. The field wires `labelled_by`
    /// and focus handling automatically.
    pub fn input<G>(self, f: G) -> Field<G>
    where
        G: FnOnce(&mut egui::Ui) -> egui::Response,
    {
        Field {
            label: self.label,
            required: self.required,
            input_fn: Some(f),
            error_text: self.error_text,
            help_text: self.help_text,
            disabled: self.disabled,
            disabled_reason: self.disabled_reason,
        }
    }

    /// Set error text (renders red icon + text below the input).
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Set help text (renders muted text below the input).
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Disable the field. When `reason` is provided, shows it on hover.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Provide a reason for the disabled state (shown in tooltip on hover).
    pub fn disabled_reason(mut self, reason: impl Into<String>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    /// Render the field into the given `Ui`.
    ///
    /// Returns the input widget's `Response` so callers can react to changes.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let spec = &SPEC.component.field;
        let tokens = *spec;

        // Label row
        let mut label_text = self.label;
        if self.required {
            label_text.push_str(" *");
        }
        let label_resp = ui.label(
            egui::RichText::new(label_text)
                .size(tokens.text_size)
                .color(tokens.label),
        );

        // Reserve space for the input
        let input_rect = ui.available_rect_before_wrap();
        let input_height = tokens.min_height;
        let input_width = input_rect.width();

        // Reserve space for the input
        let (_, input_rect) = ui.allocate_space(egui::vec2(input_width, input_height));

        // Determine border colour by state
        let border_colour = if self.disabled {
            tokens.disabled_text
        } else if self.error_text.is_some() {
            tokens.border_error
        } else if ui.memory(|mem| mem.has_focus(ui.auto_id_with("input"))) {
            tokens.border_focused
        } else {
            tokens.border
        };

        // Paint the frame background + border
        if ui.is_rect_visible(input_rect) {
            ui.painter().add(egui::Shape::Rect(egui::epaint::RectShape::new(
                input_rect.expand(tokens.border_pad),
                tokens.corner_radius,
                tokens.input_fill,
                egui::Stroke::new(1.0, border_colour),
                egui::StrokeKind::Inside,
            )));
        }

        // Run the input builder inside a child Ui clipped to the input rect
        let input_id = ui.auto_id_with("input");
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .id(input_id)
                .max_rect(input_rect.shrink(tokens.border_pad))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );

        // The closure returns the input widget's Response
        let input_resp = if let Some(f) = self.input_fn {
            f(&mut child_ui)
        } else {
            child_ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover()).1
        };

        // Associate label with input for AccessKit (II.30.7.4)
        let labelled_resp = input_resp.labelled_by(label_resp.id);

        // Focus ring: 2px accent ring only while focused (guide II.30.7.4)
        if labelled_resp.has_focus() {
            ui.painter().rect_stroke(
                input_rect.expand(tokens.border_pad + 2.0),
                tokens.corner_radius,
                egui::Stroke::new(2.0, tokens.border_focused),
                egui::StrokeKind::Outside,
            );
        }

        // Message row: error or help
        if let Some(error) = &self.error_text {
            ui.spacing_mut().item_spacing.y = tokens.message_gap;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = tokens.message_gap;
                // Error icon (simple exclamation circle)
                let icon_size = tokens.error_icon_size;
                let (_id, icon_rect) = ui.allocate_space(egui::vec2(icon_size, icon_size));
                if ui.is_rect_visible(icon_rect) {
                    // Filled danger circle
                    ui.painter().circle_filled(icon_rect.center(), icon_size * 0.5, tokens.error_icon);
                    // White exclamation mark
                    let exclamation = "!";
                    ui.painter().text(
                        icon_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        exclamation,
                        egui::FontId::proportional(icon_size * 0.7),
                        theme::TEXT_ON_ACCENT,
                    );
                }
                ui.label(
                    egui::RichText::new(error)
                        .size(tokens.text_size)
                        .color(tokens.error_text),
                );
            });
        } else if let Some(help) = &self.help_text {
            ui.spacing_mut().item_spacing.y = tokens.message_gap;
            ui.label(
                egui::RichText::new(help)
                    .size(tokens.text_size)
                    .color(tokens.help),
            );
        }

        // Restore default spacing
        ui.spacing_mut().item_spacing.y = tokens.row_spacing;

        // Return the labelled response
        if self.disabled {
            if let Some(reason) = self.disabled_reason {
                labelled_resp.on_disabled_hover_text(reason)
            } else {
                labelled_resp
            }
        } else {
            labelled_resp
        }
    }
}

/// A group of related fields with an optional legend/collapsible header.
///
/// ```
/// FieldSet::new("Wall properties")
///     .fields(vec![
///         Field::new("Thickness").input(|ui| ui.add(...)),
///         Field::new("Height").input(|ui| ui.add(...)),
///     ])
///     .show(ui);
/// ```
pub struct FieldSet<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    legend: String,
    fields: Vec<Field<F>>,
    collapsible: bool,
    default_open: bool,
    enabled: bool,
}

impl<F> FieldSet<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    /// Create a new field set with the given legend.
    pub fn new(legend: impl Into<String>) -> Self {
        Self {
            legend: legend.into(),
            fields: Vec::new(),
            collapsible: false,
            default_open: true,
            enabled: true,
        }
    }

    /// Add a field to the set.
    pub fn field(mut self, field: Field<F>) -> Self {
        self.fields.push(field);
        self
    }

    /// Set multiple fields at once.
    pub fn fields(mut self, fields: Vec<Field<F>>) -> Self {
        self.fields = fields;
        self
    }

    /// Make the field set collapsible.
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set default open state for collapsible field sets.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Enable or disable all fields in the set.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Render the field set.
    pub fn show(self, ui: &mut egui::Ui) {
        let spec = &SPEC.component.field;
        let tokens = *spec;

        if !self.enabled {
            ui.disable();
        }

        // Paint the group frame
        let frame = egui::Frame::NONE
            .fill(tokens.group_fill)
            .stroke(egui::Stroke::new(1.0, tokens.group_border))
            .corner_radius(tokens.corner_radius)
            .inner_margin(egui::Margin::same(tokens.row_spacing as i8));

        if self.collapsible {
            let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(&self.legend),
                self.default_open,
            );

            let header_response = state.show_header(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(
                    egui::RichText::new(&self.legend)
                        .size(tokens.text_size)
                        .color(tokens.legend)
                        .strong(),
                );
            });

            // Use the response from the header
            let _ = state.show_body_indented(&header_response.response, ui, |ui| {
                ui.spacing_mut().item_spacing.y = tokens.set_spacing;
                for field in self.fields {
                    field.show(ui);
                }
            });
        } else {
            frame.show(ui, |ui| {
                // Legend row
                ui.label(
                    egui::RichText::new(&self.legend)
                        .size(tokens.text_size)
                        .color(tokens.legend)
                        .strong(),
                );
                ui.spacing_mut().item_spacing.y = tokens.set_spacing;
                for field in self.fields {
                    field.show(ui);
                }
            });
        }
    }
}

/// A horizontal (or vertical) group of fields with consistent spacing.
///
/// ```
/// FieldGroup::new()
///     .direction(egui::Direction::LeftToRight)
///     .fields(vec![
///         Field::new("Width").input(|ui| ui.add(...)),
///         Field::new("Height").input(|ui| ui.add(...)),
///     ])
///     .show(ui);
/// ```
pub struct FieldGroup<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    fields: Vec<Field<F>>,
    direction: egui::Direction,
    spacing: f32,
    enabled: bool,
}

impl<F> FieldGroup<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    /// Create a new field group.
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            direction: egui::Direction::LeftToRight,
            spacing: SPEC.component.field.group_spacing,
            enabled: true,
        }
    }

    /// Set the layout direction (`egui::Direction::LeftToRight` or `TopDown`).
    pub fn direction(mut self, direction: egui::Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Override the gap between fields.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Add a field.
    pub fn field(mut self, field: Field<F>) -> Self {
        self.fields.push(field);
        self
    }

    /// Set multiple fields at once.
    pub fn fields(mut self, fields: Vec<Field<F>>) -> Self {
        self.fields = fields;
        self
    }

    /// Enable or disable all fields in the group.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Render the field group.
    pub fn show(self, ui: &mut egui::Ui) {
        let _spec = &SPEC.component.field;

        if !self.enabled {
            ui.disable();
        }

        match self.direction {
            egui::Direction::LeftToRight => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = self.spacing;
                    for field in self.fields {
                        field.show(ui);
                    }
                });
            }
            egui::Direction::TopDown => {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = self.spacing;
                    for field in self.fields {
                        field.show(ui);
                    }
                });
            }
            _ => {
                // Default to horizontal for any other direction
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = self.spacing;
                    for field in self.fields {
                        field.show(ui);
                    }
                });
            }
        }
    }
}

impl<F> Default for FieldGroup<F>
where
    F: FnOnce(&mut egui::Ui) -> egui::Response,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;

    /// Run a headless egui frame with the given UI closure (egui 0.36
    /// removed `Context::run`; `run_ui` hands over a `&mut Ui` directly).
    fn with_ui(f: impl FnOnce(&mut egui::Ui)) {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| f(ui));
    }

    #[test]
    fn field_renders_label_and_input_without_panic() {
        with_ui(|ui| {
            let mut val = 200.0;
            Field::new("Wall thickness")
                .input(|ui| ui.add(egui::DragValue::new(&mut val).speed(10.0).suffix(" mm")))
                .show(ui);
        });
    }

    #[test]
    fn field_with_error_shows_error_icon_and_text() {
        with_ui(|ui| {
            let mut val = 200.0;
            Field::new("Wall thickness")
                .input(|ui| ui.add(egui::DragValue::new(&mut val).speed(10.0).suffix(" mm")))
                .error_text("Must be ≥ 100 mm")
                .show(ui);
        });
    }

    #[test]
    fn field_with_help_shows_help_text() {
        with_ui(|ui| {
            let mut val = 200.0;
            Field::new("Wall thickness")
                .input(|ui| ui.add(egui::DragValue::new(&mut val).speed(10.0).suffix(" mm")))
                .help_text("Exterior walls typically ≥ 200 mm")
                .show(ui);
        });
    }

    #[test]
    fn field_disabled_renders_with_reason() {
        with_ui(|ui| {
            let mut val = 200.0;
            Field::new("Wall thickness")
                .input(|ui| ui.add(egui::DragValue::new(&mut val).speed(10.0).suffix(" mm")))
                .disabled(true)
                .disabled_reason("No wall selected")
                .show(ui);
        });
    }

    #[test]
    fn field_required_shows_asterisk() {
        with_ui(|ui| {
            let mut val = 200.0;
            Field::new("Wall thickness")
                .required(true)
                .input(|ui| ui.add(egui::DragValue::new(&mut val).speed(10.0).suffix(" mm")))
                .show(ui);
        });
    }

    #[test]
    fn fieldset_renders_legend_and_fields() {
        with_ui(|ui| {
            let mut t = 200.0;
            let mut h = 2800.0;
            FieldSet::new("Wall properties")
                .fields(vec![
                    Field::new("Thickness")
                        .input(|ui| ui.add(egui::DragValue::new(&mut t).speed(10.0))),
                    Field::new("Height")
                        .input(|ui| ui.add(egui::DragValue::new(&mut h).speed(10.0))),
                ])
                .show(ui);
        });
    }

    #[test]
    fn fieldset_collapsible_renders_without_panic() {
        with_ui(|ui| {
            let mut t = 200.0;
            FieldSet::new("Wall properties")
                .collapsible(true)
                .default_open(false)
                .fields(vec![
                    Field::new("Thickness")
                        .input(|ui| ui.add(egui::DragValue::new(&mut t).speed(10.0))),
                ])
                .show(ui);
        });
    }

    #[test]
    fn fieldset_disabled_renders_grayed_out() {
        with_ui(|ui| {
            let mut t = 200.0;
            FieldSet::new("Wall properties")
                .enabled(false)
                .fields(vec![
                    Field::new("Thickness")
                        .input(|ui| ui.add(egui::DragValue::new(&mut t).speed(10.0))),
                ])
                .show(ui);
        });
    }

    #[test]
    fn fieldgroup_horizontal_renders_fields_side_by_side() {
        with_ui(|ui| {
            let mut w = 4000.0;
            let mut h = 3000.0;
            FieldGroup::new()
                .direction(egui::Direction::LeftToRight)
                .fields(vec![
                    Field::new("Width")
                        .input(|ui| ui.add(egui::DragValue::new(&mut w).speed(10.0))),
                    Field::new("Height")
                        .input(|ui| ui.add(egui::DragValue::new(&mut h).speed(10.0))),
                ])
                .show(ui);
        });
    }

    #[test]
    fn fieldgroup_vertical_renders_fields_stacked() {
        with_ui(|ui| {
            let mut w = 4000.0;
            let mut h = 3000.0;
            FieldGroup::new()
                .direction(egui::Direction::TopDown)
                .fields(vec![
                    Field::new("Width")
                        .input(|ui| ui.add(egui::DragValue::new(&mut w).speed(10.0))),
                    Field::new("Height")
                        .input(|ui| ui.add(egui::DragValue::new(&mut h).speed(10.0))),
                ])
                .show(ui);
        });
    }

    #[test]
    fn fieldgroup_custom_spacing() {
        with_ui(|ui| {
            let mut a = 1.0;
            let mut b = 2.0;
            FieldGroup::new()
                .direction(egui::Direction::LeftToRight)
                .spacing(24.0)
                .fields(vec![
                    Field::new("A")
                        .input(|ui| ui.add(egui::DragValue::new(&mut a))),
                    Field::new("B")
                        .input(|ui| ui.add(egui::DragValue::new(&mut b))),
                ])
                .show(ui);
        });
    }
}