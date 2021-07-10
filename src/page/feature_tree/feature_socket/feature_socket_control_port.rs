use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_socket::FeatureSocket;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::utils::type_utils::WeakRef;
use mockall::*;
use std::rc::Rc;

/// Type for a reference count feature socket
/// trait object
pub type RcFeatureSocketControl = Rc<Box<dyn FeatureSocketControlPort>>;
/// Type for a weak reference to a feature socket
/// trait object
pub type WeakFeatureSocketControl = WeakRef<Box<dyn FeatureSocketControlPort>>;

#[automock]
pub trait FeatureSocketControlPort {
    /// Serialized feature subtree down to client
    /// digestible representation
    fn serialize(&self) -> FeatureSocketSerialization;

    /// Initialize the reference to itself.  This should
    /// only be used internally
    fn init(&self, self_ref: &RcFeatureSocketControl);

    /// Set parent ref
    fn set_parent_ref(&self, parent_feature: &RcFeatureControl);

    /// Attempts to create and fill this socket
    /// with a new feature
    fn create_feature(&self, feature: FeatureEnum) -> Result<RcFeatureControl, FeatureTreeError>;

    /// Gets the id of the feature socket in question
    fn get_id(&self) -> String;
}

pub fn mock_feature_socket_control() -> MockFeatureSocketControlPort {
    MockFeatureSocketControlPort::new()
}
