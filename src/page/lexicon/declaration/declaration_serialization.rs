use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DeclarationSerialization {
    Const(Constant),
    Def(Definition),
}
