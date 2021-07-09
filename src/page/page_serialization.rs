use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use serde::{Deserialize, Serialize};

/// Serialization of page to
/// be sent to FE clients
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PageSerialization {
    feature_tree: FeatureSocketSerialization,
    page_id: String,
}

impl PageSerialization {
    pub fn new(feature_tree: FeatureSocketSerialization, page_id: String) -> PageSerialization {
        PageSerialization {
            feature_tree,
            page_id,
        }
    }
}
