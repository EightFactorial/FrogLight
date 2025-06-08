use core::net::SocketAddr;
use std::sync::Arc;

use async_lock::RwLock;
use async_trait::async_trait;
use futures_lite::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "crypto")]
use crate::connection::ConnectionCrypto;
use crate::{
    connection::{
        RawConnection,
        split::{CombinableConnection, SplittableConnection},
        state::ConnectionError,
    },
    froglight_io::transport_fns::{
        read_packet_outer, read_raw_outer, write_packet_outer, write_raw_outer,
    },
};

/// The default [`RawConnection`] implementation.
///
/// Wraps any [`AsyncReadExt`] and [`AsyncWriteExt`] stream.
pub struct IoTransport<S> {
    stream: S,
    peer: SocketAddr,
    scratch: Vec<u8>,

    #[cfg(feature = "crypto")]
    crypto: Option<ConnectionCrypto>,
    compression: Option<i32>,
}

impl<S> IoTransport<S> {
    /// Creates a new [`IoTransport`] instance.
    #[must_use]
    pub const fn wrap(stream: S, peer: SocketAddr) -> Self {
        #[cfg(feature = "crypto")]
        {
            Self { stream, peer, scratch: Vec::new(), crypto: None, compression: None }
        }

        #[cfg(not(feature = "crypto"))]
        {
            Self { stream, peer, scratch: Vec::new(), compression: None }
        }
    }
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Send + Sync + Unpin + 'static> RawConnection
    for IoTransport<S>
{
    #[inline]
    async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> { Ok(self.peer) }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    #[inline]
    #[cfg(feature = "crypto")]
    async fn set_crypto(&mut self, crypto: Option<ConnectionCrypto>) { self.crypto = crypto; }

    #[inline]
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        read_packet_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            self.compression,
            #[cfg(feature = "crypto")]
            self.crypto.as_mut(),
        )
        .await
    }

    #[inline]
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        write_packet_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            self.compression,
            #[cfg(feature = "crypto")]
            self.crypto.as_mut(),
        )
        .await
    }

    #[inline]
    async fn read_raw(&mut self, buf: &mut [u8]) -> Result<(), ConnectionError> {
        read_raw_outer(
            buf,
            &mut self.stream,
            #[cfg(feature = "crypto")]
            &mut self.crypto.as_mut(),
        )
        .await
    }

    #[allow(unused_mut)]
    async fn write_raw(&mut self, mut buf: &[u8]) -> Result<(), ConnectionError> {
        write_raw_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            #[cfg(feature = "crypto")]
            self.crypto.as_mut(),
        )
        .await
    }
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Clone + Send + Sync + Unpin + 'static> SplittableConnection
    for IoTransport<S>
{
    async fn split(&mut self) -> (Box<dyn CombinableConnection>, Box<dyn CombinableConnection>) {
        #[cfg(feature = "crypto")]
        let crypto = Arc::new(RwLock::new(core::mem::take(&mut self.crypto)));
        let compression = Arc::new(RwLock::new(self.compression));
        let Self { stream, peer, .. } = self;

        (
            Box::new(SplitIoTransport {
                stream: stream.clone(),
                peer: *peer,
                scratch: Vec::new(),
                #[cfg(feature = "crypto")]
                crypto: Arc::clone(&crypto),
                compression: Arc::clone(&compression),
            }),
            Box::new(SplitIoTransport {
                stream: stream.clone(),
                peer: *peer,
                scratch: Vec::new(),
                #[cfg(feature = "crypto")]
                crypto,
                compression,
            }),
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// The default [`RawConnection`] implementation.
///
/// Wraps any [`AsyncReadExt`] and [`AsyncWriteExt`] stream.
#[derive(Clone)]
pub struct SplitIoTransport<S> {
    pub(super) stream: S,
    pub(super) peer: SocketAddr,
    pub(super) scratch: Vec<u8>,

    #[cfg(feature = "crypto")]
    pub(super) crypto: Arc<RwLock<Option<ConnectionCrypto>>>,
    pub(super) compression: Arc<RwLock<Option<i32>>>,
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Send + Sync + Unpin + 'static> RawConnection
    for SplitIoTransport<S>
{
    #[inline]
    async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> { Ok(self.peer) }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { *self.compression.read().await }

    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) {
        *self.compression.write().await = threshold;
    }

    #[cfg(feature = "crypto")]
    async fn set_crypto(&mut self, crypto: Option<ConnectionCrypto>) {
        *self.crypto.write().await = crypto;
    }

    #[inline]
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        read_packet_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            *self.compression.read().await,
            #[cfg(feature = "crypto")]
            self.crypto.write().await.as_mut(),
        )
        .await
    }

    #[inline]
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        write_packet_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            *self.compression.read().await,
            #[cfg(feature = "crypto")]
            self.crypto.write().await.as_mut(),
        )
        .await
    }

    #[inline]
    async fn read_raw(&mut self, buf: &mut [u8]) -> Result<(), ConnectionError> {
        read_raw_outer(
            buf,
            &mut self.stream,
            #[cfg(feature = "crypto")]
            &mut self.crypto.write().await.as_mut(),
        )
        .await
    }

    #[inline]
    async fn write_raw(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        write_raw_outer(
            buf,
            &mut self.stream,
            &mut self.scratch,
            #[cfg(feature = "crypto")]
            self.crypto.write().await.as_mut(),
        )
        .await
    }
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Clone + Send + Sync + Unpin + 'static> CombinableConnection
    for SplitIoTransport<S>
{
    #[allow(unused_mut)]
    async fn recombine(
        &mut self,
        _: &mut dyn CombinableConnection,
    ) -> Box<dyn SplittableConnection> {
        let mut transport = IoTransport::wrap(self.stream.clone(), self.peer);

        #[cfg(feature = "crypto")]
        transport.set_crypto(core::mem::take(&mut *self.crypto.write().await)).await;

        Box::new(transport)
    }
}
