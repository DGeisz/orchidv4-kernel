use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::page::page_serialization::PageSerialization;
use crate::page::Page;
use mockall::*;

/// Port for controlling the underlying page objects
#[automock]
pub trait PageControlPort {
    /// Get a clone of the id of the page
    fn get_id(&self) -> String;

    /// Serialize the full page
    fn serialize(&self) -> PageSerialization;

    /// Create feature
    fn create_feature(
        &mut self,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<(), FeatureTreeError>;
}

pub fn mock_page_control_port() -> MockPageControlPort {
    MockPageControlPort::new()
}
