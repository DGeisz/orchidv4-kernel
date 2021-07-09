//! The curator is essentially the outermost interface for
//! working with orchid pages (ie CRUD on pages), that lies
//! behind a kernel_io module.  It contains all the pages that
//! are actively loaded in-memory within the kernel
use crate::curator::curator_control_port::CuratorControlPort;
use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_tree_generator::FeatureTreeGenerator;
use crate::page::page_control_port::PageControlPort;
use crate::page::page_error::PageError;
use crate::page::page_generator::page_generator_port::PageGeneratorPort;
use crate::page::page_generator::PageGenerator;
use crate::page::page_serialization::PageSerialization;
use std::collections::HashMap;
use std::rc::Rc;

pub mod curator_control_port;

#[cfg(test)]
mod tests;

pub struct Curator {
    pages: HashMap<String, Box<dyn PageControlPort>>,
    page_generator: Box<dyn PageGeneratorPort>,
}

impl Curator {
    pub fn new(page_generator: Box<dyn PageGeneratorPort>) -> Box<dyn CuratorControlPort> {
        Box::new(Curator {
            page_generator,
            pages: HashMap::new(),
        })
    }
}

impl CuratorControlPort for Curator {
    fn new_page(&mut self) -> String {
        let new_page = self.page_generator.generate_page();
        let page_id = new_page.get_id();

        self.pages.insert(page_id.clone(), new_page);

        page_id
    }

    fn full_page(&self, page_id: String) -> Result<PageSerialization, PageError> {
        match self.pages.get(&page_id) {
            Some(page) => Ok(page.serialize()),
            None => Err(PageError::PageNotFound(page_id)),
        }
    }

    fn create_feature(
        &mut self,
        page_id: String,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<PageSerialization, PageError> {
        match self.pages.get_mut(&page_id) {
            Some(mut page) => match page.create_feature(socket_id, feature) {
                Ok(_) => Ok(page.serialize()),
                Err(e) => Err(PageError::FeatureError(e)),
            },
            None => Err(PageError::PageNotFound(page_id)),
        }
    }
}

/// We need a function that actually assembles a curator
/// that can actually be used in a running kernel.  This is
/// an external function from the curator because the curator
/// should be agnostic about the implementation of its page generator
pub fn assemble_kernel_curator() -> Box<dyn CuratorControlPort> {
    let feature_tree_generator = FeatureTreeGenerator::new();

    feature_tree_generator.init(Rc::clone(&feature_tree_generator));

    Curator::new(PageGenerator::new(feature_tree_generator))
}
