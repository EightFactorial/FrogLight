use std::time::Duration;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    entity::Entity,
    event::{Event, EventReader},
    schedule::{common_conditions::on_event, IntoSystemConfigs},
};
use bevy_log::{debug, error};
use froglight_protocol::packet::ServerStatus;

use super::{ConnectionTask, StatusTask};
use crate::connection::ConnectionError;

pub(super) fn build(app: &mut App) {
    app.add_event::<NetworkErrorEvent>().add_event::<ServerStatusResponse>();

    app.add_systems(
        PostUpdate,
        NetworkErrorEvent::log_error_events
            .run_if(on_event::<NetworkErrorEvent>())
            .after(StatusTask::poll_status_tasks)
            .after(ConnectionTask::poll_connection_tasks),
    );

    #[cfg(debug_assertions)]
    app.add_systems(
        PostUpdate,
        ServerStatusResponse::log_status_responses
            .run_if(on_event::<ServerStatusResponse>())
            .after(StatusTask::poll_status_tasks)
            .after(ConnectionTask::poll_connection_tasks),
    );
}

/// An event for server status responses.
///
/// Sent when a server responds to a status request.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ServerStatusResponse {
    /// The entity that received the status.
    pub entity: Entity,
    /// The status of the server.
    pub status: ServerStatus,
    /// The ping to the server.
    pub ping: Duration,
}

impl ServerStatusResponse {
    /// Log [`ServerStatusResponse`]s.
    #[cfg(debug_assertions)]
    fn log_status_responses(mut events: EventReader<Self>) {
        for event in events.read() {
            debug!("Entity {:?}, Ping: {:?}, Status: {:?}", event.entity, event.ping, event.status);
        }
    }
}

/// An event for network errors.
///
/// Sent when a connection error occurs.
#[derive(Debug, Event)]
pub struct NetworkErrorEvent {
    /// The entity that had the error.
    pub entity: Entity,
    /// The error that occurred.
    pub error: ConnectionError,
}

impl NetworkErrorEvent {
    /// Log [`NetworkErrorEvent`]s.
    fn log_error_events(mut events: EventReader<Self>) {
        for event in events.read() {
            error!("Entity {:?}, Error: {:?}", event.entity, event.error);
        }
    }
}
