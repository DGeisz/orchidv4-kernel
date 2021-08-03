use crate::feature_tree::compact_feature::CompactFeature;
use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature::feature_serialization::{
    FeatureSerialization, MapLatex, SocketSerialization,
};
use crate::feature_tree::feature::features::feature_ids::TYPE_MAP_ID;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::feature_tree::feature_type::built_in_types::{BASE_SCOPE, BOOLEAN_TYPE};
use crate::feature_tree::feature_type::{FeatureType, TypeHierarchyAnchor};
use crate::feature_tree::feature_utils::feature_subtree_reference_record::FeatureSubtreeRefRecord;
use crate::utils::type_utils::{SoftRef, WeakRef};
use std::cell::Cell;
use std::rc::Rc;
use xxhash_rust::xxh3::xxh3_128;

pub struct TypeMap {
    id: u128,
    self_ref: SoftRef<dyn FeatureControl>,
    parent_binding: SoftRef<dyn FeatureBindingControl>,
    term_socket: Rc<dyn SocketControl>,
    type_socket: Rc<dyn SocketControl>,
    ref_count: Cell<u32>,
}

impl FeatureControl for TypeMap {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_hash(&self) -> Option<u128> {
        match (self.term_socket.get_hash(), self.type_socket.get_hash()) {
            (Some(term_hash), Some(type_hash)) => {
                /*Start with map id, and mix in socket hashes*/
                let mut input = TYPE_MAP_ID.to_be_bytes().to_vec();

                input.append(&mut term_hash.to_be_bytes().to_vec());
                input.append(&mut type_hash.to_be_bytes().to_vec());

                Some(xxh3_128(&input))
            }
            _ => None,
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
        if let Some(socket) = self.term_socket.get_socket_by_id(socket_id) {
            Some(socket)
        } else if let Some(socket) = self.type_socket.get_socket_by_id(socket_id) {
            Some(socket)
        } else {
            None
        }
    }

    fn get_parent_socket(&self) -> Option<Rc<dyn SocketControl>> {
        match self.parent_binding.weak_fetch() {
            Some(binding) => Some(binding.get_socket_ref()),
            None => None,
        }
    }

    fn serialize(&self) -> FeatureSerialization {
        FeatureSerialization::Map {
            map: Box::new(SocketSerialization::new(
                /* Give the inaccessible socket the same id as the actual map */
                TYPE_MAP_ID,
                Some(FeatureSerialization::Leaf {
                    id: TYPE_MAP_ID,
                    latex: ":".to_string(),
                }),
            )),
            map_latex: MapLatex::MultiSource(
                vec![String::new(), " : ".to_string(), String::new()],
                vec![0, 1],
            ),
            arg: Box::new(FeatureSerialization::Tuple {
                children: vec![
                    Box::new(self.term_socket.serialize()),
                    Box::new(self.type_socket.serialize()),
                ],
            }),
        }
    }

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.term_socket))
    }

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.type_socket))
    }

    fn is_feature_compatible_with_child_socket(
        &self,
        feature: &Rc<dyn FeatureControl>,
        socket_id: u128,
    ) -> bool {
        /*
        Ok, so if the socket if the term socket, what do I need?

        I want to ensure that the feature in the term socket is compatible
        with the type socket as its type superior
        */
        if socket_id == self.term_socket.get_id() {}

        if socket_id == self.type_socket.get_id() {}

        false
    }

    fn is_compatible_with_type(&self, feature_type: FeatureType) -> bool {
        /*
        First see if the type is boolean type, because
        this sucker is definitely a boolean
        */
        if feature_type == BOOLEAN_TYPE {
            return true;
        }

        /*
        If not, check in the rest of the tree if this particular
        statement is in fact compatible with the given type
        */
        if let Some(hash) = self.get_hash() {
            if let Some(parent) = self.get_parent_socket() {
                return parent.is_feature_compatible_with_type(hash, feature_type, self.get_id());
            }
        }

        /*
        Otherwise, the type is incompatible
        */
        false
    }

    fn is_feature_compatible_with_type(
        &self,
        feature_hash: u128,
        feature_type: FeatureType,
        request_source: u128,
    ) -> bool {
        if let Some(parent) = self.get_parent_socket() {
            if request_source == self.term_socket.get_id()
                || request_source == self.type_socket.get_id()
                || request_source == parent.get_id()
            {
                if let Some(type_feature) = self.type_socket.get_feature() {
                    if let (Some(term_hash), Some(type_type)) =
                        (self.term_socket.get_hash(), type_feature.as_type())
                    {
                        if feature_hash == term_hash && type_type == feature_type {
                            return true;
                        }
                    }
                }
            }

            parent.is_feature_compatible_with_type(feature_hash, feature_type, self.get_id())
        } else {
            false
        }
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
        let mut ref_record = self
            .term_socket
            .get_subtree_ref_record()
            .reconcile(&self.type_socket.get_subtree_ref_record());

        let ref_count = self.get_ref_count();

        if ref_count > 0 {
            ref_record.add_ref_count(self.get_id(), self.get_ref_count());
        }

        ref_record
    }

    fn get_hierarchy_level(&self) -> i16 {
        0
    }

    fn get_hierarchy_anchor(&self) -> TypeHierarchyAnchor {
        TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE)
    }

    fn to_compact(&self) -> Option<CompactFeature> {
        if let (Some(term_compact), Some(type_compact)) =
            (self.term_socket.to_compact(), self.type_socket.to_compact())
        {
            Some(CompactFeature::Map {
                map: Box::new(CompactFeature::Leaf(TYPE_MAP_ID)),
                arg: Box::new(CompactFeature::Tuple(vec![
                    Box::new(term_compact),
                    Box::new(type_compact),
                ])),
            })
        } else {
            None
        }
    }
}
