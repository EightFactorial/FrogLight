use bevy_app::App;

mod connection;
pub use connection::{ConnectionClosedEvent, ConnectionTask};

mod error;
pub use error::ConnectionErrorEvent;

mod poll;
pub use poll::PolledTask;

mod status;
pub use status::{StatusResponseEvent, StatusTask};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    connection::build(app);
    status::build(app);
    error::build(app);
    poll::build(app);
}
