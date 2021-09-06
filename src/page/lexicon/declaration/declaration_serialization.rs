use crate::page::lexicon::declaration::constant::constant_serialization::ConstantSerialization;
use crate::page::lexicon::declaration::definition::definition_serialization::DefinitionSerialization;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum DeclarationSerialization {
    Const(ConstantSerialization),
    Def(DefinitionSerialization),
}
