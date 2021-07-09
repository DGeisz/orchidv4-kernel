use crate::curator::Curator;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::page_error::PageError;
use crate::page::page_serialization::PageSerialization;
use mockall::*;

/// Provides an interface for sending commands and receiving
/// responses from a curator
#[automock]
pub trait CuratorControlPort {
    /// Command to create a new orchid page.
    /// Returns the id of the new page
    fn new_page(&mut self) -> String;

    /// Command to get the serialization of a full page
    fn full_page(&self, page_id: String) -> Result<PageSerialization, PageError>;

    /// Creates a feature in the specified socket
    fn create_feature(
        &mut self,
        page_id: String,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<PageSerialization, PageError>;
}

pub fn mock_curator_control_port() -> MockCuratorControlPort {
    MockCuratorControlPort::new()
}
