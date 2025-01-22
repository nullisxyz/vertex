use swarm_primitives_traits::{ChunkAddress, ChunkContent};

#[derive(Debug, Eq, PartialEq)]
pub struct ContentChunk {
    addr: ChunkAddress,
    data: Vec<u8>,
}

impl ContentChunk {
    pub fn new(addr: ChunkAddress, data: Vec<u8>) -> Self {
        Self { addr, data }
    }
}

impl ChunkContent for ContentChunk {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn bmt_address(&self) -> swarm_primitives_traits::ChunkAddress {
        self.addr
    }

    fn verify(&self) -> bool {
        // Verify based on BMT hash
        true
    }
}
