use crate::feature_tree::compact_feature::CompactFeature;

pub mod built_in_types;

#[derive(PartialEq, Clone)]
pub struct FeatureType {
    superior_feature: CompactFeature,
    hierarchy_anchor: TypeHierarchyAnchor,
    hierarchy_level: i16,
}

impl FeatureType {
    pub const fn new(
        superior_feature: CompactFeature,
        hierarchy_anchor: TypeHierarchyAnchor,
        hierarchy_level: i16,
    ) -> FeatureType {
        FeatureType {
            superior_feature,
            hierarchy_anchor,
            hierarchy_level,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum TypeHierarchyAnchor {
    ScopeRelative(u128),
    Feature(CompactFeature),
}
