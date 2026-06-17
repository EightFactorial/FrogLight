//! TODO

use core::{
    pin::Pin,
    task::{Context, Poll},
};

use facet::{HeapValue, Partial};

use crate::format::{
    Reader, ReaderError,
    deserialize::{Deserialize, DeserializeError, Deserializer, Item, varint},
};

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

#[inline(never)]
async fn deserialize_async(
    partial: Partial<'static, false>,
    variable: bool,
    reader: &mut Reader<'_>,
) -> Result<HeapValue<'static, false>, DeserializeError> {
    let mut core = move |item: Item<'static, false>| {
        let item = match item {
            Item::Item(item) => item,
            Item::Size(..) => return varint::decode_u32_from(reader).map(Item::Size),
        };

        // Handle field attributes.
        if let Some(attrs) = item.field_attr() {
            for attr in attrs {
                // Run the custom deserializer.
                if attr.ns.is_some_and(|ns| ns == "mc")
                    && attr.key == "with"
                    && let Some(crate::facet::Attr::With(Some(with))) =
                        attr.get_as::<crate::facet::Attr>()
                {
                    return with.deserialize(item, reader).map(Item::Item);
                }
            }
        }

        // Handle type attributes.
        for attr in item.shape_attr() {
            // Run the custom deserializer.
            if attr.ns.is_some_and(|ns| ns == "mc")
                && attr.key == "with"
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.deserialize(item, reader).map(Item::Item);
            }
        }

        super::deserialize_core(item, reader).map(Item::Item)
    };

    // Create and complete the deserializer.
    let de = Deserializer::new(partial, variable, &mut core);
    de.into_future().await?.build().map_err(DeserializeError::from)
}

// -------------------------------------------------------------------------------------------------

impl<
    'facet,
    'core,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> Deserializer<'facet, 'core, BORROW, C>
{
    /// Convert this [`Deserializer`] into a [`DeserializerFuture`].
    #[inline]
    #[must_use]
    pub const fn into_future(self) -> DeserializerFuture<'facet, 'core, BORROW, C> {
        DeserializerFuture { de: self }
    }
}

impl<
    'facet,
    'core,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> DeserializerFuture<'facet, 'core, BORROW, C>
{
    /// Convert this [`DeserializerFuture`] into a [`Deserializer`].
    #[inline]
    #[must_use]
    pub fn into_sync(self) -> Deserializer<'facet, 'core, BORROW, C> { self.de }
}

// -------------------------------------------------------------------------------------------------

/// A [`Deserializer`] that implements [`Future`].
#[repr(transparent)]
pub struct DeserializerFuture<
    'facet,
    'core,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> {
    de: Deserializer<'facet, 'core, BORROW, C>,
}

impl<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> Future for DeserializerFuture<'facet, '_, BORROW, C>
{
    type Output = Result<Partial<'facet, BORROW>, DeserializeError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Iterator::next(&mut self.de) {
            Some(Ok(())) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Some(Err(err)) => Poll::Ready(Err(err)),

            None => {
                // Take the `DeserializeIterator` result.
                match core::mem::replace(&mut self.de.iter, Err(DeserializeError)) {
                    Ok(iter) => {
                        let mut partial = iter.into_partial();

                        // Make sure the `Partial` is at the correct frame.
                        while partial.frame_count() > self.de.start {
                            partial = partial.end()?;
                        }

                        Poll::Ready(Ok(partial))
                    }

                    Err(err) => Poll::Ready(Err(err)),
                }
            }
        }
    }
}
