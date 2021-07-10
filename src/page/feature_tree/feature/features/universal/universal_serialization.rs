use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct UniversalSerialization {
    constraints: FeatureSocketSerialization,
    properties: FeatureSocketSerialization,
}

impl UniversalSerialization {
    pub fn new(
        constraints: FeatureSocketSerialization,
        properties: FeatureSocketSerialization,
    ) -> UniversalSerialization {
        UniversalSerialization {
            constraints,
            properties,
        }
    }
}
