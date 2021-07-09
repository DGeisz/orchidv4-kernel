//! Port used by feature tree generators

use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use std::rc::Rc;

pub type RcFeatureTreeGenerator = Rc<Box<dyn FeatureTreeGeneratorPort>>;

pub trait FeatureTreeGeneratorPort {
    /// The struct behind the feature tree
    /// port needs a reference to itself before
    /// it can function properly, so provide an
    /// init method
    fn init(&self, self_rc: RcFeatureTreeGenerator);

    /// Creates a new feature tree
    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort>;

    /// Creates a new feature socket
    fn generate_feature_socket(&self) -> RcFeatureSocketControl;
}
