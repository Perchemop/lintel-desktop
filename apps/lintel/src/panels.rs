//! Dock panels for the workbench shell (v0.22.0).
//!
//! Every panel is a tab in the workbench. Panels can be toggled from the
//! activity bar and rendered in side panels or the central area.
//!
//! Colours come from `crate::theme` only — no raw `Color32` here
//! (guide II.30.6).

use eframe::egui;

/// Every panel in the workbench.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Panel {
    Inspector,
    Layers,
    Constraints,
    Output,
    Canvas,
}

/// All panels, in activity bar toggle order.
pub const ALL: [Panel; 5] = [
    Panel::Inspector,
    Panel::Layers,
    Panel::Constraints,
    Panel::Output,
    Panel::Canvas,
];

impl Panel {
    /// Human-readable tab title.
    pub fn title(self) -> &'static str {
        match self {
            Panel::Inspector => "Inspector",
            Panel::Layers => "Layers",
            Panel::Constraints => "Constraints",
            Panel::Output => "Output",
            Panel::Canvas => "Plan",
        }
    }

    /// Stable panel id for serialization.
    pub fn id(self) -> &'static str {
        match self {
            Panel::Inspector => "inspector",
            Panel::Layers => "layers",
            Panel::Constraints => "constraints",
            Panel::Output => "output",
            Panel::Canvas => "canvas",
        }
    }
}

/// Visibility state for each panel (persisted in settings).
#[derive(Debug, Default, Clone)]
pub struct PanelVisibility {
    pub inspector: bool,
    pub layers: bool,
    pub constraints: bool,
    pub output: bool,
    pub canvas: bool,
}

impl PanelVisibility {
    pub fn all_visible() -> Self {
        Self {
            inspector: true,
            layers: true,
            constraints: true,
            output: true,
            canvas: true,
        }
    }

    pub fn is_visible(&self, panel: Panel) -> bool {
        match panel {
            Panel::Inspector => self.inspector,
            Panel::Layers => self.layers,
            Panel::Constraints => self.constraints,
            Panel::Output => self.output,
            Panel::Canvas => self.canvas,
        }
    }

    pub fn set_visible(&mut self, panel: Panel, visible: bool) {
        match panel {
            Panel::Inspector => self.inspector = visible,
            Panel::Layers => self.layers = visible,
            Panel::Constraints => self.constraints = visible,
            Panel::Output => self.output = visible,
            Panel::Canvas => self.canvas = visible,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_titles_are_non_empty() {
        for panel in ALL {
            assert!(!panel.title().is_empty());
            assert!(!panel.id().is_empty());
        }
    }

    #[test]
    fn panel_visibility_defaults_to_all_visible() {
        let vis = PanelVisibility::all_visible();
        for panel in ALL {
            assert!(vis.is_visible(panel));
        }
    }

    #[test]
    fn panel_visibility_toggle() {
        let mut vis = PanelVisibility::all_visible();
        assert!(vis.is_visible(Panel::Inspector));
        vis.set_visible(Panel::Inspector, false);
        assert!(!vis.is_visible(Panel::Inspector));
    }
}
