use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::BasicDec;
use crate::page::lexicon::expression::expression_socket::ExprSocket;
use crate::page::lexicon::term_def::TermDef;
use crate::utils::id_generator::IdGenControl;

pub mod definition_serialization;

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
        DecSer::Def(DefSer::new())
    }
}
