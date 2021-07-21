use crate::block_tree::block_tree_control::BlockTreeControl;
use crate::block_tree::block_tree_error::BlockTreeError;

mod block;
pub mod block_tree_control;
pub mod block_tree_error;

struct BlockTree {}

impl BlockTreeControl for BlockTree {
    fn create_and_append(&self, block_request: String, top_id: u128) -> Result<(), BlockTreeError> {
        /*
        First find the Top to which this request refers.
        Make sure the top isn't already bound
        */

        /*
        Next, determine if the block request corresponds to a predefined block
        */

        /*
        If not, ask the Top if it recognizes the sequence, and can produce
        a block that would be compatible
        */

        /*
        Now assume we have the new block.  Or a reference to the bottom
        of the new block. Bind the top and the bottom
        */

        /*
        Return a response indicating a block with the given serialization
        was added to a top with the given id
        */
        unimplemented!()
    }
}
