//! A Page is the top-most container for actual orchid
//! structures.  It corresponds to a single editor page

use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::page_control_port::PageControlPort;
use uuid::Uuid;

pub mod feature_tree;
pub mod page_control_port;
pub mod page_generator;
mod term_tree;

pub struct Page {
    id: String,
    feature_tree_control: Box<dyn FeatureTreeControlPort>,
}

impl Page {
    pub fn new(feature_tree_control: Box<dyn FeatureTreeControlPort>) -> Box<dyn PageControlPort> {
        Box::new(Page {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            feature_tree_control,
        })
    }
}

impl PageControlPort for Page {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
