use std::{future::Future, net::SocketAddr};

use froglight_protocol::{
    io::{FrogRead, FrogWrite},
    states::{Handshaking, Status},
    traits::{State, Version},
};

use super::{PingRequest, PingResponse, StatusRequest, StatusResponse};
use crate::{
    resolver::{Resolver, ResolverError, ResolverServerTask},
    Connection, ConnectionError,
};

mod v1_20_0;

pub trait Queryable: Version
where
    Handshaking: State<Self>,
    <Handshaking as State<Self>>::ClientboundPacket: FrogRead,
    <Handshaking as State<Self>>::ServerboundPacket: FrogWrite,

    Status: State<Self>,
    <Status as State<Self>>::ClientboundPacket: FrogRead,
    <Status as State<Self>>::ServerboundPacket: FrogWrite,
{
    fn get_status(
        event: StatusRequest<Self>,
        resolver: Resolver,
    ) -> impl Future<Output = Result<StatusResponse, ConnectionError>> + Send {
        async move {
            let addr = get_address(&event.url, resolver).await?;

            // Connect to the server and perform the handshake
            let mut connection = Connection::<Self, Handshaking>::connect(addr).await?;
            Self::handshake(&event.url, addr.port(), &mut connection).await?;

            // Query the status of the server
            let mut connection = connection.status();
            Self::status(event, &mut connection).await
        }
    }

    fn get_ping(
        event: PingRequest<Self>,
        resolver: Resolver,
    ) -> impl Future<Output = Result<PingResponse, ConnectionError>> + Send {
        async move {
            let addr = get_address(&event.url, resolver).await?;

            // Connect to the server and perform the handshake
            let mut connection = Connection::<Self, Handshaking>::connect(addr).await?;
            Self::handshake(&event.url, addr.port(), &mut connection).await?;

            // Query the latency of the server
            let mut connection = connection.status();
            Self::ping(event, &mut connection).await
        }
    }

    /// Perform a handshake with the server.
    fn handshake(
        url: &str,
        port: u16,
        connection: &mut Connection<Self, Handshaking>,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send;

    /// Query the status of the server.
    fn status(
        event: StatusRequest<Self>,
        connection: &mut Connection<Self, Status>,
    ) -> impl Future<Output = Result<StatusResponse, ConnectionError>> + Send;

    /// Send a ping to the server.
    fn ping(
        event: PingRequest<Self>,
        connection: &mut Connection<Self, Status>,
    ) -> impl Future<Output = Result<PingResponse, ConnectionError>> + Send;
}

/// Use the resolver to get the address of the given URL.
async fn get_address(url: &str, resolver: Resolver) -> Result<SocketAddr, ResolverError> {
    ResolverServerTask::url_lookup(resolver.resolver, resolver.extractor, url.to_string()).await
}
