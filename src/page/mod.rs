use crate::page::lexicon::declaration::{BasicDec, Declaration};
use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use uuid::Uuid;

pub mod lexicon;
pub mod page_control;
pub mod page_generator;
pub mod page_serialization;

#[cfg(test)]
mod tests;

pub struct Page {
    id: String,
    /// These are the lines of the page.  Any line that
    /// is "None" is interpreted as an empty line
    declarations: Vec<Option<Declaration>>,
}

impl Page {
    /// Creates a new page
    pub fn new() -> Box<dyn PageControl> {
        Box::new(Page {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            declarations: Vec::new(),
        })
    }
}

impl PageControl for Page {
    fn serialize(&self) -> PageSerialization {
        PageSerialization::new(
            self.id.clone(),
            self.declarations
                .iter()
                .map(|dec_option| match dec_option {
                    None => None,
                    Some(dec) => Some(dec.serialize()),
                })
                .collect(),
        )
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}
