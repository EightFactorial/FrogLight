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
    /// use mc_rs_core::ConnectionEvent;
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let event = ConnectionEvent::<V1_20_0>::new(hostname.clone());
    ///
    /// assert_eq!(event.hostname, hostname);
    /// ```
    pub fn new(hostname: impl Into<CompactString>) -> Self {
        Self {
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
    /// use mc_rs_core::{ConnectionEvent, enums::ConnectionIntent};
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let intent = ConnectionIntent::Status;
    /// let event = ConnectionEvent::<V1_20_0>::new_with(hostname.clone(), intent);
    ///
    /// assert_eq!(event.hostname, hostname);
    /// assert_eq!(event.intent, intent);
    /// ```
    pub fn new_with(hostname: impl Into<CompactString>, intent: ConnectionIntent) -> Self {
        Self {
            hostname: hostname.into(),
            intent,
            _version: PhantomData,
        }
    }
}

/// An event that requests the status of a server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct StatusRequest<V: Version> {
    pub hostname: CompactString,
    _version: PhantomData<V>,
}

impl<V: Version> StatusRequest<V> {
    /// Create a new status request event.
    ///
    /// ### Example
    /// ```rust
    /// use compact_str::CompactString;
    /// use mc_rs_core::StatusRequest;
    /// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
    ///
    /// let hostname = CompactString::from("localhost:25565");
    /// let event = StatusRequest::<V1_20_0>::new(hostname.clone());
    ///
    /// assert_eq!(event.hostname, hostname);
    /// ```
    pub fn new(hostname: impl Into<CompactString>) -> Self {
        Self {
            hostname: hostname.into(),
            _version: PhantomData,
        }
    }
}

/// A response to a status request
///
/// The hostname field is the same as the one in the request.
#[derive(Debug, Clone, PartialEq, Event)]
pub struct StatusResponse {
    pub hostname: CompactString,
    pub description: FormattedText,
    pub favicon: Option<String>,
    pub player_max: i32,
    pub player_online: i32,
    pub sample_players: HashMap<CompactString, Uuid>,
    pub version: FormattedText,
    pub protocol: i32,
}

/// A response to a ping request
///
/// The hostname field is the same as the one in the request.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct PingResponse {
    pub hostname: CompactString,
    pub time: u64,
}
