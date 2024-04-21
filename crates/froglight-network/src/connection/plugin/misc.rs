use std::{marker::PhantomData, time::Duration};

use bevy_ecs::{bundle::Bundle, component::Component};
use bevy_tasks::Task;
use froglight_protocol::{packet::ServerStatus, traits::Version};

use crate::connection::ConnectionError;

/// A marker [`Component`](bevy_ecs::prelude::Component) used to identify
/// [`Entities`](bevy_ecs::prelude::Entity) that have a
/// [`Connection`](crate::connection::Connection).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionMarker<V: Version>(PhantomData<V>);

impl<V: Version> std::fmt::Debug for ConnectionMarker<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionMarker<{:?}>", V::default())
    }
}

/// A [`Bundle`] that contains all the components required to manage a
/// [`Connection`](crate::connection::Connection).
#[derive(Debug, Bundle)]
pub struct ConnectionBundle<V: Version> {
    pub marker: ConnectionMarker<V>,
    pub task: ConnectionTask,
}

impl<V: Version> ConnectionBundle<V> {
    /// Create a new [`ConnectionBundle`] with the given [`Task`].
    pub(super) fn new(task: Task<ConnectionError>) -> Self {
        Self {
            marker: ConnectionMarker(PhantomData),
            task: ConnectionTask { task, version_id: V::ID },
        }
    }
}

/// A simple [`Component`] that holds a
/// [`Task`] used to manage a connection.
///
/// Removing this component will drop the connection.
#[derive(Debug, Component)]
pub struct ConnectionTask {
    /// The task handling the connection.
    task: Task<ConnectionError>,
    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

/// A simple [`Component`] that holds a
/// [`Task`] used to manage a connection.
///
/// Removing this component will drop the connection.
#[derive(Debug, Component)]
pub struct StatusTask {
    /// The task handling the connection.
    task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
    /// The [`Version::ID`] of the connection.
    pub version_id: i32,
}

impl StatusTask {
    /// Create a new [`StatusTask`] with the given [`Task`].
    pub(super) fn new<V: Version>(
        task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
    ) -> Self {
        Self { task, version_id: V::ID }
    }
}
