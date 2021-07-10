//! Port used by feature tree generators

use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::{
    RcFeatureSocketControl, WeakFeatureSocketControl,
};
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::utils::type_utils::WeakRef;
use std::rc::{Rc, Weak};

/// Helper type alias because ain't nobody got
/// time to write out that full type
pub type RcFeatureTreeGenerator = Rc<Box<dyn FeatureTreeGeneratorPort>>;

pub type WeakFeatureTreeGenerator = WeakRef<Box<dyn FeatureTreeGeneratorPort>>;

/// Helper type alias because ain't nobody got
/// time to write out that full type
pub type FeatureTreeGenerator = Box<dyn FeatureTreeGeneratorPort>;

pub trait FeatureTreeGeneratorPort {
    /// Creates a new feature tree
    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort>;

    /// Initialize self ref.  This should never be
    /// called by anything outside structs implementing
    /// this trait
    fn init(&self, self_ref: &RcFeatureTreeGenerator);

    /// Creates a new feature socket
    fn generate_feature_socket(&self) -> RcFeatureSocketControl;

    /// Creates a new universal feature
    fn generate_universal_feature(
        &self,
        parent_socket: WeakFeatureSocketControl,
    ) -> RcFeatureControl;
}
