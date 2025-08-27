//! TODO

use alloc::{boxed::Box, string::ToString, vec::Vec};
use core::error::Error;

use async_compression::futures::write::{ZlibDecoder, ZlibEncoder};
use facet_minecraft::{AssertProtocol, DeserializeError, McSerializer, SerializeError};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use crate::network::ConnConfig;

/// Read a packet from the given reader.
///
/// # Errors
///
/// Returns an error if the connection cannot be read from,
/// or if the packet cannot be deserialized.
#[allow(clippy::used_underscore_binding, reason = "May or may not be used depending on features")]
pub async fn receive_type<'facet, T: AssertProtocol<'facet>, R: AsyncReadExt + Unpin + ?Sized>(
    buffer: &mut Vec<u8>,
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    reader: &mut R,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    receive_packet(buffer, scratch, config, reader).await?;

    #[cfg(feature = "tracing")]
    if buffer.len() < 10 {
        tracing::trace!("Received packet: `{buffer:?}`");
    } else {
        let formatted = alloc::format!("{:?}", &buffer[..10]);
        tracing::trace!("Received packet: `[{}, ...]`", &formatted[1..formatted.len() - 1]);
    }

    match facet_minecraft::deserialize_remainder(buffer) {
        Ok((packet, _rem)) => {
            #[cfg(feature = "tracing")]
            if !_rem.is_empty() {
                tracing::warn!(
                    "Successfully deserialized `{}`, but {} bytes are remaining",
                    T::SHAPE.type_identifier,
                    _rem.len()
                );
            }

            Ok(packet)
        }
        Err(err) => {
            // If `DEBUG` is enabled, print the full report to `stderr`.
            #[cfg(feature = "tracing")]
            if tracing::enabled!(tracing::Level::DEBUG) {
                err.eprint();
            }

            Err(map_deserialize(err))
        }
    }
}

/// Read a packet from the given reader.
///
/// Expects the next bytes from the reader to be the start of a packet.
///
/// # Errors
///
/// Returns an error if the connection cannot be read from.
pub async fn receive_packet<R: AsyncReadExt + Unpin + ?Sized>(
    buffer: &mut Vec<u8>,
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    reader: &mut R,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Read the length prefix and determine the full packet length.
    let length = read_encrypted_varint(reader, config).await?;

    // Read the remaining packet bytes
    scratch.resize(length as usize, 0u8);
    read_encrypted_bytes(scratch, config, reader).await?;

    // Set the start of where to read the remaining data.
    let mut scratch = scratch.as_slice();

    // Check if compression is enabled
    let compression_threshold = config.get_compression();
    if compression_threshold.is_positive() {
        // Read the length of the uncompressed data.
        let (length, remaining) =
            facet_minecraft::deserialize_remainder::<VarInt>(scratch).map_err(map_deserialize)?;
        // Update where to read the remaining data.
        scratch = remaining;

        // If the length is over the threshold, decompress the data into the output.
        #[expect(clippy::cast_sign_loss, reason = "Already checked threshold was positive")]
        if length.0 >= compression_threshold as u32 {
            let mut decoder = ZlibDecoder::new(buffer);
            decoder.write_all(scratch).await?;
            decoder.flush().await?;

            #[cfg(feature = "tracing")]
            if let Some(byte) = decoder.get_ref().first() {
                tracing::debug!("Received packet with ID {byte}");
            }

            return Ok(());
        }
    }

    // Otherwise, copy the remaining data into the output.
    buffer.resize(scratch.len(), 0u8);
    buffer.copy_from_slice(scratch);

    #[cfg(feature = "tracing")]
    if let Some(byte) = buffer.first() {
        tracing::debug!("Received packet with ID {byte}");
    }

    Ok(())
}

/// Read raw packet bytes from the given reader.
///
/// Fills the given buffer exactly,
/// decrypting it if encryption is enabled.
///
/// # Errors
///
/// Returns an error if the connection cannot be read from.
pub async fn read_encrypted_bytes<R: AsyncReadExt + Unpin + ?Sized>(
    buffer: &mut [u8],
    config: &ConnConfig,
    reader: &mut R,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Fill the buffer with bytes from the reader.
    reader.read(buffer).await?;
    // Decrypt the packet if encryption is enabled.
    config.crypto().decrypt_inplace(buffer).await;
    Ok(())
}

/// Read an encrypted VarInt from the given reader.
///
/// Has to work byte-by-byte to prevent over-reading
/// and consuming bytes that are not part of the VarInt.
///
/// # Errors
///
/// Returns an error if the connection cannot be read from.
pub async fn read_encrypted_varint<R: AsyncReadExt + Unpin + ?Sized>(
    reader: &mut R,
    config: &ConnConfig,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mut byte = [0];
    let mut number = 0;
    for i in 0..5 {
        reader.read_exact(&mut byte).await?;
        config.crypto().decrypt_inplace(&mut byte).await;
        number |= u32::from(byte[0] & 0b0111_1111) << (7 * i);
        if byte[0] & 0b1000_0000 == 0 {
            break;
        }
    }
    Ok(number)
}

// -------------------------------------------------------------------------------------------------

/// Write a packet into the given writer.
///
/// # Errors
///
/// Returns an error if packet cannot be serialized,
/// or if the connection cannot be written to.
pub async fn send_type<'facet, T: AssertProtocol<'facet>, W: AsyncWriteExt + Unpin + ?Sized>(
    value: &T,
    buffer: &mut Vec<u8>,
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    writer: &mut W,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut serializer = McSerializer(buffer);
    serializer.0.clear();

    match serializer.serialize(value) {
        Ok(()) => send_packet(serializer.0, scratch, config, writer).await,
        Err(err) => {
            #[cfg(feature = "tracing")]
            {
                // tracing::error!(
                //     "Failed to serialize `{}` -> `{}`: {}",
                //     T::SHAPE.type_identifier,
                //     err.identifier(),
                //     err.reason()
                // );
                // If `DEBUG` is enabled, print the full report to `stderr`.
                if tracing::event_enabled!(tracing::Level::DEBUG) {
                    // err.eprint();
                }
            }

            Err(map_serialize(err))
        }
    }
}

/// Write a packet into the given writer.
///
/// # Errors
///
/// Returns an error if the connection cannot be written to.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Packets should never be that large, VarInts cannot be larger than 5 bytes"
)]
pub async fn send_packet<W: AsyncWriteExt + Unpin + ?Sized>(
    buffer: &[u8],
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    writer: &mut W,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(feature = "tracing")]
    if let Some(byte) = buffer.first() {
        tracing::debug!("Sending packet with ID {byte}");
    }

    // Serialize the length prefix varint.
    let packet_len = buffer.len();
    let prefixed_len = packet_len + varint_size(packet_len as u32);
    let mut prefixed = Vec::<u8>::with_capacity(prefixed_len);

    let compression_threshold = config.get_compression();
    if compression_threshold == i32::MIN {
        // `None`, don't add a marker or compress.

        // Write the total packet length
        facet_minecraft::serialize(&VarInt(packet_len as u32), &mut prefixed)
            .map_err(map_serialize)?;
        // Copy the packet data into the buffer
        prefixed.extend_from_slice(buffer);
    } else if compression_threshold <= prefixed_len.try_into().unwrap_or(i32::MAX) {
        // `Some` and over the threshold, add marker and compress.

        // Write the total packet length
        let prefixed_prefixed_len = prefixed_len + varint_size(packet_len as u32);
        facet_minecraft::serialize(&VarInt(prefixed_prefixed_len as u32), &mut prefixed)
            .map_err(map_serialize)?;

        // Write the uncompressed length
        facet_minecraft::serialize(&VarInt(packet_len as u32), &mut prefixed)
            .map_err(map_serialize)?;

        // Compress the packet data into the buffer
        let mut encoder = ZlibEncoder::new(&mut prefixed);
        encoder.write_all(buffer).await?;
        encoder.flush().await?;
    } else {
        // `Some` but under the threshold, add marker and don't compress.

        // Write the total packet length + a `0` byte marker
        facet_minecraft::serialize(&VarInt(packet_len as u32 + 1), &mut prefixed)
            .map_err(map_serialize)?;
        prefixed.push(0);

        // Copy the packet data into the buffer
        prefixed.extend_from_slice(buffer);
    }

    // Send the final packet buffer
    send_encrypted_bytes(&prefixed, scratch, config, writer).await
}

/// Write packet bytes into the given writer.
///
/// Assumes that the packet is already ready to be sent
/// (i.e., compressed and length-prefixed), and encrypts
/// it if encryption is enabled.
///
/// # Errors
///
/// Returns an error if the connection cannot be written to.
pub async fn send_encrypted_bytes<W: AsyncWriteExt + Unpin + ?Sized>(
    mut buffer: &[u8],
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    writer: &mut W,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if config.crypto().enabled() {
        // Resize and clear the scratch buffer
        scratch.resize(buffer.len(), 0u8);
        scratch.clear();
        // Encrypt the packet and write it into the scratch buffer
        config.crypto().encrypt_into(buffer, scratch).await;
        // Write the scratch buffer instead of the original
        buffer = scratch.as_slice();
    }

    #[cfg(feature = "tracing")]
    if buffer.len() < 10 {
        tracing::trace!("Sending packet: `{buffer:?}`");
    } else {
        let formatted = alloc::format!("{:?}", &buffer[..10]);
        tracing::trace!("Sending packet: `[{}, ...]`", &formatted[1..formatted.len() - 1]);
    }

    // Write all bytes to the writer
    writer.write_all(buffer).await?;

    Ok(())
}

/// Calculate the size of a VarInt when serialized.
#[must_use]
pub fn varint_size(int: u32) -> usize {
    const MAX: usize = 5;

    for i in 1..MAX {
        if (int & ((u32::MAX >> 1) << (7 * i))) == 0 {
            return i;
        }
    }
    MAX
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(facet_macros::Facet)]
struct VarInt(#[facet(var)] u32);

/// Map a [`DeserializeError`] to a [`Box<dyn Error + Send + Sync>`].
#[must_use]
fn map_deserialize(err: DeserializeError<'_>) -> Box<dyn Error + Send + Sync> {
    let reason = err.reason();
    let message = reason.expected_note(&err);
    Box::new(std::io::Error::other(
        message.map_or_else(|| reason.to_string(), |message| alloc::format!("{reason}: {message}")),
    ))
}

/// Map a [`SerializeError`] to a [`Box<dyn Error + Send + Sync>`].
#[must_use]
fn map_serialize<T>(_err: SerializeError<'_, '_, T>) -> Box<dyn Error + Send + Sync> { todo!() }
