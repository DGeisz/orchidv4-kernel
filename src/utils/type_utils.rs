use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct HardRef<T> {
    hard_ref: RefCell<Option<Rc<T>>>,
}

impl<T> HardRef<T> {
    pub fn new() -> HardRef<T> {
        HardRef {
            hard_ref: RefCell::new(None),
        }
    }

    pub fn set_ref(&self, hard_ref: Rc<T>) {
        *self.hard_ref.borrow_mut() = Some(hard_ref);
    }

    pub fn strong_fetch(&self) -> Rc<T> {
        Rc::clone(&(&*self.hard_ref.borrow()).as_ref().unwrap())
    }

    pub fn weak_fetch(&self) -> Option<Rc<T>> {
        match (&*self.hard_ref.borrow()).as_ref() {
            Some(hard_ref) => Some(Rc::clone(hard_ref)),
            None => None,
        }
    }

    /// Indicates whether the ref has been set
    pub fn ref_set(&self) -> bool {
        match &*self.hard_ref.borrow() {
            Some(_) => true,
            None => false,
        }
    }
}

pub struct SoftRef<T> {
    soft_ref: RefCell<Option<Weak<T>>>,
}

impl<T> SoftRef<T> {
    pub fn new() -> SoftRef<T> {
        SoftRef {
            soft_ref: RefCell::new(None),
        }
    }

    pub fn set_ref(&self, soft_ref: &Rc<T>) {
        *self.soft_ref.borrow_mut() = Some(Rc::downgrade(soft_ref))
    }

    pub fn get_weak_ref(&self) -> WeakRef<T> {
        WeakRef {
            weak_ref: Weak::clone((&*self.soft_ref.borrow()).as_ref().unwrap()),
        }
    }

    pub fn strong_fetch(&self) -> Rc<T> {
        Weak::upgrade((&*self.soft_ref.borrow()).as_ref().unwrap()).unwrap()
    }

    pub fn weak_fetch(&self) -> Option<Rc<T>> {
        match (&*self.soft_ref.borrow()).as_ref() {
            Some(weak_ref) => Weak::upgrade(weak_ref),
            None => None,
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

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        WeakRef {
            weak_ref: Weak::clone(&self.weak_ref),
        }
    }
}
