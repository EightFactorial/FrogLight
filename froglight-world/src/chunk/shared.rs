use alloc::sync::Arc;
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

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
    #[inline]
    #[must_use]
    pub const fn new_from(chunk: ArcSwap<Chunk>) -> Self { Self { chunk } }

    /// Return the inner [`ArcSwap<Chunk>`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> ArcSwap<Chunk> { self.chunk }

    /// Unwrap and return the inner [`Arc<Chunk>`].
    #[must_use]
    pub fn into_arc(self) -> Arc<Chunk> { self.chunk.into_inner() }

    /// Unwrap and return the inner [`Chunk`], cloning if necessary.
    #[must_use]
    pub fn into_chunk(self) -> Chunk { Arc::unwrap_or_clone(self.into_arc()) }

    /// Return a reference to the inner [`ArcSwap<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &ArcSwap<Chunk> { &self.chunk }

    /// Return a mutable reference to the inner [`ArcSwap<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn as_arc_mut(&mut self) -> &mut ArcSwap<Chunk> { &mut self.chunk }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<ArcSwap<Chunk>> for SharedChunk {
    #[inline]
    fn as_ref(&self) -> &ArcSwap<Chunk> { &self.chunk }
}
impl AsMut<ArcSwap<Chunk>> for SharedChunk {
    #[inline]
    fn as_mut(&mut self) -> &mut ArcSwap<Chunk> { &mut self.chunk }
}

impl Borrow<ArcSwap<Chunk>> for SharedChunk {
    #[inline]
    fn borrow(&self) -> &ArcSwap<Chunk> { &self.chunk }
}
impl BorrowMut<ArcSwap<Chunk>> for SharedChunk {
    #[inline]
    fn borrow_mut(&mut self) -> &mut ArcSwap<Chunk> { &mut self.chunk }
}

impl Deref for SharedChunk {
    type Target = ArcSwap<Chunk>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.chunk }
}
impl DerefMut for SharedChunk {
    #[inline]
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
