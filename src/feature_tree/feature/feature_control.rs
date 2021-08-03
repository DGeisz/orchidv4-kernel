use crate::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::feature_tree::feature_binding::feature_binding_control::FeatureBindingControl;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::feature_tree::feature_type::FeatureType;
use crate::feature_tree::feature_utils::feature_subtree_reference_record::FeatureSubtreeRefRecord;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait FeatureControl {
    fn get_id(&self) -> u128;

    fn get_hash(&self) -> Option<u128>;

    fn get_self_ref(&self) -> Rc<dyn FeatureControl>;

    fn get_self_weak(&self) -> WeakRef<dyn FeatureControl>;

    fn set_binding(&self, binding: &Rc<dyn FeatureBindingControl>);

    fn get_socket_by_id(&self, socket_id: u128) -> Option<Rc<dyn SocketControl>>;

    fn get_parent_socket(&self) -> Option<Rc<dyn SocketControl>>;

    fn serialize(&self) -> FeatureSerialization;

    fn first_unbound_socket(&self) -> Option<Rc<dyn SocketControl>>;

    fn last_unbound_socket(&self) -> Option<Rc<dyn SocketControl>>;

    /// This checks if a child feature is compatible
    /// with one of this feature's child sockets with
    /// the given id
    fn is_feature_compatible_with_child_socket(
        &self,
        feature: &Rc<dyn FeatureControl>,
        socket_id: u128,
    ) -> bool;

    fn is_compatible_with_type(&self, feature_type: FeatureType) -> bool;

    fn is_feature_compatible_with_type(
        &self,
        feature_hash: u128,
        feature_type: FeatureType,
        request_source: u128,
    ) -> bool;

    fn inc_ref_count(&self);
    fn dec_ref_count(&self);
    fn get_ref_count(&self) -> u32;

    fn get_subtree_ref_record(&self) -> FeatureSubtreeRefRecord;

    fn any_external_subtree_dependents(&self) -> bool {
        self.get_subtree_ref_record().any_external_dependents()
    }
}
