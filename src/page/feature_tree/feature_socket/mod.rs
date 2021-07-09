use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::{
    FeatureSocketControlPort, RcFeatureSocketControl,
};
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::RcFeatureTreeGenerator;
use std::rc::Rc;
use uuid::Uuid;

pub mod feature_socket_control_port;
pub mod feature_socket_serialization;

/// A feature socket is a structure which
/// holds/contains a single feature.
pub struct FeatureSocket {
    id: String,
    feature_tree_generator: RcFeatureTreeGenerator,
}

impl FeatureSocket {
    pub fn new(feature_tree_generator: RcFeatureTreeGenerator) -> RcFeatureSocketControl {
        Rc::new(Box::new(FeatureSocket {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            feature_tree_generator,
        }))
    }
}

impl FeatureSocketControlPort for FeatureSocket {
    fn serialize(&self) -> FeatureSocketSerialization {
        FeatureSocketSerialization::new(FeatureSerialization::None)
    }

    fn create_feature(&self, feature: FeatureEnum) -> Result<RcFeatureControl, FeatureTreeError> {
        unimplemented!()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}
