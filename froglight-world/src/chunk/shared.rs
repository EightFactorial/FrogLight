use alloc::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
use froglight_registry_template::types::{ArcBorrow, AtomicArc};

use crate::chunk::Chunk;

/// A shared region of blocks in a world.
///
/// Uses [`AtomicArc`] to allow shared access across threads.
///
/// # Note
///
/// You should write to the [`Chunk`] as little/fast as possible to prevent
/// blocking read operations!
#[repr(transparent)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct SharedChunk {
    chunk: AtomicArc<Chunk>,
}

impl SharedChunk {
    /// Create a  [`SharedChunk`] from a [`Chunk`].
    #[must_use]
    pub fn new(chunk: Chunk) -> Self { Self::new_arc(Arc::new(chunk)) }

    /// Create a [`SharedChunk`] from an existing [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub fn new_arc(chunk: Arc<Chunk>) -> Self { Self { chunk: AtomicArc::new(chunk) } }

    /// Get a reference to the inner [`Arc`] without cloning it.
    ///
    /// There are a limited number of free borrow slots and so this should not
    /// be used for long-term storage.
    ///
    /// Consider using [`SharedChunk::load_owned`] instead.
    #[inline]
    pub fn load(&self) -> ArcBorrow<Chunk> { self.chunk.load() }

    /// Clone the inner [`Arc`] and return it.
    ///
    /// This should be used when you want to store or send read-only access to
    /// the [`Chunk`] to another thread.
    ///
    /// For example, mesh generation or physics calculations.
    #[inline]
    #[must_use]
    pub fn load_owned(&self) -> Arc<Chunk> { self.chunk.load_owned() }

    /// Replace the existing [`Chunk`] immediately.
    ///
    /// # Note
    ///
    /// This will fully replace the inner shared [`Chunk`]. Existing threads
    /// will continue to use the old [`Chunk`], while new threads will use
    /// the new [`Chunk`].
    ///
    /// This is identical to [`SharedChunk::store_arc`].
    #[inline]
    pub fn store(&mut self, chunk: Chunk) { self.store_arc(Arc::new(chunk)); }

    /// Replace the existing [`Arc<Chunk>`] immediately.
    ///
    /// # Note
    ///
    /// This will fully replace the inner shared [`Chunk`]. Existing threads
    /// will continue to use the old [`Chunk`], while new threads will use
    /// the new [`Chunk`].
    ///
    /// This is identical to [`SharedChunk::store`].
    #[inline]
    pub fn store_arc(&mut self, chunk: Arc<Chunk>) { self.chunk.store(chunk); }

    /// Clone and return the inner [`Chunk`].
    ///
    /// # Note
    ///
    /// This should only be used when you need to modify the [`Chunk`].
    ///
    /// If you only need read-only access, use [`SharedChunk::load_owned`]
    /// instead.
    #[inline]
    #[must_use]
    pub fn clone_chunk(&self) -> Chunk { Arc::as_ref(&self.load()).clone() }

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

impl Clone for SharedChunk {
    #[inline]
    fn clone(&self) -> Self { Self::new_arc(self.load_owned()) }
}

impl<T: Into<Chunk>> From<T> for SharedChunk {
    #[inline]
    fn from(chunk: T) -> Self { Self::new(chunk.into()) }
}
impl From<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn from(chunk: Arc<Chunk>) -> Self { Self::new_arc(chunk) }
}
