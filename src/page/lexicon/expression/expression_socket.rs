use crate::page::lexicon::expression::expression_serialization::ExprSocketSer;
use crate::page::lexicon::expression::Expression;
use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::page::scoped_entity::ScopedEntity;

pub struct ExprSocket {
    id: String,
    expr: Option<Expression>,
}

impl ExprSocket {
    pub fn new(id: String) -> ExprSocket {
        ExprSocket { id, expr: None }
    }

    pub fn serialize(&self) -> ExprSocketSer {
        ExprSocketSer::new(
            self.id.clone(),
            match &self.expr {
                None => None,
                Some(expr) => Some(expr.serialize()),
            },
        )
    }
}

impl ScopedEntity for ExprSocket {
    fn get_tds(&mut self, tds_id: &String) -> Option<&mut TermDefSocket> {
        None
    }
}
