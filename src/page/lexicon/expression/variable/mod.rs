use crate::page::lexicon::expression::variable::variable_serialization::VarSer;

pub mod variable_serialization;

pub struct Variable;

impl Variable {
    pub fn serialize(&self) -> VarSer {
        VarSer::new()
    }
}
