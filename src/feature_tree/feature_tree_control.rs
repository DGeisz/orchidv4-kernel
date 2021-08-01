use crate::feature_tree::feature::feature_request::FeatureRequest;
use crate::feature_tree::feature_tree_diff::FeatureTreeDiff;
use crate::feature_tree::feature_tree_error::FeatureTreeError;

pub trait FeatureTreeControl {
    fn create_and_append(
        &self,
        feature_request: FeatureRequest,
        socket_id: u128,
    ) -> Result<FeatureTreeDiff, FeatureTreeError>;

    // fn create_and_insert(
    //     &self,
    //     feature_request: FeatureRequest,
    //     socket_id: u128,
    //     rebind_to_first: bool,
    // ) -> Result<FeatureTreeDiff, FeatureTreeError>;

    fn release_and_delete(&self, socket_id: u128) -> Result<FeatureTreeDiff, FeatureTreeError>;

    // fn remove_and_rebind(
    //     &self,
    //     socket_id: u128,
    //     rebind_to_first: bool,
    // ) -> Result<FeatureTreeDiff, FeatureTreeError>;
}
