use alloy_primitives::{Address, Keccak256, PrimitiveSignature, B256};
use swarm_primitives_traits::{ChunkAddress, ChunkBody, ChunkHeader};

use super::bmt_body::BMTBody;

pub struct SingleOwnerChunk {
    id: B256,
    owner: Address,
    signature: PrimitiveSignature,
    body: BMTBody,
}

impl SingleOwnerChunk {
    pub fn new(id: B256, owner: Address, signature: PrimitiveSignature, body: BMTBody) -> Self {
        Self {
            id,
            owner,
            signature,
            body,
        }
    }

    async fn to_sign(&self) -> FixedBytes<32> {
        let mut hasher = Keccak256::new();
        hasher.update(id);
        hasher.update(self.body.hash().await);

        hasher.finalize()
    }
}

impl swarm_primitives_traits::Chunk for SingleOwnerChunk {
    async fn address(&self) -> ChunkAddress {
        let mut hasher = Keccak256::new();
        hasher.update(self.header.id);
        hasher.update(self.header.owner);
        hasher.finalize()
    }

    async fn verify(&self, address: ChunkAddress) -> bool {
        // TODO: Signature verification needs to be inserted here to make sure that the body of the
        // chunk is correctly verified.
        address == self.address().await
    }
}
