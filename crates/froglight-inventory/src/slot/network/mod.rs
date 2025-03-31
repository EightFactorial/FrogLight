//! [RawInventorySlot] and [HashedInventorySlot] for network serialization.

mod hash;
pub use hash::{HashedInventorySlot, HashedInventorySlotRef};

mod raw;
pub use raw::{RawInventorySlot, RawInventorySlotRef};
