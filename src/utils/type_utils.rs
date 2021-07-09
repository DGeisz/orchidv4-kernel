use std::cell::RefCell;

/// I use a lot of RefCell<Option<>> type
/// Structs so I'm creating a type alias for it
pub type Relltion<T> = RefCell<Option<T>>;
