use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature::feature_serialization::{
    FeatureSerialization, MapLatex, SocketSerialization,
};
use crate::feature_tree::feature::features::feature_ids::CONJUNCTION_MAP_ID;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::feature_tree::feature_type::built_in_types::BOOLEAN_TYPE;
use crate::feature_tree::feature_type::FeatureType;
use crate::feature_tree::feature_utils::feature_subtree_reference_record::FeatureSubtreeRefRecord;
use crate::utils::type_utils::{SoftRef, WeakRef};
use std::cell::Cell;
use std::rc::Rc;
use xxhash_rust::xxh3::xxh3_128;

pub struct Conjunction {
    id: u128,
    self_ref: SoftRef<dyn FeatureControl>,
    parent_binding: SoftRef<dyn FeatureBindingControl>,
    socket_1: Rc<dyn SocketControl>,
    socket_2: Rc<dyn SocketControl>,
    ref_count: Cell<u32>,
}

impl FeatureControl for Conjunction {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_hash(&self) -> Option<u128> {
        match (self.socket_1.get_hash(), self.socket_2.get_hash()) {
            (Some(hash_1), Some(hash_2)) => {
                /*Start with map id, and mix in socket hashes*/
                let mut input = CONJUNCTION_MAP_ID.to_be_bytes().to_vec();

                input.append(&mut hash_1.to_be_bytes().to_vec());
                input.append(&mut hash_2.to_be_bytes().to_vec());

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
        if let Some(socket) = self.socket_1.get_socket_by_id(socket_id) {
            Some(socket)
        } else if let Some(socket) = self.socket_2.get_socket_by_id(socket_id) {
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
                CONJUNCTION_MAP_ID,
                Some(FeatureSerialization::Leaf {
                    id: CONJUNCTION_MAP_ID,
                    latex: "∧".to_string(),
                }),
            )),
            map_latex: MapLatex::MultiSource(
                vec![String::new(), " ∧ ".to_string(), String::new()],
                vec![0, 1],
            ),
            arg: Box::new(FeatureSerialization::Tuple {
                children: vec![
                    Box::new(self.socket_1.serialize()),
                    Box::new(self.socket_2.serialize()),
                ],
            }),
        }
    }

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.socket_1))
    }

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.socket_2))
    }

    fn is_feature_compatible_with_child_socket(
        &self,
        feature: &Rc<dyn FeatureControl>,
        socket_id: u128,
    ) -> bool {
        /*
        First make sure socket is actually a child
        */
        if socket_id == self.socket_1.get_id() || socket_id == self.socket_2.get_id() {
            return feature.is_compatible_with_type(BOOLEAN_TYPE);
        }

        /*
        Otherwise, we don't know what the socket is, so return false
        */
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
        /*
        If the request is coming from the second socket, we can
        pass it to the first socket because the first socket
        is also in the validity scope
        */
        if request_source == self.socket_2.get_id() {
            /*
            Only return true from this block.  If it's
            false we pass request up to parent in the next block
            */
            if self.socket_1.is_feature_compatible_with_type(
                feature_hash,
                feature_type.clone(),
                self.get_id(),
            ) {
                return true;
            }
        }

        /*
        Now get the parent, because what happens next won't work without it
        */
        if let Some(parent) = self.get_parent_socket() {
            /*
            If request is coming from child sockets, and we already checked with
            socket_1 (if that's even and option, pass the request to the parent
            */
            if request_source == self.socket_1.get_id() || request_source == self.socket_2.get_id()
            {
                return parent.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }

            /*
            If the request if coming from the parent, then we forward it
            to the second socket
            */
            if request_source == parent.get_id() {
                return self.socket_2.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }
        }

        /*
        Otherwise, we either don't have a parent or
        any relation to the request source, so it's false
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
        let mut ref_record = self
            .socket_1
            .get_subtree_ref_record()
            .reconcile(&self.socket_2.get_subtree_ref_record());

        let ref_count = self.get_ref_count();

        if ref_count > 0 {
            ref_record.add_ref_count(self.get_id(), self.get_ref_count());
        }

        ref_record
    }
}
