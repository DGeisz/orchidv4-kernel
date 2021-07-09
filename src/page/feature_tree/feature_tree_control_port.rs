use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::page::feature_tree::FeatureTree;

pub trait FeatureTreeControlPort {
    /// Serialize the entire feature tree
    /// into a representation digestible by clients
    fn serialize(&self) -> FeatureSocketSerialization;

    /// Creates a feature in the given socket
    fn create_feature(
        &mut self,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<(), FeatureTreeError>;
}
