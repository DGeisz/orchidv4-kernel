use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum FeatureSerialization {
    Leaf {
        id: u128,
        latex: String,
    },
    Map {
        map: Box<FeatureSerialization>,
        map_latex: MapLatex,
        arg_latex: Vec<Box<FeatureSerialization>>,
    },
    Tuple {
        children: Vec<Box<FeatureSerialization>>,
    },
    Incomplete,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum MapLatex {
    Basic,
    SingleSource(String, String),
    MultiSource(Vec<String>),
}
