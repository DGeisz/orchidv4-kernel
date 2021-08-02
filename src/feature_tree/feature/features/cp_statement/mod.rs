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

const CP_MAP_ID: u128 = 1001;

pub struct CpStatement {
    id: u128,
    self_ref: SoftRef<dyn FeatureControl>,
    parent_binding: SoftRef<dyn FeatureBindingControl>,
    c_socket: Rc<dyn SocketControl>,
    p_socket: Rc<dyn SocketControl>,
    ref_count: Cell<u32>,
}

impl FeatureControl for CpStatement {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_hash(&self) -> Option<u128> {
        match (self.c_socket.get_hash(), self.p_socket.get_hash()) {
            (Some(c_hash), Some(p_hash)) => {
                let mut input = c_hash.to_be_bytes().to_vec();
                input.append(&mut p_hash.to_be_bytes().to_vec());

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
        if let Some(socket) = self.c_socket.get_socket_by_id(socket_id) {
            Some(socket)
        } else if let Some(socket) = self.p_socket.get_socket_by_id(socket_id) {
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
            map: Box::new(FeatureSerialization::Leaf {
                id: CP_MAP_ID,
                latex: "CP".to_string(),
            }),
            map_latex: MapLatex::Basic,
            arg_latex: vec![
                Box::new(self.c_socket.serialize()),
                Box::new(self.p_socket.serialize()),
            ],
        }
    }

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.c_socket))
    }

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.p_socket))
    }

    fn is_feature_compatible_with_child_socket(
        &self,
        feature: &Rc<dyn FeatureControl>,
        socket_id: u128,
    ) -> bool {
        /*
        First make sure this socket is actually a child
        */
        if socket_id == self.c_socket.get_id() || socket_id == self.p_socket.get_id() {
            /*
            The only requirement for these sockets is
            that the feature be of type boolean
            */
            return feature.is_compatible_with_type(BOOLEAN_TYPE);
        }

        /*
        Otherwise, we don't know what the socket is, so
        automatically return false
        */
        false
    }

    fn is_compatible_with_type(&self, feature_type: FeatureType) -> bool {
        /*
        First see if the type is statement type, because
        this sucker is definitely a statement
        */
        if feature_type == STATEMENT_TYPE {
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
        If the request is coming from child sockets, then we pass the request
        up the tree
        */
        if request_source == self.c_socket.get_id() || request_source == self.p_socket.get_id() {
            if let Some(parent) = self.get_parent_socket() {
                return parent.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }
        }

        /*
        Otherwise, we return false because neither cp_statement
        is in the validity scope of a feature higher in the tree
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
        self.c_socket
            .get_subtree_ref_record()
            .reconcile(&self.p_socket.get_subtree_ref_record())
    }

    fn any_external_subtree_dependents(&self) -> bool {
        self.get_subtree_ref_record().any_external_dependents()
    }
}
