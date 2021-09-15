use crate::page::lexicon::declaration::constant::Constant;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::definition::Definition;
use crate::page::lexicon::term_def::TermDef;
use crate::page::scoped_entity::{Scope, ScopedEntity};

pub mod constant;
pub mod dec_names;
pub mod declaration_serialization;
pub mod declaration_socket;
pub mod definition;

pub enum Declaration {
    Const(Constant),
    Def(Definition),
}

impl BasicDec for Declaration {
    fn serialize(&self) -> DecSer {
        match self {
            Declaration::Const(constant) => constant.serialize(),
            Declaration::Def(definition) => definition.serialize(),
        }
    }
}

pub trait BasicDec {
    fn serialize(&self) -> DecSer;
}

impl ScopedEntity for Declaration {
    fn get_term_def_with_scope(&mut self, tds_id: &String) -> Option<(&mut TermDef, Scope)> {
        match self {
            Declaration::Const(constant) => constant.get_term_def_with_scope(tds_id),
            Declaration::Def(def) => def.get_term_def_with_scope(tds_id),
        }
    }

    fn get_term_def(&mut self) -> Option<&mut TermDef> {
        match self {
            Declaration::Const(constant) => constant.get_term_def(),
            Declaration::Def(def) => def.get_term_def(),
        }
    }
}
