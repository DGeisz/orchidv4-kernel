use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::declaration::declaration_socket::DecSocket;
use crate::page::lexicon::declaration::{BasicDec, Declaration};
use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;
use crate::page::lexicon::term_def::TermDef;
use crate::page::page_control::PageControl;
use crate::page::page_serialization::PageSerialization;
use crate::page::scoped_entity::{Scope, ScopedEntity};
use crate::utils::id_generator::IdGenControl;
use std::rc::Rc;
use uuid::Uuid;

pub mod lexicon;
pub mod page_control;
pub mod page_generator;
pub mod page_serialization;
pub mod scoped_entity;

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
        let dec_id = id_generator.gen_id();

        Box::new(Page {
            id: id_generator.gen_id(),
            id_generator,
            /* Init with one dec socket for the first line of the page */
            dec_sockets: vec![DecSocket::new(dec_id)],
        })
    }

    fn find_dec_socket(&mut self, socket_id: &String) -> Option<&mut DecSocket> {
        match self
            .dec_sockets
            .iter()
            .position(|socket| socket.get_id() == socket_id)
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

        match self.find_dec_socket(&socket_id) {
            None => None,
            Some(socket) => socket.fill_socket(dec_name, &id_gen),
        }
    }

    /// @NOT TESTED
    fn append_dec_socket(&mut self) -> DecSocketSer {
        let new_socket = DecSocket::new(self.id_generator.gen_id());
        let socket_ser = new_socket.serialize();

        self.dec_sockets.push(new_socket);

        socket_ser
    }

    /// @NOT TESTED
    ///
    /// TODO: Go through the socket tree and make note of every erroneous
    ///  term that has resulted from this socket being deleted (check for errors)
    fn delete_dec_socket(&mut self, socket_id: String) -> bool {
        match self.find_dec_socket(&socket_id) {
            None => false,
            Some(_) => {
                self.dec_sockets
                    .retain(|socket| socket.get_id() != &socket_id);

                true
            }
        }
    }

    /// @NOT TESTED
    ///
    /// TODO: Check for errors that result from deleting this dec sockets contents
    fn delete_dec_socket_contents(&mut self, socket_id: String) -> Option<DecSocketSer> {
        match self.find_dec_socket(&socket_id) {
            None => None,
            Some(socket) => socket.delete_contents(),
        }
    }

    fn insert_dec_socket(
        &mut self,
        rel_socket_id: &String,
        before_rel: bool,
    ) -> Option<DecSocketSer> {
        match self
            .dec_sockets
            .iter()
            .position(|socket| socket.get_id() == rel_socket_id)
        {
            None => None,
            Some(index) => {
                let new_socket = DecSocket::new(self.id_generator.gen_id());
                let socket_ser = new_socket.serialize();

                if before_rel {
                    self.dec_sockets.insert(index, new_socket);
                } else {
                    self.dec_sockets.insert(index + 1, new_socket);
                }

                Some(socket_ser)
            }
        }
    }

    fn fill_term_def_socket(
        &mut self,
        tds_id: &String,
        term_seq: String,
    ) -> Option<TermDefSocketSer> {
        /* First we have to find the tds in question, along with its
        scope*/
        match self.get_term_def_with_scope(tds_id) {
            None => None,
            Some((mut term_def, mut scope)) => {
                /* Ok, now we want to go through the previous term defs
                in the scope, and see if any of them define the character
                sequence that we're trying to assign to our current term def*/

                for td in scope.iter_mut() {
                    if let Some(s_term_seq) = td.get_def_socket().get_term_seq() {
                        if s_term_seq == &term_seq {
                            /* This means a socket previously defined in the scope
                            uses the same char seq, which is illegal, because each
                             char seq in the scope must be unique. So we return None */
                            return None;
                        }
                    }
                }

                /* Otherwise, this means none of the previous tds in the scope
                use this char seq, so we can go ahead and assign it to this term def */
                Some(term_def.get_mut_def_socket().assign_term_seq(term_seq))
            }
        }
    }

    /// TODO: Add extra f11y that checks what breaks when we delete this term
    fn delete_tds_contents(&mut self, tds_id: &String) -> Option<TermDefSocketSer> {
        /* First find the tds in question here */
        match self.get_term_def_with_scope(tds_id) {
            None => None,
            Some((mut term_def, _)) => {
                /* Now we go in and actually delete the lad */
                Some(term_def.get_mut_def_socket().delete_term_seq())
            }
        }
    }
}

impl ScopedEntity for Page {
    fn get_term_def_with_scope(&mut self, tds_id: &String) -> Option<(&mut TermDef, Scope)> {
        let mut scope: Scope = Vec::new();
        /* Use this to determine if this is actually the scope we're looking for
        and if so, hold a ref to the term def we found */
        let mut td = None;

        /* Go through the dec sockets to see if any have the tds in question*/
        for socket in &mut self.dec_sockets {
            /* This bizarre control structure pattern is a result of a
            long fight with the borrow checker
             TODO: Figure out if there's something you can do about this with lifetimes? (This is super inefficient)*/
            if let Some(_) = socket.get_term_def_with_scope(tds_id) {
                if let Some(td_scope) = socket.get_term_def_with_scope(tds_id) {
                    /* In this case, go ahead and return the boi */
                    return Some(td_scope);
                }
            } else {
                if let Some(term_def) = socket.get_term_def() {
                    /* Check if the term def has a socket with the appropriate id */
                    if term_def.get_def_socket().get_id() == tds_id {
                        /* In this case return the term def in question
                        with the scope we've constructed up until this point */
                        td = Some(term_def);
                    } else {
                        scope.push(term_def)
                    }
                }
            }
        }

        match td {
            None => None,
            Some(term_def) => Some((term_def, scope)),
        }
    }
}
