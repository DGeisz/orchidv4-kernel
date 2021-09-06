use crate::page::page_serialization::PageSerialization;
use mockall::*;

/// Control port for the curator
#[automock]
pub trait CuratorControl {
    /// Create a new page, and turn the page's serialization
    fn new_page(&self) -> PageSerialization;
}

pub fn mock_curator_control() -> MockCuratorControl {
    MockCuratorControl::new()
}
