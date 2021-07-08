//! Port for generating feature sockets

use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use std::rc::Rc;

pub type RcFeatureSocketGenerator = Rc<Box<dyn FeatureSocketGeneratorPort>>;

pub trait FeatureSocketGeneratorPort {
    /// This is a bit of a shifty method, but it takes
    /// a special reference to a generator so that
    /// the new feature socket can pass along the ability
    /// to create feature sockets
    fn generate_feature_socket(&self) -> RcFeatureSocketControl;

    /// A proper generator requires a reference to itself to
    /// work properly, so this method gives the generator the
    /// chance to properly initialize before being used
    fn init(&self, self_rc: RcFeatureSocketGenerator);
}
