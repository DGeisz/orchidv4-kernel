use crate::feature_tree::compact_feature::CompactFeature;
use crate::feature_tree::feature_type::{FeatureType, TypeHierarchyAnchor};

/// This is the scope that's general for all pages.
/// Essentially the prelude scope
pub const BASE_SCOPE: u128 = 0;

pub const BOOLEAN_TERM_ID: u128 = 0;

pub const BOOLEAN_TYPE: FeatureType = FeatureType::new(
    /* Give the bool type id 0 */
    CompactFeature::Leaf(BOOLEAN_TERM_ID),
    TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE),
    1,
);

pub const BASE_TYPE_TERM_ID: u128 = 1;

pub const BASE_TYPE_TYPE: FeatureType = FeatureType::new(
    CompactFeature::Leaf(BASE_TYPE_TERM_ID),
    TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE),
    2,
);

pub const MAP_TYPE_MAP_TERM_ID: u128 = 2;

pub fn gen_map_term(source: CompactFeature, target: CompactFeature) -> CompactFeature {
    CompactFeature::Map {
        map: Box::new(CompactFeature::Leaf(MAP_TYPE_MAP_TERM_ID)),
        arg: Box::new(CompactFeature::Tuple(vec![
            Box::new(source),
            Box::new(target),
        ])),
    }
}

pub fn gen_map_type(source: CompactFeature, target: CompactFeature) -> FeatureType {
    let superior = gen_map_term(source, target);

    FeatureType::new(superior.clone(), TypeHierarchyAnchor::Feature(superior), 1)
}
