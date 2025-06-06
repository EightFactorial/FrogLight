use aes::{
    Aes128,
    cipher::{BlockDecryptMut, BlockEncryptMut, inout::InOutBuf},
};
use cfb8::{Decryptor, Encryptor};

/// The cipher used for encrypting and decrypting packets.
pub struct ConnectionCrypto {
    /// The encryptor is used to encrypt packets.
    pub encryptor: Encryptor<Aes128>,
    /// The decryptor is used to decrypt packets.
    pub decryptor: Decryptor<Aes128>,
}

impl ConnectionCrypto {
    /// Encrypt a slice of bytes in-place.
    pub fn encrypt_inplace(&mut self, buf: &mut [u8]) {
        let (head, tail) = InOutBuf::from(buf).into_chunks();
        debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
        self.encryptor.encrypt_blocks_inout_mut(head);
    }

    /// Decrypt a slice of bytes in-place.
    pub fn decrypt_inplace(&mut self, buf: &mut [u8]) {
        let (head, tail) = InOutBuf::from(buf).into_chunks();
        debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
        self.decryptor.decrypt_blocks_inout_mut(head);
    }

    /// Encrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub fn encrypt_into(&mut self, input: &[u8], output: &mut [u8]) {
        let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
        debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
        self.encryptor.encrypt_blocks_inout_mut(head);
    }

    /// Decrypts a slice of bytes into another slice.
    ///
    /// # Panics
    /// This will panic if the input and output slices
    /// are not of the same length.
    pub fn decrypt_into(&mut self, input: &[u8], output: &mut [u8]) {
        let (head, tail) = InOutBuf::new(input, output).unwrap().into_chunks();
        debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
        self.decryptor.decrypt_blocks_inout_mut(head);
    }
}
