//! Layers and per-entity style.
//!
//! A layer groups drawing elements so their visibility, lock state, colour and
//! linework can be changed together. Each [`crate::level::Level`] owns its own
//! layer table; the default layer, [`LayerId::DEFAULT`] (`"0"`), always exists
//! and cannot be deleted, so every entity has a legal fallback.
//!
//! Entities keep the layer association in their own `layer_id` field as well as
//! in their [`EntityStyle`]. The two are kept in step by every constructor and
//! command;  the entity field is what the domain reads, and the copy inside the
//! style travels with the style when an entity is serialised on its own.
//!
//! Lengths are millimetres and colours are plain 8-bit RGB, so nothing here
//! depends on a window, a theme or a renderer.

use serde::{Deserialize, Serialize};

use crate::ids::{DimensionId, LabelId, LayerId, OpeningId, WallId};

/// Name of the layer every level starts with.
pub const DEFAULT_LAYER_NAME: &str = "0";

/// Lineweight a new layer starts with, in millimetres.
pub const DEFAULT_LINEWEIGHT_MM: f64 = 0.25;

/// Thinnest lineweight the editor accepts, in millimetres.
pub const MIN_LINEWEIGHT_MM: f64 = 0.05;

/// Thickest lineweight the editor accepts, in millimetres.
pub const MAX_LINEWEIGHT_MM: f64 = 5.0;

/// Longest layer name, in characters. Long names cannot be told apart in the
/// layer list, so they are rejected instead of silently truncated.
pub const MAX_LAYER_NAME_CHARS: usize = 64;

fn default_true() -> bool {
    true
}

fn default_lineweight_mm() -> f64 {
    DEFAULT_LINEWEIGHT_MM
}

/// An 8-bit-per-channel colour, independent of any UI framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
    pub const WHITE: Rgb = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse `#RRGGBB` or `RRGGBB`. `None` for anything else, so a typed colour
    /// never turns into a silent black.
    pub fn from_hex(text: &str) -> Option<Self> {
        let digits = text.strip_prefix('#').unwrap_or(text);
        if digits.len() != 6 || !digits.is_ascii() {
            return None;
        }
        let r = u8::from_str_radix(&digits[0..2], 16).ok()?;
        let g = u8::from_str_radix(&digits[2..4], 16).ok()?;
        let b = u8::from_str_radix(&digits[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    /// `#RRGGBB`, the form the colour picker and the file both use.
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// How a layer (or an override) draws its lines.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Linetype {
    #[default]
    Continuous,
    Dashed { dash: f64, gap: f64 },
    Hidden { dash: f64, gap: f64 },
    Center { dash: f64, gap: f64 },
}

impl Linetype {
    /// Short human-readable name for menus and the inspector.
    pub fn label(self) -> &'static str {
        match self {
            Linetype::Continuous => "Continuous",
            Linetype::Dashed { .. } => "Dashed",
            Linetype::Hidden { .. } => "Hidden",
            Linetype::Center { .. } => "Center",
        }
    }

    /// The dash/gap pair, in millimetres. `None` for a continuous line.
    pub fn dash_gap(self) -> Option<(f64, f64)> {
        match self {
            Linetype::Continuous => None,
            Linetype::Dashed { dash, gap }
            | Linetype::Hidden { dash, gap }
            | Linetype::Center { dash, gap } => Some((dash, gap)),
        }
    }

    /// Dashes must have a positive length and gaps may not be negative.
    pub fn is_valid(self) -> bool {
        match self.dash_gap() {
            None => true,
            Some((dash, gap)) => dash.is_finite() && dash > 0.0 && gap.is_finite() && gap >= 0.0,
        }
    }

    /// The repeating pattern in millimetres, empty for a continuous line.
    /// The UI and a future vector export can both consume this directly.
    pub fn pattern_mm(self) -> Vec<f64> {
        match self.dash_gap() {
            Some((dash, gap)) if self.is_valid() => vec![dash, gap],
            _ => Vec::new(),
        }
    }
}

/// A named group of drawing elements with shared appearance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    /// `None` means "the viewer's default colour", not black.
    #[serde(default)]
    pub color: Option<Rgb>,
    #[serde(default)]
    pub linetype: Linetype,
    /// Plotted linewidth in millimetres.
    #[serde(default = "default_lineweight_mm")]
    pub lineweight_mm: f64,
    /// Hidden layers are neither drawn nor picked.
    #[serde(default = "default_true")]
    pub visible: bool,
    /// A locked layer is still drawn but cannot be edited.
    #[serde(default)]
    pub locked: bool,
    /// Printable layers reach the PDF; construction helpers do not.
    #[serde(default = "default_true")]
    pub printable: bool,
}

impl Layer {
    /// A new layer with the default appearance.
    pub fn new(id: LayerId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            color: None,
            linetype: Linetype::Continuous,
            lineweight_mm: DEFAULT_LINEWEIGHT_MM,
            visible: true,
            locked: false,
            printable: true,
        }
    }

    /// The reserved layer `"0"` that every level must have.
    pub fn default_layer() -> Self {
        Self::new(LayerId::DEFAULT, DEFAULT_LAYER_NAME)
    }

    pub fn is_default(&self) -> bool {
        self.id == LayerId::DEFAULT
    }

    /// A usable name, a sane lineweight and a drawable linetype.
    pub fn is_valid(&self) -> bool {
        let name = self.name.trim();
        !name.is_empty()
            && name.chars().count() <= MAX_LAYER_NAME_CHARS
            && self.lineweight_mm.is_finite()
            && (MIN_LINEWEIGHT_MM..=MAX_LINEWEIGHT_MM).contains(&self.lineweight_mm)
            && self.linetype.is_valid()
    }

    /// True when the layer should be painted.
    pub fn is_drawable(&self) -> bool {
        self.visible
    }

    /// True when entities on this layer may be edited.
    pub fn accepts_edits(&self) -> bool {
        !self.locked
    }
}

/// One property change, as a value so the undo history can replay it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayerProperty {
    Name(String),
    Color(Option<Rgb>),
    Linetype(Linetype),
    Lineweight(f64),
    Visible(bool),
    Locked(bool),
    Printable(bool),
}

impl LayerProperty {
    /// Short name for a status line after the change.
    pub fn label(&self) -> &'static str {
        match self {
            LayerProperty::Name(_) => "Name",
            LayerProperty::Color(_) => "Colour",
            LayerProperty::Linetype(_) => "Linetype",
            LayerProperty::Lineweight(_) => "Lineweight",
            LayerProperty::Visible(_) => "Visibility",
            LayerProperty::Locked(_) => "Lock",
            LayerProperty::Printable(_) => "Print",
        }
    }
}

/// Per-entity presentation state: the layer it is on plus any overrides of the
/// layer's appearance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityStyle {
    /// Mirrors the owning entity's `layer_id`; the entity field is canonical.
    #[serde(default)]
    pub layer_id: LayerId,
    #[serde(default)]
    pub color_override: Option<Rgb>,
    #[serde(default)]
    pub linetype_override: Option<Linetype>,
    /// Plotted width override in millimetres; `None` inherits the layer's.
    #[serde(default)]
    pub lineweight_override: Option<f64>,
    /// 0.0 = opaque, 1.0 = invisible.
    #[serde(default)]
    pub transparency: f32,
}

impl Default for EntityStyle {
    fn default() -> Self {
        Self {
            layer_id: LayerId::DEFAULT,
            color_override: None,
            linetype_override: None,
            lineweight_override: None,
            transparency: 0.0,
        }
    }
}

impl EntityStyle {
    /// Default appearance, on `layer_id`.
    pub fn on_layer(layer_id: LayerId) -> Self {
        Self {
            layer_id,
            ..Self::default()
        }
    }

    /// Transparency in range and every override sane.
    pub fn is_valid(&self) -> bool {
        self.transparency.is_finite()
            && (0.0..=1.0).contains(&self.transparency)
            && self.linetype_override.is_none_or(Linetype::is_valid)
            && self.lineweight_override.is_none_or(|weight| {
                weight.is_finite() && (MIN_LINEWEIGHT_MM..=MAX_LINEWEIGHT_MM).contains(&weight)
            })
    }
}

/// Which entity an [`EntityStyle`]-level command refers to.
///
/// Ids come from one project-wide counter, so the kind is carried explicitly:
/// the command can then report exactly what was not found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitySelector {
    Wall(WallId),
    Opening(OpeningId),
    Dimension(DimensionId),
    Label(LabelId),
}

impl EntitySelector {
    /// Short human-readable name for status lines and errors.
    pub fn label(self) -> &'static str {
        match self {
            EntitySelector::Wall(_) => "wall",
            EntitySelector::Opening(_) => "opening",
            EntitySelector::Dimension(_) => "dimension",
            EntitySelector::Label(_) => "label",
        }
    }

    /// The raw identity, useful for logging.
    pub fn value(self) -> u64 {
        match self {
            EntitySelector::Wall(id) => id.value(),
            EntitySelector::Opening(id) => id.value(),
            EntitySelector::Dimension(id) => id.value(),
            EntitySelector::Label(id) => id.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layer_is_the_reserved_zero_layer() {
        let layer = Layer::default_layer();
        assert_eq!(layer.id, LayerId::DEFAULT);
        assert_eq!(layer.name, "0");
        assert!(layer.is_default());
        assert!(layer.visible);
        assert!(!layer.locked);
        assert!(layer.printable);
        assert_eq!(layer.linetype, Linetype::Continuous);
        assert_eq!(layer.lineweight_mm, DEFAULT_LINEWEIGHT_MM);
        assert!(layer.is_valid());
    }

    #[test]
    fn a_named_layer_starts_visible_and_unlocked() {
        let layer = Layer::new(LayerId(2), "Walls");
        assert_eq!(layer.id, LayerId(2));
        assert!(!layer.is_default());
        assert!(layer.is_valid());
        assert!(layer.color.is_none());
    }

    #[test]
    fn blank_names_and_absurd_lineweights_are_invalid() {
        let mut layer = Layer::new(LayerId(2), "Walls");
        layer.name = "   ".into();
        assert!(!layer.is_valid());
        layer.name = "Walls".into();
        layer.lineweight_mm = 0.0;
        assert!(!layer.is_valid());
        layer.lineweight_mm = MAX_LINEWEIGHT_MM + 1.0;
        assert!(!layer.is_valid());
        layer.lineweight_mm = f64::NAN;
        assert!(!layer.is_valid());
        layer.lineweight_mm = MIN_LINEWEIGHT_MM;
        assert!(layer.is_valid());
        layer.lineweight_mm = MAX_LINEWEIGHT_MM;
        assert!(layer.is_valid());
    }

    #[test]
    fn linetypes_report_their_dash_pattern() {
        assert_eq!(Linetype::Continuous.dash_gap(), None);
        assert!(Linetype::Continuous.pattern_mm().is_empty());

        let dashed = Linetype::Dashed {
            dash: 6.0,
            gap: 3.0,
        };
        assert_eq!(dashed.dash_gap(), Some((6.0, 3.0)));
        assert_eq!(dashed.pattern_mm(), vec![6.0, 3.0]);
        assert!(dashed.is_valid());
        assert_eq!(dashed.label(), "Dashed");

        let hidden = Linetype::Hidden {
            dash: 3.0,
            gap: 1.5,
        };
        assert_eq!(hidden.label(), "Hidden");
        let center = Linetype::Center {
            dash: 12.0,
            gap: 2.0,
        };
        assert_eq!(center.label(), "Center");
    }

    #[test]
    fn a_linetype_needs_a_positive_dash_and_a_non_negative_gap() {
        let zero_dash = Linetype::Dashed {
            dash: 0.0,
            gap: 1.0,
        };
        assert!(!zero_dash.is_valid());

        let negative_gap = Linetype::Dashed {
            dash: 1.0,
            gap: -1.0,
        };
        assert!(!negative_gap.is_valid());

        let nan_dash = Linetype::Dashed {
            dash: f64::NAN,
            gap: 1.0,
        };
        assert!(!nan_dash.is_valid());

        let zero_gap = Linetype::Dashed {
            dash: 1.0,
            gap: 0.0,
        };
        assert!(zero_gap.is_valid());
    }

    #[test]
    fn every_linetype_round_trips_through_json() {
        for linetype in [
            Linetype::Continuous,
            Linetype::Dashed {
                dash: 6.0,
                gap: 3.0,
            },
            Linetype::Hidden {
                dash: 3.0,
                gap: 1.5,
            },
            Linetype::Center {
                dash: 12.0,
                gap: 2.0,
            },
        ] {
            let json = serde_json::to_string(&linetype).expect("serialize");
            let back: Linetype = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, linetype);
        }
    }

    #[test]
    fn colours_parse_and_format_as_hex() {
        assert_eq!(Rgb::from_hex("#FF8000"), Some(Rgb::new(255, 128, 0)));
        assert_eq!(Rgb::from_hex("00FF00"), Some(Rgb::new(0, 255, 0)));
        assert_eq!(Rgb::new(1, 2, 3).to_hex(), "#010203");
        assert_eq!(Rgb::from_hex("#FFF"), None);
        assert_eq!(Rgb::from_hex("#GGGGGG"), None);
        assert_eq!(Rgb::from_hex(""), None);
        assert_eq!(Rgb::BLACK.to_hex(), "#000000");
        assert_eq!(Rgb::WHITE.to_hex(), "#FFFFFF");
    }

    #[test]
    fn colours_round_trip_through_json() {
        let original = Rgb::new(12, 34, 56);
        let json = serde_json::to_string(&original).expect("serialize");
        assert_eq!(json, r#"{"r":12,"g":34,"b":56}"#);
        let back: Rgb = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn a_layer_round_trips_through_json() {
        let mut original = Layer::new(LayerId(3), "Walls");
        original.color = Some(Rgb::new(20, 40, 60));
        original.linetype = Linetype::Hidden {
            dash: 4.0,
            gap: 2.0,
        };
        original.lineweight_mm = 0.5;
        original.visible = false;
        original.locked = true;
        original.printable = false;

        let json = serde_json::to_string(&original).expect("serialize");
        let back: Layer = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn a_layer_saved_before_the_new_fields_still_loads() {
        let json = r#"{ "id": 2, "name": "Walls" }"#;
        let layer: Layer = serde_json::from_str(json).expect("old layer");
        assert_eq!(layer.id, LayerId(2));
        assert_eq!(layer.name, "Walls");
        assert!(layer.color.is_none());
        assert_eq!(layer.linetype, Linetype::Continuous);
        assert_eq!(layer.lineweight_mm, DEFAULT_LINEWEIGHT_MM);
        assert!(layer.visible);
        assert!(!layer.locked);
        assert!(layer.printable);
    }

    #[test]
    fn an_entity_style_defaults_to_the_default_layer() {
        let style = EntityStyle::default();
        assert_eq!(style.layer_id, LayerId::DEFAULT);
        assert!(style.color_override.is_none());
        assert!(style.linetype_override.is_none());
        assert!(style.lineweight_override.is_none());
        assert_eq!(style.transparency, 0.0);
        assert!(style.is_valid());
        assert_eq!(EntityStyle::on_layer(LayerId(4)).layer_id, LayerId(4));
    }

    #[test]
    fn an_entity_style_validates_its_overrides() {
        let mut style = EntityStyle::default();
        style.transparency = 1.0;
        assert!(style.is_valid());
        style.transparency = 1.5;
        assert!(!style.is_valid());
        style.transparency = f32::NAN;
        assert!(!style.is_valid());

        style.transparency = 0.0;
        style.lineweight_override = Some(0.0);
        assert!(!style.is_valid());
        style.lineweight_override = Some(0.3);
        assert!(style.is_valid());
        style.linetype_override = Some(Linetype::Dashed {
            dash: 0.0,
            gap: 1.0,
        });
        assert!(!style.is_valid());
    }

    #[test]
    fn an_entity_style_round_trips_through_json() {
        let mut original = EntityStyle::on_layer(LayerId(5));
        original.color_override = Some(Rgb::new(1, 2, 3));
        original.linetype_override = Some(Linetype::Center {
            dash: 10.0,
            gap: 2.0,
        });
        original.lineweight_override = Some(0.35);
        original.transparency = 0.25;

        let json = serde_json::to_string(&original).expect("serialize");
        let back: EntityStyle = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn layer_properties_round_trip_through_json() {
        let properties = [
            LayerProperty::Name("Walls".into()),
            LayerProperty::Color(Some(Rgb::new(1, 2, 3))),
            LayerProperty::Linetype(Linetype::Continuous),
            LayerProperty::Lineweight(0.5),
            LayerProperty::Visible(false),
            LayerProperty::Locked(true),
            LayerProperty::Printable(false),
        ];
        for property in properties {
            let json = serde_json::to_string(&property).expect("serialize");
            let back: LayerProperty = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, property);
            assert!(!property.label().is_empty());
        }
    }

    #[test]
    fn entity_selectors_round_trip_and_name_their_kind() {
        let selectors = [
            EntitySelector::Wall(WallId(1)),
            EntitySelector::Opening(OpeningId(2)),
            EntitySelector::Dimension(DimensionId(3)),
            EntitySelector::Label(LabelId(4)),
        ];
        for selector in selectors {
            let json = serde_json::to_string(&selector).expect("serialize");
            let back: EntitySelector = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, selector);
            assert!(!selector.label().is_empty());
            assert!(selector.value() > 0);
        }
    }
}
