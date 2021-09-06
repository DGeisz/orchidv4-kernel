use crate::page::lexicon::declaration::constant::Constant;
use crate::page::lexicon::declaration::definition::Definition;

pub mod constant;
pub mod declaration_serialization;
pub mod definition;

pub enum Declaration {
    Const(Constant),
    Def(Definition),
}
