use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;

mod feature;
pub mod feature_socket;
pub mod feature_tree_control_port;
pub mod feature_tree_generator;

pub struct FeatureTree {
    base_socket_control: RcFeatureSocketControl,
}

impl FeatureTree {
    pub fn new(base_socket_control: RcFeatureSocketControl) -> Box<dyn FeatureTreeControlPort> {
        Box::new(FeatureTree {
            base_socket_control,
        })
    }
}

impl FeatureTreeControlPort for FeatureTree {
    fn serialize(&self) -> FeatureSocketSerialization {
        self.base_socket_control.serialize()
    }
}
