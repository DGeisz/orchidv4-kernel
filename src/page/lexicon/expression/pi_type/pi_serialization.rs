use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct PiSer {}

impl PiSer {
    pub fn new() -> PiSer {
        PiSer {}
    }
}
