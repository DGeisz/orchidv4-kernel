use serde::{Deserialize, Serialize};

/// This is the serialized output of a page
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PageSerialization {
    id: String,
}

impl PageSerialization {
    pub fn new(page_id: u128) -> PageSerialization {
        PageSerialization {
            id: format!("{:X}", page_id),
        }
    }
}
