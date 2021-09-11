use crate::page::lexicon::expression::lambda_abstraction::lambda_serialization::LamSer;

pub mod lambda_serialization;

pub struct LambdaAbstraction;

impl LambdaAbstraction {
    pub fn serialize(&self) -> LamSer {
        LamSer::new()
    }
}
