use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct BlockSer {}

impl BlockSer {
    pub fn new() -> BlockSer {
        BlockSer {}
    }
}
