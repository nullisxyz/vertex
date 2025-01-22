use swarm_primitives_traits::ChunkContainer;

mod content;
use content::ContentChunk;
mod single_owner;
use single_owner::SingleOwnerChunk;
mod bmt_body;

#[derive(Debug, Eq, PartialEq)]
pub enum Chunk {
    Content(ChunkContainer<ContentChunk>),
    SingleOwner(ChunkContainer<SingleOwnerChunk>),
}

impl Chunk {
    pub fn verify(&self) -> bool {
        match self {
            Chunk::Content(c) => c.verify(),
            Chunk::SingleOwner(c) => c.verify(),
        }
    }
}
