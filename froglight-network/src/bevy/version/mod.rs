//! Generated [`NetworkVersion`] implementations.
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
        RuntimeRead, RuntimeWrite, encryption::write_slice_prefix,
    },
    event::{ClientboundEventEnum, ServerboundEventEnum},
};

/// A trait for defining a [`Version`](froglight_common::version::Version)'s
/// network behavior.
pub trait NetworkVersion: PacketVersion {
    /// Create a new [`ClientConnection`] for this
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    fn wrap_connection<R: Runtime<C>, C: Send>(
        connection: C,
        exit_on_error: bool,
    ) -> ClientConnection {
        let (channel_a, channel_b) = Channel::new_pair(Some(64));
        let (receiver, sender) = channel_a.into_split();
        let connection = AsyncConnection::<R, C, Self>::new(connection, channel_b);

        ClientConnection::new_from_parts(
            // Receive events from the ECS and send them as packets.
            Box::new(move |event, entity| {
                match Self::event_to_packet(event, entity)?.map(|packet| sender.try_send(packet)) {
                    Some(Ok(())) | None => Ok(()),
                    Some(Err(err)) => match err {
                        TrySendError::Full(_) => Err(ConnectionError::Full),
                        TrySendError::Closed(_) => Err(ConnectionError::Closed),
                    },
                }
            }),
            // Receive packets from the server and convert them into events.
            Box::new(move |entity| match receiver.try_recv() {
                Ok(packet) => Self::packet_to_event(packet, entity),
                Err(TryRecvError::Empty) => Ok(None),
                Err(TryRecvError::Closed) => Err(ConnectionError::Closed),
            }),
            // Spawn the connection handler task to communicate with the server.
            R::spawn_task(Self::connection_handler(connection, exit_on_error)),
        )
    }

    /// A connection handler that sends/receives packets from/to the server.
    #[allow(clippy::too_many_lines, reason = "Contains multiple async functions and packet logic")]
    fn connection_handler<R: Runtime<C>, C: Send>(
        connection: AsyncConnection<R, C, Self>,
        exit_on_error: bool,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'static {
        let (connection, channel) = connection.into_parts();
        let (mut reader, mut writer) = connection.into_split();
        let (mut read_buf, mut write_buf_a, mut write_buf_b) =
            (Vec::<u8>::new(), Vec::<u8>::new(), Vec::<u8>::new());

        async move {
            let state = Mutex::new(PacketStateEnum::Handshake);

            // Receive a packet from the client and send it to the server.
            // Note: Exits if `state` is changed to cancel the `server_to_client` future.
            let client_to_server =
                async |writer: &mut EncryptorMut<R, R::Write>,
                       writer_buf_a: &mut Vec<u8>,
                       writer_buf_b: &mut Vec<u8>| {
                    loop {
                        let packet: VersionPacket<Self, Serverbound> = channel.recv_async().await?;

                        #[cfg(feature = "tracing")]
                        tracing::trace!(target: "froglight_network", "Sending Packet: {packet:?}");

                        // Note: Holding this lock after receiving the packet to prevent
                        // `server_to_client` from reading it while we potentially update it.
                        let mut state = state.lock().await;

                        match (packet, *state) {
                            (VersionPacket::Handshake(packet), PacketStateEnum::Handshake) => {
                                let transition = Self::Handshake::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf_a, writer_buf_b).await?;
                                if let Some(transition) = transition {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!(target: "froglight_network", "Transitioning connection from `Handshake` to `{transition}`");
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Status(packet), PacketStateEnum::Status) => {
                                let transition = Self::Status::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf_a, writer_buf_b).await?;
                                if let Some(transition) = transition {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!(target: "froglight_network", "Transitioning connection from `Status` to `{transition}`");
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Login(packet), PacketStateEnum::Login) => {
                                let transition = Self::Login::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf_a, writer_buf_b).await?;
                                if let Some(transition) = transition {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!(target: "froglight_network", "Transitioning connection from `Login` to `{transition}`");
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Config(packet), PacketStateEnum::Config) => {
                                let transition = Self::Config::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf_a, writer_buf_b).await?;
                                if let Some(transition) = transition {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!(target: "froglight_network", "Transitioning connection from `Config` to `{transition}`");
                                    *state = transition;
                                    return Ok(None);
                                }
                            }
                            (VersionPacket::Play(packet), PacketStateEnum::Play) => {
                                let transition = Self::Play::transition_state_to(&packet);
                                write_packet(&packet, writer, writer_buf_a, writer_buf_b).await?;
                                if let Some(transition) = transition {
                                    #[cfg(feature = "tracing")]
                                    tracing::debug!(target: "froglight_network", "Transitioning connection from `Play` to `{transition}`");
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
                        let state = *state.lock().await;

                        match state {
                            PacketStateEnum::Handshake => {
                                let Some(packet) = read_packet(reader, reader_buf, exit_on_error)
                                    .await?
                                    .map(VersionPacket::Handshake)
                                else {
                                    continue;
                                };

                                #[cfg(feature = "tracing")]
                                tracing::trace!(target: "froglight_network", "Received Packet: {packet:?}");

                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Status => {
                                let Some(packet) = read_packet(reader, reader_buf, exit_on_error)
                                    .await?
                                    .map(VersionPacket::Status)
                                else {
                                    continue;
                                };

                                #[cfg(feature = "tracing")]
                                tracing::trace!(target: "froglight_network", "Received Packet: {packet:?}");

                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Login => {
                                let Some(packet) = read_packet(reader, reader_buf, exit_on_error)
                                    .await?
                                    .map(VersionPacket::Login)
                                else {
                                    continue;
                                };

                                #[cfg(feature = "tracing")]
                                tracing::trace!(target: "froglight_network", "Received Packet: {packet:?}");

                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Config => {
                                let Some(packet) = read_packet(reader, reader_buf, exit_on_error)
                                    .await?
                                    .map(VersionPacket::Config)
                                else {
                                    continue;
                                };

                                #[cfg(feature = "tracing")]
                                tracing::trace!(target: "froglight_network", "Received Packet: {packet:?}");

                                let update = Self::update_connection_details(&packet);
                                channel.send_async(packet).await?;
                                if update.is_some() {
                                    return Ok(update);
                                }
                            }
                            PacketStateEnum::Play => {
                                let Some(packet) = read_packet(reader, reader_buf, exit_on_error)
                                    .await?
                                    .map(VersionPacket::Play)
                                else {
                                    continue;
                                };

                                #[cfg(feature = "tracing")]
                                tracing::trace!(target: "froglight_network", "Received Packet: {packet:?}");

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
                let result =
                    or::<Result<Option<ConnectionUpdate>, Box<dyn Error + Send + Sync>>, _, _>(
                        (client_to_server)(&mut writer, &mut write_buf_a, &mut write_buf_b),
                        (server_to_client)(&mut reader, &mut read_buf),
                    )
                    .await;

                match result {
                    Ok(None) => {}
                    Ok(Some(update)) => {
                        if let Some(threshold) = update.compression_threshold {
                            #[cfg(feature = "tracing")]
                            tracing::trace!(
                                target: "froglight_network",
                                "Updating connection compression threshold: {} -> {threshold}",
                                reader.compression().load(Ordering::Relaxed)
                            );
                            reader.compression().store(threshold, Ordering::Relaxed);
                            writer.compression().store(threshold, Ordering::Relaxed);
                        }
                        if let Some(_key) = update.encrypion_key {
                            #[cfg(feature = "tracing")]
                            tracing::trace!(
                                target: "froglight_network",
                                "Updating connection encryption key: <redacted>"
                            );
                            *reader.decryptor() = Decryptor::new(&[0; _].into(), &[0; _].into());
                            *writer.encryptor() = Encryptor::new(&[0; _].into(), &[0; _].into());
                            reader.enabled().store(true, Ordering::Relaxed);
                            writer.enabled().store(true, Ordering::Relaxed);
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
        }
    }

    /// Update connection details based on a received packet.
    fn update_connection_details(
        packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate>;

    /// Convert a [`ServerboundEventEnum`] into a
    /// [`VersionPacket<Self, Serverbound>`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn event_to_packet(
        event: ServerboundEventEnum,
        entity: EntityRef<'_>,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError>;

    /// Convert a [`VersionPacket<Self, Clientbound>`] into a
    /// [`ClientboundEventEnum`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the conversion fails.
    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        entity: EntityRef<'_>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError>;
}

/// Details for updating a connection.
#[derive(Debug, Default, Clone)]
pub struct ConnectionUpdate {
    /// A new compression threshold to set.
    pub compression_threshold: Option<i32>,
    /// A new encryption key to set.
    pub encrypion_key: Option<Vec<u8>>,
}

/// Read a packet of type `T` from the connection.
///
/// # Errors
///
/// Returns an error if reading the packet fails.
pub async fn read_packet<R: RuntimeRead<C>, C: Send, T: Facet<'static>>(
    reader: &mut DecryptorMut<R, C>,
    buffer: &mut Vec<u8>,
    exit_on_error: bool,
) -> Result<Option<T>, Box<dyn Error + Send + Sync>> {
    // Read the packet length prefix.
    let packet_length = read_varint_bytewise(reader).await? as usize;
    // Read the packet data.
    buffer.resize(packet_length, 0);
    reader.read_exact(buffer.as_mut_slice()).await?;

    // Decompress the packet.
    let packet = reader.decompress(buffer).await?;

    #[cfg(feature = "tracing")]
    tracing::trace!(target: "froglight_network", "Reading packet as: {packet:?}");

    // Deserialize the packet.
    match facet_minecraft::from_slice::<T>(packet) {
        #[allow(unused_variables, reason = "Used in tracing only")]
        Ok((val, rem)) => {
            #[cfg(feature = "tracing")]
            if !rem.is_empty() {
                if tracing::enabled!(target: "froglight_network", tracing::Level::DEBUG) {
                    tracing::error!(
                        target: "froglight_network",
                        "Bytes remaining after reading packet `{}` ({}) \u{f149}\n    {rem:?}", T::SHAPE.type_name(), rem.len()
                    );
                } else {
                    tracing::warn!(
                        target: "froglight_network",
                        "Bytes remaining after reading packet `{}`: {}", T::SHAPE.type_name(), rem.len()
                    );
                }
            }

            Ok(Some(val))
        }
        #[allow(unused_variables, reason = "Used if tracing is enabled")]
        Err(err) if !exit_on_error => {
            #[cfg(feature = "tracing")]
            tracing::error!(
                target: "froglight_network",
                "Failed to read packet: {err}"
            );
            Ok(None)
        }
        Err(err) => Err(Box::new(err)),
    }
}

/// Write a packet of type `T` to the connection.
///
/// # Errors
///
/// Returns an error if writing the packet fails.
pub async fn write_packet<R: RuntimeWrite<C>, C: Send, T: Facet<'static>>(
    packet: &T,
    writer: &mut EncryptorMut<R, C>,
    buffer_a: &mut Vec<u8>,
    buffer_b: &mut Vec<u8>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    buffer_a.clear();
    buffer_b.clear();

    // Serialize the packet.
    facet_minecraft::to_buffer(packet, buffer_a)?;

    #[cfg(feature = "tracing")]
    tracing::trace!(target: "froglight_network", "Writing packet as: {buffer_a:?}");

    // Compress the packet.
    let compressed = writer.compress(buffer_a).await?;
    buffer_b.extend_from_slice(compressed);

    // Add the length prefix.
    let len = write_slice_prefix(buffer_b.len(), buffer_b);
    buffer_b.rotate_right(len);

    // Write packet data.
    writer.write_all(buffer_b.as_mut_slice()).await.map_err(Into::into)
}

/// Read a VarInt per-byte from the connection.
async fn read_varint_bytewise<R: RuntimeRead<C>, C: Send>(
    reader: &mut DecryptorMut<R, C>,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mut byte = [0];
    let mut number = 0;
    for i in 0..5 {
        reader.read_exact(byte.as_mut_slice()).await?;
        number |= u32::from(byte[0] & 0b0111_1111) << (7 * i);
        if byte[0] & 0b1000_0000 == 0 {
            break;
        }
    }
    Ok(number)
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;
