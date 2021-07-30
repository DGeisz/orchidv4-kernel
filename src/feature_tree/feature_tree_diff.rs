use crate::feature_tree::feature::feature_serialization::FeatureSerialization;

pub enum FeatureTreeDiff {
    Append(u128, FeatureSerialization),
    Replace(u128, FeatureSerialization),
    Detach(u128),
    Rebind(u128, u128),
}
