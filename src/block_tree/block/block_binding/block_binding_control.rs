use crate::block_tree::block::bottom::BindingBottom;
use crate::block_tree::block::top::BindingTop;
use std::rc::Rc;

pub trait BlockBindingControl {
    fn get_top_ref(&self) -> Rc<dyn BindingTop>;
    fn get_bottom_ref(&self) -> Rc<dyn BindingBottom>;
}
