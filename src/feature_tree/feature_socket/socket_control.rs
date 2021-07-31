use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_tree_error::FeatureTreeError;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait SocketControl {
    fn get_id(&self) -> u128;

    fn get_self_ref(&self) -> Rc<dyn SocketControl>;

    fn get_self_weak(&self) -> WeakRef<dyn SocketControl>;

    fn set_binding(&self, binding: &Rc<dyn FeatureBindingControl>);

    fn detach(&self) -> Result<(), FeatureTreeError>;

    fn get_socket_by_id(&self, socket_id: u128) -> Option<Rc<dyn SocketControl>> {
        if self.get_id() == socket_id {
            Some(self.get_self_ref())
        } else {
            match self.get_feature() {
                Some(feature) => feature.get_socket_by_id(socket_id),
                None => None,
            }
        }
    }

    fn is_compatible_with(&self, feature: &Rc<dyn FeatureControl>) -> bool;

    fn get_feature(&self) -> Option<Rc<dyn FeatureControl>>;

    fn generate_compatible_feature(
        &self,
        feature_request: &String,
    ) -> Option<Rc<dyn FeatureControl>>;
}
