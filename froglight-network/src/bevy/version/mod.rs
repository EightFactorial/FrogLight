//! Generated [`NetworkVersion`](super::NetworkVersion) implementations.
//!
//! Feel free to edit anything *except* the modules at the end of this file!

use core::error::Error;

use async_channel::{TryRecvError, TrySendError};
use bevy_ecs::world::EntityRef;
use bevy_tasks::IoTaskPool;
use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};

#[cfg(feature = "futures-lite")]
use crate::connection::FuturesLite;
use crate::{
    bevy::ClientConnection,
    connection::{AsyncConnection, Channel, ConnectionError, Runtime},
    event::{ClientboundEvent, ServerboundEvent},
};

/// A trait for defining a [`Version`](froglight_common::version::Version)'s
/// network behavior.
pub trait NetworkVersion: PacketVersion {
    /// Create a new [`ClientConnection`] for this
    /// [`Version`](froglight_common::version::Version).
    ///
    /// ## Note
    ///
    /// This method is only available when the `futures-lite` feature is
    /// enabled, as it relies on the [`FuturesLite`] [`Runtime`].
    #[must_use]
    #[cfg(feature = "futures-lite")]
    fn wrap_connection<C>(connection: C) -> ClientConnection
    where
        FuturesLite: Runtime<C>,
    {
        let (channel_a, channel_b) = Channel::new_pair(Some(64));
        let (receiver, sender) = channel_a.into_split();

        ClientConnection::new_from_parts(
            // Receive events from the ECS and send them as packets.
            Box::new(move |event, entity| {
                match sender.try_send(Self::event_to_packet(event, entity)?) {
                    Ok(()) => Ok(()),
                    Err(TrySendError::Closed(_)) => Err(ConnectionError::Closed),
                    Err(TrySendError::Full(_)) => Err(ConnectionError::Full),
                }
            }),
            // Receive packets from the server and convert them into events.
            Box::new(move |entity| match receiver.try_recv() {
                Ok(packet) => Self::packet_to_event(packet, entity).map(Some),
                Err(TryRecvError::Empty) => Ok(None),
                Err(TryRecvError::Closed) => Err(ConnectionError::Closed),
            }),
            // Spawn the packet handler task to communicate with the server.
            IoTaskPool::get().spawn(Self::packet_handler(
                AsyncConnection::<FuturesLite, C, Self>::new(connection, channel_b),
            )),
        )
    }

    /// Convert a [`ServerboundEvent`] into a
    /// [`VersionPacket<Self, Serverbound>`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn event_to_packet(
        event: ServerboundEvent,
        entity: EntityRef<'_>,
    ) -> Result<VersionPacket<Self, Serverbound>, ConnectionError>;

    /// Convert a [`VersionPacket<Self, Clientbound>`] into a
    /// [`ClientboundEvent`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        entity: EntityRef<'_>,
    ) -> Result<ClientboundEvent, ConnectionError>;

    /// An asynchronous packet handler that sends and receives packets over
    /// the given [`ConnectionChannel`].
    fn packet_handler<R: Runtime<C>, C>(
        connection: AsyncConnection<R, C, Self>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + 'static;
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
