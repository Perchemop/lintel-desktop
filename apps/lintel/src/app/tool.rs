//! Tool palette: all drawing and construction tools available in Lintel.

use eframe::egui;

/// Active drawing/editing tool.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Select,
    Wall,
    Door,
    Window,
    Dimension,
    Label,
    Measurements,
    ConstructionLine,
    ConstructionRay,
    ConstructionCircle,
    ConstructionArc,
    Block,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Wall => "Wall",
            Tool::Door => "Door",
            Tool::Window => "Window",
            Tool::Dimension => "Dimension",
            Tool::Label => "Label",
            Tool::Measurements => "Measurements",
            Tool::ConstructionLine => "Construction line",
            Tool::ConstructionRay => "Construction ray",
            Tool::ConstructionCircle => "Construction circle",
            Tool::ConstructionArc => "Construction arc",
            Tool::Block => "Block",
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            Tool::Select => "S",
            Tool::Wall => "W",
            Tool::Door => "D",
            Tool::Window => "N",
            Tool::Dimension => "M",
            Tool::Label => "L",
            Tool::Measurements => "Shift+M",
            Tool::ConstructionLine => "C",
            Tool::ConstructionRay => "Shift+C",
            Tool::ConstructionCircle => "Q",
            Tool::ConstructionArc => "X",
            Tool::Block => "B",
        }
    }

    pub fn role(self) -> crate::components::buttons::ButtonRole {
        match self {
            Tool::Select => crate::components::buttons::ButtonRole::Ghost,
            Tool::Wall => crate::components::buttons::ButtonRole::Primary,
            Tool::Door | Tool::Window | Tool::Dimension | Tool::Label | Tool::Block => {
                crate::components::buttons::ButtonRole::Secondary
            }
            Tool::Measurements => crate::components::buttons::ButtonRole::Secondary,
            Tool::ConstructionLine
            | Tool::ConstructionRay
            | Tool::ConstructionCircle
            | Tool::ConstructionArc => crate::components::buttons::ButtonRole::Ghost,
        }
    }
}