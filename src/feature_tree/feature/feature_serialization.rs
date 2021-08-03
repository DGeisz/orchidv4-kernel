use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum FeatureSerialization {
    Leaf {
        id: u128,
        latex: String,
    },
    Map {
        map: Box<SocketSerialization>,
        map_latex: MapLatex,
        arg: Box<FeatureSerialization>,
    },
    Tuple {
        children: Vec<Box<SocketSerialization>>,
    },
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SocketSerialization {
    id: u128,
    feature: Option<FeatureSerialization>,
}

impl SocketSerialization {
    pub fn new(id: u128, feature: Option<FeatureSerialization>) -> SocketSerialization {
        SocketSerialization { id, feature }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum MapLatex {
    Basic,
    SingleSource(String, String),
    MultiSource(Vec<String>, Vec<u16>),
}
