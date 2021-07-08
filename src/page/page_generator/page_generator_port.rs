use crate::page::page_control_port::PageControlPort;
use mockall::*;

/// Port for structures that generate orchid
/// pages
#[automock]
pub trait PageGeneratorPort {
    /// Generates a page and provides the port for controlling
    /// said page
    fn generate_page(&self) -> Box<dyn PageControlPort>;
}
