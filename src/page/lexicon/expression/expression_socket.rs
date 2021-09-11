use crate::page::lexicon::expression::expression_serialization::ExprSocketSer;
use crate::page::lexicon::expression::Expression;

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
