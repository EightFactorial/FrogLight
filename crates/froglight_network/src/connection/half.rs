use alloc::boxed::Box;
use core::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};

use downcast_rs::DowncastSync;

/// The readable half of a raw connection.
pub trait RawReadable: DowncastSync + 'static {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>>;

    #[cfg(feature = "std")]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }

        self.poll_read(cx, &mut [])
    }
}

// -------------------------------------------------------------------------------------------------

impl<R: RawReadable + ?Sized> RawReadableExt for R {}
pub trait RawReadableExt: RawReadable {
    /// Reads some bytes from the byte stream.
    ///
    /// On success, returns the total number of bytes read.
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a, Self>
    where
        Self: Unpin,
    {
        ReadFuture { reader: self, buf }
    }

    /// Reads the exact number of bytes required to fill buf.
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExactFuture<'a, Self>
    where
        Self: Unpin,
    {
        ReadExactFuture { reader: self, buf }
    }
}

pub struct ReadFuture<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
}

impl<R: Unpin + ?Sized> Unpin for ReadFuture<'_, R> {}
impl<R: RawReadable + Unpin + ?Sized> Future for ReadFuture<'_, R> {
    type Output = Result<usize, Box<dyn Error + Send + Sync>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { reader, buf } = &mut *self;
        RawReadable::poll_read(Pin::new(*reader), cx, buf)
    }
}

pub struct ReadExactFuture<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
}

impl<R: Unpin + ?Sized> Unpin for ReadExactFuture<'_, R> {}
impl<R: RawReadable + Unpin + ?Sized> Future for ReadExactFuture<'_, R> {
    type Output = Result<(), Box<dyn Error + Send + Sync>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { reader, buf } = &mut *self;

        while !buf.is_empty() {
            let n = futures_lite::ready!(RawReadable::poll_read(Pin::new(*reader), cx, buf))?;
            let (_, rest) = core::mem::take(buf).split_at_mut(n);
            *buf = rest;

            if n == 0 {
                return Poll::Ready(Err("std::io::ErrorKind::UnexpectedEoF".into()));
            }
        }

        Poll::Ready(Ok(()))
    }
}

// -------------------------------------------------------------------------------------------------

/// The writable half of a raw connection.
pub trait RawWritable: DowncastSync + 'static {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>>;

    #[cfg(feature = "std")]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_write(cx, b);
            }
        }

        self.poll_write(cx, &[])
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync>>>;

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync>>>;
}

// -------------------------------------------------------------------------------------------------

impl<R: RawWritable + ?Sized> RawWritableExt for R {}
pub trait RawWritableExt: RawWritable {
    /// Writes some bytes into the byte stream.
    ///
    /// Returns the number of bytes written from the start of the buffer.
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> WriteFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteFuture { writer: self, buf }
    }

    /// Writes an entire buffer into the byte stream.
    ///
    /// It will not return before the entire buffer is successfully written or
    /// an error occurs.
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAllFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteAllFuture { writer: self, buf }
    }
}

pub struct WriteFuture<'a, R: ?Sized> {
    writer: &'a mut R,
    buf: &'a [u8],
}

impl<R: Unpin + ?Sized> Unpin for WriteFuture<'_, R> {}
impl<R: RawWritable + Unpin + ?Sized> Future for WriteFuture<'_, R> {
    type Output = Result<usize, Box<dyn Error + Send + Sync>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { writer, buf } = &mut *self;
        RawWritable::poll_write(Pin::new(*writer), cx, buf)
    }
}

pub struct WriteAllFuture<'a, R: ?Sized> {
    writer: &'a mut R,
    buf: &'a [u8],
}

impl<R: Unpin + ?Sized> Unpin for WriteAllFuture<'_, R> {}
impl<R: RawWritable + Unpin + ?Sized> Future for WriteAllFuture<'_, R> {
    type Output = Result<(), Box<dyn Error + Send + Sync>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { writer, buf } = &mut *self;

        while !buf.is_empty() {
            let n = futures_lite::ready!(Pin::new(&mut **writer).poll_write(cx, buf))?;
            let (_, rest) = core::mem::take(buf).split_at(n);
            *buf = rest;

            if n == 0 {
                return Poll::Ready(Err("std::io::ErrorKind::WriteZero".into()));
            }
        }

        Poll::Ready(Ok(()))
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
impl<T: futures_lite::AsyncRead + Send + Sync + 'static> RawReadable for T {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncRead>::poll_read(self, cx, buf)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }

    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncRead>::poll_read_vectored(self, cx, bufs)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }
}

#[cfg(feature = "std")]
impl<T: futures_lite::AsyncWrite + Send + Sync + 'static> RawWritable for T {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncWrite>::poll_write(self, cx, buf)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncWrite>::poll_write_vectored(self, cx, bufs)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncWrite>::poll_flush(self, cx)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync>>> {
        <Self as futures_lite::AsyncWrite>::poll_close(self, cx)
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })
    }
}
