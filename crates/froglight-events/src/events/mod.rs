//! [`Events`](bevy_ecs::event::Event) and event handling systems.

mod keepalive;
pub use keepalive::*;

mod serverping;
pub use serverping::*;

mod timeupdate;
pub use timeupdate::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    keepalive::build(app);
    serverping::build(app);
    timeupdate::build(app);
}
