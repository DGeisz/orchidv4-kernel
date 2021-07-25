use crate::block_tree::block::block_request::BlockRequest;
use crate::block_tree::block_tree_diff::BlockTreeDiff;
use crate::block_tree::block_tree_error::BlockTreeError;

pub trait BlockTreeControl {
    fn create_and_append(
        &self,
        block_request: BlockRequest,
        bottom_id: u128,
    ) -> Result<BlockTreeDiff, BlockTreeError>;

    /// The last argument indicates where we
    /// want the displaced block tree to go
    /// relative to the new block that's created
    fn create_and_insert(
        &self,
        block_request: BlockRequest,
        bottom_id: u128,
        rebind_to_first: bool,
    ) -> Result<BlockTreeDiff, BlockTreeError>;

    fn detach_and_delete(&self, bottom_id: u128) -> Result<BlockTreeDiff, BlockTreeError>;

    fn remove_and_rebind(
        &self,
        bottom_id: u128,
        rebind_to_first: bool,
    ) -> Result<BlockTreeDiff, BlockTreeError>;
}
