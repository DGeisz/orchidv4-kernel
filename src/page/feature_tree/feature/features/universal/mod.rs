use crate::page::feature_tree::feature::feature_control_port::{
    FeatureControlPort, RcFeatureControl,
};
use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature::features::universal::universal_serialization::UniversalSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::{
    RcFeatureSocketControl, WeakFeatureSocketControl,
};
use crate::page::feature_tree::feature_socket::FeatureSocket;
use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::WeakFeatureTreeGenerator;
use crate::utils::type_utils::SoftRef;
use std::rc::Rc;
use uuid::Uuid;

pub mod universal_serialization;

pub struct Universal {
    id: String,
    self_ref: SoftRef<Box<dyn FeatureControlPort>>,
    constraints_socket: RcFeatureSocketControl,
    properties_socket: RcFeatureSocketControl,
    parent_socket: WeakFeatureSocketControl,
    feature_tree_generator: WeakFeatureTreeGenerator,
}

impl Universal {
    pub fn new(
        parent_socket: WeakFeatureSocketControl,
        feature_tree_generator: WeakFeatureTreeGenerator,
    ) -> RcFeatureControl {
        let generator = feature_tree_generator.get();

        let new: RcFeatureControl = Rc::new(Box::new(Universal {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            self_ref: SoftRef::new(),
            constraints_socket: generator.generate_feature_socket(),
            properties_socket: generator.generate_feature_socket(),
            parent_socket,
            feature_tree_generator,
        }));

        new.init(&new);

        new
    }
}

impl FeatureControlPort for Universal {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn init(&self, self_rc: &RcFeatureControl) {
        /* Set up our own self reference */
        self.self_ref.set_ref(self_rc);

        /* Set up parent references in child sockets */
        self.constraints_socket.set_parent_ref(self_rc);
        self.properties_socket.set_parent_ref(self_rc);
    }

    fn get_child_sockets(&self) -> Vec<RcFeatureSocketControl> {
        vec![
            Rc::clone(&self.constraints_socket),
            Rc::clone(&self.properties_socket),
        ]
    }

    fn serialize(&self) -> FeatureSerialization {
        FeatureSerialization::Universal(UniversalSerialization::new(
            self.constraints_socket.serialize(),
            self.properties_socket.serialize(),
        ))
    }
}
