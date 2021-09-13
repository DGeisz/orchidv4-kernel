use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
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

    /// Instruction to fill a dec socket with a structure
    /// with a particular declaration
    fn fill_dec_socket(
        &mut self,
        page_id: String,
        socket_id: String,
        dec_name: String,
    ) -> Option<DecSocketSer>;

    /// Instruction to append a dec socket on the end of
    /// a page
    fn append_dec_socket(&mut self, page_id: String) -> Option<DecSocketSer>;

    fn delete_dec_socket(&mut self, page_id: String, socket_id: String) -> bool;
}

pub fn mock_curator_control() -> MockCuratorControl {
    MockCuratorControl::new()
}
