use alloy_primitives::{Address, PrimitiveSignature, B256};
use swarm_primitives_traits::{ChunkAddress, ChunkContent};

#[derive(Debug, Eq, PartialEq)]
pub struct SingleOwnerChunk {
    addr: ChunkAddress,
    data: Vec<u8>,
    id: B256,
    owner: Address,
    signature: PrimitiveSignature,
}

impl SingleOwnerChunk {
    pub fn new(
        addr: ChunkAddress,
        data: Vec<u8>,
        id: B256,
        owner: Address,
        signature: PrimitiveSignature,
    ) -> Self {
        Self {
            owner,
            signature,
            id,
            addr,
            data,
        }
    }
}

impl ChunkContent for SingleOwnerChunk {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn bmt_address(&self) -> ChunkAddress {
        self.addr
    }

    fn verify(&self) -> bool {
        // Verify using signature and owner
        true // Placeholder logic
    }
}
