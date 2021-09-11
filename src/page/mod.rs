use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::declaration::declaration_socket::DecSocket;
use crate::page::lexicon::declaration::{BasicDec, Declaration};
use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use crate::utils::id_generator::IdGenControl;
use std::rc::Rc;
use uuid::Uuid;

pub mod lexicon;
pub mod page_control;
pub mod page_generator;
pub mod page_serialization;

#[cfg(test)]
mod tests;

pub struct Page {
    id: String,
    id_generator: Rc<Box<dyn IdGenControl>>,
    /// These are the lines of the page.  Any line that
    /// is "None" is interpreted as an empty line
    dec_sockets: Vec<DecSocket>,
}

impl Page {
    /// Creates a new page
    pub fn new(id_generator: Rc<Box<dyn IdGenControl>>) -> Box<dyn PageControl> {
        let dec_socket_id = id_generator.gen_id();

        Box::new(Page {
            id: id_generator.gen_id(),
            id_generator,
            /* Init with one dec socket for the first line of the page */
            dec_sockets: vec![DecSocket::new(dec_socket_id)],
        })
    }

    fn find_dec_socket(&mut self, socket_id: String) -> Option<&mut DecSocket> {
        match self
            .dec_sockets
            .iter()
            .position(|socket| socket.get_id() == &socket_id)
        {
            None => None,
            Some(index) => self.dec_sockets.get_mut(index),
        }
    }
}

impl PageControl for Page {
    fn serialize(&self) -> PageSerialization {
        PageSerialization::new(
            self.id.clone(),
            self.dec_sockets
                .iter()
                .map(|dec_socket| dec_socket.serialize())
                .collect(),
        )
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    /// @NOT TESTED
    fn fill_dec_socket(&mut self, socket_id: String, dec_name: String) -> Option<DecSocketSer> {
        let id_gen = Rc::clone(&self.id_generator);

        match self.find_dec_socket(socket_id) {
            None => None,
            Some(socket) => socket.fill_socket(dec_name, &id_gen),
        }
    }
}
