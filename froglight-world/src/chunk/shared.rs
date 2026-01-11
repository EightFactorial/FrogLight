use core::ops::{Deref, DerefMut};

use arc_swap::ArcSwap;
#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;

use crate::chunk::Chunk;

/// A shared region of blocks in a world.
///
/// Useful for sharing chunk data between threads without waiting for locks.
///
/// See [`ArcSwap`](arc_swap::ArcSwapAny) for more information.
#[repr(transparent)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct SharedChunk {
    chunk: ArcSwap<Chunk>,
}

impl SharedChunk {
    /// Create a  [`SharedChunk`] from a [`Chunk`].
    #[must_use]
    pub fn new(chunk: Chunk) -> Self { Self { chunk: ArcSwap::from_pointee(chunk) } }

    /// Create a [`SharedChunk`] from an existing [`ArcSwap<Chunk>`].
    #[must_use]
    pub const fn new_from(chunk: ArcSwap<Chunk>) -> Self { Self { chunk } }

    /// Return the inner [`ArcSwap<Chunk>`].
    #[must_use]
    pub fn into_inner(self) -> ArcSwap<Chunk> { self.chunk }
}

impl Deref for SharedChunk {
    type Target = ArcSwap<Chunk>;

    fn deref(&self) -> &Self::Target { &self.chunk }
}
impl DerefMut for SharedChunk {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.chunk }
}

impl<T: Into<Chunk>> From<T> for SharedChunk {
    #[inline]
    fn from(chunk: T) -> Self { Self::new(chunk.into()) }
}
impl From<ArcSwap<Chunk>> for SharedChunk {
    #[inline]
    fn from(chunk: ArcSwap<Chunk>) -> Self { Self::new_from(chunk) }
}
