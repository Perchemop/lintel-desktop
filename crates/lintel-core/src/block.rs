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
    #[must_use]
    pub fn add_entity(mut self, entity: BlockEntity) -> Self {
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

    /// Move the instance to a new position. Fails when the position is not
    /// finite; a rejected move leaves the instance where it was.
    pub fn set_position(&mut self, position: Point2) -> Result<(), CoreError> {
        if !position.is_finite() {
            return Err(CoreError::invalid(
                "a block instance needs a finite position",
            ));
        }
        self.param = position;
        Ok(())
    }

    /// Set the rotation in radians, measured about the block origin. Fails
    /// when the rotation is not finite; a rejected rotation leaves the
    /// instance as it was.
    pub fn set_rotation(&mut self, rotation: f64) -> Result<(), CoreError> {
        if !rotation.is_finite() {
            return Err(CoreError::invalid(
                "a block instance needs a finite rotation",
            ));
        }
        self.rotation = rotation;
        Ok(())
    }

    /// Toggle the mirror across the block's local x-axis.
    pub fn toggle_flip(&mut self) {
        self.flipped = !self.flipped;
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
        let def = BlockDef::new(BlockId(1), "Header");
        let entity = BlockEntity::Wall(Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            200.0,
            2800.0,
        ));
        let returned = def.add_entity(entity);
        assert_eq!(returned.entities.len(), 1);
        assert!(returned.validate().is_ok());
    }

    #[test]
    fn block_def_entities_can_be_mixed_kinds() {
        let def = BlockDef::new(BlockId(2), "Title").add_entity(BlockEntity::Wall(Wall::new(
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
    fn set_position_moves_the_instance_and_refuses_non_finite() {
        let mut instance = BlockInstance::new(BlockId(7), Point2::new(100.0, 200.0));
        instance
            .set_position(Point2::new(300.0, 400.0))
            .expect("move");
        assert_eq!(instance.param, Point2::new(300.0, 400.0));

        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = instance
                .set_position(Point2::new(bad, 0.0))
                .expect_err("not finite");
            assert!(error.to_string().contains("finite position"));
        }
        assert_eq!(
            instance.param,
            Point2::new(300.0, 400.0),
            "a rejected move leaves the instance alone"
        );
    }

    #[test]
    fn set_rotation_sets_the_angle_and_refuses_non_finite() {
        let mut instance = BlockInstance::new(BlockId(7), Point2::ORIGIN);
        instance.set_rotation(0.5).expect("rotate");
        assert_eq!(instance.rotation, 0.5);

        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = instance.set_rotation(bad).expect_err("not finite");
            assert!(error.to_string().contains("finite rotation"));
        }
        assert_eq!(
            instance.rotation, 0.5,
            "a rejected rotation leaves the instance alone"
        );
    }

    #[test]
    fn toggle_flip_toggles_the_mirror() {
        let mut instance = BlockInstance::new(BlockId(7), Point2::ORIGIN);
        assert!(!instance.flipped);
        instance.toggle_flip();
        assert!(instance.flipped);
        instance.toggle_flip();
        assert!(!instance.flipped);
    }

    #[test]
    fn block_def_round_trips_through_json() {
        let def = BlockDef::new(BlockId(3), "Door").add_entity(BlockEntity::Wall(Wall::new(
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
    fn a_rotated_flipped_instance_round_trips_through_json() {
        let mut instance = BlockInstance::new(BlockId(4), Point2::new(100.0, 200.0));
        instance.set_rotation(-1.25).expect("rotate");
        instance.toggle_flip();
        let json = serde_json::to_string(&instance).expect("serialize");
        let back: BlockInstance = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, instance);
    }

    #[test]
    fn block_id_renders_with_its_value() {
        assert_eq!(BlockId(9).to_string(), "block #9");
    }
}
