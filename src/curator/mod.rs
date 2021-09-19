use crate::curator::curator_control::CuratorControl;
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;
use crate::page::page_control::PageControl;
use crate::page::page_generator::page_generator_control::PageGeneratorControl;
use crate::page::page_serialization::PageSerialization;
use crate::utils::id_generator::IdGenControl;
use std::collections::HashMap;

pub mod curator_control;

#[cfg(test)]
mod tests;

pub struct Curator {
    pages: HashMap<String, Box<dyn PageControl>>,
    page_generator: Box<dyn PageGeneratorControl>,
}

impl Curator {
    pub fn new(page_generator: Box<dyn PageGeneratorControl>) -> Box<dyn CuratorControl> {
        Box::new(Curator {
            pages: HashMap::new(),
            page_generator,
        })
    }
}

impl CuratorControl for Curator {
    fn new_page(&mut self) -> PageSerialization {
        /* First create a new page */
        let new_page = self.page_generator.generate_new_page();

        /* Then grab the serialization for later */
        let serialization = new_page.serialize();

        /* Add the page to the list of pages */
        self.pages.insert(new_page.get_id().clone(), new_page);

        /* Return the page serialization */
        serialization
    }

    fn get_page(&self, id: String) -> Option<PageSerialization> {
        match self.pages.get(&id) {
            None => None,
            Some(page) => Some(page.serialize()),
        }
    }

    /// @NOT TESTED
    fn fill_dec_socket(
        &mut self,
        page_id: &String,
        socket_id: String,
        dec_name: String,
    ) -> Option<DecSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => page.fill_dec_socket(socket_id, dec_name),
        }
    }

    fn append_dec_socket(&mut self, page_id: &String) -> Option<DecSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => Some(page.append_dec_socket()),
        }
    }

    fn delete_dec_socket(&mut self, page_id: &String, socket_id: String) -> bool {
        match self.pages.get_mut(page_id) {
            None => false,
            Some(page) => page.delete_dec_socket(socket_id),
        }
    }

    fn delete_dec_socket_contents(
        &mut self,
        page_id: &String,
        socket_id: String,
    ) -> Option<DecSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => page.delete_dec_socket_contents(socket_id),
        }
    }

    fn insert_dec_socket(
        &mut self,
        page_id: &String,
        rel_socket_id: &String,
        before_rel: bool,
    ) -> Option<DecSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => page.insert_dec_socket(rel_socket_id, before_rel),
        }
    }

    fn fill_term_def_socket(
        &mut self,
        page_id: &String,
        tds_id: &String,
        term_seq: String,
    ) -> Option<TermDefSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => page.fill_term_def_socket(tds_id, term_seq),
        }
    }

    fn delete_tds_contents(
        &mut self,
        page_id: &String,
        tds_id: &String,
    ) -> Option<TermDefSocketSer> {
        match self.pages.get_mut(page_id) {
            None => None,
            Some(page) => page.delete_tds_contents(tds_id),
        }
    }
}
