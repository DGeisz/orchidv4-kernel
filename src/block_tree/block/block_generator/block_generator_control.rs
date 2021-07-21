use crate::block_tree::block::top::BindingTop;
use std::rc::Rc;

pub trait BlockGeneratorControl {
    fn generate_block(&self, block_request: &String) -> Option<Rc<dyn BindingTop>>;
}
