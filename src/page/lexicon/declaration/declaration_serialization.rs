use crate::page::lexicon::declaration::constant::constant_serialization::ConstSer;
use crate::page::lexicon::declaration::definition::definition_serialization::DefSer;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DecSocketSer {
    id: String,
    dec_ser: Option<DecSer>,
}

impl DecSocketSer {
    pub fn new(id: String, dec_ser: Option<DecSer>) -> DecSocketSer {
        DecSocketSer { id, dec_ser }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum DecSer {
    Const(ConstSer),
    Def(DefSer),
}
