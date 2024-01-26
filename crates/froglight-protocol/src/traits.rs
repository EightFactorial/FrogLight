//! Traits for packets and structs that can be read and written.

/// A packet that can be sent or received.
pub trait Packet: Send + Sync {}
