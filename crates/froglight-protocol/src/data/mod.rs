//! Common data structures used in the protocol.
//!
//! These are version-independent and can be used in any version of the
//! protocol.

mod bytes;
pub use bytes::*;

mod position;
pub use position::*;

mod string;
pub use string::*;

mod nonzero;
pub use nonzero::NonZero;
