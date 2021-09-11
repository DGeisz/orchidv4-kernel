use crate::page::lexicon::declaration::definition::DefVariation;
use crate::page::lexicon::expression::expression_serialization::ExprSocketSer;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSer;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DefSer {
    variation: DefVariation,
    term_def_ser: TermDefSer,
    term_expr_ser: ExprSocketSer,
}

impl DefSer {
    pub fn new(
        variation: DefVariation,
        term_def_ser: TermDefSer,
        term_expr_ser: ExprSocketSer,
    ) -> DefSer {
        DefSer {
            variation,
            term_def_ser,
            term_expr_ser,
        }
    }
}
