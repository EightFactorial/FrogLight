//! Events that are sent and received by the connection plugin.

mod recv;
pub use recv::*;

mod send;
pub use send::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    recv::build(app);
    send::build(app);
}
