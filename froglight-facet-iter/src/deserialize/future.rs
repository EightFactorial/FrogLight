use core::{
    pin::Pin,
    task::{Context, Poll},
};

use facet::Partial;

use crate::{
    ReaderError,
    deserialize::{DeserializeError, Deserializer, Item},
};

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
    'core,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> DeserializerFuture<'facet, 'core, BORROW, C>
{
    /// Create a new [`DeserializerFuture`] from a [`Deserializer`].
    #[inline]
    #[must_use]
    pub const fn from_sync(de: Deserializer<'facet, 'core, BORROW, C>) -> Self { Self { de } }

    /// Convert this [`DeserializerFuture`] into a [`Deserializer`].
    #[inline]
    #[must_use]
    pub fn into_sync(self) -> Deserializer<'facet, 'core, BORROW, C> { self.de }
}

// -------------------------------------------------------------------------------------------------

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
                let starting_frame = self.de.start();
                let mut partial = self.de.complete_mut()?;

                // Make sure the `Partial` is at the correct frame.
                while partial.frame_count() > starting_frame {
                    partial = partial.end()?;
                }

                Poll::Ready(Ok(partial))
            }
        }
    }
}
