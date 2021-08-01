#[derive(PartialEq, Clone)]
pub enum CompactFeature {
    Leaf(u128),
    Map {
        map: Box<CompactFeature>,
        arg: Box<CompactFeature>,
    },
    Tuple(Vec<Box<CompactFeature>>),
}
