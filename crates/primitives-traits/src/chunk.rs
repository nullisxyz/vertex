use crate::{ChunkAddress, BRANCHES, SEGMENT_SIZE, SPAN_SIZE};
use std::fmt::Display;

pub trait ChunkContent {
    fn data(&self) -> &[u8];
    fn bmt_address(&self) -> ChunkAddress;
}

#[derive(Debug)]
pub struct ChunkContainer<C: ChunkContent> {
    inner: C,
}

impl<C: ChunkContent> ChunkContainer<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }

    pub fn data(&self) -> &[u8] {
        self.inner.data()
    }

    pub fn bmt_address(&self) -> ChunkAddress {
        self.inner.bmt_address()
    }
}

//impl Display for ChunkType {}

//pub trait Chunk {
//    fn address(&self) -> ChunkAddress;
//    fn data(&self) -> &[u8; CHUNK_SIZE as usize];
//    fn tag_id(&self) -> Option<u32>;
//    fn stamp(&self) -> Option<impl Stamp>;
//    fn depth(&self) -> Option<u8>;
//    fn bucket_depth(&self) -> Option<u8>;
//    fn immutable(&self) -> Option<bool>;
//    fn with_stamp(&mut self, stamp: impl Stamp) -> Self;
//    fn with_batch(&mut self, bucket_depth: u8, immutable: bool) -> Self;
//    fn with_tag_id(&mut self, tag_id: u32) -> Self;
//}

pub const CHUNK_SIZE: usize = SEGMENT_SIZE * BRANCHES;
const CHUNK_WITH_SPAN_SIZE: usize = CHUNK_SIZE + SPAN_SIZE;
const SOC_SIGNATURE_SIZE: usize = 65;
const SOC_MIN_CHUNK_SIZE: usize = SEGMENT_SIZE + SOC_SIGNATURE_SIZE + SPAN_SIZE;
const SOC_MAX_CHUNK_SIZE: usize = SOC_MIN_CHUNK_SIZE + CHUNK_SIZE;
