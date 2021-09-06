use crate::page::lexicon::declaration::constant::constant_serialization::ConstantSerialization;
use crate::page::lexicon::declaration::declaration_serialization::DeclarationSerialization;
use crate::page::lexicon::declaration::BasicDec;

pub mod constant_serialization;

pub struct Constant;

impl BasicDec for Constant {
    fn serialize(&self) -> DeclarationSerialization {
        DeclarationSerialization::Const(ConstantSerialization::new())
    }
}
