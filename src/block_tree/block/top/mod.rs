use crate::block_tree::block::block_binding::block_binding_control::BlockBindingControl;
use crate::block_tree::block::block_serialization::BlockSerialization;
use crate::block_tree::block::bottom::BindingBottom;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait BindingTop {
    fn get_id(&self) -> u128;

    fn get_self_ref(&self) -> Rc<dyn BindingTop>;

    fn get_self_weak(&self) -> WeakRef<dyn BindingTop>;

    fn set_binding(&self, binding: Rc<dyn BlockBindingControl>);

    fn get_bottom_by_id(&self, _bottom_id: u128) -> Option<Rc<dyn BindingBottom>>;

    fn get_bound_bottom(&self) -> Option<Rc<dyn BindingBottom>>;

    fn serialize(&self) -> BlockSerialization;

    fn can_detach(&self) -> bool;

    fn first_unbound_bottom(&self) -> Option<Rc<dyn BindingBottom>>;

    fn last_unbound_bottom(&self) -> Option<Rc<dyn BindingBottom>>;
}
