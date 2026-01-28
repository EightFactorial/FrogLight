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
use facet::Facet;
use froglight_packet::version::{
    Clientbound, PacketState, PacketStateEnum, PacketVersion, Serverbound, VersionPacket,
};
use futures_lite::future::or;

use crate::{
    bevy::ClientConnection,
    connection::{
        AsyncConnection, Channel, ConnectionError, DecryptorMut, EncryptorMut, Runtime,
        RuntimeRead, RuntimeWrite,
    },
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
    #[allow(clippy::too_many_lines, reason = "Contains multiple async functions and packet logic")]
    fn connection_handler<R: Runtime<C>, C>(
        connection: AsyncConnection<R, C, Self>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'static {
        let (connection, channel) = connection.into_parts();
        let (mut reader, mut writer) = connection.into_split();
        let (mut read_buf, mut write_buf) = (Vec::<u8>::new(), Vec::<u8>::new());

        async move {
            let state = Mutex::new(PacketStateEnum::Handshake);

            // Receive a packet from the client and send it to the server.
            // Note: Exits if `state` is changed to cancel the `server_to_client` future.
            let client_to_server =
                async |writer: &mut EncryptorMut<R, R::Write>, writer_buf: &mut Vec<u8>| {
                    loop {
                        let packet: VersionPacket<Self, Serverbound> = channel.recv_async().await?;

                        // Note: Holding this lock after receiving the packet to prevent
                        // `server_to_client` from reading it while we potentially update it.
                        let mut state = state.lock().await;

                        match (packet, *state) {
                            (VersionPacket::Handshake(packet), PacketStateEnum::Handshake) => {
                                let transition = Self::Handshake::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf).await?;
                                if let Some(transition) = transition {
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Status(packet), PacketStateEnum::Status) => {
                                let transition = Self::Status::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf).await?;
                                if let Some(transition) = transition {
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Login(packet), PacketStateEnum::Login) => {
                                let transition = Self::Login::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf).await?;
                                if let Some(transition) = transition {
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Config(packet), PacketStateEnum::Config) => {
                                let transition = Self::Config::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf).await?;
                                if let Some(transition) = transition {
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Play(packet), PacketStateEnum::Play) => {
                                let transition = Self::Play::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf).await?;
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
            // Note: Exits if a `ConnectionUpdate` is returned to update the connection.
            let server_to_client =
                async |reader: &mut DecryptorMut<R, R::Read>, reader_buf: &mut Vec<u8>| {
                    loop {
                        match *state.lock().await {
                            PacketStateEnum::Handshake => {
                                let packet = VersionPacket::Handshake(
                                    read_packet(reader, reader_buf).await?,
                                );
                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Status => {
                                let packet =
                                    VersionPacket::Status(read_packet(reader, reader_buf).await?);
                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Login => {
                                let packet =
                                    VersionPacket::Login(read_packet(reader, reader_buf).await?);
                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Config => {
                                let packet =
                                    VersionPacket::Config(read_packet(reader, reader_buf).await?);
                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Play => {
                                let packet =
                                    VersionPacket::Play(read_packet(reader, reader_buf).await?);
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
                if let Some(update) =
                    or::<Result<Option<ConnectionUpdate>, Box<dyn Error + Send + Sync>>, _, _>(
                        (client_to_server)(&mut writer, &mut write_buf),
                        (server_to_client)(&mut reader, &mut read_buf),
                    )
                    .await?
                {
                    if let Some(threshold) = update.compression_threshold {
                        reader.compression().store(threshold, Ordering::Relaxed);
                        writer.compression().store(threshold, Ordering::Relaxed);
                    }
                    if let Some(_key) = update.encrypion_key {
                        *reader.decryptor() = Decryptor::new(&[0; _].into(), &[0; _].into());
                        *writer.encryptor() = Encryptor::new(&[0; _].into(), &[0; _].into());
                        reader.enabled().store(true, Ordering::Relaxed);
                        writer.enabled().store(true, Ordering::Relaxed);
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

/// Details for updating a connection.
#[derive(Debug, Default, Clone)]
pub struct ConnectionUpdate {
    /// A new compression threshold to set.
    pub compression_threshold: Option<i32>,
    /// A new encryption key to set.
    pub encrypion_key: Option<()>,
}

/// Read a packet of type `T` from the connection.
///
/// # Errors
///
/// Returns an error if reading the packet fails.
pub async fn read_packet<R: RuntimeRead<C>, C, T: Facet<'static>>(
    _reader: &mut DecryptorMut<R, C>,
    _buffer: &mut Vec<u8>,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    todo!()
}

/// Write a packet of type `T` to the connection.
///
/// # Errors
///
/// Returns an error if writing the packet fails.
pub async fn write_packet<R: RuntimeWrite<C>, C, T: Facet<'static>>(
    _packet: &T,
    _writer: &mut EncryptorMut<R, C>,
    _buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    todo!()
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
