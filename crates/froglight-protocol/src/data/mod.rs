//! Common data structures used in the protocol.
//!
//! These are version-independent and can be used in any version of the
//! protocol.

mod nonzero;
pub use nonzero::NonZero;

mod resourcekey;
pub use resourcekey::{ResourceKey, ResourceKeyError};

mod unsized_buffer;
pub use unsized_buffer::UnsizedByteBuffer;
