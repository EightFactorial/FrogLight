//! Generated [`NetworkVersion`](super::NetworkVersion) implementations.
//!
//! Feel free to edit anything *except* the modules at the end of this file!

use core::error::Error;

use async_channel::{TryRecvError, TrySendError};
use bevy_ecs::world::DeferredWorld;
use bevy_tasks::IoTaskPool;
use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};

#[cfg(feature = "futures-lite")]
use crate::connection::FuturesLite;
use crate::{
    bevy::ClientConnection,
    connection::{AsyncConnection, ConnectionChannel, ConnectionError, EventConnection, Runtime},
    event::{ClientboundEvent, ServerboundEvent},
};

/// A trait for defining a [`Version`](froglight_common::version::Version)'s
/// network behavior.
pub trait NetworkVersion: PacketVersion {
    /// Create a new [`ClientConnection`] for this
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    #[cfg(feature = "futures-lite")]
    fn wrap_connection<C>(connection: C) -> ClientConnection
    where
        FuturesLite: Runtime<C>,
    {
        let (channel_a, channel_b) = ConnectionChannel::<
            VersionPacket<Self, Clientbound>,
            VersionPacket<Self, Serverbound>,
        >::new_pair(None);

        let (receiver, sender) = channel_a.into_split();
        ClientConnection::new_from(
            EventConnection::new(
                move |event, world| match sender.try_send(Self::event_to_packet(event, world)?) {
                    Ok(()) => Ok(()),
                    Err(TrySendError::Closed(_)) => todo!(),
                    Err(TrySendError::Full(_)) => unreachable!("ConnectionChannel is unbounded!"),
                },
                move |world| match receiver.try_recv() {
                    Ok(packet) => Self::packet_to_event(packet, world).map(Some),
                    Err(TryRecvError::Empty) => Ok(None),
                    Err(TryRecvError::Closed) => todo!(),
                },
            ),
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
        world: &mut DeferredWorld,
    ) -> Result<VersionPacket<Self, Serverbound>, ConnectionError>;

    /// Convert a [`VersionPacket<Self, Clientbound>`] into a
    /// [`ClientboundEvent`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        world: &mut DeferredWorld,
    ) -> Result<ClientboundEvent, ConnectionError>;

    /// An asynchronous packet handler that sends and receives packets over
    /// the given [`ConnectionChannel`].
    fn packet_handler<R: Runtime<C>, C>(
        connection: AsyncConnection<R, C, Self>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + 'static;
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
