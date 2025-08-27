use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    hash::DefaultHasher,
    io::{IoSlice, IoSliceMut},
};

use async_net::TcpStream;
use futures_lite::{AsyncRead, AsyncWrite};

use crate::connection::{Combinable, RawWritable, Splittable};

impl Splittable for TcpStream {
    fn into_split(self: Box<Self>) -> (Box<dyn Combinable>, Box<dyn RawWritable>) {
        let mut hasher = DefaultHasher::new();

        // Hash the local address, peer address, and TTL
        if let Ok(local) = self.local_addr() {
            local.hash(&mut hasher);
        }
        if let Ok(peer) = self.peer_addr() {
            peer.hash(&mut hasher);
        }
        if let Ok(ttl) = self.ttl() {
            ttl.hash(&mut hasher);
        }

        let stream = HashedTcpStream(*self, hasher.finish());

        (Box::new(stream.clone()), Box::new(stream))
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`TcpStream`] with a hash for verifying matching copies.
#[derive(Clone)]
struct HashedTcpStream(TcpStream, u64);

impl Combinable for HashedTcpStream {
    fn into_combined(
        self: Box<Self>,
        write: Box<dyn RawWritable>,
    ) -> Result<Box<dyn Splittable>, Box<dyn Error + Send + Sync>> {
        if let Ok(write) = write.into_any().downcast::<Self>() {
            if self.1 == write.1 {
                Ok(Box::new(self.0))
            } else {
                Err(Box::new(AsyncNetError::SocketMismatch))
            }
        } else {
            Err(Box::new(AsyncNetError::IncorrectType))
        }
    }
}

impl AsyncRead for HashedTcpStream {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <TcpStream as AsyncRead>::poll_read(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <TcpStream as AsyncRead>::poll_read_vectored(Pin::new(&mut self.get_mut().0), cx, bufs)
    }
}
impl AsyncWrite for HashedTcpStream {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        <TcpStream as AsyncWrite>::poll_write(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        <TcpStream as AsyncWrite>::poll_write_vectored(Pin::new(&mut self.get_mut().0), cx, bufs)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        <TcpStream as AsyncWrite>::poll_flush(Pin::new(&mut self.get_mut().0), cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        <TcpStream as AsyncWrite>::poll_close(Pin::new(&mut self.get_mut().0), cx)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum AsyncNetError {
    SocketMismatch,
    IncorrectType,
}

impl AsyncNetError {
    const fn message(self) -> &'static str {
        match self {
            AsyncNetError::SocketMismatch => "Socket halves are for different connections",
            AsyncNetError::IncorrectType => "Attempted to recombine incorrect socket types",
        }
    }
}

impl Error for AsyncNetError {}
impl Display for AsyncNetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AsyncNetError: {}", self.message())
    }
}
