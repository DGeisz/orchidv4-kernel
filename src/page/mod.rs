use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use uuid::Uuid;

pub mod page_control;
pub mod page_generator;
pub mod page_serialization;

#[cfg(test)]
mod tests;

pub struct Page {
    id: String,
}

impl Page {
    /// Creates a new page
    pub fn new() -> Box<dyn PageControl> {
        Box::new(Page {
            id: Uuid::new_v4().to_hyphenated().to_string(),
        })
    }
}

impl PageControl for Page {
    fn serialize(&self) -> PageSerialization {
        PageSerialization::new(self.id.clone())
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}
