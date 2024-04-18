//! Events that are sent by the connection plugin.

use bevy_app::App;

mod packet;
pub use packet::SendPacketEvent;

#[doc(hidden)]
pub(super) fn build(_app: &mut App) {}
