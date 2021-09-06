use crate::page::page_serialization::PageSerialization;
use mockall::*;

#[automock]
pub trait PageControl {
    fn serialize(&self) -> PageSerialization;

    fn get_id(&self) -> &String;
}

pub fn mock_page_control() -> MockPageControl {
    MockPageControl::new()
}
