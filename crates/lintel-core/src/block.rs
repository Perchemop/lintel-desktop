//! Block definitions and block instances.
//!
//! A block definition is a named group of entities stored in local
//! coordinates relative to the block origin. A block instance places one
//! definition on a level at a position, with a rotation and an optional flip.

use serde::{Deserialize, Serialize};

use crate::dimension::Dimension;
use crate::error::CoreError;
use crate::geom::Point2;
use crate::label::PlanLabel;
use crate::opening::Opening;
use crate::wall::Wall;

/// Identity of a block definition or a block instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BlockId(pub u32);

impl BlockId {
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "block #{}", self.0)
    }
}

/// One thing a block definition contains, in block-local coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockEntity {
    Wall(Wall),
    Opening(Opening),
    Dimension(Dimension),
    Label(PlanLabel),
}

/// A named collection of entities that can be instanced on a level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDef {
    pub id: BlockId,
    pub name: String,
    pub entities: Vec<BlockEntity>,
}

impl BlockDef {
    /// A block definition with the given name and no entities yet.
    pub fn new(id: BlockId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            entities: Vec::new(),
        }
    }

    /// Add an entity to the definition, in block-local coordinates.
    pub fn add_entity(&mut self, entity: BlockEntity) -> &mut Self {
        self.entities.push(entity);
        self
    }

    /// A definition needs a non-empty name.
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.name.trim().is_empty() {
            return Err(CoreError::invalid("a block needs a name"));
        }
        Ok(())
    }
}

/// One placement of a block definition on a level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockInstance {
    pub id: BlockId,
    /// Which definition this instance draws from.
    pub block_id: BlockId,
    /// Position of the block origin on the level, in millimetres.
    pub param: Point2,
    /// Rotation in radians, applied about the block origin.
    pub rotation: f64,
    /// Mirror the block across its local x-axis before placing.
    pub flipped: bool,
}

impl BlockInstance {
    /// Place `block_id` at `position` with no rotation and no flip.
    pub fn new(block_id: BlockId, position: Point2) -> Self {
        Self {
            id: BlockId(0),
            block_id,
            param: position,
            rotation: 0.0,
            flipped: false,
        }
    }

    /// True when the position and rotation are finite.
    pub fn is_valid(&self) -> bool {
        self.param.is_finite() && self.rotation.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::WallId;

    #[test]
    fn block_def_new_has_no_entities() {
        let def = BlockDef::new(BlockId(1), "Header");
        assert_eq!(def.id, BlockId(1));
        assert_eq!(def.name, "Header");
        assert!(def.entities.is_empty());
        assert!(def.validate().is_ok());
    }

    #[test]
    fn block_def_with_blank_name_is_invalid() {
        let def = BlockDef::new(BlockId(1), "   ");
        assert!(def.validate().is_err());
    }

    #[test]
    fn add_entity_appends_and_returns_self() {
        let mut def = BlockDef::new(BlockId(1), "Header");
        let entity = BlockEntity::Wall(Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            200.0,
            2800.0,
        ));
        let returned = def.add_entity(entity);
        assert_eq!(def.entities.len(), 1);
        assert_eq!(returned.entities.len(), 1);
        assert!(def.validate().is_ok());
    }

    #[test]
    fn block_def_entities_can_be_mixed_kinds() {
        let mut def = BlockDef::new(BlockId(2), "Title");
        def.add_entity(BlockEntity::Wall(Wall::new(
            WallId(1),
            Point2::ORIGIN,
            Point2::new(500.0, 0.0),
            100.0,
            2500.0,
        )));
        assert_eq!(def.entities.len(), 1);
        assert!(def.validate().is_ok());
    }

    #[test]
    fn block_instance_new_is_valid() {
        let instance = BlockInstance::new(BlockId(7), Point2::new(2500.0, 1200.0));
        assert_eq!(instance.block_id, BlockId(7));
        assert_eq!(instance.param, Point2::new(2500.0, 1200.0));
        assert_eq!(instance.rotation, 0.0);
        assert!(!instance.flipped);
        assert!(instance.is_valid());
    }

    #[test]
    fn block_instance_with_non_finite_position_is_invalid() {
        let mut instance = BlockInstance::new(BlockId(7), Point2::new(f64::NAN, 0.0));
        assert!(!instance.is_valid());
        instance.param = Point2::new(100.0, f64::INFINITY);
        assert!(!instance.is_valid());
    }

    #[test]
    fn block_instance_with_non_finite_rotation_is_invalid() {
        let mut instance = BlockInstance::new(BlockId(7), Point2::ORIGIN);
        instance.rotation = f64::NAN;
        assert!(!instance.is_valid());
    }

    #[test]
    fn block_def_round_trips_through_json() {
        let mut def = BlockDef::new(BlockId(3), "Door");
        def.add_entity(BlockEntity::Wall(Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(900.0, 0.0),
            100.0,
            2100.0,
        )));
        let json = serde_json::to_string(&def).expect("serialize");
        let back: BlockDef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, def);
    }

    #[test]
    fn block_instance_round_trips_through_json() {
        let instance = BlockInstance::new(BlockId(4), Point2::new(100.0, 200.0));
        let json = serde_json::to_string(&instance).expect("serialize");
        let back: BlockInstance = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, instance);
    }

    #[test]
    fn block_id_renders_with_its_value() {
        assert_eq!(BlockId(9).to_string(), "block #9");
    }
}
