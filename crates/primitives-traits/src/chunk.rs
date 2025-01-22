use crate::{ChunkAddress, BRANCHES, SEGMENT_SIZE, SPAN_SIZE};

pub trait Chunk {
    async fn address(&self) -> ChunkAddress;
    async fn verify(&self, address: ChunkAddress) -> bool;
}

pub trait ChunkBody {
    async fn hash(&self) -> ChunkAddress;
}

pub const CHUNK_SIZE: usize = SEGMENT_SIZE * BRANCHES;
const CHUNK_WITH_SPAN_SIZE: usize = CHUNK_SIZE + SPAN_SIZE;
const SOC_SIGNATURE_SIZE: usize = 65;
const SOC_MIN_CHUNK_SIZE: usize = SEGMENT_SIZE + SOC_SIGNATURE_SIZE + SPAN_SIZE;
const SOC_MAX_CHUNK_SIZE: usize = SOC_MIN_CHUNK_SIZE + CHUNK_SIZE;
