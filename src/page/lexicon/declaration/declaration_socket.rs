use crate::page::lexicon::declaration::constant::{ConstVariation, Constant};
use crate::page::lexicon::declaration::dec_names::{name_to_dec_autocomplete, DeclarationEnum};
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::declaration::definition::{DefVariation, Definition};
use crate::page::lexicon::declaration::{BasicDec, Declaration};
use crate::page::lexicon::term_def::term_def_socket::TermDefSocket;
use crate::page::lexicon::term_def::TermDef;
use crate::page::scoped_entity::{Scope, ScopedEntity};
use crate::utils::id_generator::IdGenControl;
use uuid::Uuid;

pub struct DecSocket {
    id: String,
    dec: Option<Declaration>,
}

impl DecSocket {
    pub fn new(id: String) -> DecSocket {
        DecSocket { id, dec: None }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn serialize(&self) -> DecSocketSer {
        DecSocketSer::new(
            self.id.clone(),
            match &self.dec {
                None => None,
                Some(dec) => Some(dec.serialize()),
            },
        )
    }

    /// @NOT TESTED
    pub fn fill_socket(
        &mut self,
        dec_name: String,
        id_generator: &Box<dyn IdGenControl>,
    ) -> Option<DecSocketSer> {
        /* First check if socket is already full */
        match self.dec {
            Some(_) => None,
            /* If the socket isn't full, then do an autocomplete
            to try to get the type of declaration */
            None => match name_to_dec_autocomplete(dec_name) {
                None => None,
                Some(dec_type) => {
                    /* Stick the new declaration into the socket */
                    self.dec = Some(match dec_type {
                        DeclarationEnum::Constant => Declaration::Const(Constant::new(
                            ConstVariation::Constant,
                            id_generator,
                        )),
                        DeclarationEnum::Axiom => {
                            Declaration::Const(Constant::new(ConstVariation::Axiom, id_generator))
                        }
                        DeclarationEnum::Definition => Declaration::Def(Definition::new(
                            DefVariation::Definition,
                            id_generator,
                        )),
                        DeclarationEnum::Theorem => {
                            Declaration::Def(Definition::new(DefVariation::Theorem, id_generator))
                        }
                        DeclarationEnum::Lemma => {
                            Declaration::Def(Definition::new(DefVariation::Lemma, id_generator))
                        }
                    });

                    Some(self.serialize())
                }
            },
        }
    }

    /// @NOT TESTED
    ///
    /// TODO: Probably build in a better check to determine if this breaks stuff
    pub fn delete_contents(&mut self) -> Option<DecSocketSer> {
        match self.dec {
            None => None,
            Some(_) => {
                self.dec = None;

                Some(self.serialize())
            }
        }
    }
}

impl ScopedEntity for DecSocket {
    fn get_term_def_with_scope(&mut self, tds_id: &String) -> Option<(&mut TermDef, Scope)> {
        match &mut self.dec {
            None => None,
            Some(dec) => dec.get_term_def_with_scope(tds_id),
        }
    }

    fn get_term_def(&mut self) -> Option<&mut TermDef> {
        match &mut self.dec {
            None => None,
            Some(dec) => dec.get_term_def(),
        }
    }

    fn get_tds(&mut self, tds_id: &String) -> Option<&mut TermDefSocket> {
        match &mut self.dec {
            None => None,
            Some(dec) => dec.get_tds(tds_id),
        }
    }
}
