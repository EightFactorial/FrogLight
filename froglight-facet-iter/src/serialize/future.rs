use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    WriterError,
    serialize::{Item, SerializeError, Serializer},
};

/// A [`Serializer`] that implements [`Future`].
#[repr(transparent)]
pub struct SerializerFuture<
    'mem,
    'facet,
    'core,
    C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
> {
    ser: Serializer<'mem, 'facet, 'core, C>,
}

impl<'mem, 'facet, 'core, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>
    SerializerFuture<'mem, 'facet, 'core, C>
{
    /// Create a new [`SerializerFuture`] from a [`Serializer`].
    #[inline]
    #[must_use]
    pub const fn from_sync(ser: Serializer<'mem, 'facet, 'core, C>) -> Self { Self { ser } }

    /// Convert this [`SerializerFuture`] into a [`Serializer`].
    #[inline]
    #[must_use]
    pub fn into_sync(self) -> Serializer<'mem, 'facet, 'core, C> { self.ser }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>> Future
    for SerializerFuture<'mem, 'facet, '_, C>
{
    type Output = Result<(), SerializeError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Iterator::next(&mut self.ser) {
            Some(Ok(())) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Some(Err(err)) => Poll::Ready(Err(err)),

            None => Poll::Ready(Ok(())),
        }
    }
}
