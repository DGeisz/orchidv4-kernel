use crate::block_tree::block::block_request::BlockRequest;
use crate::block_tree::block::bottom::BindingBottom;
use crate::block_tree::block::top::BindingTop;
use std::rc::Rc;

pub trait BlockGeneratorControl {
    fn generate_block(
        &self,
        block_request: &BlockRequest,
        bottom: &Rc<dyn BindingBottom>,
    ) -> Option<Rc<dyn BindingTop>>;
}
