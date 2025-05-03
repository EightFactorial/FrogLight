//! TODO

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use bevy_platform::collections::HashMap;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use serde::{Deserialize, Serialize, ser::SerializeSeq};
use smol_str::SmolStr;
use uuid::Uuid;

/// A status response from a server
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Serialize, Deserialize))]
pub struct ServerStatus {
    /// The server's description
    pub description: SmolStr,
    /// The server's icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<SmolStr>,
    /// The server's player information
    pub players: ServerPlayers,
    /// The server's version information
    pub version: ServerVersion,
    /// Whether the server enforces secure chat
    #[serde(default, rename = "enforcesSecureChat", skip_serializing_if = "Option::is_none")]
    pub enforces_secure_chat: Option<bool>,

    /// Other fields that are not part of the standard status response
    #[serde(default, flatten, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    pub other: HashMap<SmolStr, serde_json::Value>,
}

// -------------------------------------------------------------------------------------------------

/// The server's version information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Serialize, Deserialize))]
pub struct ServerVersion {
    /// The version's name
    pub name: SmolStr,
    /// The version's protocol id
    pub protocol: i32,
}

// -------------------------------------------------------------------------------------------------

/// The server's player information
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Serialize, Deserialize))]
pub struct ServerPlayers {
    /// The maximum number of players
    pub max: i32,
    /// The number of online players
    pub online: i32,
    /// A sample of online players
    #[serde(default, with = "ServerSamplePlayer")]
    pub sample: Vec<ServerSamplePlayer>,
}

// -------------------------------------------------------------------------------------------------

/// Player information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Serialize, Deserialize))]
pub struct ServerSamplePlayer {
    /// The player's UUID
    #[serde(default, skip_serializing_if = "Uuid::is_nil")]
    pub uuid: Uuid,
    /// The player's username
    #[serde(default, skip_serializing_if = "SmolStr::is_empty")]
    pub username: SmolStr,
}

impl ServerSamplePlayer {
    /// Serialize a list of players, filtering out players with
    /// empty usernames and nil UUIDs
    fn serialize<S>(list: &[Self], serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut seq = serializer.serialize_seq(None)?;
        for player in list.iter().filter(|p| !(p.uuid.is_nil() && p.username.is_empty())) {
            seq.serialize_element(player)?;
        }
        seq.end()
    }

    /// Deserialize a list of players, filtering out players with
    /// empty usernames and nil UUIDs
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<ServerSamplePlayer>, D::Error>
    where D: serde::Deserializer<'de> {
        let list = Vec::<ServerSamplePlayer>::deserialize(deserializer)?;
        Ok(list.into_iter().filter(|p| !(p.uuid.is_nil() && p.username.is_empty())).collect())
    }
}
