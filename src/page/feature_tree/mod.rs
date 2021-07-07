use crate::page::feature_tree::feature_socket::feature_socket_control_port::FeatureSocketControlPort;

mod feature;
mod feature_socket;
pub mod feature_tree_control_port;

pub struct FeatureTree {
    base_socket_control: Box<dyn FeatureSocketControlPort>,
}
