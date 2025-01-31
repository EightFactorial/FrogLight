use std::{
    any::TypeId,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use derive_more::derive::Deref;
use downcast_rs::DowncastSync;
use froglight_common::Identifier;
use glam::IVec3;
use hashbrown::HashMap;
use parking_lot::RwLock;

use super::{ChunkHandle, StoredChunk};
use crate::chunk::VecChunk;

/// A trait for types that can be stored in a [`ChunkStorage`].
pub trait Storable: DowncastSync + 'static {
    /// The identifier of the storage.
    fn identifier(&self) -> &'static Identifier;

    /// Insert a chunk into the storage.
    fn insert_chunk(&mut self, chunk: VecChunk) -> ChunkHandle;
    /// Remove a chunk from the storage.
    fn remove_chunk(&mut self, handle: ChunkHandle);

    /// Get a block from the chunk.
    fn get_block(&self, handle: &ChunkHandle, position: IVec3) -> Option<u32>;
    /// Set a block in the chunk.
    fn set_block(&mut self, handle: &ChunkHandle, position: IVec3, block: u32) -> Option<u32>;
}

macro_rules! create_storage {
    ($ident:ident, $name:expr, $height:expr, $offset:expr) => {
        #[derive(Clone, Deref)]
        #[expect(missing_docs)]
        pub struct $ident(Arc<(AtomicU32, RwLock<HashMap<u32, StoredChunk<$height, $offset>>>)>);
        impl Storable for $ident {
            fn identifier(&self) -> &'static Identifier {
                static IDENTIFIER: Identifier = Identifier::const_new($name);
                &IDENTIFIER
            }

            fn insert_chunk(&mut self, chunk: VecChunk) -> ChunkHandle {
                let index = self.0 .0.fetch_add(1, Ordering::Relaxed);
                self.0 .1.write().insert(
                    index,
                    match chunk.try_into_array() {
                        Ok(array) => StoredChunk::Array(array),
                        Err(chunk) => StoredChunk::Vec(chunk),
                    },
                );
                ChunkHandle { type_id: TypeId::of::<Self>(), index }
            }
            fn remove_chunk(&mut self, handle: ChunkHandle) {
                self.0 .1.write().remove(&handle.index);
            }

            fn get_block(&self, handle: &ChunkHandle, position: IVec3) -> Option<u32> {
                self.0 .1.read().get(&handle.index)?.get_block_raw(position)
            }
            fn set_block(
                &mut self,
                handle: &ChunkHandle,
                position: IVec3,
                block: u32,
            ) -> Option<u32> {
                self.0 .1.write().get_mut(&handle.index)?.set_block_raw(position, block)
            }
        }
    };
}

create_storage!(Overworld, "minecraft:overworld", 24, -16);
create_storage!(Nether, "minecraft:the_nether", 16, 0);
create_storage!(End, "minecraft:the_end", 16, 0);
