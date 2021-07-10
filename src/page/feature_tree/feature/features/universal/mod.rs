use crate::page::feature_tree::feature::feature_control_port::FeatureControlPort;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use std::rc::Rc;

pub struct Universal {
    id: String,
    constraints_socket: RcFeatureSocketControl,
    properties_socket: RcFeatureSocketControl,
}

impl FeatureControlPort for Universal {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_child_sockets(&self) -> Vec<RcFeatureSocketControl> {
        vec![
            Rc::clone(&self.constraints_socket),
            Rc::clone(&self.properties_socket),
        ]
    }
}
