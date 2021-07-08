use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use serde::{Deserialize, Serialize};

/// Serialization of feature socket to be
/// sent to FE clients
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSocketSerialization {
    feature: FeatureSerialization,
}

impl FeatureSocketSerialization {
    /// Creates a new feature socket serialization
    pub fn new(feature: FeatureSerialization) -> FeatureSocketSerialization {
        FeatureSocketSerialization { feature }
    }
}
