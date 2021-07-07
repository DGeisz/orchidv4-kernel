//! A Page is the top-most container for actual orchid
//! structures.  It corresponds to a single editor page

use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::page_control_port::PageControlPort;

mod feature_tree;
pub mod page_control_port;
pub mod page_generator;
mod term_tree;

pub struct Page {
    feature_tree_control: Box<dyn FeatureTreeControlPort>,
}

impl PageControlPort for Page {}
