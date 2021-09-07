use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstSer {}

impl ConstSer {
    pub fn new() -> ConstSer {
        ConstSer {}
    }
}
