use crate::page::feature_tree::feature_tree_generator::feature_tree_generator_port::FeatureTreeGeneratorPort;
use crate::page::page_control_port::PageControlPort;
use crate::page::page_generator::page_generator_port::PageGeneratorPort;
use crate::page::Page;

pub mod page_generator_port;

pub struct PageGenerator {
    feature_tree_generator: Box<dyn FeatureTreeGeneratorPort>,
}

impl PageGenerator {
    pub fn new(
        feature_tree_generator: Box<dyn FeatureTreeGeneratorPort>,
    ) -> Box<dyn PageGeneratorPort> {
        Box::new(PageGenerator {
            feature_tree_generator,
        })
    }
}

impl PageGeneratorPort for PageGenerator {
    fn generate_page(&self) -> Box<dyn PageControlPort> {
        Page::new(self.feature_tree_generator.generate_feature_tree())
    }
}
