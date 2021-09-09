use crate::page::lexicon::declaration::constant::Constant;
use crate::page::lexicon::declaration::declaration_serialization::DecSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use crate::page::lexicon::declaration::definition::Definition;

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
