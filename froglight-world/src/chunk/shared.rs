use alloc::sync::Arc;
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

use crate::chunk::Chunk;

/// A shared region of blocks in a world.
///
/// Uses [`Arc`] to allow read-only access to the [`Chunk`] from multiple
/// threads.
///
/// # Note
///
/// There is no shared mutable access to the [`Chunk`]!
///
/// To modify it you must clone the existing [`Chunk`], modify that, and
/// replace the [`SharedChunk`] contents via [`SharedChunk::store`].
#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, Component))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SharedChunk(Arc<Chunk>);

impl SharedChunk {
    /// Create a  [`SharedChunk`] from a [`Chunk`].
    #[inline]
    #[must_use]
    pub fn new(chunk: Chunk) -> Self { Self::new_arc(Arc::new(chunk)) }

    /// Create a [`SharedChunk`] from an existing [`Arc<Chunk>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(chunk: Arc<Chunk>) -> Self { Self(chunk) }

    /// Clone the inner [`Arc<Chunk>`] and return it.
    ///
    /// Useful for sending read-only references to other threads.
    #[inline]
    #[must_use]
    pub fn load(&self) -> Arc<Chunk> { Arc::clone(&self.0) }

    /// Clone and return the inner [`Chunk`].
    ///
    /// # Note
    ///
    /// This should only be used when you need to modify the [`Chunk`].
    ///
    /// If you only need read-only access consider [`SharedChunk::load`]
    /// instead.
    #[inline]
    #[must_use]
    pub fn clone_inner(&self) -> Chunk { Arc::as_ref(&self.0).clone() }

    /// Replace the existing [`Chunk`] immediately.
    ///
    /// # Note
    ///
    /// This will fully replace the inner [`Chunk`]. The existing [`Chunk`] will
    /// remain in memory until all references to it are dropped.
    ///
    /// This is identical to [`SharedChunk::store_arc`].
    #[inline]
    pub fn store(&mut self, chunk: Chunk) { self.0 = Arc::new(chunk); }

    /// Replace the existing [`Arc<Chunk>`] immediately.
    ///
    /// # Note
    ///
    /// This will fully replace the inner [`Chunk`]. The existing [`Chunk`] will
    /// remain in memory until all references to it are dropped.
    ///
    /// This is identical to [`SharedChunk::store`].
    #[inline]
    pub fn store_arc(&mut self, chunk: Arc<Chunk>) { self.0 = chunk; }

    /// Return the inner [`AtomicArc<Chunk>`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Arc<Chunk> { self.0 }

    /// Unwrap and return the inner [`Chunk`], cloning if necessary.
    #[inline]
    #[must_use]
    pub fn into_chunk(self) -> Chunk { Arc::unwrap_or_clone(self.0) }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn as_ref(&self) -> &Arc<Chunk> { &self.0 }
}
impl AsRef<Chunk> for SharedChunk {
    #[inline]
    fn as_ref(&self) -> &Chunk { &self.0 }
}
impl AsMut<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn as_mut(&mut self) -> &mut Arc<Chunk> { &mut self.0 }
}

impl Borrow<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn borrow(&self) -> &Arc<Chunk> { &self.0 }
}
impl Borrow<Chunk> for SharedChunk {
    #[inline]
    fn borrow(&self) -> &Chunk { &self.0 }
}
impl BorrowMut<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Arc<Chunk> { &mut self.0 }
}

impl Deref for SharedChunk {
    type Target = Arc<Chunk>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for SharedChunk {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: Into<Chunk>> From<T> for SharedChunk {
    #[inline]
    fn from(chunk: T) -> Self { Self::new(chunk.into()) }
}
impl From<Arc<Chunk>> for SharedChunk {
    #[inline]
    fn from(chunk: Arc<Chunk>) -> Self { Self::new_arc(chunk) }
}
