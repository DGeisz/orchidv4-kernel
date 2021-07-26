use crate::block_tree::block::block_binding::block_binding_control::BlockBindingControl;
use crate::block_tree::block::bottom::BindingBottom;
use crate::block_tree::block::top::BindingTop;
use crate::block_tree::block_tree_error::BlockTreeError;
use crate::utils::type_utils::{SoftRef, WeakRef};
use std::cell::RefCell;
use std::rc::Rc;

struct CpBottom {
    id: u128,
    self_ref: SoftRef<dyn BindingBottom>,
    binding: RefCell<Option<Rc<dyn BlockBindingControl>>>,
}

impl BindingBottom for CpBottom {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_self_ref(&self) -> Rc<dyn BindingBottom> {
        self.self_ref.strong_fetch()
    }

    fn get_self_weak(&self) -> WeakRef<dyn BindingBottom> {
        self.self_ref.get_weak_ref()
    }

    fn set_binding(&self, binding: Rc<dyn BlockBindingControl>) {
        *self.binding.borrow_mut() = Some(binding);
    }

    fn detach(&self) -> Result<(), BlockTreeError> {
        unimplemented!()
    }

    fn is_compatible_with(&self, top: &Rc<dyn BindingTop>) -> bool {
        unimplemented!()
    }

    fn get_bound_top(&self) -> Option<Rc<dyn BindingTop>> {
        match &*self.binding.borrow() {
            None => None,
            Some(binding) => Some(binding.get_top_ref()),
        }
    }

    fn generate_compatible_block(&self, block_request: &String) -> Option<Rc<dyn BindingTop>> {
        unimplemented!()
    }
}
