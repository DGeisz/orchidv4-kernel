use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use serde::{Deserialize, Serialize};

/// Serialization of page to
/// be sent to FE clients
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PageSerialization {
    feature_tree: FeatureSocketSerialization,
}

impl PageSerialization {
    pub fn new(feature_tree: FeatureSocketSerialization) -> PageSerialization {
        PageSerialization { feature_tree }
    }
}
