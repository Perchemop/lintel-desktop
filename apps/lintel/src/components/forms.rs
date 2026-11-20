//! Form composition components — [`Field`], [`FieldSet`], [`FieldGroup`].
//!
//! shadcn-egui Field/FieldSet/FieldGroup pattern, copied not adopted (guide
//! II.30.4 — egui 0.36.2 is incompatible with the source crates). Ticket
//! T-002, spec v0.20.0 AC-03.
//!
//! Accessibility (guide II.30.7):
//! - the label is associated with the input through
//!   [`egui::Response::labelled_by`], so a screen reader announces the field
//!   name with the input;
//! - an error is never colour-only: a painted icon, a 2px danger stroke on
//!   the input and the error text itself carry the state (II.30.7.2);
//! - a disabled field explains why on hover (II.30.11);
//! - Tab order follows visual order because every row is a plain vertical
//!   (or horizontal) stack of focusable widgets.
//!
//! Every colour, spacing and radius resolves through `crate::theme` — no raw
//! `Color32` outside the theme module (guide II.30.6).

// The component API ships ahead of the Inspector slice that wires it into the
// live forms; until then the unit tests in `src/tests/forms.rs` and the
// evidence generators in `src/tests/evidence.rs` are the only callers, so
// silence dead-code for the module as a whole (same pattern as icons.rs).
#![allow(dead_code)]

use crate::theme;
use eframe::egui;

/// One labelled input with optional error and help slots.
///
/// The `input` closure paints the actual editor (a [`egui::TextEdit`], a
/// [`egui::DragValue`], …) and returns its response, so the field can wrap
/// any widget without a trait hierarchy.
pub struct Field<'a> {
    label: String,
    input: Option<Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>>,
    error_text: Option<String>,
    help_text: Option<String>,
    disabled_reason: Option<String>,
}

impl<'a> Field<'a> {
    /// Create a field with a label; attach the editor with [`Field::input`].
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            input: None,
            error_text: None,
            help_text: None,
            disabled_reason: None,
        }
    }

    /// Attach the input painter — any widget-returning closure (a
    /// [`egui::TextEdit`], a [`egui::DragValue`], …).
    pub fn input(
        mut self,
        input: impl FnOnce(&mut egui::Ui) -> egui::Response + 'a,
    ) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    /// Attach an error message: danger stroke on the input, painted icon and
    /// the message itself below (never colour alone, guide II.30.7.2).
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    /// Attach a help hint below the input.
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Explain why the field is disabled (hover tooltip, guide II.30.11).
    pub fn disabled_reason(mut self, reason: impl Into<String>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    /// Render label, input, error and help slots in reading order.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let Field {
            label,
            input,
            error_text,
            help_text,
            disabled_reason,
        } = self;
        let has_error = error_text.is_some();

        // The label gets its own AccessKit node; the input points at it via
        // `labelled_by` so screen readers announce "label: value".
        let label_colour = if has_error {
            theme::TEXT_DANGER
        } else {
            theme::TEXT_PRIMARY
        };
        let label_response = ui.add(
            egui::Label::new(egui::RichText::new(&label).color(label_colour)),
        );

        // A field without an input is a display-only row; it still shows its
        // error/help slots so callers never need a second widget type.
        let response = match input {
            Some(input) => input(ui).labelled_by(label_response.id),
            None => label_response,
        };

        let mut response = if let Some(reason) = disabled_reason {
            response.on_disabled_hover_text(reason)
        } else {
            response
        };

        if let Some(error) = error_text {
            if let Some(input_rect) = input_rect {
                // A 2px danger stroke marks the input itself; the stroke
                // alone is not the signal — the icon and the text repeat it
                // (II.30.7.2).
                ui.painter().rect_stroke(
                    input_rect,
                    theme::RADIUS_XS,
                    egui::Stroke::new(2.0, theme::DANGER_DEFAULT),
                    egui::StrokeKind::Inside,
                );
            }
            let row = ui.horizontal(|ui| {
                paint_error_icon(ui);
                ui.label(egui::RichText::new(error).color(theme::TEXT_DANGER));
            });
            response = response.union(row.response);
        }

        if let Some(help) = help_text {
            let row = ui.label(egui::RichText::new(help).small().color(theme::TEXT_HINT));
            response = response.union(row);
        }

        response
    }
}

/// Painted error marker: a filled disc with a bang, in the danger and
/// on-accent roles. Drawn with painter primitives (no emoji, no icon crate),
/// and always paired with the error text so colour is never the only signal.
fn paint_error_icon(ui: &mut egui::Ui) {
    let (_, rect) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
    let center = rect.center();
    ui.painter()
        .circle_filled(center, 6.0, theme::DANGER_DEFAULT);
    ui.painter().text(
        center,
        egui::Align2::CENTER_CENTER,
        "!",
        egui::FontId::proportional(9.0),
        theme::TEXT_ON_ACCENT,
    );
}

/// A legend plus a group of fields, framed as one visual unit.
///
/// ```ignore
/// FieldSet::new("Storey")
///     .fields(vec![name_field, elevation_field])
///     .show(ui);
/// ```
pub struct FieldSet<'a> {
    legend: String,
    fields: Vec<Field<'a>>,
}

impl<'a> FieldSet<'a> {
    /// Create a field set with a legend.
    pub fn new(legend: impl Into<String>) -> Self {
        Self {
            legend: legend.into(),
            fields: Vec::new(),
        }
    }

    /// Replace the fields of this set.
    pub fn fields(mut self, fields: Vec<Field<'a>>) -> Self {
        self.fields = fields;
        self
    }

    /// Render the framed legend + fields.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let FieldSet { legend, fields } = self;
        let frame = egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
            .corner_radius(theme::RADIUS_MD)
            .inner_margin(theme::SPACING_MD);
        frame
            .show(ui, |ui| {
                ui.label(egui::RichText::new(legend).strong().color(theme::TEXT_STRONG));
                ui.separator();
                for field in fields {
                    field.show(ui);
                }
            })
            .response
    }
}

/// Inline arrangement of fields — left-to-right or top-down.
pub struct FieldGroup<'a> {
    fields: Vec<Field<'a>>,
    direction: egui::Direction,
}

impl<'a> FieldGroup<'a> {
    /// Create an empty group (defaults to left-to-right).
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            direction: egui::Direction::LeftToRight,
        }
    }

    /// Replace the fields of this group.
    pub fn fields(mut self, fields: Vec<Field<'a>>) -> Self {
        self.fields = fields;
        self
    }

    /// Set the layout direction (`LeftToRight` = horizontal).
    pub fn direction(mut self, direction: egui::Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Render the group.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let FieldGroup { fields, direction } = self;
        match direction {
            egui::Direction::LeftToRight | egui::Direction::RightToLeft => {
                crate::layout::helpers::hstack(ui, theme::SPACING_MD, |ui| {
                    for field in fields {
                        field.show(ui);
                    }
                })
                .response
            }
            egui::Direction::TopDown | egui::Direction::BottomUp => {
                crate::layout::helpers::vstack(ui, theme::SPACING_SM, |ui| {
                    for field in fields {
                        field.show(ui);
                    }
                })
                .response
            }
        }
    }
}

impl<'a> Default for FieldGroup<'a> {
    fn default() -> Self {
        Self::new()
    }
}
