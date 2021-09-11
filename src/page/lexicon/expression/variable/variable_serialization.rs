use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct VarSer {}

impl VarSer {
    pub fn new() -> VarSer {
        VarSer {}
    }
}
