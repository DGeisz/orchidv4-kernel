//! The curator is essentially the outermost interface for
//! working with orchid pages (ie CRUD on pages), that lies
//! behind a kernel_io module.  It contains all the pages that
//! are actively loaded in-memory within the kernel
use crate::curator::curator_control_port::CuratorControlPort;
use crate::page::feature_tree::feature_socket::feature_socket_generator::FeatureSocketGenerator;
use crate::page::feature_tree::feature_tree_generator::FeatureTreeGenerator;
use crate::page::page_control_port::PageControlPort;
use crate::page::page_generator::page_generator_port::PageGeneratorPort;
use crate::page::page_generator::PageGenerator;
use std::collections::HashMap;
use std::rc::Rc;

pub mod curator_control_port;

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
}

/// We need a function that actually assembles a curator
/// that can actually be used in a running kernel.  This is
/// an external function from the curator because the curator
/// should be agnostic about the implementation of its page generator
pub fn assemble_kernel_curator() -> Box<dyn CuratorControlPort> {
    let feature_socket_generator = FeatureSocketGenerator::new();
    feature_socket_generator.init(Rc::clone(&feature_socket_generator));

    Curator::new(PageGenerator::new(FeatureTreeGenerator::new(
        feature_socket_generator,
    )))
}
