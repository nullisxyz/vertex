use swarm_primitives_traits::{ChunkAddress, Stamp, CHUNK_SIZE};

type ChunkData = [u8; CHUNK_SIZE];

pub struct Chunk {
    addr: ChunkAddress,
    data: ChunkData,
    tag_id: Option<u32>,
    stamp: Option<Box<dyn Stamp>>,
    depth: Option<u8>,
    bucket_depth: Option<u8>,
    immutable: Option<bool>,
}

impl Chunk {
    pub fn new(addr: ChunkAddress, data: ChunkData) -> Self {
        Self {
            addr,
            data,
            tag_id: None,
            stamp: None,
            depth: None,
            bucket_depth: None,
            immutable: None,
        }
    }
}

impl swarm_primitives_traits::Chunk for Chunk {
    fn address(&self) -> ChunkAddress {
        self.addr
    }

    fn data(&self) -> &[u8; CHUNK_SIZE as usize] {
        &self.data
    }

    fn tag_id(&self) -> Option<u32> {
        self.tag_id
    }

    fn stamp(&self) -> Option<impl Stamp> {
        todo!()
    }

    fn depth(&self) -> Option<u8> {
        self.depth
    }

    fn bucket_depth(&self) -> Option<u8> {
        self.bucket_depth
    }

    fn immutable(&self) -> Option<bool> {
        self.immutable
    }

    fn with_stamp(&mut self, stamp: impl swarm_primitives_traits::Stamp) -> Self {
        self.stamp = Some(Box::new(stamp));

        *self
    }

    fn with_batch(&mut self, bucket_depth: u8, immutable: bool) -> Self {
        self.bucket_depth = Some(bucket_depth);
        self.immutable = Some(immutable);

        *self
    }

    fn with_tag_id(&mut self, tag_id: u32) -> Self {
        self.tag_id = Some(tag_id);

        self
    }
}
