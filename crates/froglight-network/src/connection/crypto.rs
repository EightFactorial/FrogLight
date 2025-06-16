use aes::{
    Aes128,
    cipher::{BlockModeDecrypt, BlockModeEncrypt, inout::InOutBuf},
};
use async_lock::RwLock;
use cfb8::{Decryptor, Encryptor};

/// The cipher used for encrypting and decrypting packets.
#[derive(Default)]
#[expect(clippy::large_enum_variant)]
pub enum ConnectionCrypto {
    /// No encryption is used for the connection.
    #[default]
    None,
    /// The connection has encryption enabled.
    Some {
        /// The encryptor is used to encrypt packets.
        encryptor: RwLock<Encryptor<Aes128>>,
        /// The decryptor is used to decrypt packets.
        decryptor: RwLock<Decryptor<Aes128>>,
    },
}

impl ConnectionCrypto {
    /// Returns `true` if the connection has encryption enabled.
    #[inline]
    #[must_use]
    pub const fn is_some(&self) -> bool { matches!(self, ConnectionCrypto::Some { .. }) }

    /// Encrypt a slice of bytes in-place.
    pub async fn encrypt_inplace(&self, buf: &mut [u8]) {
        if let ConnectionCrypto::Some { encryptor, .. } = self {
            let mut encryptor = encryptor.write().await;
            let (head, tail) = InOutBuf::from(buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            encryptor.encrypt_blocks_inout(head);
        }
    }

    /// Decrypt a slice of bytes in-place.
    pub async fn decrypt_inplace(&self, buf: &mut [u8]) {
        if let ConnectionCrypto::Some { decryptor, .. } = self {
            let mut decryptor = decryptor.write().await;
            let (head, tail) = InOutBuf::from(buf).into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            decryptor.decrypt_blocks_inout(head);
        }
    }

    /// Encrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub async fn encrypt_into(&self, input: &[u8], output: &mut [u8]) {
        if let ConnectionCrypto::Some { encryptor, .. } = self {
            let mut encryptor = encryptor.write().await;
            let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            encryptor.encrypt_blocks_inout(head);
        }
    }

    /// Decrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub async fn decrypt_into(&self, input: &[u8], output: &mut [u8]) {
        if let ConnectionCrypto::Some { decryptor, .. } = self {
            let mut decryptor = decryptor.write().await;
            let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            decryptor.decrypt_blocks_inout(head);
        }
    }
}
