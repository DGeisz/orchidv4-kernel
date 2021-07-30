use crate::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait FeatureControl {
    fn get_id(&self) -> u128;

    fn get_self_ref(&self) -> Rc<dyn FeatureControl>;

    fn get_self_weak(&self) -> WeakRef<dyn FeatureControl>;

    fn set_binding(&self, binding: Rc<dyn FeatureBindingControl>);

    fn get_socket_by_id(&self, socket_id: u128) -> Option<Rc<dyn SocketControl>>;

    fn get_socket(&self) -> Option<Rc<dyn SocketControl>>;

    fn serialize(&self) -> FeatureSerialization;

    fn can_detach(&self) -> bool;

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>>;

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>>;
}
