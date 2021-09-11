use crate::page::lexicon::expression::expression_socket::ExprSocket;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSer;
use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::utils::id_generator::IdGenControl;

pub mod term_def_serialization;
pub mod term_def_socket;

pub struct TermDef {
    term_def_socket: TermDefSocket,
    type_socket: ExprSocket,
}

impl TermDef {
    pub fn new(id_generator: &Box<dyn IdGenControl>) -> TermDef {
        TermDef {
            term_def_socket: TermDefSocket::new(id_generator.gen_id()),
            type_socket: ExprSocket::new(id_generator.gen_id()),
        }
    }

    pub fn serialize(&self) -> TermDefSer {
        TermDefSer::new(
            self.term_def_socket.serialize(),
            self.type_socket.serialize(),
        )
    }
}
