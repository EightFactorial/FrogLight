//! Generated [`NetworkVersion`](super::NetworkVersion) implementations.
//!
//! Feel free to edit anything *except* the modules at the end of this file!

use core::error::Error;
use std::sync::atomic::Ordering;

use aes::cipher::KeyIvInit;
use async_channel::{TryRecvError, TrySendError};
use async_lock::Mutex;
use bevy_ecs::world::EntityRef;
use cfb8::{Decryptor, Encryptor};
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
    #[expect(clippy::too_many_lines, reason = "Contains multiple async functions and packet logic")]
    fn connection_handler<R: Runtime<C>, C>(
        connection: AsyncConnection<R, C, Self>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'static {
        let (conn, channel) = connection.into_parts();
        let (mut read, mut write) = conn.into_split();
        let (_read_buf, _write_buf) = (Vec::<u8>::new(), Vec::<u8>::new());

        let state = Mutex::new(PacketStateEnum::Handshake);

        async move {
            // Receive a packet from the client and send it to the server.
            // Note: Exits if `state` is changed to cancel the `server_to_client` future.
            #[expect(unreachable_code, unused_mut, unused_variables, reason = "WIP")]
            let client_to_server = async || {
                loop {
                    let packet: VersionPacket<Self, Serverbound> = channel.recv_async().await?;

                    // Note: Holding this lock after receiving the packet to prevent
                    // `server_to_client` from reading it while we potentially update it.
                    let mut state = state.lock().await;

                    match (packet, *state) {
                        (VersionPacket::Handshake(packet), PacketStateEnum::Handshake) => {
                            let transition = Self::Handshake::transition_state_to(&packet);
                            todo!("Send packet to the server");
                            if let Some(transition) = transition {
                                *state = transition;
                                return Ok(None);
                            }
                        }
                        (VersionPacket::Status(packet), PacketStateEnum::Status) => {
                            let transition = Self::Status::transition_state_to(&packet);
                            todo!("Send packet to the server");
                            if let Some(transition) = transition {
                                *state = transition;
                                return Ok(None);
                            }
                        }
                        (VersionPacket::Login(packet), PacketStateEnum::Login) => {
                            let transition = Self::Login::transition_state_to(&packet);
                            todo!("Send packet to the server");
                            if let Some(transition) = transition {
                                *state = transition;
                                return Ok(None);
                            }
                        }
                        (VersionPacket::Config(packet), PacketStateEnum::Config) => {
                            let transition = Self::Config::transition_state_to(&packet);
                            todo!("Send packet to the server");
                            if let Some(transition) = transition {
                                *state = transition;
                                return Ok(None);
                            }
                        }
                        (VersionPacket::Play(packet), PacketStateEnum::Play) => {
                            let transition = Self::Play::transition_state_to(&packet);
                            todo!("Send packet to the server");
                            if let Some(transition) = transition {
                                *state = transition;
                                return Ok(None);
                            }
                        }
                        #[cfg(feature = "tracing")]
                        (packet, state) => {
                            if tracing::enabled!(target: "froglight_network", tracing::Level::DEBUG)
                            {
                                tracing::error!(
                                    target: "froglight_network",
                                    "Received mismatched server packet for state \"{state}\": {packet:?}"
                                );
                            } else {
                                tracing::warn!(
                                    target: "froglight_network",
                                    "Received mismatched server packet for state \"{state}\""
                                );
                            }
                        }
                        #[cfg(not(feature = "tracing"))]
                        _ => {}
                    }
                }
            };

            // Receive a packet from the server and send it to the client.
            // Note: Loops indefinitely unless a `ConnectionUpdate` is returned.
            #[expect(unreachable_code, unused_variables, reason = "WIP")]
            let server_to_client = async || {
                loop {
                    match *state.lock().await {
                        PacketStateEnum::Handshake => {
                            let packet = VersionPacket::Handshake(todo!());
                            let update = Self::update_connection_details(&packet);
                            channel.send_async(packet).await?;
                            if update.is_some() {
                                return Ok(update);
                            }
                        }
                        PacketStateEnum::Status => {
                            let packet = VersionPacket::Status(todo!());
                            let update = Self::update_connection_details(&packet);
                            channel.send_async(packet).await?;
                            if update.is_some() {
                                return Ok(update);
                            }
                        }
                        PacketStateEnum::Login => {
                            let packet = VersionPacket::Login(todo!());
                            let update = Self::update_connection_details(&packet);
                            channel.send_async(packet).await?;
                            if update.is_some() {
                                return Ok(update);
                            }
                        }
                        PacketStateEnum::Config => {
                            let packet = VersionPacket::Config(todo!());
                            let update = Self::update_connection_details(&packet);
                            channel.send_async(packet).await?;
                            if update.is_some() {
                                return Ok(update);
                            }
                        }
                        PacketStateEnum::Play => {
                            let packet = VersionPacket::Play(todo!());
                            let update = Self::update_connection_details(&packet);
                            channel.send_async(packet).await?;
                            if update.is_some() {
                                return Ok(update);
                            }
                        }
                    }
                }
            };

            // Continuously handle packets from both directions.
            //
            // If a `ConnectionUpdate` is received, update the connection's settings
            // and continue.
            loop {
                if let Some(update) = or::<
                    Result<Option<ConnectionUpdate>, Box<dyn Error + Send + Sync>>,
                    _,
                    _,
                >((client_to_server)(), (server_to_client)())
                .await?
                {
                    if let Some(threshold) = update.compression_threshold {
                        read.compression().store(threshold, Ordering::Relaxed);
                        write.compression().store(threshold, Ordering::Relaxed);
                    }
                    if let Some(_key) = update.encrypion_key {
                        *read.decryptor() = Decryptor::new(&[0; _].into(), &[0; _].into());
                        *write.encryptor() = Encryptor::new(&[0; _].into(), &[0; _].into());
                    }
                }
            }
        }
    }

    /// Update connection details based on a received packet.
    fn update_connection_details(
        packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate>;

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

pub struct ConnectionUpdate {
    pub compression_threshold: Option<i32>,
    pub encrypion_key: Option<()>,
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
