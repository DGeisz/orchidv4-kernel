use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use std::rc::Rc;

/// Type for a reference count feature socket
/// trait object
pub type RcFeatureSocketControl = Rc<Box<dyn FeatureSocketControlPort>>;

pub trait FeatureSocketControlPort {
    fn serialize(&self) -> FeatureSocketSerialization;
}
