use swarm_primitives_traits::{ChunkAddress, ChunkBody, ChunkHeader};

use super::bmt_body::BMTBody;

#[derive(Debug, PartialEq, Eq)]
pub struct ContentChunk {
    body: BMTBody,
}

impl ContentChunk {
    pub fn new(body: BMTBody) -> Self {
        Self { body }
    }
}

impl swarm_primitives_traits::Chunk for ContentChunk {
    async fn address(&self) -> swarm_primitives_traits::ChunkAddress {
        self.body.hash().await
    }

    async fn verify(&self, address: ChunkAddress) -> bool {
        address == self.address().await
    }
}
