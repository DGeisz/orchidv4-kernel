use crate::page::lexicon::declaration::constant::constant_serialization::ConstSer;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::BasicDec;
use crate::page::lexicon::term_def::TermDef;
use crate::utils::id_generator::IdGenControl;
use serde::{Deserialize, Serialize};

pub mod constant_serialization;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ConstVariation {
    Constant,
    Axiom,
}

pub struct Constant {
    variation: ConstVariation,
    term_def: TermDef,
}

impl Constant {
    pub fn new(variation: ConstVariation, id_generator: &Box<dyn IdGenControl>) -> Constant {
        Constant {
            variation,
            term_def: TermDef::new(id_generator),
        }
    }
}

impl BasicDec for Constant {
    fn serialize(&self) -> DecSer {
        DecSer::Const(ConstSer::new(
            self.variation.clone(),
            self.term_def.serialize(),
        ))
    }
}
