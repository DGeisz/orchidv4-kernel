use crate::page::lexicon::expression::block::block_serialization::BlockSer;

pub mod block_serialization;

pub struct Block;

impl Block {
    pub fn serialize(&self) -> BlockSer {
        BlockSer::new()
    }
}
