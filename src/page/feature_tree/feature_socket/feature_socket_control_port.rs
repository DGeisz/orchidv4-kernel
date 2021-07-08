use std::rc::Rc;

/// Type for a reference count feature socket
/// trait object
pub type RcFeatureSocketControl = Rc<Box<dyn FeatureSocketControlPort>>;

pub trait FeatureSocketControlPort {}
