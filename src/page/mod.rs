use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use uuid::Uuid;

pub mod page_control;
pub mod page_serialization;

#[cfg(test)]
mod tests;

pub struct Page {
    id: u128,
}

impl Page {
    /// Creates a new page
    /// NO TEST
    pub fn new() -> Box<dyn PageControl> {
        Box::new(Page {
            id: Uuid::new_v4().as_u128(),
        })
    }
}

impl PageControl for Page {
    /// NO TEST
    fn serialize(&self) -> PageSerialization {
        PageSerialization::new(self.id)
    }
}
