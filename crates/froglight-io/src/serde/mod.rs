//! Traits and implementations for encoding and decoding types as json.

use serde::{Serialize, de::DeserializeOwned};

use crate::standard::{FrogRead, FrogWrite, ReadError, WriteError};

/// A trait for reading data from a buffer as json.
pub trait FrogJson: Serialize + DeserializeOwned {
    /// Read the data as json from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    fn frog_from_json(buffer: &mut impl std::io::Read) -> Result<Self, ReadError>;
    /// Write the data as json to the given buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_to_json(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError>;

    /// Get the length of the data if it were to be written as json.
    ///
    /// # Warning
    /// In order to calculate this the data is serialized into a string.
    ///
    /// This is very, very inefficient and should be avoided if possible.
    ///
    /// # Panics
    /// Panics if the data fails to be serialized.
    fn frog_json_len(&self) -> usize {
        serde_json::to_string(self)
            .map_or_else(|err| panic!("Failed to serialize: {err}"), |content| content.len())
    }
}

impl<T: Serialize + DeserializeOwned> FrogJson for T {
    fn frog_from_json(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let content = String::frog_read(buffer)?;
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing \"{}\" from JSON", <Self as TypeName>::type_name());
        serde_json::from_str(&content).map_err(ReadError::Json)
    }

    fn frog_to_json(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing \"{}\" as JSON", <Self as TypeName>::type_name());
        let content = serde_json::to_string(self).map_err(WriteError::Json)?;
        content.frog_write(buffer)
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for retrieving the name of a type.
#[allow(dead_code)]
trait TypeName {
    /// The name of the type.
    ///
    /// If the `nightly` feature is enabled,
    /// the path to the type is stripped at compile time.
    #[inline]
    fn type_name() -> &'static str {
        #[cfg(feature = "nightly")]
        return Self::STRIPPED_TYPE_NAME;
        #[cfg(not(feature = "nightly"))]
        return core::any::type_name::<Self>();
    }

    /// The name of the type without the path.
    #[cfg(feature = "nightly")]
    #[expect(clippy::single_match)]
    const STRIPPED_TYPE_NAME: &'static str = {
        let mut index = 0;
        let mut last = 0;

        let name = core::any::type_name::<Self>();
        while index < name.len() {
            let (start, _) = name.split_at(index);
            let (_, last_two) = start.split_at(start.len().saturating_sub(2));
            match last_two.as_bytes() {
                b"::" => last = index,
                _ => {}
            }
            index += 1;
        }

        name.split_at(last).1
    };
}

impl<T: FrogJson> TypeName for T {}
