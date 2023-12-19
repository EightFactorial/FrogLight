use std::marker::PhantomData;

use azalea_chat::FormattedText;
use bevy::{prelude::*, utils::HashMap};
use compact_str::CompactString;
use mc_rs_protocol::{types::enums::ConnectionIntent, Version};
use uuid::Uuid;

pub(super) fn setup(app: &mut App) {
    app.add_event::<StatusResponse>();
    app.add_event::<PingResponse>();
}

/// An event that is sent to create a new connection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct ConnectionEvent<V: Version> {
    pub entity: Option<Entity>,
    pub hostname: CompactString,
    pub intent: ConnectionIntent,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionEvent<V> {
    /// Create a new connection event with the [default intent](ConnectionIntent::Login).
    ///
    /// ### Example
    /// ```rust
    /// use compact_str::CompactString;
    /// use mc_rs_core::events::ConnectionEvent;
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let event = ConnectionEvent::<V1_20_0>::new(hostname.clone());
    ///
    /// assert_eq!(event.hostname, hostname);
    /// ```
    pub fn new(hostname: impl Into<CompactString>) -> Self {
        Self {
            entity: None,
            hostname: hostname.into(),
            intent: ConnectionIntent::Login,
            _version: PhantomData,
        }
    }

    /// Create a new connection event with a [custom intent](ConnectionIntent).
    ///
    /// It is recommended to use the [`ConnectionEvent::new`](ConnectionEvent) method to login to a
    /// server, or use the [StatusRequest] event to get the status of a server.
    ///
    /// ### Example
    /// ```rust
    /// use compact_str::CompactString;
    /// use mc_rs_core::{events::ConnectionEvent, enums::ConnectionIntent};
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let intent = ConnectionIntent::Status;
    /// let event = ConnectionEvent::<V1_20_0>::new_with(None, hostname.clone(), intent);
    ///
    /// assert_eq!(event.entity, None);
    /// assert_eq!(event.hostname, hostname);
    /// assert_eq!(event.intent, intent);
    /// ```
    pub fn new_with(
        entity: Option<Entity>,
        hostname: impl Into<CompactString>,
        intent: ConnectionIntent,
    ) -> Self {
        Self {
            entity,
            hostname: hostname.into(),
            intent,
            _version: PhantomData,
        }
    }
}

/// An event that requests the status of a server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct StatusRequest<V: Version> {
    pub entity: Option<Entity>,
    pub hostname: CompactString,
    _version: PhantomData<V>,
}

impl<V: Version> StatusRequest<V> {
    /// Create a new status request event.
    ///
    /// ### Example
    /// ```rust
    /// use compact_str::CompactString;
    /// use mc_rs_core::events::StatusRequest;
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let event = StatusRequest::<V1_20_0>::new(hostname.clone());
    ///
    /// assert_eq!(event.hostname, hostname);
    /// ```
    pub fn new(hostname: impl Into<CompactString>) -> Self {
        Self {
            entity: None,
            hostname: hostname.into(),
            _version: PhantomData,
        }
    }

    /// Create a new status request event with a custom entity.
    pub fn new_with(entity: Option<Entity>, hostname: impl Into<CompactString>) -> Self {
        Self {
            entity,
            hostname: hostname.into(),
            _version: PhantomData,
        }
    }
}

/// A response to a status request
///
/// The hostname field is the same as the one in the request.
#[derive(Clone, PartialEq, Event)]
pub struct StatusResponse {
    pub entity: Option<Entity>,
    pub hostname: CompactString,
    pub description: FormattedText,
    pub favicon: Option<CompactString>,
    pub player_max: i32,
    pub player_online: i32,
    pub sample_players: HashMap<CompactString, Uuid>,
    pub version: FormattedText,
    pub protocol: i32,
}

impl std::fmt::Debug for StatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusResponse")
            .field("hostname", &self.hostname)
            .field("description", &self.description)
            .field("player_max", &self.player_max)
            .field("player_online", &self.player_online)
            .field("sample_players", &self.sample_players)
            .field("version", &self.version)
            .field("protocol", &self.protocol)
            .finish()
    }
}

/// A response to a ping request
///
/// The hostname field is the same as the one in the request.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct PingResponse {
    pub entity: Option<Entity>,
    pub hostname: CompactString,
    pub time: u64,
}
