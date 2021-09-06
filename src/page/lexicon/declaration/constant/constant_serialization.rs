use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstantSerialization {}

impl ConstantSerialization {
    pub fn new() -> ConstantSerialization {
        ConstantSerialization {}
    }
}
