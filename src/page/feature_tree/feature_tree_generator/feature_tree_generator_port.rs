//! Port used by feature tree generators

use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;

pub trait FeatureTreeGeneratorPort {
    /// Creates a new feature tree
    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort>;
}
