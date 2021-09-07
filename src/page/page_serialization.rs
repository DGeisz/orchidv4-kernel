use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use serde::{Deserialize, Serialize};

/// This is the serialized output of a page
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PageSerialization {
    id: String,
    declarations: Vec<DecSocketSer>,
}

impl PageSerialization {
    pub fn new(id: String, declarations: Vec<DecSocketSer>) -> PageSerialization {
        PageSerialization { id, declarations }
    }
}
