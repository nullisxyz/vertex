use swarm_primitives_traits::{ChunkAddress, ChunkBody, CHUNK_SIZE, SEGMENT_SIZE};

use crate::bmt::{Hasher, HasherBuilder};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BMTBody {
    span: u64,
    data: Vec<u8>,
}

impl BMTBody {
    pub fn new(span: u64, data: Vec<u8>) -> Self {
        assert!(data.len() <= CHUNK_SIZE, "Data size exceeds maximum");
        Self { span, data }
    }
}

impl ChunkBody for BMTBody {
    async fn hash(&self) -> ChunkAddress {
        // TODO: Implement BMT hasher pooling from a global static
        let mut hasher: Hasher = HasherBuilder::default().build().unwrap();
        hasher.set_span(self.span);
        hasher.write(&self.data).await;

        let mut result = [0u8; SEGMENT_SIZE];
        hasher.hash(&mut result);

        result.into()
    }
}
