use crate::page::page_serialization::PageSerialization;
use mockall::*;

/// Port for controlling the underlying page objects
#[automock]
pub trait PageControlPort {
    /// Get a clone of the id of the page
    fn get_id(&self) -> String;

    /// Serialize the full page
    fn serialize(&self) -> PageSerialization;
}

pub fn mock_page_control_port() -> MockPageControlPort {
    MockPageControlPort::new()
}
