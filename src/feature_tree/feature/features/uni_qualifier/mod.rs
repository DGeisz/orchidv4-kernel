//! Universal Qualifier

use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature::feature_serialization::{FeatureSerialization, MapLatex};
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::feature_tree::feature_type::built_in_types::{BOOLEAN_TYPE, STATEMENT_TYPE};
use crate::feature_tree::feature_type::FeatureType;
use crate::feature_tree::feature_utils::feature_subtree_reference_record::FeatureSubtreeRefRecord;
use crate::utils::type_utils::{SoftRef, WeakRef};
use std::cell::Cell;
use std::rc::Rc;
use xxhash_rust::xxh3::xxh3_128;

const UNI_MAP_ID: u128 = 1002;

pub struct UniQualifier {
    id: u128,
    self_ref: SoftRef<dyn FeatureControl>,
    parent_binding: SoftRef<dyn FeatureBindingControl>,
    cp_socket: Rc<dyn SocketControl>,
    ref_count: Cell<u32>,
}

impl FeatureControl for UniQualifier {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_hash(&self) -> Option<u128> {
        match self.cp_socket.get_hash() {
            Some(cp_hash) => {
                let mut input = UNI_MAP_ID.to_be_bytes().to_vec();
                input.append(&mut cp_hash.to_be_bytes().to_vec());

                Some(xxh3_128(&input))
            }
            None => None,
        }
    }

    fn get_self_ref(&self) -> Rc<dyn FeatureControl> {
        self.self_ref.strong_fetch()
    }

    fn get_self_weak(&self) -> WeakRef<dyn FeatureControl> {
        self.self_ref.get_weak_ref()
    }

    fn set_binding(&self, binding: &Rc<dyn FeatureBindingControl>) {
        self.parent_binding.set_ref(binding)
    }

    fn get_socket_by_id(&self, socket_id: u128) -> Option<Rc<dyn SocketControl>> {
        self.cp_socket.get_socket_by_id(socket_id)
    }

    fn get_parent_socket(&self) -> Option<Rc<dyn SocketControl>> {
        match self.parent_binding.weak_fetch() {
            Some(binding) => Some(binding.get_socket_ref()),
            None => None,
        }
    }

    fn serialize(&self) -> FeatureSerialization {
        FeatureSerialization::Map {
            map: Box::new(FeatureSerialization::Leaf {
                id: UNI_MAP_ID,
                latex: "∀".to_string(),
            }),
            map_latex: MapLatex::Basic,
            arg_latex: Box::new(self.cp_socket.serialize()),
        }
    }

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.cp_socket))
    }

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.cp_socket))
    }

    fn is_feature_compatible_with_child_socket(
        &self,
        feature: &Rc<dyn FeatureControl>,
        socket_id: u128,
    ) -> bool {
        /*
        First make sure we're talking about our child socket,
        and make sure the feature is compatible with the statement type
        */
        if socket_id == self.cp_socket.get_id() && feature.is_compatible_with_type(STATEMENT_TYPE) {
            /*
            Now let's see if this feature returns something in get_cp_sockets
            */
            if let (Some(_), Some(_)) = feature.get_cp_sockets() {
                /*
                That's all we need
                */
                return true;
            }
        }

        false
    }

    fn is_compatible_with_type(&self, feature_type: FeatureType) -> bool {
        /*
        First see if the type is boolean type,
        as this is def a boolean
        */
        if feature_type == BOOLEAN_TYPE {
            return true;
        }

        /*
        If not, let's check the rest of the tree to see if
        this feature is secretly compatible with the given type
        */
        if let Some(hash) = self.get_hash() {
            if let Some(parent) = self.get_parent_socket() {
                return parent.is_feature_compatible_with_type(hash, feature_type, self.get_id());
            }
        }

        /*
        Otherwise, this bad boi is incompatible
        */
        false
    }

    fn is_feature_compatible_with_type(
        &self,
        feature_hash: u128,
        feature_type: FeatureType,
        request_source: u128,
    ) -> bool {
        /*
        If the request is coming from cp_socket,
        then pass the request up the tree
        */
        if request_source == self.cp_socket.get_id() {
            if let Some(parent) = self.get_parent_socket() {
                return parent.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }
        }

        /*
        Otherwise we return false because we can't pass
        the request down the tree
        */
        false
    }

    fn inc_ref_count(&self) {
        self.ref_count.replace(self.get_ref_count() + 1);
    }

    fn dec_ref_count(&self) {
        self.ref_count.replace(self.get_ref_count() - 1);
    }

    fn get_ref_count(&self) -> u32 {
        self.ref_count.get()
    }

    fn get_subtree_ref_record(&self) -> FeatureSubtreeRefRecord {
        let mut ref_record = self.cp_socket.get_subtree_ref_record();
        let ref_count = self.get_ref_count();

        if ref_count > 0 {
            ref_record.add_ref_count(self.get_id(), self.get_ref_count())
        }

        ref_record
    }
}
