use async_compression::futures::write::ZlibEncoder;
use async_std::io::WriteExt;
use froglight_protocol::{
    protocol::{FrogVarWrite, FrogWrite, WriteError},
    traits::{State, Version},
};

use super::{Connection, ConnectionError, NetworkDirection};

impl<V, S, D> Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        self.send_packet(&packet.into()).await
    }

    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send_packet(&mut self, packet: &D::Send) -> Result<(), ConnectionError> {
        // Create a buffer to store the packet bytes
        let mut buffer: Vec<u8> = packet.fg_to_bytes();

        if let Some(threshold) = self.compression {
            if i32::try_from(buffer.len()).expect("Packet length overflow") >= threshold {
                // Compress the buffer, prefixing it with `0` to indicate that it is compressed
                let mut compressor = ZlibEncoder::new(vec![0u8]);
                compressor.write_all(&buffer).await?;
                buffer = compressor.into_inner();
            } else {
                // Prefix the buffer with its uncompressed length
                Self::prefix_length(&mut buffer)?;
            }
        }

        // Prefix the buffer with its length
        Self::prefix_length(&mut buffer)?;

        // TODO: Encryption

        // Write the buffer to the stream
        Ok(self.stream.write_all(&buffer).await?)
    }

    /// Prefixes the given buffer with its length.
    fn prefix_length(buf: &mut Vec<u8>) -> Result<(), WriteError> {
        let mut prefixed_buffer = Vec::with_capacity(buf.len() + 5);
        buf.len().fg_var_write(&mut prefixed_buffer)?;
        prefixed_buffer.append(buf);
        *buf = prefixed_buffer;
        Ok(())
    }
}
