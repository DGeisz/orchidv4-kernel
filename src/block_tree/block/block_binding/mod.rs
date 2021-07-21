use crate::block_tree::block::block_binding::block_binding_control::BlockBindingControl;
use crate::block_tree::block::bottom::BindingBottom;
use crate::block_tree::block::top::BindingTop;
use crate::utils::type_utils::WeakRef;
use std::rc::Rc;

pub mod block_binding_control;

pub struct BlockBinding {
    top: Rc<dyn BindingTop>,
    bottom: WeakRef<dyn BindingBottom>,
}

impl BlockBinding {
    pub fn bind(top: &Rc<dyn BindingTop>, bottom: &Rc<dyn BindingBottom>) {
        let binding: Rc<dyn BlockBindingControl> = Rc::new(BlockBinding {
            top: Rc::clone(top),
            bottom: bottom.get_self_weak(),
        });

        top.set_binding(Rc::clone(&binding));
        bottom.set_binding(binding);
    }
}

impl BlockBindingControl for BlockBinding {
    fn get_top_ref(&self) -> Rc<dyn BindingTop> {
        Rc::clone(&self.top)
    }

    fn get_bottom_ref(&self) -> Rc<dyn BindingBottom> {
        self.bottom.get()
    }
}
