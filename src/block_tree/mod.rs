use crate::block_tree::block::block_binding::BlockBinding;
use crate::block_tree::block::block_generator::block_generator_control::BlockGeneratorControl;
use crate::block_tree::block::block_request::BlockRequest;
use crate::block_tree::block::bottom::BindingBottom;
use crate::block_tree::block::top::BindingTop;
use crate::block_tree::block_tree_control::BlockTreeControl;
use crate::block_tree::block_tree_diff::BlockTreeDiff;
use crate::block_tree::block_tree_error::BlockTreeError;
use std::rc::Rc;

mod block;
pub mod block_tree_control;
pub mod block_tree_diff;
pub mod block_tree_error;

struct BlockTree {
    base_bottom: Rc<dyn BindingBottom>,
    block_generator: Box<dyn BlockGeneratorControl>,
}

impl BlockTreeControl for BlockTree {
    fn create_and_append(
        &self,
        block_request: BlockRequest,
        bottom_id: u128,
    ) -> Result<BlockTreeDiff, BlockTreeError> {
        /*
        First find the Bottom to which this request refers.
        Make sure the bottom isn't already bound
        */
        let binding_bottom =
            if let Some(binding_bottom) = self.base_bottom.get_bottom_by_id(bottom_id) {
                binding_bottom
            } else {
                return Err(BlockTreeError::BottomNotFound(bottom_id));
            };

        /*
        Make sure the bottom isn't already filled
        */
        if let Some(_) = binding_bottom.get_bound_top() {
            return Err(BlockTreeError::BottomAlreadyBound(bottom_id));
        }

        /*
        Next, attempt to use the block generator and the current bottom
        to create a compatible block for this space
        */
        let block_binding_top = match self
            .block_generator
            .generate_block(&block_request, &binding_bottom)
        {
            Some(top) => top,
            None => {
                /*
                If the block generator didn't find anything, then the request
                isn't compatible
                */
                return Err(BlockTreeError::RequestNotCompatible(
                    block_request,
                    bottom_id,
                ));
            }
        };

        /*
        Now assume we have the new block.  Or a reference to the bottom
        of the new block. Bind the top and the bottom
        */
        BlockBinding::bind(&block_binding_top, &binding_bottom);

        /*
        Return a response indicating a block with the given serialization
        was added to a bottom with the given id
        */
        Ok(BlockTreeDiff::Append(
            bottom_id,
            block_binding_top.serialize(),
        ))
    }

    fn create_and_insert(
        &self,
        block_request: BlockRequest,
        bottom_id: u128,
        rebind_to_first: bool,
    ) -> Result<BlockTreeDiff, BlockTreeError> {
        /*
        First find the Bottom to which this request refers.
        Make sure the bottom is already bound
        */
        let old_binding_bottom =
            if let Some(binding_bottom) = self.base_bottom.get_bottom_by_id(bottom_id) {
                binding_bottom
            } else {
                return Err(BlockTreeError::BottomNotFound(bottom_id));
            };

        let old_binding_top = if let Some(top) = old_binding_bottom.get_bound_top() {
            top
        } else {
            return Err(BlockTreeError::BottomAlreadyDetached(bottom_id));
        };

        /*
        Next, determine if the block request corresponds to a predefined block
        */
        let new_binding_top = match self
            .block_generator
            .generate_block(&block_request, &old_binding_bottom)
        {
            Some(top) => top,
            None => {
                /*
                If the block generator didn't find anything, then the request
                isn't compatible
                */
                return Err(BlockTreeError::RequestNotCompatible(
                    block_request,
                    bottom_id,
                ));
            }
        };

        /*
        Now attempt to find the bottom that's on top of this structure
        */
        let new_binding_bottom = if rebind_to_first {
            if let Some(bottom) = new_binding_top.first_unbound_bottom() {
                bottom
            } else {
                return Err(BlockTreeError::NewBlockDoesNotHaveDetachedBottoms(
                    block_request,
                    bottom_id,
                ));
            }
        } else {
            if let Some(bottom) = new_binding_top.last_unbound_bottom() {
                bottom
            } else {
                return Err(BlockTreeError::NewBlockDoesNotHaveDetachedBottoms(
                    block_request,
                    bottom_id,
                ));
            }
        };

        /*
        Ok, first check if the new block and the stump block are compatible
        */
        if !old_binding_bottom.is_compatible_with(&new_binding_top) {
            return Err(BlockTreeError::RequestNotCompatible(
                block_request,
                bottom_id,
            ));
        }

        /*
        If so, temporarily bind new block and stump
        */
        BlockBinding::bind(&new_binding_top, &old_binding_bottom);

        /*
        Now see if branch and new block are compatible
        */
        if !new_binding_bottom.is_compatible_with(&old_binding_top) {
            /*
            If not, rebind all the old pair, and indicate
            there was an error
            */
            BlockBinding::bind(&old_binding_top, &old_binding_bottom);

            return Err(BlockTreeError::RequestNotCompatible(
                block_request,
                bottom_id,
            ));
        }

        /*
        If so, bind them
        */
        BlockBinding::bind(&old_binding_top, &new_binding_bottom);

        Ok(BlockTreeDiff::Replace(
            bottom_id,
            new_binding_top.serialize(),
        ))
    }

    fn detach_and_delete(&self, bottom_id: u128) -> Result<BlockTreeDiff, BlockTreeError> {
        /*
        First find the Bottom to which this request refers.
        Make sure the bottom isn't already bound
        */
        let binding_bottom =
            if let Some(binding_bottom) = self.base_bottom.get_bottom_by_id(bottom_id) {
                binding_bottom
            } else {
                return Err(BlockTreeError::BottomNotFound(bottom_id));
            };

        /*
        Attempt to detach
        */
        if let Err(e) = binding_bottom.detach() {
            return Err(e);
        }

        /*
        Otherwise, indicate that the binding
        was detached
        */
        Ok(BlockTreeDiff::Detach(bottom_id))
    }

    fn remove_and_rebind(
        &self,
        bottom_id: u128,
        rebind_to_first: bool,
    ) -> Result<BlockTreeDiff, BlockTreeError> {
        /*
        First find the Bottom to which this request refers.
        Make sure the bottom isn't already bound
        */
        let lower_bottom = if let Some(lower_bottom) = self.base_bottom.get_bottom_by_id(bottom_id)
        {
            lower_bottom
        } else {
            return Err(BlockTreeError::BottomNotFound(bottom_id));
        };

        /*
        Get the top to which the bottom corresponds
        */
        let lower_top = if let Some(top) = lower_bottom.get_bound_top() {
            top
        } else {
            return Err(BlockTreeError::BottomAlreadyDetached(bottom_id));
        };

        /*
        Get the bottom whose top we're attempting bind to
        */
        let upper_bottom_option = if rebind_to_first {
            lower_top.first_unbound_bottom()
        } else {
            lower_top.last_unbound_bottom()
        };

        let upper_bottom = if let Some(bottom) = upper_bottom_option {
            bottom
        } else {
            return Err(BlockTreeError::NoUpperBlockFoundForRebind);
        };

        /*
        Now attempt to get the top to which upper_bottom is presumably bound to
        */
        let upper_top = if let Some(top) = upper_bottom.get_bound_top() {
            top
        } else {
            return Err(BlockTreeError::NoUpperBlockFoundForRebind);
        };

        /*
        See if lower bottom and upper top are compatible.  If not, we can't do the rebind.
        */
        if !lower_bottom.is_compatible_with(&upper_top) {
            return Err(BlockTreeError::RebindIncompatible);
        }

        /*
        Otherwise, bind lower bottom with upper top, and return the replacement
        */
        BlockBinding::bind(&upper_top, &lower_bottom);

        Ok(BlockTreeDiff::Rebind(bottom_id, upper_top.get_id()))
    }
}
