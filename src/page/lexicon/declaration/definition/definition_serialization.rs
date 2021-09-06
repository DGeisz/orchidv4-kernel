use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DefinitionSerialization {}

impl DefinitionSerialization {
    pub fn new() -> DefinitionSerialization {
        DefinitionSerialization {}
    }
}
