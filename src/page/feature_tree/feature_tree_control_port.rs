use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;

pub trait FeatureTreeControlPort {
    /// Serialize the entire feature tree
    /// into a representation digestible by clients
    fn serialize(&self) -> FeatureSocketSerialization;
}
