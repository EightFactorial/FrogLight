use std::io::Cursor;

use async_compression::futures::write::{ZlibDecoder, ZlibEncoder};
use froglight_io::prelude::{FrogVarRead, FrogVarWrite};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "crypto")]
use crate::connection::ConnectionCrypto;
use crate::connection::state::ConnectionError;

/// TODO: Test decompression
pub(super) async fn read_packet_outer(
    buf: &mut Vec<u8>,
    stream: &mut (impl AsyncReadExt + Unpin),
    scratch: &mut Vec<u8>,
    compression: Option<i32>,
    #[cfg(feature = "crypto")] mut crypto: Option<&mut ConnectionCrypto>,
) -> Result<(), ConnectionError> {
    // Make sure the buffer is large enough to hold the packet length
    if buf.len() < 5 {
        buf.resize(5, 0u8);
    }

    // Read the length bytes of the packet
    let (len_buf, _) = buf.split_first_chunk_mut::<5>().unwrap();
    read_raw_outer(
        len_buf.as_mut_slice(),
        stream,
        #[cfg(feature = "crypto")]
        &mut crypto,
    )
    .await?;

    // Read the length of the packet and count the number of bytes
    let mut len_cursor = Cursor::new(len_buf);
    let len = <u32 as FrogVarRead>::frog_var_read(&mut len_cursor);
    let len = len.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;
    #[expect(clippy::cast_possible_truncation)]
    let len_size = len_cursor.position() as usize;

    // Resize the buffer to hold the packet data
    buf.resize(len + len_size, 0u8);
    // Create a slice positioned to read the rest of the packet
    let pbuf = &mut buf[5usize..];
    read_raw_outer(
        pbuf,
        stream,
        #[cfg(feature = "crypto")]
        &mut crypto,
    )
    .await?;

    // Create a cursor to read packet bytes from the buffer
    let mut cursor = Cursor::new(&buf[len_size..]);

    // Check whether the packet is compressed
    if compression.is_some_and(|c| c >= 0) {
        let size = <u32 as FrogVarRead>::frog_var_read(&mut cursor);
        let size = size.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;

        if size != 0 {
            // Create a ZlibDecoder to decompress the packet
            let mut decoder = ZlibDecoder::new(futures_lite::io::Cursor::new(scratch));

            // Uncompress the data into the scratch buffer
            let result = decoder.write_all(buf).await;
            result.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))?;

            // Replace the buffer with the decompressed data
            buf.resize(size, 0u8);
            buf.copy_from_slice(&decoder.into_inner().into_inner()[..size]);

            // Log the ID and length of the packet
            #[cfg(feature = "trace")]
            if let Some(id) = buf.first() {
                tracing::trace!(target: "froglight_network::froglight_io", "Reading Packet ID {id} ({size})");
            }

            return Ok(());
        }
    }

    // Move the packet to the beginning of the buffer and set the buffer size
    #[expect(clippy::cast_possible_truncation)]
    let position = cursor.position() as usize;
    buf.copy_within(len_size + position.., 0);
    buf.truncate(len);

    // Log the ID and length of the packet
    #[cfg(feature = "trace")]
    if let Some(id) = buf.first() {
        tracing::trace!(target: "froglight_network::froglight_io", "Reading Packet ID {id} ({})", buf.len());
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

/// TODO: Test compression
pub(super) async fn write_packet_outer(
    buf: &[u8],
    stream: &mut (impl AsyncWriteExt + Unpin),
    scratch: &mut Vec<u8>,
    compression: Option<i32>,
    #[cfg(feature = "crypto")] crypto: Option<&mut ConnectionCrypto>,
) -> Result<(), ConnectionError> {
    // Log the ID and length of the packet
    #[cfg(feature = "trace")]
    if let Some(id) = buf.first() {
        tracing::trace!(target: "froglight_network::froglight_io", "Writing Packet ID {id} ({})", buf.len());
    }

    // Get the packet length and prefix length
    let packet_len = buf.len();
    let prefixed_len = packet_len + packet_len.frog_var_len();
    let mut prefixed = Vec::<u8>::with_capacity(prefixed_len);

    if compression.is_some_and(|c| c <= prefixed_len.try_into().unwrap_or_default()) {
        // Write the total packet length
        let prefixed_prefixed_len = prefixed_len + prefixed_len.frog_var_len();
        #[expect(clippy::cast_possible_truncation)]
        let result = FrogVarWrite::frog_var_write(&(prefixed_prefixed_len as u32), &mut prefixed);
        result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;

        // Write the uncompressed packet length
        #[expect(clippy::cast_possible_truncation)]
        let result = FrogVarWrite::frog_var_write(&(packet_len as u32), &mut prefixed);
        result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;

        // Create a ZlibEncoder to compress the packet
        let mut encoder = ZlibEncoder::new(prefixed);
        // Compress the packet data with the ZlibEncoder
        let result = encoder.write_all(buf).await;
        result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;

        // Write the final packet to the stream
        write_raw_outer(
            &encoder.into_inner(),
            stream,
            scratch,
            #[cfg(feature = "crypto")]
            crypto,
        )
        .await
    } else {
        if compression.is_some() {
            // Write the total packet length and a `0` byte to indicate no compression
            #[expect(clippy::cast_possible_truncation)]
            let result = FrogVarWrite::frog_var_write(&(packet_len as u32 + 1), &mut prefixed);
            result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
            prefixed.push(0);
        } else {
            // Write the total packet length without any compression marker
            #[expect(clippy::cast_possible_truncation)]
            let result = FrogVarWrite::frog_var_write(&(packet_len as u32), &mut prefixed);
            result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
        }

        // Append the rest of the packet data
        prefixed.extend_from_slice(buf);
        // Write the final packet to the stream
        write_raw_outer(
            &prefixed,
            stream,
            scratch,
            #[cfg(feature = "crypto")]
            crypto,
        )
        .await
    }
}

// -------------------------------------------------------------------------------------------------

/// TODO: Test decryption
pub(super) async fn read_raw_outer(
    buf: &mut [u8],
    stream: &mut (impl AsyncReadExt + Unpin),
    #[cfg(feature = "crypto")] crypto: &mut Option<&mut ConnectionCrypto>,
) -> Result<(), ConnectionError> {
    let result = stream.read_exact(buf).await;
    result.map_err(|err| ConnectionError::ReadRawConnection(Box::new(err)))?;

    #[cfg(feature = "crypto")]
    if let Some(crypto) = crypto {
        // Decrypt the buffer in place
        crypto.decrypt_inplace(buf);
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

/// TODO: Test encryption
#[allow(unused_variables, unused_mut)]
pub(super) async fn write_raw_outer(
    mut buf: &[u8],
    stream: &mut (impl AsyncWriteExt + Unpin),
    scratch: &mut Vec<u8>,
    #[cfg(feature = "crypto")] crypto: Option<&mut ConnectionCrypto>,
) -> Result<(), ConnectionError> {
    #[cfg(feature = "crypto")]
    if let Some(crypto) = crypto {
        // Clear and resize the scratch buffer to fit the packet
        scratch.clear();
        scratch.resize(buf.len(), 0u8);
        // Encrypt the packet and write into the scratch buffer
        crypto.encrypt_into(buf, scratch);
        // Use the scratch buffer as the input buffer
        buf = scratch.as_slice();
    }

    let result = stream.write_all(buf).await;
    result.map_err(|err| ConnectionError::WriteRawConnection(Box::new(err)))
}
