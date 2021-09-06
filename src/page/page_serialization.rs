use crate::page::lexicon::declaration::declaration_serialization::DeclarationSerialization;
use serde::{Deserialize, Serialize};

/// This is the serialized output of a page
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PageSerialization {
    id: String,
    declarations: Vec<Option<DeclarationSerialization>>,
}

impl PageSerialization {
    pub fn new(
        id: String,
        declarations: Vec<Option<DeclarationSerialization>>,
    ) -> PageSerialization {
        PageSerialization { id, declarations }
    }
}
