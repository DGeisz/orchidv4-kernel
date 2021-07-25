use crate::block_tree::block::block_request::BlockRequest;

pub enum BlockTreeError {
    BottomNotFound(u128),
    BadBlockRequest(BlockRequest),
    RequestNotCompatible(BlockRequest, u128),
    BottomAlreadyBound(u128),
    BottomAlreadyDetached(u128),
    BottomCannotDetach(u128),
    NewBlockDoesNotHaveDetachedBottoms(BlockRequest, u128),
    NoUpperBlockFoundForRebind,
    RebindIncompatible,
}
