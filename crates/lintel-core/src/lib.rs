//! Lintel core: the drafting domain, with no window, GPU or display in sight.
//!
//! Layering (addendum K.3): this crate owns the model, the commands that change
//! it, the geometry derived from it, the project file and the drawing-set
//! export. The desktop shell in `apps/lintel` paints this state and turns
//! input into [`Command`]s; it is the only place that knows about pixels.
//!
//! Every workflow test runs headless against this crate, so the behaviour a
//! user gets is the behaviour CI proves.

pub mod command;
pub mod construction;
pub mod dimension;
pub mod document;
pub mod error;
pub mod geom;
pub mod ids;
pub mod join;
pub mod label;
pub mod layer;
pub mod level;
pub mod measurements;
pub mod opening;
pub mod pdf;
pub mod persistence;
pub mod project;
pub mod room;
pub mod schedule;
pub mod selection;
pub mod snap;
pub mod units;
pub mod wall;

pub use command::{Applied, Command};
pub use construction::{ConstructionEntity, ConstructionKind};
pub use dimension::{Dimension, DimensionKind};
pub use document::Document;
pub use error::CoreError;
pub use geom::{Bounds, Point2};
pub use ids::{
    ConstructionId, DimensionId, EntityId, LabelId, LayerId, LevelId, OpeningId, WallId,
};
pub use join::{Hit, PlanGeometry};
pub use label::PlanLabel;
pub use layer::{EntitySelector, EntityStyle, Layer, LayerProperty, Linetype, Rgb};
pub use measurements::{LevelTotals, Measurements, ProjectTotals};
pub use project::Project;
pub use room::Room;
pub use schedule::ScheduleRow;
pub use selection::{EntityType, Selection, SelectionFilter, SelectionMode, select};
pub use units::{UnitSystem, format_area};
