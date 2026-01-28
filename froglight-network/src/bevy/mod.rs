//! TODO

mod client;
use bevy_app::{App, Plugin};
pub use client::ClientConnection;

mod event;
pub use event::{ClientboundMessage, ServerboundMessage};

pub mod version;
pub use version::NetworkVersion;

/// A [`Plugin`] that adds [`ClientboundMessage`] and [`ServerboundMessage`]
/// for communicating over the network.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClientboundMessage>().add_message::<ClientboundMessage>();
        app.register_type::<ServerboundMessage>().add_message::<ServerboundMessage>();
    }
}
