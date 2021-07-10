use crate::page::feature_tree::feature::features::universal::universal_serialization::UniversalSerialization;
use serde::{Deserialize, Serialize};

/// Serialization of features to be
/// sent to FE clients
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum FeatureSerialization {
    /*
    This option is for feature sockets
    that don't yet contain a feature
    */
    None,

    /*
    Alright, actual feature serialization
    */
    Universal(UniversalSerialization),
}
