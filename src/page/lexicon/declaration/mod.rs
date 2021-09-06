use crate::page::lexicon::declaration::constant::Constant;
use crate::page::lexicon::declaration::declaration_serialization::DeclarationSerialization;
use crate::page::lexicon::declaration::definition::definition_serialization::DefinitionSerialization;
use crate::page::lexicon::declaration::definition::Definition;

pub mod constant;
pub mod declaration_serialization;
pub mod definition;

pub enum Declaration {
    Const(Constant),
    Def(Definition),
}

impl BasicDec for Declaration {
    fn serialize(&self) -> DeclarationSerialization {
        DeclarationSerialization::Def(DefinitionSerialization::new())
    }
}

pub trait BasicDec {
    fn serialize(&self) -> DeclarationSerialization;
}
