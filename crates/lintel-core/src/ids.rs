//! Stable identifiers.
//!
//! Identifiers are allocation-ordered integers owned by the project, so a
//! saved file round-trips byte-for-byte and tests can name entities directly.
//! A newtype per kind means a wall id cannot be passed where an opening id is
//! expected.

use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($name:ident, $label:literal) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            Default,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub u64);

        impl $name {
            pub const fn value(self) -> u64 {
                self.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $label, self.0)
            }
        }
    };
}

id_type!(WallId, "wall #{}");
id_type!(OpeningId, "opening #{}");
id_type!(LevelId, "level #{}");
id_type!(DimensionId, "dimension #{}");
id_type!(LabelId, "label #{}");
id_type!(LayerId, "layer #{}");
id_type!(ConstructionId, "construction #{}");

impl LayerId {
    /// The one layer every level always has: `"0"`, the default.
    ///
    /// It is reserved, never allocated from the project counter and cannot be
    /// deleted, so an entity always has a legal layer to fall back to.
    pub const DEFAULT: LayerId = LayerId(0);
}

/// A kind-tagged identity for any element on a level.
///
/// Ids come from a single project-wide counter, so the raw number is unique;
/// carrying the kind lets a caller say exactly what an id refers to and reject
/// a mismatch — the same reason [`crate::layer::EntitySelector`] tags its ids.
/// It is the currency of the selection and grouping layers above the model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EntityId {
    Wall(WallId),
    Opening(OpeningId),
    Dimension(DimensionId),
    Label(LabelId),
    Construction(ConstructionId),
}

impl EntityId {
    /// The raw allocation number, useful for sorting and for logging.
    pub fn value(self) -> u64 {
        match self {
            EntityId::Wall(id) => id.value(),
            EntityId::Opening(id) => id.value(),
            EntityId::Dimension(id) => id.value(),
            EntityId::Label(id) => id.value(),
            EntityId::Construction(id) => id.value(),
        }
    }

    /// Short human-readable name of the kind this id names.
    pub fn label(self) -> &'static str {
        match self {
            EntityId::Wall(_) => "wall",
            EntityId::Opening(_) => "opening",
            EntityId::Dimension(_) => "dimension",
            EntityId::Label(_) => "label",
            EntityId::Construction(_) => "construction",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_render_with_their_kind() {
        assert_eq!(WallId(7).to_string(), "wall #7");
        assert_eq!(OpeningId(3).to_string(), "opening #3");
        assert_eq!(LevelId(1).to_string(), "level #1");
        assert_eq!(DimensionId(5).to_string(), "dimension #5");
        assert_eq!(LabelId(9).to_string(), "label #9");
        assert_eq!(LayerId(2).to_string(), "layer #2");
        assert_eq!(ConstructionId(4).to_string(), "construction #4");
    }

    #[test]
    fn the_default_layer_id_is_zero() {
        assert_eq!(LayerId::DEFAULT, LayerId(0));
        assert_eq!(LayerId::default(), LayerId::DEFAULT);
    }

    #[test]
    fn ids_round_trip_through_json() {
        let json = serde_json::to_string(&WallId(42)).expect("serialize");
        assert_eq!(json, "42");
        let back: WallId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, WallId(42));

        let json = serde_json::to_string(&DimensionId(42)).expect("serialize");
        assert_eq!(json, "42");
        let back: DimensionId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, DimensionId(42));

        let json = serde_json::to_string(&LabelId(42)).expect("serialize");
        assert_eq!(json, "42");
        let back: LabelId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, LabelId(42));

        let json = serde_json::to_string(&LayerId(42)).expect("serialize");
        assert_eq!(json, "42");
        let back: LayerId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, LayerId(42));

        let json = serde_json::to_string(&ConstructionId(42)).expect("serialize");
        assert_eq!(json, "42");
        let back: ConstructionId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, ConstructionId(42));
    }

    #[test]
    fn entity_ids_name_their_kind_and_value() {
        let ids = [
            EntityId::Wall(WallId(1)),
            EntityId::Opening(OpeningId(2)),
            EntityId::Dimension(DimensionId(3)),
            EntityId::Label(LabelId(4)),
            EntityId::Construction(ConstructionId(5)),
        ];
        let labels = ["wall", "opening", "dimension", "label", "construction"];
        for (index, id) in ids.iter().enumerate() {
            assert_eq!(id.value(), index as u64 + 1);
            assert_eq!(id.label(), labels[index]);
        }
    }

    #[test]
    fn entity_ids_round_trip_through_json() {
        for id in [
            EntityId::Wall(WallId(1)),
            EntityId::Opening(OpeningId(2)),
            EntityId::Dimension(DimensionId(3)),
            EntityId::Label(LabelId(4)),
            EntityId::Construction(ConstructionId(5)),
        ] {
            let json = serde_json::to_string(&id).expect("serialize");
            let back: EntityId = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, id);
        }
    }
}
