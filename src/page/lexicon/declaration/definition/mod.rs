use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::BasicDec;
use crate::page::lexicon::expression::expression_socket::ExprSocket;
use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::page::lexicon::term_def::TermDef;
use crate::page::scoped_entity::{Scope, ScopedEntity};
use crate::utils::id_generator::IdGenControl;
use serde::{Deserialize, Serialize};

pub mod definition_serialization;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum DefVariation {
    Definition,
    Theorem,
    Lemma,
}

pub struct Definition {
    variation: DefVariation,
    term_def: TermDef,
    term_expr: ExprSocket,
}

impl Definition {
    pub fn new(variation: DefVariation, id_generator: &Box<dyn IdGenControl>) -> Definition {
        Definition {
            variation,
            term_def: TermDef::new(id_generator),
            term_expr: ExprSocket::new(id_generator.gen_id()),
        }
    }
}

impl BasicDec for Definition {
    fn serialize(&self) -> DecSer {
        DecSer::Def(DefSer::new(
            self.variation.clone(),
            self.term_def.serialize(),
            self.term_expr.serialize(),
        ))
    }
}

impl ScopedEntity for Definition {
    fn get_term_def_with_scope(&mut self, tds_id: &String) -> Option<(&mut TermDef, Scope)> {
        self.term_expr.get_term_def_with_scope(tds_id)
    }

    fn get_term_def(&mut self) -> Option<&mut TermDef> {
        Some(&mut self.term_def)
    }
}
