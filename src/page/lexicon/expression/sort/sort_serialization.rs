use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct SortSer {}

impl SortSer {
    pub fn new() -> SortSer {
        SortSer {}
    }
}
