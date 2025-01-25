//! Traits and implementations for encoding and decoding types as json.

use std::io::{Cursor, Write};

use serde::{de::DeserializeOwned, Serialize};

use crate::standard::{FrogRead, FrogWrite, ReadError, WriteError};

/// A trait for reading data from a buffer as json.
pub trait FrogJson: FrogRead + FrogWrite + Serialize + DeserializeOwned {
    /// Read the data as json from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    fn frog_from_json(buffer: &mut Cursor<&[u8]>) -> Result<Self, ReadError>;
    /// Write the data as json to the given buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_to_json(&self, buffer: &mut impl Write) -> Result<usize, WriteError>;
}

impl<T: FrogRead + FrogWrite + Serialize + DeserializeOwned> FrogJson for T {
    fn frog_from_json(buffer: &mut Cursor<&[u8]>) -> Result<Self, ReadError> {
        String::frog_read(buffer)
            .and_then(|content| serde_json::from_str(&content).map_err(ReadError::Json))
    }

    fn frog_to_json(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        serde_json::to_string(self)
            .map_or_else(|err| Err(WriteError::Json(err)), |content| content.frog_write(buffer))
    }
}
