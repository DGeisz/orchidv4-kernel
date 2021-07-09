use crate::page::feature_tree::feature_tree_error::FeatureTreeError;

pub enum PageError {
    /// Error indicates that a particular page
    /// wasn't found.  Returns the id of the
    /// page requested
    PageNotFound(String),

    /// Indicates there was a feature tree level error
    FeatureError(FeatureTreeError),
}
