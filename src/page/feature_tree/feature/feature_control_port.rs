use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait FeatureControlPort {
    /// Gets the feature id
    fn get_id(&self) -> String;

    /// Initialize new feature with
    /// self reference
    fn init(&self, self_rc: &RcFeatureControl);

    /// Gets references to the child
    /// sockets of this feature
    fn get_child_sockets(&self) -> Vec<RcFeatureSocketControl>;

    fn serialize(&self) -> FeatureSerialization;
}

pub type RcFeatureControl = Rc<Box<dyn FeatureControlPort>>;
pub type WeakFeatureControl = WeakRef<Box<dyn FeatureControlPort>>;
