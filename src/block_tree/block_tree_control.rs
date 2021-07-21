use crate::block_tree::block_tree_error::BlockTreeError;

pub trait BlockTreeControl {
    fn create_and_append(&self, block_request: String, top_id: u128) -> Result<(), BlockTreeError>;
}
