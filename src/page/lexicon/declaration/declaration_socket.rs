use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::declaration::{BasicDec, Declaration};
use uuid::Uuid;

pub struct DecSocket {
    id: String,
    dec: Option<Declaration>,
}

impl DecSocket {
    pub fn new(id: String) -> DecSocket {
        DecSocket { id, dec: None }
    }

    pub fn serialize(&self) -> DecSocketSer {
        DecSocketSer::new(
            self.id.clone(),
            match &self.dec {
                None => None,
                Some(dec) => Some(dec.serialize()),
            },
        )
    }
}
