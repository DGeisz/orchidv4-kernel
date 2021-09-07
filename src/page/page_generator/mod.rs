use crate::page::page_control::PageControl;
use crate::page::page_generator::page_generator_control::PageGeneratorControl;
use crate::page::Page;
use crate::utils::id_generator::IdGenControl;
use std::rc::Rc;

pub mod page_generator_control;

pub struct PageGenerator {
    id_generator: Rc<Box<dyn IdGenControl>>,
}

impl PageGenerator {
    pub fn new(id_generator: Rc<Box<dyn IdGenControl>>) -> Box<dyn PageGeneratorControl> {
        Box::new(PageGenerator { id_generator })
    }
}

impl PageGeneratorControl for PageGenerator {
    fn generate_new_page(&self) -> Box<dyn PageControl> {
        Page::new(Rc::clone(&self.id_generator))
    }
}
