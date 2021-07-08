use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::FeatureTreeGeneratorPort;
use crate::page::feature_tree::FeatureTree;
use crate::page::feature_tree::feature_socket::feature_socket_generator::feature_socket_generator_port::RcFeatureSocketGenerator;

pub mod feature_tree_generator_port;

/// This is the struct used to generate
/// feature trees for pages in non-testing
/// contexts
pub struct FeatureTreeGenerator {
    feature_socket_generator: RcFeatureSocketGenerator,
}

impl FeatureTreeGenerator {
    pub fn new(
        feature_socket_generator: RcFeatureSocketGenerator,
    ) -> Box<dyn FeatureTreeGeneratorPort> {
        Box::new(FeatureTreeGenerator {
            feature_socket_generator,
        })
    }
}

impl FeatureTreeGeneratorPort for FeatureTreeGenerator {
    fn generate_feature_tree(&self) -> Box<dyn FeatureTreeControlPort> {
        FeatureTree::new(self.feature_socket_generator.generate_feature_socket())
    }
}
