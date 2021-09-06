use serde::{Deserialize, Serialize};

/// This is the serialized output of a page
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PageSerialization {
    id: String,
}

impl PageSerialization {
    pub fn new(id: String) -> PageSerialization {
        PageSerialization { id }
    }
}
