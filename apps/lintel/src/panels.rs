//! Dock panels for the workbench shell (v0.22.0).
//!
//! Every panel is a tab in `egui_dock`. The enum is also used as the
//! `egui_dock::DockState` tab type so panels can be tabbed, reordered and
//! dragged around the workbench.
//!
//! Colours come from `crate::theme` only — no raw `Color32` here
//! (guide II.30.6).

use crate::theme::SPEC;
use eframe::egui;
use egui_dock::{NodeIndex, Tree};

/// Every dockable panel in the workbench.
///
/// `Canvas` is special: it lives in the centre cell of the dock tree and is
/// not tabbable or closeable. All other panels can be tabbed together.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

    /// Stable dock tab id (used for serialization and tab identity).
    pub fn id(self) -> &'static str {
        match self {
            Panel::Inspector => "inspector",
            Panel::Layers => "layers",
            Panel::Constraints => "constraints",
            Panel::Output => "output",
            Panel::Canvas => "canvas",
        }
    }

    /// Whether this panel may be closed by the user.
    pub fn closable(self) -> bool {
        matches!(
            self,
            Panel::Inspector | Panel::Layers | Panel::Constraints | Panel::Output
        )
    }
}

/// Routes each [`Panel`] to its content function inside a `DockArea`.
pub struct TabViewer<'app> {
    pub app: &'app mut crate::LintelApp,
    pub now: i64,
}

impl<'app> egui_dock::TabViewer for TabViewer<'app> {
    type Tab = Panel;

    fn ui(&mut self, ui: &mut egui::Ui, panel: &mut Panel) {
        match panel {
            Panel::Inspector => crate::inspector::inspector_dock(ui, self.app, self.now),
            Panel::Layers => crate::layers::layers_panel(ui, self.app, self.now),
            Panel::Constraints => crate::constraints_panel::constraints_panel(ui, self.app, self.now),
            Panel::Output => crate::output::output_panel(ui, self.app, self.now),
            Panel::Canvas => self.app.canvas_dock(ui, self.now),
        }
    }

    fn title(&mut self, panel: &mut Panel) -> egui::WidgetText {
        egui::WidgetText::from(panel.title().to_string())
    }

    fn closable(&mut self, panel: &mut Panel) -> bool {
        panel.closable()
    }

    /// Menu shown on the tab's `…` button.
    fn context_menu(&mut self, _ui: &mut egui::Ui, _panel: &mut Panel) {}

    fn on_close(&mut self, _panel: &mut Panel) -> bool {
        // Hiding a panel just removes it from the dock tree; state is kept.
        true
    }

    fn id(&mut self, panel: &mut Panel) -> egui_dock::TabId {
        egui_dock::TabId::from(panel.id())
    }
}

/// Build the default dock tree: Inspector+Layers+Constraints tabbed on the
/// right, Output at the bottom, Canvas in the centre.
pub fn default_tree() -> Tree<Panel> {
    let mut tree = Tree::new(vec![egui::vec2(1400.0, 900.0)], vec![Panel::Canvas]);

    // Split right: Inspector | Layers | Constraints (tabbed)
    let right = tree
        .split_right(NodeIndex::root(), 0.75, vec![Panel::Inspector])
        .unwrap();
    let right2 = tree
        .split_right(right, 0.5, vec![Panel::Layers])
        .unwrap();
    tree.split_right(right2, 0.5, vec![Panel::Constraints]).unwrap();

    // Split bottom of the remaining centre cell: Output
    let centre = NodeIndex::root();
    tree.split_below(centre, 0.2, vec![Panel::Output]).unwrap();

    tree
}

/// Serialise the dock state to a JSON string for settings persistence.
pub fn serialize(tree: &Tree<Panel>) -> Option<String> {
    serde_json::to_string(tree).ok()
}

/// Deserialize the dock state from a JSON string, falling back to default.
pub fn deserialize(json: &str) -> Tree<Panel> {
    serde_json::from_str::<Tree<Panel>>(json).unwrap_or_else(|_| default_tree())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_titles_and_ids_are_non_empty() {
        for panel in [
            Panel::Inspector,
            Panel::Layers,
            Panel::Constraints,
            Panel::Output,
            Panel::Canvas,
        ] {
            assert!(!panel.title().is_empty());
            assert!(!panel.id().is_empty());
        }
    }

    #[test]
    fn default_tree_contains_all_panels() {
        let tree = default_tree();
        let mut found: Vec<Panel> = Vec::new();
        tree.each_node(&mut |_, nodes| {
            for tab in nodes.tabs.iter() {
                found.push(*tab);
            }
        });
        assert!(found.contains(&Panel::Inspector));
        assert!(found.contains(&Panel::Layers));
        assert!(found.contains(&Panel::Constraints));
        assert!(found.contains(&Panel::Output));
        assert!(found.contains(&Panel::Canvas));
    }

    #[test]
    fn serialize_deserialize_round_trip() {
        let tree = default_tree();
        let json = serialize(&tree).expect("serialise");
        let restored = deserialize(&json);
        // Round-trip preserves the canvas tab.
        let mut found_canvas = false;
        restored.each_node(&mut |_, nodes| {
            if nodes.tabs.contains(&Panel::Canvas) {
                found_canvas = true;
            }
        });
        assert!(found_canvas);
    }
}
