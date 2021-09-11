use crate::page::lexicon::expression::block::block_serialization::BlockSer;
use crate::page::lexicon::expression::lambda_abstraction::lambda_serialization::LamSer;
use crate::page::lexicon::expression::let_assignment::let_serialization::LetSer;
use crate::page::lexicon::expression::pi_type::pi_serialization::PiSer;
use crate::page::lexicon::expression::sort::sort_serialization::SortSer;
use crate::page::lexicon::expression::variable::variable_serialization::VarSer;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ExprSer {
    Block(BlockSer),
    Lam(LamSer),
    Let(LetSer),
    Pi(PiSer),
    Sort(SortSer),
    Var(VarSer),
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ExprSocketSer {
    id: String,
    expr_ser: Option<ExprSer>,
}

impl ExprSocketSer {
    pub fn new(id: String, expr_ser: Option<ExprSer>) -> ExprSocketSer {
        ExprSocketSer { id, expr_ser }
    }
}
