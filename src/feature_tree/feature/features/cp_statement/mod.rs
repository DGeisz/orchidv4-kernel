use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::utils::type_utils::{SoftRef, WeakRef};
use std::rc::Rc;

pub struct CpStatement {
    id: u128,
    self_ref: SoftRef<dyn FeatureControl>,
    parent_binding: SoftRef<dyn FeatureBindingControl>,
    c_socket: Rc<dyn SocketControl>,
    p_socket: Rc<dyn SocketControl>,
}

impl FeatureControl for CpStatement {
    fn get_id(&self) -> u128 {
        self.id
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
        unimplemented!()
    }

    fn can_detach(&self) -> bool {
        unimplemented!()
    }

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.c_socket))
    }

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>> {
        Some(Rc::clone(&self.p_socket))
    }
}
