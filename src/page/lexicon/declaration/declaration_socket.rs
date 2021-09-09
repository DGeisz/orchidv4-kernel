use crate::page::lexicon::declaration::constant::{ConstVariation, Constant};
use crate::page::lexicon::declaration::dec_names::{name_to_dec_autocomplete, DeclarationEnum};
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::declaration::definition::{DefVariation, Definition};
use crate::page::lexicon::declaration::{BasicDec, Declaration};
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
    pub fn fill_socket(&mut self, dec_name: String) -> Option<DecSocketSer> {
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
                        DeclarationEnum::Constant => {
                            Declaration::Const(Constant::new(ConstVariation::Constant))
                        }
                        DeclarationEnum::Axiom => {
                            Declaration::Const(Constant::new(ConstVariation::Axiom))
                        }
                        DeclarationEnum::Definition => {
                            Declaration::Def(Definition::new(DefVariation::Definition))
                        }
                        DeclarationEnum::Theorem => {
                            Declaration::Def(Definition::new(DefVariation::Theorem))
                        }
                        DeclarationEnum::Lemma => {
                            Declaration::Def(Definition::new(DefVariation::Lemma))
                        }
                    });

                    Some(self.serialize())
                }
            },
        }
    }
}
