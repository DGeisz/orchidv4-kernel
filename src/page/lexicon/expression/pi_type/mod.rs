use crate::page::lexicon::expression::pi_type::pi_serialization::PiSer;

pub mod pi_serialization;

pub struct PiType;

impl PiType {
    pub fn serialize(&self) -> PiSer {
        PiSer::new()
    }
}
