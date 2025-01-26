//! [`Client`](ClientConnection), [`Server`](ServerConnection),
//! and [`Raw`](RawConnection) connections.

pub mod raw;
pub use raw::RawConnection;

pub mod state;
pub use state::{ClientConnection, ServerConnection};
