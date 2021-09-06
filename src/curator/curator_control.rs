use crate::page::page_serialization::PageSerialization;
use mockall::*;

/// Control port for the curator
#[automock]
pub trait CuratorControl {
    /// Create a new page, and turn the page's serialization
    fn new_page(&mut self) -> PageSerialization;

    /// Get a serialization of a page with a given id.
    /// Return None if we don't have that page
    fn get_page(&self, id: String) -> Option<PageSerialization>;
}

pub fn mock_curator_control() -> MockCuratorControl {
    MockCuratorControl::new()
}
