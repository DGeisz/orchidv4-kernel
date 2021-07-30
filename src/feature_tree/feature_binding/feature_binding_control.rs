use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use std::rc::Rc;

pub trait FeatureBindingControl {
    fn get_feature_ref(&self) -> Rc<dyn FeatureControl>;
    fn get_bottom_ref(&self) -> Rc<dyn SocketControl>;
}
