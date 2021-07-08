use mockall::*;

/// Provides an interface for sending commands and receiving
/// responses from a curator
#[automock]
pub trait CuratorControlPort {
    /// Command to create a new orchid page.
    /// Returns the id of the new page
    fn new_page(&mut self) -> String;
}

pub fn mock_curator_control_port() -> MockCuratorControlPort {
    MockCuratorControlPort::new()
}
