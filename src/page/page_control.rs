use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::page_serialization::PageSerialization;
use mockall::*;

#[automock]
pub trait PageControl {
    fn serialize(&self) -> PageSerialization;

    fn get_id(&self) -> &String;

    fn fill_dec_socket(&mut self, socket_id: String, dec_name: String) -> Option<DecSocketSer>;

    fn append_dec_socket(&mut self) -> DecSocketSer;

    fn delete_dec_socket(&mut self, socket_id: String) -> bool;
}

pub fn mock_page_control() -> MockPageControl {
    MockPageControl::new()
}
