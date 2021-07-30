use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature::feature_request::FeatureRequest;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use std::rc::Rc;

pub trait FeatureGeneratorControl {
    fn generate_feature(
        &self,
        feature_request: &FeatureRequest,
        socket: &Rc<dyn SocketControl>,
    ) -> Option<Rc<dyn FeatureControl>>;
}
