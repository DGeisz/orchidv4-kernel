use crate::page::lexicon::expression::block::block_serialization::BlockSerialization;
use crate::page::lexicon::expression::lambda_abstraction::lambda_serialization::LambdaSerialization;
use crate::page::lexicon::expression::let_assignment::let_serialization::LetSerialization;
use crate::page::lexicon::expression::pi_type::pi_serialization::PiSerialization;
use crate::page::lexicon::expression::sort::sort_serialization::SortSerialization;
use crate::page::lexicon::expression::variable::variable_serialization::VariableSerialization;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ExpressionSerialization {
    Block(BlockSerialization),
    Lam(LambdaSerialization),
    Let(LetSerialization),
    Pi(PiSerialization),
    Sort(SortSerialization),
    Var(VariableSerialization),
}
