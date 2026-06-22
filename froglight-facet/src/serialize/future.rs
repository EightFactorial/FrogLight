//! TODO

use alloc::vec::Vec;

use facet::{Facet, Peek};
use froglight_facet_iter::{
    Writer,
    serialize::{SerializeError, Serializer, SerializerFuture},
};

use crate::serialize::Serialize;

/// A trait for types that can be serialized asynchronously.
pub trait SerializeAsync<'facet>: Serialize<'facet> {
    /// Serialize the value into a new [`Vec`].
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn to_vec_async(value: &Self) -> impl Future<Output = Result<Vec<u8>, SerializeError>>;

    /// Serialize the value into the given [`Writer`],
    /// returning the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn to_writer_async(
        value: &Self,
        variable: bool,
        writer: Writer<'_>,
    ) -> impl Future<Output = Result<usize, SerializeError>>;
}

impl<'facet, T: Serialize<'facet> + Facet<'facet>> SerializeAsync<'facet> for T {
    #[inline]
    async fn to_vec_async(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::with_capacity(8); // TODO: Size hint
        <Self as SerializeAsync>::to_writer_async(value, false, Writer::new(&mut buffer))
            .await
            .map(|_| buffer)
    }

    #[inline]
    async fn to_writer_async(
        value: &Self,
        variable: bool,
        writer: Writer<'_>,
    ) -> Result<usize, SerializeError> {
        serialize_async(Peek::new(value), variable, writer).await
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
async fn serialize_async(
    peek: Peek<'_, '_>,
    variable: bool,
    mut writer: Writer<'_>,
) -> Result<usize, SerializeError> {
    // Create and complete the serializer.
    let mut core = super::serialize_core(&mut writer);
    SerializerFuture::from_sync(Serializer::new(peek, variable, &mut core, Some("mc"))).await?;

    // Return the number of bytes written.
    drop(core);
    Ok(writer.position())
}
