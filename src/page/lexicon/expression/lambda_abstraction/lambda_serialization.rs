use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct LamSer {}

impl LamSer {
    pub fn new() -> LamSer {
        LamSer {}
    }
}
