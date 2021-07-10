use crate::page::feature_tree::feature::feature_control_port::RcFeatureControl;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::RcFeatureSocketControl;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use log::error;
use std::collections::HashMap;
use std::rc::Rc;

pub mod feature;
pub mod feature_socket;
pub mod feature_tree_control_port;
pub mod feature_tree_error;
pub mod feature_tree_generator;

#[cfg(test)]
mod tests;

pub struct FeatureTree {
    base_socket_control: RcFeatureSocketControl,
    socket_reference: HashMap<String, RcFeatureSocketControl>,
    feature_reference: HashMap<String, RcFeatureControl>,
}

impl FeatureTree {
    pub fn new(base_socket_control: RcFeatureSocketControl) -> Box<dyn FeatureTreeControlPort> {
        let mut socket_reference = HashMap::new();

        socket_reference.insert(
            base_socket_control.get_id(),
            Rc::clone(&base_socket_control),
        );

        Box::new(FeatureTree {
            base_socket_control,
            socket_reference,
            feature_reference: HashMap::new(),
        })
    }
}

impl FeatureTreeControlPort for FeatureTree {
    fn serialize(&self) -> FeatureSocketSerialization {
        self.base_socket_control.serialize()
    }

    fn create_feature(
        &mut self,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<(), FeatureTreeError> {
        match self.socket_reference.get(&socket_id) {
            Some(socket) => match socket.create_feature(feature) {
                Ok(feature) => {
                    self.feature_reference
                        .insert(feature.get_id(), Rc::clone(&feature));

                    for socket in feature.get_child_sockets() {
                        self.socket_reference.insert(socket.get_id(), socket);
                    }

                    Ok(())
                }
                Err(e) => Err(e),
            },
            None => {
                error!("Tried to fetch non existing socket with id: {}", socket_id);

                Err(FeatureTreeError::SocketDoesNotExist(socket_id))
            }
        }
    }
}
