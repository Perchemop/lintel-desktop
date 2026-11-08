//! A storey: a named elevation holding its own walls, openings and dimensions.
//!
//! v0.1.0 edits one storey at a time, but the file format already holds a
//! list, so multi-storey work is a UI milestone rather than a format change.

use serde::{Deserialize, Serialize};

use crate::construction::ConstructionEntity;
use crate::dimension::Dimension;
use crate::geom::Bounds;
use crate::ids::{ConstructionId, DimensionId, LabelId, LayerId, LevelId, OpeningId, WallId};
use crate::label::PlanLabel;
use crate::layer::{EntitySelector, EntityStyle, Layer};
use crate::opening::Opening;
use crate::wall::Wall;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub id: LevelId,
    pub name: String,
    /// Floor level in millimetres above the project datum.
    pub elevation: f64,
    /// Layer table for this storey. Always contains
    /// [`LayerId::DEFAULT`](crate::ids::LayerId::DEFAULT).
    #[serde(default)]
    pub layers: Vec<Layer>,
    pub walls: Vec<Wall>,
    pub openings: Vec<Opening>,
    /// Annotation drawn over the plan. `serde(default)` keeps files written
    /// before dimensions existed readable.
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
    /// Free text annotations drawn over the plan.
    #[serde(default)]
    pub labels: Vec<PlanLabel>,
    /// Non-printing guides used for snapping and alignment.
    #[serde(default)]
    pub construction: Vec<ConstructionEntity>,
}

impl Level {
    pub fn new(id: LevelId, name: impl Into<String>, elevation: f64) -> Self {
        Self {
            id,
            name: name.into(),
            elevation,
            layers: vec![Layer::default_layer()],
            walls: Vec::new(),
            openings: Vec::new(),
            dimensions: Vec::new(),
            labels: Vec::new(),
            construction: Vec::new(),
        }
    }

    pub fn wall(&self, id: WallId) -> Option<&Wall> {
        self.walls.iter().find(|w| w.id == id)
    }

    pub fn wall_mut(&mut self, id: WallId) -> Option<&mut Wall> {
        self.walls.iter_mut().find(|w| w.id == id)
    }

    pub fn opening(&self, id: OpeningId) -> Option<&Opening> {
        self.openings.iter().find(|o| o.id == id)
    }

    pub fn opening_mut(&mut self, id: OpeningId) -> Option<&mut Opening> {
        self.openings.iter_mut().find(|o| o.id == id)
    }

    pub fn dimension(&self, id: DimensionId) -> Option<&Dimension> {
        self.dimensions.iter().find(|d| d.id == id)
    }

    pub fn dimension_mut(&mut self, id: DimensionId) -> Option<&mut Dimension> {
        self.dimensions.iter_mut().find(|d| d.id == id)
    }

    pub fn label(&self, id: LabelId) -> Option<&PlanLabel> {
        self.labels.iter().find(|l| l.id == id)
    }

    pub fn label_mut(&mut self, id: LabelId) -> Option<&mut PlanLabel> {
        self.labels.iter_mut().find(|l| l.id == id)
    }

    pub fn layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|layer| layer.id == id)
    }

    pub fn layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|layer| layer.id == id)
    }

    /// Layer by name, ignoring case. Names are unique within a level.
    pub fn layer_by_name(&self, name: &str) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|layer| layer.name.eq_ignore_ascii_case(name.trim()))
    }

    /// The layer an entity sits on, if both the entity and its layer exist.
    pub fn layer_of(&self, entity: EntitySelector) -> Option<&Layer> {
        self.layer(self.entity_layer(entity)?)
    }

    /// The layer id recorded on an entity, whatever its kind.
    pub fn entity_layer(&self, entity: EntitySelector) -> Option<LayerId> {
        match entity {
            EntitySelector::Wall(id) => self.wall(id).map(|wall| wall.layer_id),
            EntitySelector::Opening(id) => self.opening(id).map(|opening| opening.layer_id),
            EntitySelector::Dimension(id) => self.dimension(id).map(|dimension| dimension.layer_id),
            EntitySelector::Label(id) => self.label(id).map(|label| label.layer_id),
        }
    }

    /// The style of an entity, whatever its kind.
    pub fn entity_style(&self, entity: EntitySelector) -> Option<&EntityStyle> {
        match entity {
            EntitySelector::Wall(id) => self.wall(id).map(|wall| &wall.style),
            EntitySelector::Opening(id) => self.opening(id).map(|opening| &opening.style),
            EntitySelector::Dimension(id) => self.dimension(id).map(|dimension| &dimension.style),
            EntitySelector::Label(id) => self.label(id).map(|label| &label.style),
        }
    }

    /// Move an entity onto `layer_id`, keeping its style mirror in step.
    /// Returns false when the entity does not exist.
    pub fn set_entity_layer(&mut self, entity: EntitySelector, layer_id: LayerId) -> bool {
        let (layer, style_layer) = match entity {
            EntitySelector::Wall(id) => match self.wall_mut(id) {
                Some(wall) => (&mut wall.layer_id, &mut wall.style.layer_id),
                None => return false,
            },
            EntitySelector::Opening(id) => match self.opening_mut(id) {
                Some(opening) => (&mut opening.layer_id, &mut opening.style.layer_id),
                None => return false,
            },
            EntitySelector::Dimension(id) => match self.dimension_mut(id) {
                Some(dimension) => (&mut dimension.layer_id, &mut dimension.style.layer_id),
                None => return false,
            },
            EntitySelector::Label(id) => match self.label_mut(id) {
                Some(label) => (&mut label.layer_id, &mut label.style.layer_id),
                None => return false,
            },
        };
        *layer = layer_id;
        *style_layer = layer_id;
        true
    }

    /// How many walls, openings, dimensions and labels sit on a layer.
    pub fn entity_count_on_layer(&self, layer_id: LayerId) -> usize {
        self.walls.iter().filter(|w| w.layer_id == layer_id).count()
            + self
                .openings
                .iter()
                .filter(|o| o.layer_id == layer_id)
                .count()
            + self
                .dimensions
                .iter()
                .filter(|d| d.layer_id == layer_id)
                .count()
            + self.labels.iter().filter(|l| l.layer_id == layer_id).count()
    }

    pub fn construction_entity(&self, id: ConstructionId) -> Option<&ConstructionEntity> {
        self.construction.iter().find(|entity| entity.id == id)
    }

    pub fn construction_mut(&mut self, id: ConstructionId) -> Option<&mut ConstructionEntity> {
        self.construction.iter_mut().find(|entity| entity.id == id)
    }

    /// Guides that are currently painted.
    pub fn visible_construction(&self) -> impl Iterator<Item = &ConstructionEntity> {
        self.construction.iter().filter(|entity| entity.visible)
    }

    pub fn construction_count(&self) -> usize {
        self.construction.len()
    }

    pub fn openings_on(&self, wall: WallId) -> impl Iterator<Item = &Opening> {
        self.openings.iter().filter(move |o| o.host == wall)
    }

    pub fn wall_count(&self) -> usize {
        self.walls.len()
    }

    pub fn opening_count(&self) -> usize {
        self.openings.len()
    }

    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }

    /// Bounds of every wall band on this level, for zoom-to-fit and the sheet
    /// layout. `None` when the level is empty.
    pub fn extent(&self) -> Option<Bounds> {
        let mut bounds: Option<Bounds> = None;
        for wall in &self.walls {
            if let Some(corners) = wall.footprint() {
                if let Some(wall_bounds) = Bounds::from_points(&corners) {
                    match &mut bounds {
                        Some(existing) => existing.include(wall_bounds),
                        None => bounds = Some(wall_bounds),
                    }
                }
            }
        }
        bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::construction::ConstructionKind;
    use crate::dimension::{Dimension, DimensionKind};
    use crate::geom::Point2;

    #[test]
    fn empty_level_has_no_extent() {
        let level = Level::new(LevelId(1), "Level 0", 0.0);
        assert!(level.extent().is_none());
        assert_eq!(level.wall_count(), 0);
        assert_eq!(level.dimension_count(), 0);
    }

    #[test]
    fn extent_covers_the_wall_bands_not_just_centrelines() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(2000.0, 0.0),
            200.0,
            2800.0,
        ));
        let bounds = level.extent().expect("extent");
        assert_eq!(bounds.width(), 2000.0);
        assert_eq!(bounds.height(), 200.0);
        assert_eq!(bounds.min.y, -100.0);
        assert_eq!(bounds.max.y, 100.0);
    }

    #[test]
    fn openings_are_found_by_host() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.openings.push(Opening::new(
            OpeningId(1),
            crate::opening::OpeningKind::Door,
            WallId(1),
            2000.0,
        ));
        level.openings.push(Opening::new(
            OpeningId(2),
            crate::opening::OpeningKind::Window,
            WallId(2),
            2000.0,
        ));
        let on_first: Vec<_> = level.openings_on(WallId(1)).collect();
        assert_eq!(on_first.len(), 1);
        assert_eq!(on_first[0].id, OpeningId(1));
    }

    #[test]
    fn dimensions_are_found_by_id() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.dimensions.push(Dimension::new(
            DimensionId(7),
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        assert_eq!(level.dimension_count(), 1);
        let dimension = level.dimension(DimensionId(7)).expect("dimension");
        assert_eq!(dimension.kind, DimensionKind::LinearHorizontal);
        level
            .dimension_mut(DimensionId(7))
            .expect("dimension")
            .offset = 700.0;
        assert_eq!(
            level.dimension(DimensionId(7)).expect("dimension").offset,
            700.0
        );
        assert!(level.dimension(DimensionId(8)).is_none());
    }

    #[test]
    fn levels_saved_before_dimensions_still_load() {
        let json = r#"{
            "id": 1,
            "name": "Level 0",
            "elevation": 0.0,
            "walls": [],
            "openings": []
        }"#;
        let level: Level = serde_json::from_str(json).expect("old level");
        assert!(level.dimensions.is_empty());
        assert!(level.labels.is_empty());
        assert!(level.layers.is_empty(), "the loader migrates the default layer");
        assert!(level.construction.is_empty());
    }

    #[test]
    fn labels_are_found_by_id() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.labels.push(PlanLabel::new(
            LabelId(7),
            Point2::new(1000.0, 500.0),
            "Living room",
        ));
        let label = level.label(LabelId(7)).expect("label");
        assert_eq!(label.text, "Living room");
        level.label_mut(LabelId(7)).expect("label").text = "Kitchen".into();
        assert_eq!(level.label(LabelId(7)).expect("label").text, "Kitchen");
        assert!(level.label(LabelId(8)).is_none());
    }

    #[test]
    fn labels_survive_a_save_and_load() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.labels.push(PlanLabel::new(
            LabelId(3),
            Point2::new(1200.0, 800.0),
            "Bedroom",
        ));
        let json = serde_json::to_string(&level).expect("serialize");
        let back: Level = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.labels, level.labels);
        assert_eq!(back, level);
    }

    #[test]
    fn a_new_level_has_the_default_layer() {
        let level = Level::new(LevelId(1), "Level 0", 0.0);
        assert_eq!(level.layers.len(), 1);
        let layer = level.layer(LayerId::DEFAULT).expect("default layer");
        assert_eq!(layer.name, "0");
        assert!(layer.is_default());
    }

    #[test]
    fn layers_are_found_by_id_and_name() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.layers.push(Layer::new(LayerId(2), "Walls"));
        assert!(level.layer(LayerId(2)).is_some());
        assert_eq!(level.layer_by_name("walls").map(|l| l.id), Some(LayerId(2)));
        assert!(level.layer_by_name("missing").is_none());
        level.layer_mut(LayerId(2)).expect("layer").visible = false;
        assert!(!level.layer(LayerId(2)).expect("layer").visible);
    }

    #[test]
    fn an_entity_can_move_layer_and_both_copies_follow() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        let wall_id = WallId(2);
        level.walls.push(Wall::new(
            wall_id,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            200.0,
            2800.0,
        ));
        let selector = EntitySelector::Wall(wall_id);
        assert_eq!(level.entity_layer(selector), Some(LayerId::DEFAULT));
        assert!(level.set_entity_layer(selector, LayerId(5)));
        assert_eq!(level.entity_layer(selector), Some(LayerId(5)));
        assert_eq!(
            level.entity_style(selector).map(|style| style.layer_id),
            Some(LayerId(5))
        );
        assert!(!level.set_entity_layer(EntitySelector::Wall(WallId(99)), LayerId(5)));
    }

    #[test]
    fn entity_counts_are_per_layer() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.layers.push(Layer::new(LayerId(2), "Walls"));
        level.walls.push(
            Wall::new(
                WallId(3),
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                200.0,
                2800.0,
            )
            .on_layer(LayerId(2)),
        );
        level.labels.push(
            PlanLabel::new(LabelId(4), Point2::new(100.0, 100.0), "Hall").on_layer(LayerId(2)),
        );
        assert_eq!(level.entity_count_on_layer(LayerId(2)), 2);
        assert_eq!(level.entity_count_on_layer(LayerId::DEFAULT), 0);
    }

    #[test]
    fn construction_guides_are_found_and_filtered_by_visibility() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.construction.push(ConstructionEntity::new(
            ConstructionId(2),
            ConstructionKind::Line {
                start: Point2::new(0.0, 0.0),
                end: Point2::new(1000.0, 0.0),
            },
        ));
        level.construction.push(ConstructionEntity::new(
            ConstructionId(3),
            ConstructionKind::Circle {
                center: Point2::new(500.0, 500.0),
                radius: 100.0,
            },
        ));
        level
            .construction_mut(ConstructionId(3))
            .expect("guide")
            .visible = false;

        assert_eq!(level.construction_count(), 2);
        assert!(level.construction_entity(ConstructionId(2)).is_some());
        assert_eq!(level.visible_construction().count(), 1);
        assert!(level.construction_entity(ConstructionId(9)).is_none());
    }

    #[test]
    fn layers_and_guides_survive_a_save_and_load() {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.layers.push(Layer::new(LayerId(2), "Walls"));
        level.construction.push(ConstructionEntity::new(
            ConstructionId(3),
            ConstructionKind::Ray {
                origin: Point2::ORIGIN,
                direction: crate::geom::Vec2::new(1.0, 0.0),
            },
        ));
        let json = serde_json::to_string(&level).expect("serialize");
        let back: Level = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, level);
    }
}
