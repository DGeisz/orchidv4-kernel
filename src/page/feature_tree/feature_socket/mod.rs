use crate::page::feature_tree::feature::feature_control_port::{
    FeatureControlPort, RcFeatureControl, WeakFeatureControl,
};
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::{
    FeatureSocketControlPort, RcFeatureSocketControl, WeakFeatureSocketControl,
};
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::{
    FeatureTreeGeneratorPort, RcFeatureTreeGenerator, WeakFeatureTreeGenerator,
};
use crate::utils::type_utils::{Relltion, SelfRef, WeakRef};
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use uuid::Uuid;

pub mod feature_socket_control_port;
pub mod feature_socket_serialization;

/// A feature socket is a structure which
/// holds/contains a single feature.
pub struct FeatureSocket {
    id: String,
    feature_tree_generator: WeakFeatureTreeGenerator,
    self_ref: SelfRef<Box<dyn FeatureSocketControlPort>>,
    parent_weak: Option<WeakFeatureControl>,
}

impl FeatureSocket {
    pub fn new(feature_tree_generator: WeakFeatureTreeGenerator) -> RcFeatureSocketControl {
        Rc::new(Box::new(FeatureSocket {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            feature_tree_generator,
            self_ref: SelfRef::new(),
            parent_weak: None,
        }))
    }

    pub fn new_with_parent(
        feature_tree_generator: WeakFeatureTreeGenerator,
        parent_weak: WeakFeatureControl,
    ) -> RcFeatureSocketControl {
        let new: RcFeatureSocketControl = Rc::new(Box::new(FeatureSocket {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            feature_tree_generator,
            self_ref: SelfRef::new(),
            parent_weak: Some(parent_weak),
        }));

        new.init(&new);

        new
    }
}

impl FeatureSocketControlPort for FeatureSocket {
    fn serialize(&self) -> FeatureSocketSerialization {
        /*
        TODO: Test this once the implementation gets spicier
        */
        FeatureSocketSerialization::new(FeatureSerialization::None)
    }

    fn init(&self, self_ref: &RcFeatureSocketControl) {
        self.self_ref.set_ref(self_ref);
    }

    fn create_feature(&self, feature: FeatureEnum) -> Result<RcFeatureControl, FeatureTreeError> {
        unimplemented!()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}
