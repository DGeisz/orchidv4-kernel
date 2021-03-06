use crate::page::lexicon::declaration::constant::constant_serialization::ConstSer;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::BasicDec;
use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::page::lexicon::term_def::TermDef;
use crate::page::scoped_entity::ScopedEntity;
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

impl ScopedEntity for Constant {
    fn get_term_def(&mut self) -> Option<&mut TermDef> {
        Some(&mut self.term_def)
    }

    fn get_tds(&mut self, tds_id: &String) -> Option<&mut TermDefSocket> {
        let tds = self.term_def.get_mut_def_socket();

        if tds.get_id() == tds_id {
            Some(tds)
        } else {
            None
        }
    }
}
