use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use serde::{Deserialize, Serialize};

/// This is the serialized output of a page
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PageSerialization {
    id: String,
    dec_sockets: Vec<DecSocketSer>,
}

impl PageSerialization {
    pub fn new(id: String, dec_sockets: Vec<DecSocketSer>) -> PageSerialization {
        PageSerialization { id, dec_sockets }
    }
}
