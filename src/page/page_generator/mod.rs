use crate::page::page_control::PageControl;
use crate::page::page_generator::page_generator_control::PageGeneratorControl;
use crate::page::Page;

pub mod page_generator_control;

pub struct PageGenerator {}

impl PageGeneratorControl for PageGenerator {
    fn generate_new_page(&self) -> Box<dyn PageControl> {
        Page::new()
    }
}
