use std::marker::PhantomData;

use azalea_chat::FormattedText;
use bevy::{prelude::*, utils::HashMap};
use mc_rs_protocol::{types::enums::ConnectionIntent, Version};
use uuid::Uuid;

pub(super) fn configure(app: &mut App) {
    app.add_event::<StatusResponse>();
    app.add_event::<PingResponse>();
}

/// An event that is sent to create a new connection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct ConnectionEvent<V: Version> {
    pub addr: String,
    pub intent: ConnectionIntent,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionEvent<V> {
    #[allow(dead_code)]
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            intent: ConnectionIntent::Login,
            _version: PhantomData,
        }
    }

    pub fn new_with(addr: impl Into<String>, intent: ConnectionIntent) -> Self {
        Self {
            addr: addr.into(),
            intent,
            _version: PhantomData,
        }
    }
}

/// An event that requests the status of a server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct StatusRequest<V: Version> {
    pub host: String,
    _version: PhantomData<V>,
}

#[allow(dead_code)]
impl<V: Version> StatusRequest<V> {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            host: address.into(),
            _version: PhantomData,
        }
    }
}

/// A response to a status request
#[derive(Debug, Clone, PartialEq, Event)]
pub struct StatusResponse {
    pub hostname: String,
    pub description: FormattedText,
    pub favicon: Option<String>,
    pub player_max: i32,
    pub player_online: i32,
    pub sample_players: HashMap<String, Uuid>,
    pub version: FormattedText,
    pub protocol: i32,
}

/// A response to a ping request
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct PingResponse {
    pub hostname: String,
    pub time: u64,
}
