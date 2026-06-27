use alloc::sync::Arc;
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
use froglight_registry_template::types::AtomicArc;

use crate::chunk::Chunk;

/// A shared region of blocks in a world.
///
/// Useful for sharing chunk data between threads without waiting for locks.
///
/// See [`AtomicArc`] for more information.
#[repr(transparent)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct SharedChunk {
    chunk: AtomicArc<Chunk>,
}

impl SharedChunk {
    /// Create a  [`SharedChunk`] from a [`Chunk`].
    #[must_use]
    pub fn new(chunk: Chunk) -> Self { Self { chunk: AtomicArc::from(chunk) } }

    /// Create a [`SharedChunk`] from an existing [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn new_from(chunk: AtomicArc<Chunk>) -> Self { Self { chunk } }

    /// Return the inner [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> AtomicArc<Chunk> { self.chunk }

    /// Unwrap and return the inner [`Arc<Chunk>`].
    #[must_use]
    pub fn into_arc(self) -> Arc<Chunk> { self.chunk.into_owned() }

    /// Unwrap and return the inner [`Chunk`], cloning if necessary.
    #[must_use]
    pub fn into_chunk(self) -> Chunk { Arc::unwrap_or_clone(self.into_arc()) }

    /// Return a reference to the inner [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &AtomicArc<Chunk> { &self.chunk }

    /// Return a mutable reference to the inner [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn as_arc_mut(&mut self) -> &mut AtomicArc<Chunk> { &mut self.chunk }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<AtomicArc<Chunk>> for SharedChunk {
    #[inline]
    fn as_ref(&self) -> &AtomicArc<Chunk> { &self.chunk }
}
impl AsMut<AtomicArc<Chunk>> for SharedChunk {
    #[inline]
    fn as_mut(&mut self) -> &mut AtomicArc<Chunk> { &mut self.chunk }
}

impl Borrow<AtomicArc<Chunk>> for SharedChunk {
    #[inline]
    fn borrow(&self) -> &AtomicArc<Chunk> { &self.chunk }
}
impl BorrowMut<AtomicArc<Chunk>> for SharedChunk {
    #[inline]
    fn borrow_mut(&mut self) -> &mut AtomicArc<Chunk> { &mut self.chunk }
}

impl Clone for SharedChunk {
    #[inline]
    fn clone(&self) -> Self { Self::new_from(AtomicArc::new(self.load().clone())) }
}

impl Deref for SharedChunk {
    type Target = AtomicArc<Chunk>;

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
impl From<AtomicArc<Chunk>> for SharedChunk {
    #[inline]
    fn from(chunk: AtomicArc<Chunk>) -> Self { Self::new_from(chunk) }
}
