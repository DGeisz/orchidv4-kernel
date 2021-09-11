use crate::page::lexicon::expression::block::Block;
use crate::page::lexicon::expression::expression_serialization::ExprSer;
use crate::page::lexicon::expression::lambda_abstraction::LambdaAbstraction;
use crate::page::lexicon::expression::let_assignment::LetAssignment;
use crate::page::lexicon::expression::pi_type::PiType;
use crate::page::lexicon::expression::sort::Sort;
use crate::page::lexicon::expression::variable::Variable;

pub mod block;
pub mod expression_socket;
pub mod lambda_abstraction;
pub mod let_assignment;
pub mod pi_type;
pub mod sort;
pub mod variable;

pub mod expression_serialization;

pub enum Expression {
    Block(Block),
    Lam(LambdaAbstraction),
    Let(LetAssignment),
    Pi(PiType),
    Sort(Sort),
    Var(Variable),
}

impl Expression {
    pub fn serialize(&self) -> ExprSer {
        match self {
            Expression::Block(block) => ExprSer::Block(block.serialize()),
            Expression::Lam(lam) => ExprSer::Lam(lam.serialize()),
            Expression::Let(let_expr) => ExprSer::Let(let_expr.serialize()),
            Expression::Pi(pi) => ExprSer::Pi(pi.serialize()),
            Expression::Sort(sort) => ExprSer::Sort(sort.serialize()),
            Expression::Var(var) => ExprSer::Var(var.serialize()),
        }
    }
}
