use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;

pub struct TermDefSocket {
    id: String,
    term_seq: Option<String>,
    representation: Option<String>,
}

impl TermDefSocket {
    pub fn new(id: String) -> TermDefSocket {
        TermDefSocket {
            id,
            term_seq: None,
            representation: None,
        }
    }

    pub fn serialize(&self) -> TermDefSocketSer {
        TermDefSocketSer::new(
            self.id.clone(),
            self.term_seq.clone(),
            self.representation.clone(),
        )
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_term_seq(&self) -> Option<&String> {
        match &self.term_seq {
            None => None,
            Some(seq) => Some(seq),
        }
    }

    pub fn assign_term_seq(&mut self, term_seq: String) -> TermDefSocketSer {
        self.term_seq = Some(term_seq.clone());
        self.representation = Some(term_seq);

        self.serialize()
    }

    pub fn delete_term_seq(&mut self) -> TermDefSocketSer {
        self.term_seq = None;
        self.representation = None;

        self.serialize()
    }
}
