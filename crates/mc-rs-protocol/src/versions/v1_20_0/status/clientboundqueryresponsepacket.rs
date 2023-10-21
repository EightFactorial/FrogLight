use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Transcode, Serialize, Deserialize)]
#[json]
pub struct ClientboundQueryResponsePacket {
    pub description: FormattedText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub players: QueryPlayers,
    pub version: QueryVersion,
    #[serde(
        default,
        rename = "enforcesSecureChat",
        skip_serializing_if = "Option::is_none"
    )]
    pub enforces_secure_chat: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryVersion {
    pub name: FormattedText,
    pub protocol: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryPlayers {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<QuerySamplePlayer>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuerySamplePlayer {
    pub uuid: Uuid,
    pub name: String,
}

#[test]
fn clientbound_query_response_packet_test() {
    use crate::{buffer::Encode, traits::Version, versions::v1_20_0::V1_20_0};

    let mut buf = Vec::new();
    let packet = ClientboundQueryResponsePacket {
        description: "Hello world!".into(),
        favicon: None,
        players: QueryPlayers {
            max: 100,
            online: 50,
            sample: vec![],
        },
        version: QueryVersion {
            name: "1.20.1".into(),
            protocol: V1_20_0::ID,
        },
        enforces_secure_chat: None,
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        String::from_utf8(buf[2..].to_vec()),
        Ok(
            r#"{"description":{"text":"Hello world!"},"players":{"max":100,"online":50,"sample":[]},"version":{"name":{"text":"1.20.1"},"protocol":763}}"#
        .to_string())
    );
}
