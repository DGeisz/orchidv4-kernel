use crate::page::feature_tree::feature_socket::feature_socket_control_port::{FeatureSocketControlPort, RcFeatureSocketControl};
use crate::page::feature_tree::feature_socket::feature_socket_generator::feature_socket_generator_port::RcFeatureSocketGenerator;
use std::rc::Rc;

pub mod feature_socket_control_port;
pub mod feature_socket_generator;

/// A feature socket is a structure which
/// holds/contains a single feature.
pub struct FeatureSocket {
    feature_socket_generator: RcFeatureSocketGenerator,
}

impl FeatureSocket {
    pub fn new(feature_socket_generator: RcFeatureSocketGenerator) -> RcFeatureSocketControl {
        Rc::new(Box::new(FeatureSocket {
            feature_socket_generator,
        }))
    }
}

impl FeatureSocketControlPort for FeatureSocket {}
