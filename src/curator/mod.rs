use crate::curator::curator_control::CuratorControl;
use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use std::collections::HashMap;

pub mod curator_control;

pub struct Curator {
    pages: HashMap<String, Box<dyn PageControl>>,
}

impl Curator {
    pub fn new() -> Curator {
        Curator {
            pages: HashMap::new(),
        }
    }
}

impl CuratorControl for Curator {
    fn new_page(&self) -> PageSerialization {
        unimplemented!()
    }
}
