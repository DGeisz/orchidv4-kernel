//! A Page is the top-most container for actual orchid
//! structures.  It corresponds to a single editor page

use crate::page::feature_tree::feature::feature_enum::FeatureEnum;
use crate::page::feature_tree::feature_tree_control_port::FeatureTreeControlPort;
use crate::page::feature_tree::feature_tree_error::FeatureTreeError;
use crate::page::page_control_port::PageControlPort;
use crate::page::page_serialization::PageSerialization;
use uuid::Uuid;

pub mod feature_tree;
pub mod page_control_port;
pub mod page_error;
pub mod page_generator;
pub mod page_serialization;
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

    fn serialize(&self) -> PageSerialization {
        PageSerialization::new(self.feature_tree_control.serialize(), self.get_id())
    }

    fn create_feature(
        &mut self,
        socket_id: String,
        feature: FeatureEnum,
    ) -> Result<(), FeatureTreeError> {
        self.feature_tree_control.create_feature(socket_id, feature)
    }
}
