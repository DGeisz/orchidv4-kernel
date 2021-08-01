use crate::feature_tree::feature::feature_control::FeatureControl;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_tree_error::FeatureTreeError;
use crate::feature_tree::feature_type::FeatureType;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait SocketControl {
    fn get_id(&self) -> u128;

    fn get_hash(&self) -> Option<u128> {
        match self.get_feature() {
            Some(feature) => feature.get_hash(),
            None => None,
        }
    }

    fn get_self_ref(&self) -> Rc<dyn SocketControl>;

    fn get_self_weak(&self) -> WeakRef<dyn SocketControl>;

    fn set_binding(&self, binding: &Rc<dyn FeatureBindingControl>);

    fn release_binding(&self);

    fn detach(&self) -> Result<(), FeatureTreeError> {
        if let Some(feature) = self.get_feature() {
            if feature.any_refs_in_subtree() {
                self.release_binding();

                Ok(())
            } else {
                Err(FeatureTreeError::ChildFeatureInUse(self.get_id()))
            }
        } else {
            Err(FeatureTreeError::SocketAlreadyEmpty(self.get_id()))
        }
    }

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

    fn get_parent_feature(&self) -> Option<Rc<dyn FeatureControl>>;

    fn is_compatible_with(&self, feature: &Rc<dyn FeatureControl>) -> bool {
        match self.get_parent_feature() {
            Some(parent) => parent.is_feature_compatible_with_child_socket(feature, self.get_id()),
            None => false,
        }
    }

    fn get_feature(&self) -> Option<Rc<dyn FeatureControl>>;

    fn generate_compatible_feature(
        &self,
        feature_request: &String,
    ) -> Option<Rc<dyn FeatureControl>>;

    fn is_feature_compatible_with_type(
        &self,
        feature_hash: u128,
        feature_type: FeatureType,
        request_source: u128,
    ) -> bool {
        /*
        Basically, we just pass the request along in the direction
        it's flowing.  So if the request comes from parent, pass to
        child, and if it comes from child, pass to parent
        */
        if let (Some(parent), Some(child)) = (self.get_parent_feature(), self.get_feature()) {
            if parent.get_id() == request_source {
                return child.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }

            if child.get_id() == request_source {
                return parent.is_feature_compatible_with_type(
                    feature_hash,
                    feature_type,
                    self.get_id(),
                );
            }
        }

        false
    }

    fn any_refs_in_subtree(&self) -> bool;
}
