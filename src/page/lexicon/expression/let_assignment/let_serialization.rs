use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct LetSer {}

impl LetSer {
    pub fn new() -> LetSer {
        LetSer {}
    }
}
