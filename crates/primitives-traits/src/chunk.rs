use crate::{ChunkAddress, BRANCHES, SEGMENT_SIZE};
use std::error::Error;

pub trait Chunk {
    async fn address(&self) -> ChunkAddress;
    async fn verify(&self, address: ChunkAddress) -> bool;
}

pub trait ChunkBody {
    async fn hash(&self) -> ChunkAddress;
}

pub trait ChunkEncoding {
    fn size(&self) -> usize;
    fn to_boxed_slice(&self) -> Box<[u8]>;
}

pub trait ChunkDecoding
where
    Self: Sized,
{
    async fn from_slice(buf: &[u8]) -> Result<Self, impl Error>;
}

pub const CHUNK_SIZE: usize = SEGMENT_SIZE * BRANCHES;
