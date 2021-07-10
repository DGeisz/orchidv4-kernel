use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature::features::universal::Universal;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::{
    RcFeatureSocketControl, WeakFeatureSocketControl,
};
use crate::page::feature_tree::feature_socket::FeatureSocket;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::{
    FeatureTreeGeneratorPort, RcFeatureTreeGenerator,
};
use crate::page::feature_tree::FeatureTree;
use crate::utils::type_utils::SoftRef;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    self_ref: SoftRef<Box<dyn FeatureTreeGeneratorPort>>,
}

impl FeatureTreeGenerator {
    pub fn new() -> RcFeatureTreeGenerator {
        let new: RcFeatureTreeGenerator = Rc::new(Box::new(FeatureTreeGenerator {
            self_ref: SoftRef::new(),
        }));

        new.init(&new);

        new
    }
}

impl FeatureTreeGeneratorPort for FeatureTreeGenerator {
    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort> {
        FeatureTree::new(self.generate_feature_socket())
    }

    fn init(&self, self_ref: &RcFeatureTreeGenerator) {
        self.self_ref.set_ref(&self_ref);
    }

    fn generate_feature_socket(&self) -> RcFeatureSocketControl {
        FeatureSocket::new(self.self_ref.get_weak_ref())
    }

    fn generate_universal_feature(
        &self,
        parent_socket: WeakFeatureSocketControl,
    ) -> RcFeatureControl {
        Universal::new(parent_socket, self.self_ref.get_weak_ref())
    }
}
