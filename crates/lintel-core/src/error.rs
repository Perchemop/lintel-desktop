//! Typed failures. Nothing in the domain panics on bad user input: a command
//! that cannot be honoured returns one of these instead.

use crate::ids::{ConstructionId, DimensionId, LabelId, LayerId, LevelId, OpeningId, WallId};

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("{0} does not exist")]
    WallNotFound(WallId),
    #[error("{0} does not exist")]
    OpeningNotFound(OpeningId),
    #[error("{0} does not exist")]
    DimensionNotFound(DimensionId),
    #[error("{0} does not exist")]
    LabelNotFound(LabelId),
    #[error("{0} does not exist")]
    LevelNotFound(LevelId),
    #[error("{0} does not exist")]
    LayerNotFound(LayerId),
    #[error("{0} does not exist")]
    ConstructionNotFound(ConstructionId),
    #[error("the last level cannot be deleted; give the project another level first")]
    LastLevel,
    #[error("the default layer cannot be deleted")]
    DefaultLayer,
    #[error("{0} still has entities; move them to another layer first")]
    LayerInUse(LayerId),
    #[error("{0}")]
    Invalid(String),
    #[error("unsupported project format {found}; this build reads up to {supported}")]
    UnsupportedVersion { found: u32, supported: u32 },
    #[error("the file is not a valid Lintel project: {0}")]
    InvalidProject(String),
    #[error("could not read or write the project file: {0}")]
    Io(#[from] std::io::Error),
    #[error("the project file is not valid JSON: {0}")]
    Json(#[from] serde_json::Error),
}

impl CoreError {
    /// Shorthand for a rejected command, with a plain-language reason.
    pub fn invalid(reason: impl Into<String>) -> Self {
        CoreError::Invalid(reason.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_are_plain_language() {
        let e = CoreError::invalid("a wall needs a length of at least 10 mm");
        assert_eq!(e.to_string(), "a wall needs a length of at least 10 mm");
        let e = CoreError::LastLevel;
        assert!(e.to_string().contains("last level"));
        let e = CoreError::WallNotFound(WallId(4));
        assert_eq!(e.to_string(), "wall #4 does not exist");
        let e = CoreError::DimensionNotFound(DimensionId(9));
        assert_eq!(e.to_string(), "dimension #9 does not exist");
        let e = CoreError::LabelNotFound(LabelId(2));
        assert_eq!(e.to_string(), "label #2 does not exist");
        let e = CoreError::LayerNotFound(LayerId(3));
        assert_eq!(e.to_string(), "layer #3 does not exist");
        let e = CoreError::ConstructionNotFound(ConstructionId(8));
        assert_eq!(e.to_string(), "construction #8 does not exist");
        let e = CoreError::DefaultLayer;
        assert!(e.to_string().contains("default layer"));
        let e = CoreError::LayerInUse(LayerId(4));
        assert!(e.to_string().contains("move them"));
    }
}
