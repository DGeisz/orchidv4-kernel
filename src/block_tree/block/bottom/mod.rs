use crate::block_tree::block::block_binding::block_binding_control::BlockBindingControl;
use crate::block_tree::block::top::BindingTop;
use crate::block_tree::block_tree_error::BlockTreeError;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub trait BindingBottom {
    fn get_id(&self) -> u128;

    fn get_self_ref(&self) -> Rc<dyn BindingBottom>;

    fn get_self_weak(&self) -> WeakRef<dyn BindingBottom>;

    fn set_binding(&self, binding: Rc<dyn BlockBindingControl>);

    fn detach(&self) -> Result<(), BlockTreeError>;

    fn get_bottom_by_id(&self, bottom_id: u128) -> Option<Rc<dyn BindingBottom>> {
        if self.get_id() == bottom_id {
            Some(self.get_self_ref())
        } else {
            match self.get_bound_top() {
                Some(top) => top.get_bottom_by_id(bottom_id),
                None => None,
            }
        }
    }

    fn is_compatible_with(&self, top: &Rc<dyn BindingTop>) -> bool;

    fn get_bound_top(&self) -> Option<Rc<dyn BindingTop>>;

    fn generate_compatible_block(&self, block_request: &String) -> Option<Rc<dyn BindingTop>>;
}
