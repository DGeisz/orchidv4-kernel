use crate::page::lexicon::expression::sort::sort_serialization::SortSer;

pub mod sort_serialization;

pub struct Sort;

impl Sort {
    pub fn serialize(&self) -> SortSer {
        SortSer::new()
    }
}
