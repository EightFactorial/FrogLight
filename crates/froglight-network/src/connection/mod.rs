//! [`StateConnection`], [`RawConnection`], and [`RawPacket`](raw::RawPacket)

pub mod raw;
pub use raw::RawConnection;

pub mod state;
pub use state::StateConnection;
