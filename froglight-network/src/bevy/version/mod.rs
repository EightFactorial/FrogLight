//! Generated [`NetworkVersion`](super::NetworkVersion) implementations.
//!
//! Feel free to edit anything *except* the modules at the end of this file!

use core::error::Error;

use async_channel::{TryRecvError, TrySendError};
use async_lock::Mutex;
use bevy_ecs::world::EntityRef;
use froglight_packet::version::{
    Clientbound, PacketState, PacketStateEnum, PacketVersion, Serverbound, VersionPacket,
};
use futures_lite::future::or;

use crate::{
    bevy::ClientConnection,
    connection::{AsyncConnection, Channel, ConnectionError, Runtime},
    event::{ClientboundEventEnum, ServerboundEventEnum},
};

/// A trait for defining a [`Version`](froglight_common::version::Version)'s
/// network behavior.
pub trait NetworkVersion: PacketVersion {
    /// Create a new [`ClientConnection`] for this
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    fn wrap_connection<R: Runtime<C>, C>(connection: C) -> ClientConnection {
        let (channel_a, channel_b) = Channel::new_pair(Some(64));
        let (receiver, sender) = channel_a.into_split();
        let connection = AsyncConnection::<R, C, Self>::new(connection, channel_b);

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
            // Spawn the connection handler task to communicate with the server.
            R::spawn_task(Self::connection_handler(connection)),
        )
    }

    /// A connection handler that sends/receives packets from/to the server.
    #[expect(clippy::option_map_unit_fn, reason = "Readability")]
    fn connection_handler<R: Runtime<C>, C>(
        connection: AsyncConnection<R, C, Self>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'static {
        let (conn, channel) = connection.into_parts();
        let (_read, _write) = conn.into_split();
        let (_read_buf, _write_buf) = (Vec::<u8>::new(), Vec::<u8>::new());
        let state = Mutex::new(PacketStateEnum::Handshake);

        async move {
            // Receive a packet from the client and send it to the server.
            // Note: Exits after every packet.
            let client_to_server = async || {
                let packet: VersionPacket<Self, Serverbound> = channel.recv_async().await?;
                // Note: Holding this lock after receiving the packet to prevent
                // `server_to_client` from reading it while we potentially update it.
                let mut state = state.lock().await;

                match (packet, *state) {
                    (VersionPacket::Handshake(packet), PacketStateEnum::Handshake) => {
                        Self::Handshake::transition_state_to(&packet).map(|s| *state = s);
                        todo!("Send packet to the server")
                    }
                    (VersionPacket::Status(packet), PacketStateEnum::Status) => {
                        Self::Status::transition_state_to(&packet).map(|s| *state = s);
                        todo!("Send packet to the server")
                    }
                    (VersionPacket::Login(packet), PacketStateEnum::Login) => {
                        Self::Login::transition_state_to(&packet).map(|s| *state = s);
                        todo!("Send packet to the server")
                    }
                    (VersionPacket::Config(packet), PacketStateEnum::Config) => {
                        Self::Config::transition_state_to(&packet).map(|s| *state = s);
                        todo!("Send packet to the server")
                    }
                    (VersionPacket::Play(packet), PacketStateEnum::Play) => {
                        Self::Play::transition_state_to(&packet).map(|s| *state = s);
                        todo!("Send packet to the server")
                    }
                    #[cfg(feature = "tracing")]
                    (packet, state) => {
                        if tracing::enabled!(target: "froglight_network", tracing::Level::DEBUG) {
                            tracing::error!(
                                "Received mismatched server packet for state \"{state}\": {packet:?}"
                            );
                        } else {
                            tracing::warn!(
                                "Received mismatched server packet for state \"{state}\""
                            );
                        }
                    }
                    #[cfg(not(feature = "tracing"))]
                    _ => {}
                }

                Result::<(), Box<dyn Error + Send + Sync>>::Ok(())
            };

            // Receive a packet from the server and send it to the client.
            // Note: Loops indefinitely.
            #[expect(unreachable_code, unused_variables, reason = "WIP")]
            let server_to_client = async || {
                loop {
                    match *state.lock().await {
                        PacketStateEnum::Handshake => {
                            let packet = todo!();
                            channel.send_async(VersionPacket::Handshake(packet)).await?;
                        }
                        PacketStateEnum::Status => {
                            let packet = todo!();
                            channel.send_async(VersionPacket::Status(packet)).await?;
                        }
                        PacketStateEnum::Login => {
                            let packet = todo!();
                            channel.send_async(VersionPacket::Login(packet)).await?;
                        }
                        PacketStateEnum::Config => {
                            let packet = todo!();
                            channel.send_async(VersionPacket::Config(packet)).await?;
                        }
                        PacketStateEnum::Play => {
                            let packet = todo!();
                            channel.send_async(VersionPacket::Play(packet)).await?;
                        }
                    }
                }
            };

            // Continuously handle packets from both directions.
            //
            // Note: When `client_to_server` finishes it will restart `server_to_client`
            // to update the connection state, ensuring the state is always updated.
            loop {
                or((client_to_server)(), (server_to_client)()).await?;
            }
        }
    }

    /// Convert a [`ServerboundEvent`] into a
    /// [`VersionPacket<Self, Serverbound>`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn event_to_packet(
        event: ServerboundEventEnum,
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
    ) -> Result<ClientboundEventEnum, ConnectionError>;
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
