use crate::page::page_control::PageControl;
use mockall::*;

/// Control port for the page generator
#[automock]
pub trait PageGeneratorControl {
    fn generate_new_page(&self) -> Box<dyn PageControl>;
}

pub fn mock_page_generator_control() -> MockPageGeneratorControl {
    MockPageGeneratorControl::new()
}
