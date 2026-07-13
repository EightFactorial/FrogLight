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
/// Uses [`AtomicArc`] to allow shared access across threads.
///
/// # Note
///
/// Reading and writing to the chunk can be done at the same time, but all
/// current reads will not see the changes until the chunk is read again.
///
/// Due to how writing to a [`SharedChunk`] can cause race conditions, it is
/// recommended to batch write operations together where possible. If many
/// writes are performed at once changes may be lost!
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

    /// Clone the inner [`Arc`] and return it.
    ///
    /// This should be used when you want to send the [`Chunk`] to another
    /// thread.
    #[inline]
    #[must_use]
    pub fn load(&self) -> Arc<Chunk> { self.chunk.load_owned() }

    /// Return the inner [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> AtomicArc<Chunk> { self.chunk }

    /// Unwrap and return the inner [`Chunk`], cloning if necessary.
    #[inline]
    #[must_use]
    pub fn into_chunk(self) -> Chunk { Arc::unwrap_or_clone(self.chunk.into_owned()) }
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
    fn clone(&self) -> Self { Self::new_from(AtomicArc::new(self.load_owned())) }
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
