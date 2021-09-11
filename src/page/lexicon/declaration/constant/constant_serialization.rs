use crate::page::lexicon::declaration::constant::ConstVariation;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSer;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstSer {
    variation: ConstVariation,
    term_def_ser: TermDefSer,
}

impl ConstSer {
    pub fn new(variation: ConstVariation, term_def_ser: TermDefSer) -> ConstSer {
        ConstSer {
            variation,
            term_def_ser,
        }
    }
}
