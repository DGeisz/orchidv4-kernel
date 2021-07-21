use crate::block_tree::block::block_serialization::BlockSerialization;

pub enum BlockTreeDiff {
    Append(u128, BlockSerialization),
    Replace(u128, BlockSerialization),
    Detach(u128),
}
