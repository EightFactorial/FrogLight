//! TODO
#![allow(missing_docs, reason = "WIP")]

use alloc::boxed::Box;
use core::{
    error::Error,
    ops::{Deref, DerefMut},
};
#[cfg(feature = "std")]
use core::{
    pin::Pin,
    task::{Context, Poll},
};

mod half;
pub use half::{RawReadable, RawReadableExt, RawWritable, RawWritableExt};

mod split;
pub use split::{Combinable, Splittable};

#[repr(transparent)]
pub struct RawConnection {
    conn: Box<dyn Splittable>,
}

impl RawConnection {
    /// Create a new [`Connection`].
    #[must_use]
    pub fn new<T: Splittable>(conn: T) -> Self { Self::new_boxed(Box::new(conn)) }

    /// Create a new [`Connection`] from an already boxed connection.
    #[inline]
    #[must_use]
    pub const fn new_boxed(conn: Box<dyn Splittable>) -> Self { Self { conn } }

    /// Split this [`Connection`] into a [`ReadConnection`] and a
    /// [`WriteConnection`].
    #[must_use]
    pub fn into_split(self) -> (RawReadConnection, RawWriteConnection) {
        let (read, write) = self.conn.into_split();
        (RawReadConnection { conn: read }, RawWriteConnection { conn: write })
    }
}

impl AsRef<dyn RawReadable> for RawConnection {
    fn as_ref(&self) -> &dyn RawReadable { self.conn.as_ref() }
}
impl AsMut<dyn RawReadable> for RawConnection {
    fn as_mut(&mut self) -> &mut dyn RawReadable { self.conn.as_mut() }
}

impl AsRef<dyn RawWritable> for RawConnection {
    fn as_ref(&self) -> &dyn RawWritable { self.conn.as_ref() }
}
impl AsMut<dyn RawWritable> for RawConnection {
    fn as_mut(&mut self) -> &mut dyn RawWritable { self.conn.as_mut() }
}

impl<T: Splittable> From<T> for RawConnection {
    fn from(value: T) -> Self { Self::new(value) }
}
impl From<Box<dyn Splittable>> for RawConnection {
    fn from(value: Box<dyn Splittable>) -> Self { Self::new_boxed(value) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
pub struct RawReadConnection {
    conn: Box<dyn Combinable>,
}

impl RawReadConnection {
    /// Create a new [`ReadConnection`].
    #[must_use]
    pub fn new<T: Combinable>(conn: T) -> Self { Self::new_boxed(Box::new(conn)) }

    /// Create a new [`ReadConnection`] from an already boxed connection.
    #[inline]
    #[must_use]
    pub const fn new_boxed(conn: Box<dyn Combinable>) -> Self { Self { conn } }

    /// Attempt to recombine this [`ReadConnection`] with a [`WriteConnection`].
    ///
    /// # Errors
    ///
    /// If the underlying connections cannot be combined, an error is returned.
    pub fn try_combine(
        self,
        write: RawWriteConnection,
    ) -> Result<RawConnection, Box<dyn Error + Send + Sync>> {
        self.conn.into_combined(write.conn).map(RawConnection::new_boxed)
    }
}

impl Deref for RawReadConnection {
    type Target = dyn RawReadable;

    fn deref(&self) -> &Self::Target { self.conn.as_ref() }
}
impl DerefMut for RawReadConnection {
    fn deref_mut(&mut self) -> &mut Self::Target { self.conn.as_mut() }
}

impl AsRef<dyn RawReadable> for RawReadConnection {
    fn as_ref(&self) -> &dyn RawReadable { self.conn.as_ref() }
}
impl AsMut<dyn RawReadable> for RawReadConnection {
    fn as_mut(&mut self) -> &mut dyn RawReadable { self.conn.as_mut() }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
pub struct RawWriteConnection {
    conn: Box<dyn RawWritable>,
}

impl RawWriteConnection {
    /// Create a new [`WriteConnection`].
    #[must_use]
    pub fn new<T: RawWritable>(conn: T) -> Self { Self::new_boxed(Box::new(conn)) }

    /// Create a new [`WriteConnection`] from an already boxed connection.
    #[inline]
    #[must_use]
    pub const fn new_boxed(conn: Box<dyn RawWritable>) -> Self { Self { conn } }

    /// Attempt to recombine this [`WriteConnection`] with a [`ReadConnection`].
    ///
    /// # Errors
    ///
    /// If the underlying connections cannot be combined, an error is returned.
    pub fn try_combine(
        self,
        read: RawReadConnection,
    ) -> Result<RawConnection, Box<dyn Error + Send + Sync>> {
        read.conn.into_combined(self.conn).map(RawConnection::new_boxed)
    }
}

impl Deref for RawWriteConnection {
    type Target = dyn RawWritable;

    fn deref(&self) -> &Self::Target { self.conn.as_ref() }
}
impl DerefMut for RawWriteConnection {
    fn deref_mut(&mut self) -> &mut Self::Target { self.conn.as_mut() }
}

impl AsRef<dyn RawWritable> for RawWriteConnection {
    fn as_ref(&self) -> &dyn RawWritable { self.conn.as_ref() }
}
impl AsMut<dyn RawWritable> for RawWriteConnection {
    fn as_mut(&mut self) -> &mut dyn RawWritable { self.conn.as_mut() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
impl futures_lite::AsyncRead for RawConnection {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawReadable>::poll_read(self.as_mut(), cx, buf).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawReadable>::poll_read_vectored(self.as_mut(), cx, bufs)
            .map_err(std::io::Error::other)
    }
}
#[cfg(feature = "std")]
impl futures_lite::AsyncWrite for RawConnection {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawWritable>::poll_write(self.as_mut(), cx, buf).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawWritable>::poll_write_vectored(self.as_mut(), cx, bufs)
            .map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        <dyn RawWritable>::poll_flush(self.as_mut(), cx).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        <dyn RawWritable>::poll_close(self.as_mut(), cx).map_err(std::io::Error::other)
    }
}

#[cfg(feature = "std")]
impl futures_lite::AsyncRead for RawReadConnection {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawReadable>::poll_read(self.as_mut(), cx, buf).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawReadable>::poll_read_vectored(self.as_mut(), cx, bufs)
            .map_err(std::io::Error::other)
    }
}

#[cfg(feature = "std")]
impl futures_lite::AsyncWrite for RawWriteConnection {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawWritable>::poll_write(self.as_mut(), cx, buf).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <dyn RawWritable>::poll_write_vectored(self.as_mut(), cx, bufs)
            .map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        <dyn RawWritable>::poll_flush(self.as_mut(), cx).map_err(std::io::Error::other)
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        <dyn RawWritable>::poll_close(self.as_mut(), cx).map_err(std::io::Error::other)
    }
}
