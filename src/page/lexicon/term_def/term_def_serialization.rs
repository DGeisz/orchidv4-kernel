use crate::page::lexicon::expression::expression_serialization::ExprSocketSer;
use crate::page::lexicon::term_def::TermDef;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct TermDefSocketSer {
    id: String,
}

impl TermDefSocketSer {
    pub fn new(id: String) -> TermDefSocketSer {
        TermDefSocketSer { id }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct TermDefSer {
    term_def_socket_ser: TermDefSocketSer,
    type_socket_ser: ExprSocketSer,
}

impl TermDefSer {
    pub fn new(
        term_def_socket_ser: TermDefSocketSer,
        type_socket_ser: ExprSocketSer,
    ) -> TermDefSer {
        TermDefSer {
            term_def_socket_ser,
            type_socket_ser,
        }
    }
}
