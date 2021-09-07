use crate::page::lexicon::declaration::constant::constant_serialization::ConstSer;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::BasicDec;

pub mod constant_serialization;

pub struct Constant;

impl BasicDec for Constant {
    fn serialize(&self) -> DecSer {
        DecSer::Const(ConstSer::new())
    }
}
