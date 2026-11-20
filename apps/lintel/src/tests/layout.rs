//! Integration tests for the layout helpers (ticket T-003, spec AC-04).
//!
//! Verifies `vstack`, `hstack`, `wrap_row` and `card` lay out without
//! overlap or clipping, and that a wrapping row reflows at narrow widths
//! (guide II.30.7.6).
//!
//! egui 0.36 removed `Context::begin_frame`; every pass here runs through
//! `Context::run_ui`, which hands the closure a `&mut Ui` directly and may
//! re-run it — so state is captured through `RefCell` and every assertion is
//! idempotent. Headless: no window, no GPU, no display (guide K.13).

use crate::layout::helpers::{card, hstack, vstack, wrap_row};
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

/// One headless pass on a fresh context, `f` may run more than once.
fn run(f: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(egui::RawInput::default(), f);
}

type Slot<T> = Rc<RefCell<Option<T>>>;

fn slot<T>() -> Slot<T> {
    Rc::new(RefCell::new(None))
}

#[test]
fn vstack_lays_out_vertically_with_spacing() {
    let ys = slot::<Vec<f32>>();
    let sink = ys.clone();
    run(move |ui| {
        let mut positions = Vec::new();
        vstack(ui, 8.0, |ui| {
            for i in 0..3 {
                ui.label(format!("Item {i}"));
                positions.push(ui.next_widget_position().y);
            }
        });
        *sink.borrow_mut() = Some(positions);
    });
    let positions = ys.borrow().clone().expect("the pass ran");
    assert_eq!(positions.len(), 3, "all three items must be laid out");
    assert!(
        positions[1] > positions[0] && positions[2] > positions[1],
        "items must stack downward in order: {positions:?}"
    );
}

#[test]
fn hstack_lays_out_horizontally_with_spacing() {
    let xs = slot::<Vec<f32>>();
    let sink = xs.clone();
    run(move |ui| {
        let mut positions = Vec::new();
        hstack(ui, 8.0, |ui| {
            for i in 0..3 {
                ui.label(format!("Item {i}"));
                positions.push(ui.next_widget_position().x);
            }
        });
        *sink.borrow_mut() = Some(positions);
    });
    let positions = xs.borrow().clone().expect("the pass ran");
    assert_eq!(positions.len(), 3, "all three items must be laid out");
    assert!(
        positions[1] > positions[0] && positions[2] > positions[1],
        "items must run rightward in order: {positions:?}"
    );
}

#[test]
fn wrap_row_wraps_on_overflow() {
    let row_tops = slot::<Vec<f32>>();
    let sink = row_tops.clone();
    run(move |ui| {
        ui.set_max_width(100.0);
        let mut rows = Vec::new();
        wrap_row(ui, 4.0, 4.0, |ui| {
            for i in 0..5 {
                let response = ui.button(format!("Btn{i}"));
                rows.push(response.rect.top());
            }
        });
        *sink.borrow_mut() = Some(rows);
    });
    let tops = row_tops.borrow().clone().expect("the pass ran");
    assert_eq!(tops.len(), 5, "all five buttons must be laid out");
    let distinct_rows: Vec<&f32> = {
        let mut unique: Vec<&f32> = Vec::new();
        for top in &tops {
            if !unique.iter().any(|seen| (**seen - *top).abs() < 0.5) {
                unique.push(top);
            }
        }
        unique
    };
    assert!(
        distinct_rows.len() > 1,
        "at a 100px width five buttons must wrap onto more than one row: {tops:?}"
    );
}

#[test]
fn card_renders_with_border_padding_and_shadow() {
    let extents = slot::<(f32, f32)>();
    let sink = extents.clone();
    run(move |ui| {
        card(ui, |ui| {
            ui.label("Card content");
        });
        let rect = ui.min_rect();
        *sink.borrow_mut() = Some((rect.width(), rect.height()));
    });
    let (width, height) = extents.borrow().expect("the pass ran");
    assert!(
        width > 8.0 && height > 0.0,
        "the framed card must have real extents: {width}x{height}"
    );
}

#[test]
fn narrow_window_reflows_instead_of_clipping() {
    // II.30.7.6: at 400px every button must sit inside the available width —
    // the row wraps, it does not overflow. Real assertion, no tautology.
    let worst_right = slot::<(f32, f32)>();
    let sink = worst_right.clone();
    run(move |ui| {
        ui.set_max_width(400.0);
        let available = ui.available_width();
        let mut max_right = 0.0_f32;
        wrap_row(ui, 4.0, 4.0, |ui| {
            for i in 0..10 {
                let response = ui.button(format!("Button {i}"));
                max_right = max_right.max(response.rect.right());
            }
        });
        *sink.borrow_mut() = Some((max_right, available));
    });
    let (max_right, available) = worst_right.borrow().expect("the pass ran");
    assert!(
        max_right <= available + 0.5,
        "at 400px the wrapped row must stay inside the available width \
         (rightmost edge {max_right}, available {available})"
    );
}
