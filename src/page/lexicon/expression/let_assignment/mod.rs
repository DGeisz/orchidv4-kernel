use crate::page::lexicon::expression::let_assignment::let_serialization::LetSer;

pub mod let_serialization;

pub struct LetAssignment;

impl LetAssignment {
    pub fn serialize(&self) -> LetSer {
        LetSer::new()
    }
}
