use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::page::feature_tree::feature_socket::FeatureSocket;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::{
    FeatureTreeGeneratorPort, RcFeatureTreeGenerator,
};
use crate::page::feature_tree::FeatureTree;
use crate::utils::type_utils::Relltion;
use std::cell::RefCell;
use std::rc::Rc;

pub mod feature_tree_generator_port;

/// This is the struct used to generate the
/// different modular structs within a feature
/// tree (ie feature trees, features, feature sockets)
pub struct FeatureTreeGenerator {
    /// Needs a reference to itself for
    /// giving multiple structs the ability
    /// to generate different feature tree structs
    ///
    /// Note we unwrap self rc everywhere because
    /// init should be immediately called after creating
    /// this struct
    self_rc: Relltion<RcFeatureTreeGenerator>,
}

impl FeatureTreeGenerator {
    pub fn new() -> RcFeatureTreeGenerator {
        Rc::new(Box::new(FeatureTreeGenerator {
            self_rc: RefCell::new(None),
        }))
    }

    /// Creates a clone of itself for passing self references
    /// to other structs that need to generate features
    fn clone_self_rc(&self) -> RcFeatureTreeGenerator {
        match &*self.self_rc.borrow() {
            Some(generator) => Rc::clone(generator),
            None => panic!("This feature tree generator hasn't been initialized yet"),
        }
    }
}

impl FeatureTreeGeneratorPort for FeatureTreeGenerator {
    fn init(&self, self_rc: RcFeatureTreeGenerator) {
        *self.self_rc.borrow_mut() = Some(self_rc);
    }

    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort> {
        FeatureTree::new(self.generate_feature_socket())
    }

    fn generate_feature_socket(&self) -> RcFeatureSocketControl {
        FeatureSocket::new(self.clone_self_rc())
    }
}
