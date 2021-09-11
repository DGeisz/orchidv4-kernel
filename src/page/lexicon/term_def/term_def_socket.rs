use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;

pub struct TermDefSocket {
    id: String,
}

impl TermDefSocket {
    pub fn new(id: String) -> TermDefSocket {
        TermDefSocket { id }
    }

    pub fn serialize(&self) -> TermDefSocketSer {
        TermDefSocketSer::new(self.id.clone())
    }
}
