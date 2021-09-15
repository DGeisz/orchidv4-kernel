use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;
use crate::page::page_serialization::PageSerialization;
use mockall::*;

#[automock]
pub trait PageControl {
    fn serialize(&self) -> PageSerialization;

    fn get_id(&self) -> &String;

    fn fill_dec_socket(&mut self, socket_id: String, dec_name: String) -> Option<DecSocketSer>;

    fn append_dec_socket(&mut self) -> DecSocketSer;

    fn delete_dec_socket(&mut self, socket_id: String) -> bool;

    fn delete_dec_socket_contents(&mut self, socket_id: String) -> Option<DecSocketSer>;

    fn insert_dec_socket(
        &mut self,
        /* Rel socket is the dec socket the new socket is
        inserted relative to*/
        rel_socket_id: &String,
        /* Whether we place the new socket before of after
        the rel socket*/
        before_rel: bool,
    ) -> Option<DecSocketSer>;

    fn fill_term_def_socket(
        &mut self,
        tds_id: &String,
        char_seq: String,
    ) -> Option<TermDefSocketSer>;
}

pub fn mock_page_control() -> MockPageControl {
    MockPageControl::new()
}
