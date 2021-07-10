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
use crate::utils::type_utils::{HardRef, SoftRef, WeakRef};
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
    self_ref: SoftRef<Box<dyn FeatureSocketControlPort>>,
    parent_feature: SoftRef<Box<dyn FeatureControlPort>>,
    feature: HardRef<Box<dyn FeatureControlPort>>,
}

impl FeatureSocket {
    pub fn new(feature_tree_generator: WeakFeatureTreeGenerator) -> RcFeatureSocketControl {
        let new: RcFeatureSocketControl = Rc::new(Box::new(FeatureSocket {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            feature_tree_generator,
            self_ref: SoftRef::new(),
            parent_feature: SoftRef::new(),
            feature: HardRef::new(),
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

    fn set_parent_ref(&self, parent_feature: &RcFeatureControl) {
        self.parent_feature.set_ref(parent_feature);
    }

    fn create_feature(&self, feature: FeatureEnum) -> Result<RcFeatureControl, FeatureTreeError> {
        /*
        First let's check to be sure our socket isn't even full
        */
        if !self.feature.ref_set() {
            return Err(FeatureTreeError::SocketAlreadyFilled(self.get_id()));
        }

        match feature {
            FeatureEnum::Universal => {
                /* Create the feature */
                let feature = self
                    .feature_tree_generator
                    .get()
                    .generate_universal_feature(self.self_ref.get_weak_ref());

                /* Set this feature as this socket's feature */
                self.feature.set_ref(Rc::clone(&feature));

                Ok(feature)
            }
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}
