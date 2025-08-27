//! TODO

use alloc::{string::String, vec::Vec};

pub use sha1::{Digest as ShaDigest, Sha1};

/// A wrapper around [`Sha1`] for digesting data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digest;

impl Digest {
    /// Return the [`Sha1`] digest of the input.
    #[must_use]
    pub fn digest(input: &[u8]) -> Vec<u8> {
        Self::digest_using(Sha1::new().chain_update(input)).to_vec()
    }

    /// Return the [`Sha1`] digest of the input.
    #[must_use]
    pub fn digest_using(hasher: Sha1) -> [u8; 20] { hasher.finalize().0 }
}

// -------------------------------------------------------------------------------------------------

/// A hexadecimal wrapper around [`Sha1`] for digesting data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexDigest;

impl HexDigest {
    /// Returns the [`Sha1`] digest as a hexadecimal [`String`].
    #[must_use]
    pub fn digest(input: &[u8]) -> String { Self::digest_using(Sha1::new().chain_update(input)) }

    /// Returns the [`Sha1`] digest as a hexadecimal [`String`].
    #[must_use]
    pub fn digest_using(hasher: Sha1) -> String {
        let mut hash = Digest::digest_using(hasher);
        let mut output = String::with_capacity(41);

        // If the output is negative
        let mut negative = false;
        if let Some(first) = hash.first()
            && *first > 0x7f
        {
            // Push a negative sign
            output.push('-');
            negative = true;

            // Perform two's compliment on the hash
            let mut carry = true;
            for byte in hash.iter_mut().rev() {
                *byte = !*byte;
                if carry {
                    *byte = byte.wrapping_add(1);
                    carry = *byte == 0;
                }
            }
        }

        // Create a closure for appending bytes as hexadecimal
        let mut append = |byte: u8| {
            match byte {
                // Skip adding leading zeros
                0u8 if output.len() == usize::from(negative) => {}
                // Add the decimal char
                byte @ 0..10u8 => {
                    output.push((byte + b'0') as char);
                }
                // Add the alphabetic char
                byte @ 10..16u8 => {
                    output.push((byte - 10u8 + b'a') as char);
                }
                _ => unreachable!("Byte will always be between 0x0 - 0xf"),
            }
        };

        // Convert the hash to a hexadecimal string
        for byte in hash {
            // Append the first hex digit
            append((byte / 16) % 16);
            // Append the second hex digit
            append(byte % 16);
        }

        output
    }
}
