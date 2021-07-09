use serde::{Deserialize, Serialize};

/// This enum enumerates over
/// all the different features
/// that exist
#[derive(Clone, Eq, PartialEq, Serialize, Debug, Deserialize)]
pub enum FeatureEnum {
    Universal,
}
