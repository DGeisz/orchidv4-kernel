use crate::page::feature_tree::feature_socket::feature_socket_generator::feature_socket_generator_port::{FeatureSocketGeneratorPort, RcFeatureSocketGenerator};
use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::page::feature_tree::feature_socket::FeatureSocket;
use std::rc::Rc;
use std::cell::RefCell;

pub mod feature_socket_generator_port;

/// The actual struct used in the running
/// kernel to generate feature sockets
pub struct FeatureSocketGenerator {
    /// Reference to itself used for giving
    /// multiple structs the ability to generate
    /// sockets
    self_rc: RefCell<Option<RcFeatureSocketGenerator>>,
}

impl FeatureSocketGenerator {
    pub fn new() -> RcFeatureSocketGenerator {
        Rc::new(Box::new(FeatureSocketGenerator {
            self_rc: RefCell::new(None),
        }))
    }
}

impl FeatureSocketGeneratorPort for FeatureSocketGenerator {
    fn generate_feature_socket(&self) -> RcFeatureSocketControl {
        match &*self.self_rc.borrow() {
            Some(rc_generator) => FeatureSocket::new(Rc::clone(rc_generator)),
            None => panic!(
                "This feature socket generator hasn't been \
                properly initialized by setting self_rc"
            ),
        }
    }

    fn init(&self, self_rc: RcFeatureSocketGenerator) {
        *self.self_rc.borrow_mut() = Some(self_rc);
    }
}
