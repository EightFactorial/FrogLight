//! TODO

mod direction;
pub use direction::{Client, Direction, Server};

#[expect(clippy::module_inception, reason = "That's what it's called")]
mod state;
pub use state::{Config, Handshake, Login, Play, State, Status, ValidState};
