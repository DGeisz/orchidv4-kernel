use serde::{Deserialize, Serialize};

/// Serialization of features to be
/// sent to FE clients
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum FeatureSerialization {
    /// This option is for feature sockets
    /// that don't yet contain a feature
    None,
}
