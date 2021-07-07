use crate::page::page_control_port::PageControlPort;

/// Port for structures that generate orchid
/// pages
pub trait PageGeneratorPort {
    /// Generates a page and provides the port for controlling
    /// said page
    fn generate_page(&self) -> Box<dyn PageControlPort>;
}
