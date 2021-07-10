use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// I use a lot of RefCell<Option<>> type
/// Structs so I'm creating a type alias for it
pub type Relltion<T> = RefCell<Option<T>>;

pub struct SelfRef<T> {
    self_ref: RefCell<Option<Weak<T>>>,
}

impl<T> SelfRef<T> {
    pub fn new() -> SelfRef<T> {
        SelfRef {
            self_ref: RefCell::new(None),
        }
    }

    pub fn set_ref(&self, self_ref: &Rc<T>) {
        *self.self_ref.borrow_mut() = Some(Rc::downgrade(self_ref))
    }

    pub fn get_weak_ref(&self) -> WeakRef<T> {
        WeakRef {
            weak_ref: Weak::clone((&*self.self_ref.borrow()).as_ref().unwrap()),
        }
    }
}

pub struct WeakRef<T> {
    weak_ref: Weak<T>,
}

impl<T> WeakRef<T> {
    pub fn get(&self) -> Rc<T> {
        Weak::upgrade(&self.weak_ref).unwrap()
    }
}
