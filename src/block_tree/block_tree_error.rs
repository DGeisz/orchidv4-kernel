pub enum BlockTreeError {
    BottomNotFound(u128),
    BadBlockRequest(String),
    RequestNotCompatible(String, u128),
    BottomAlreadyBound(u128),
    BottomAlreadyDetached(u128),
    BottomCannotDetach(u128),
    NewBlockDoesNotHaveDetachedBottoms(String, u128),
}
