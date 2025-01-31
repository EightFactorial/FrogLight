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

use super::{HandleInternal, StoredChunk};
use crate::chunk::VecChunk;

/// A trait for types that can be stored in a [`ChunkStorage`].
pub trait Storable: DowncastSync + 'static {
    /// The storage identifier.
    fn identifier(&self) -> &'static Identifier;

    /// Insert a chunk into storage.
    fn insert_chunk(&mut self, chunk: VecChunk) -> HandleInternal;
    /// Remove a chunk from storage.
    fn remove_chunk(&mut self, handle: HandleInternal);

    /// Get a block from the chunk.
    fn get_block(&self, handle: &HandleInternal, position: IVec3) -> Option<u32>;
    /// Set a block in the chunk.
    fn set_block(&mut self, handle: &HandleInternal, position: IVec3, block: u32) -> Option<u32>;
}

macro_rules! create_storage {
    ($ident:ident, $name:expr, $height:expr, $offset:expr) => {
        /// A thread-safe storage for chunks.
        ///
        /// Can be cloned to share across multiple threads.
        #[derive(Clone, Deref)]
        pub struct $ident(Arc<(AtomicU32, RwLock<HashMap<u32, StoredChunk<$height, $offset>>>)>);
        impl Storable for $ident {
            fn identifier(&self) -> &'static Identifier {
                static IDENTIFIER: Identifier = Identifier::const_new($name);
                &IDENTIFIER
            }

            fn insert_chunk(&mut self, chunk: VecChunk) -> HandleInternal {
                let index = self.0 .0.fetch_add(1, Ordering::Relaxed);
                self.0 .1.write().insert(
                    index,
                    match chunk.try_into_array() {
                        Ok(array) => StoredChunk::Array(array),
                        Err(chunk) => StoredChunk::Vec(chunk),
                    },
                );
                HandleInternal(TypeId::of::<Self>(), index)
            }
            fn remove_chunk(&mut self, handle: HandleInternal) {
                self.0 .1.write().remove(&handle.1);
            }

            fn get_block(&self, handle: &HandleInternal, position: IVec3) -> Option<u32> {
                self.0 .1.read().get(&handle.1)?.get_block_raw(position)
            }
            fn set_block(
                &mut self,
                handle: &HandleInternal,
                position: IVec3,
                block: u32,
            ) -> Option<u32> {
                self.0 .1.write().get_mut(&handle.1)?.set_block_raw(position, block)
            }
        }
    };
}

create_storage!(Overworld, "minecraft:overworld", 24, -16);
create_storage!(Nether, "minecraft:the_nether", 16, 0);
create_storage!(End, "minecraft:the_end", 16, 0);
