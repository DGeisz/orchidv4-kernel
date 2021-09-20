use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::page::lexicon::term_def::TermDef;

pub type Scope<'a> = Vec<&'a TermDef>;

pub trait ScopedEntity {
    fn get_term_def_with_scope(&mut self, _tds_id: &String) -> Option<(&mut TermDef, Scope)> {
        None
    }

    fn get_term_def(&mut self) -> Option<&mut TermDef> {
        None
    }

    fn get_tds(&mut self, _tds_id: &String) -> Option<&mut TermDefSocket>;
}
