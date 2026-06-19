//! TODO

use facet::{HeapValue, Partial};
use froglight_facet_iter::{
    Reader,
    deserialize::{DeserializeError, Deserializer, DeserializerFuture},
};

use crate::deserialize::Deserialize;

/// A trait for types that can be deserialized asynchronously.
pub trait DeserializeAsync: Deserialize<'static> {
    /// Deserialize a value from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    #[inline]
    #[must_use]
    fn from_slice_async(
        slice: &[u8],
        variable: bool,
    ) -> impl Future<Output = Result<Self, DeserializeError>> + '_ {
        async move {
            <Self as DeserializeAsync>::from_slice_remainder_async(slice, variable)
                .await
                .map(|(val, _)| val)
        }
    }

    /// Deserialize a value from the given byte slice,
    /// returning the remaining slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    fn from_slice_remainder_async(
        slice: &[u8],
        variable: bool,
    ) -> impl Future<Output = Result<(Self, &[u8]), DeserializeError>> + '_;
}

impl<T: Deserialize<'static>> DeserializeAsync for T {
    #[inline]
    async fn from_slice_remainder_async(
        slice: &[u8],
        variable: bool,
    ) -> Result<(Self, &[u8]), DeserializeError> {
        let mut cursor = Reader::new(slice);
        let value = deserialize_async(Partial::alloc_owned::<T>()?, variable, &mut cursor).await?;
        Ok((value.materialize::<T>()?, cursor.remaining()))
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
async fn deserialize_async(
    partial: Partial<'static, false>,
    variable: bool,
    reader: &mut Reader<'_>,
) -> Result<HeapValue<'static, false>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = super::deserialize_owned_core(reader);
    let de = Deserializer::new(partial, variable, &mut core);
    DeserializerFuture::from_sync(de).await?.build().map_err(DeserializeError::from)
}
