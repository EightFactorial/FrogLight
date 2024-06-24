#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
use serde::{ser::SerializeSeq, Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// A status response from a server
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(json)]
pub struct ServerStatus {
    // TODO: Text
    /// The server's description
    pub description: Value,
    /// The server's icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    /// The server's player information
    pub players: ServerPlayers,
    /// The server's version information
    pub version: ServerVersion,
    /// Whether the server enforces secure chat
    #[serde(default, rename = "enforcesSecureChat", skip_serializing_if = "Option::is_none")]
    pub enforces_secure_chat: Option<bool>,

    /// Other fields that are not part of the standard status response
    #[serde(default, flatten, skip_serializing_if = "HashMap::is_empty")]
    pub other: HashMap<CompactString, Value>,
}

/// The server's version information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerVersion {
    // TODO: Text
    /// The version's name
    pub name: Value,
    /// The version's protocol id
    pub protocol: i32,
}

/// The server's player information
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerPlayers {
    /// The maximum number of players
    pub max: i32,
    /// The number of online players
    pub online: i32,
    /// A sample of online players
    #[serde(default, with = "ServerSamplePlayer")]
    pub sample: Vec<ServerSamplePlayer>,
}

/// Player information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerSamplePlayer {
    /// The player's UUID
    #[serde(default, skip_serializing_if = "Uuid::is_nil")]
    pub uuid: Uuid,
    /// The player's username
    #[serde(default, skip_serializing_if = "CompactString::is_empty")]
    pub username: CompactString,
}

impl ServerSamplePlayer {
    /// Serialize a list of players, filtering out players with
    /// empty usernames and nil UUIDs
    fn serialize<S>(list: &[Self], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for player in list.iter().filter(|p| !(p.uuid.is_nil() && p.username.is_empty())) {
            seq.serialize_element(player)?;
        }
        seq.end()
    }

    /// Deserialize a list of players, filtering out players with
    /// empty usernames and nil UUIDs
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<ServerSamplePlayer>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let list = Vec::<ServerSamplePlayer>::deserialize(deserializer)?;
        Ok(list.into_iter().filter(|p| !(p.uuid.is_nil() && p.username.is_empty())).collect())
    }
}

// TODO: FormattedText
// #[test]
// fn serverstatus_read_write_verify() {
//     use crate::{
//         io::{FrogRead, FrogWrite},
//         traits::Version,
//         versions::v1_20_0::V1_20_0,
//     };

//     let mut buf = Vec::new();
//     let status = ServerStatus {
//         description: "Hello world!".into(),
//         favicon: None,
//         players: ServerPlayers { max: 100, online: 50, sample: vec![] },
//         version: ServerVersion { name: "1.20.1".into(), protocol:
// V1_20_0::PROTOCOL_VERSION },         enforces_secure_chat: None,
//     };

//     // Write the status to the buffer
//     status.fg_write(&mut buf).unwrap();
//     // Read a string from the buffer
//     let mut cursor = std::io::Cursor::new(buf.as_slice());
//     let string = String::fg_read(&mut cursor).unwrap();

//     // Verify that the string matches the status as json
//     assert_eq!(
//         string,
//         r#"{"description":{"text":"Hello
// world!"},"players":{"max":100,"online":50,"sample":[]},"version":{"name":{"
// text":"1.20.1"},"protocol":763}}"#     );
// }
