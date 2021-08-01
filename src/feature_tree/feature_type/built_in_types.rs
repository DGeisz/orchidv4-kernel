use crate::feature_tree::compact_feature::CompactFeature;
use crate::feature_tree::feature_type::{FeatureType, TypeHierarchyAnchor};

/// This is the scope that's general for all pages.
/// Essentially the prelude scope
const BASE_SCOPE: u128 = 0;

pub const BOOLEAN_TYPE: FeatureType = FeatureType::new(
    /* Give the bool type id 0 */
    CompactFeature::Leaf(0),
    TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE),
    1,
);

pub const BASE_TYPE_TYPE: FeatureType = FeatureType::new(
    CompactFeature::Leaf(1),
    TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE),
    2,
);

pub const STATEMENT_TYPE: FeatureType = FeatureType::new(
    CompactFeature::Leaf(2),
    TypeHierarchyAnchor::ScopeRelative(BASE_SCOPE),
    1,
);
