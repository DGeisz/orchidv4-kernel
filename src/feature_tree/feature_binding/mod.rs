use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub mod feature_binding_control;

pub struct FeatureBinding {
    feature: Rc<dyn FeatureControl>,
    socket: WeakRef<dyn SocketControl>,
}

impl FeatureBinding {
    pub fn bind(feature: &Rc<dyn FeatureControl>, socket: &Rc<dyn SocketControl>) {
        let binding: Rc<dyn FeatureBindingControl> = Rc::new(FeatureBinding {
            feature: Rc::clone(feature),
            socket: socket.get_self_weak(),
        });

        socket.set_binding(Rc::clone(&binding));
        feature.set_binding(binding);
    }
}

impl FeatureBindingControl for FeatureBinding {
    fn get_feature_ref(&self) -> Rc<dyn FeatureControl> {
        Rc::clone(&self.feature)
    }

    fn get_bottom_ref(&self) -> Rc<dyn SocketControl> {
        self.socket.get()
    }
}
