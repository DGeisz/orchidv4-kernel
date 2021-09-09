use crate::page::lexicon::declaration::constant::ConstVariation;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstSer {
    variation: ConstVariation,
}

impl ConstSer {
    pub fn new(variation: ConstVariation) -> ConstSer {
        ConstSer { variation }
    }
}
