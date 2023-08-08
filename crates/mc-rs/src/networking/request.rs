use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};
use mc_rs_proto::Version;
use uuid::Uuid;

/// Add request response events to the app
pub(super) fn setup(app: &mut App) {
    app.add_event::<StatusResponse>();
    app.add_event::<PingResponse>();
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
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusResponse {
    pub host: String,
    pub description: String,
    pub favicon: Option<String>,
    pub player_max: i32,
    pub player_online: i32,
    pub sample_players: HashMap<String, Uuid>,
    pub version: String,
    pub protocol: i32,
}

/// A response to a ping request
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct PingResponse {
    pub host: String,
    pub time: u64,
}
