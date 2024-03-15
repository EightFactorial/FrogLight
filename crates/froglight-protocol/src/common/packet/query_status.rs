use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A status response from a server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[frog(json)]
pub struct ServerStatus {
    // TODO: FormattedText
    /// The server's description
    pub description: String,
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
}

/// The server's version information
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ServerVersion {
    // TODO: FormattedText
    /// The version's name
    pub name: String,
    /// The version's protocol id
    pub protocol: i32,
}

/// The server's player information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ServerPlayers {
    /// The maximum number of players
    pub max: i32,
    /// The number of online players
    pub online: i32,
    /// A sample of online players
    #[serde(default)]
    pub sample: Vec<ServerSamplePlayer>,
}

/// Player information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ServerSamplePlayer {
    /// The player's UUID
    pub uuid: Uuid,
    /// The player's username
    pub username: CompactString,
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
