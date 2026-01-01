//! TODO

use crate::storage::{BlockStorage, GlobalBlockStorage};

/// A [`Version`]'s associated block data.
pub trait BlockVersion: 'static {
    /// The [`GlobalBlockStorage`] for this [`Version`].
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    const BLOCKS: &'static GlobalBlockStorage;

    /// Create a new [`BlockStorage`] for this [`Version`].
    ///
    /// # Note
    ///
    /// This will create a new [`BlockStorage`] instance each time it is called!
    fn blocks() -> BlockStorage;
}
