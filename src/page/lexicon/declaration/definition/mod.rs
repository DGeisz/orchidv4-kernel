use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::BasicDec;

pub mod definition_serialization;

pub struct Definition;

impl BasicDec for Definition {
    fn serialize(&self) -> DecSer {
        DecSer::Def(DefSer::new())
    }
}
