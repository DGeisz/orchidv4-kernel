use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DefSer {}

impl DefSer {
    pub fn new() -> DefSer {
        DefSer {}
    }
}
