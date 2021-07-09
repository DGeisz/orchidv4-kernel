use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_socket::FeatureSocket;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use std::rc::Rc;

/// Type for a reference count feature socket
/// trait object
pub type RcFeatureSocketControl = Rc<Box<dyn FeatureSocketControlPort>>;

pub trait FeatureSocketControlPort {
    /// Serialized feature subtree down to client
    /// digestible representation
    fn serialize(&self) -> FeatureSocketSerialization;

    /// Attempts to create and fill this socket
    /// with a new feature
    fn create_feature(&self, feature: FeatureEnum) -> Result<RcFeatureControl, FeatureTreeError>;

    /// Gets the id of the feature socket in question
    fn get_id(&self) -> String;
}
