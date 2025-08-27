use alloc::boxed::Box;
use core::error::Error;

use crate::connection::half::{RawReadable, RawWritable};

/// A connection that can be split into readable and writable halves.
pub trait Splittable: RawReadable + RawWritable + 'static {
    /// Split this [`Splittable`] into a [`RawReadable`] and a [`RawWritable`].
    ///
    /// # Note
    ///
    /// The read half is guaranteed to be [`Combinable`],
    /// and can be recombined with the write half into the original boxed type.
    fn into_split(self: Box<Self>) -> (Box<dyn Combinable>, Box<dyn RawWritable>);
}

// -------------------------------------------------------------------------------------------------

/// A readable connection that can be recombined with a writable half.
pub trait Combinable: RawReadable + 'static {
    /// Attempt to recombine this [`Combinable`] with a [`RawWritable`].
    ///
    /// # Errors
    ///
    /// If the underlying connections cannot be combined, an error is returned.
    fn into_combined(
        self: Box<Self>,
        write: Box<dyn RawWritable>,
    ) -> Result<Box<dyn Splittable>, Box<dyn Error + Send + Sync>>;
}
