use crate::page::lexicon::declaration::constant::constant_serialization::ConstSer;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::BasicDec;
use serde::{Deserialize, Serialize};

pub mod constant_serialization;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ConstVariation {
    Constant,
    Axiom,
}

pub struct Constant {
    variation: ConstVariation,
}

impl Constant {
    pub fn new(variation: ConstVariation) -> Constant {
        Constant { variation }
    }
}

impl BasicDec for Constant {
    fn serialize(&self) -> DecSer {
        DecSer::Const(ConstSer::new(self.variation.clone()))
    }
}
