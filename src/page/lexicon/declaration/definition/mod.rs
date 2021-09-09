use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::BasicDec;

pub mod definition_serialization;

pub enum DefVariation {
    Definition,
    Theorem,
    Lemma,
}

pub struct Definition {
    variation: DefVariation,
}

impl Definition {
    pub fn new(variation: DefVariation) -> Definition {
        Definition { variation }
    }
}

impl BasicDec for Definition {
    fn serialize(&self) -> DecSer {
        DecSer::Def(DefSer::new())
    }
}
