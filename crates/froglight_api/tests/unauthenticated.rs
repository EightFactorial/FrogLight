//! TODO

use froglight_api::client::ApiClient;
use futures_lite::future::block_on;
use uuid::Uuid;

#[cfg(feature = "tracing")]
fn init_subscriber() {
    use tracing_subscriber::{EnvFilter, fmt};
    let env =
        EnvFilter::builder().parse_lossy("hickory_proto=error,rustls=error,ureq_proto=error,debug");
    let _ = fmt().with_env_filter(env).try_init();
}

#[test]
fn public_keys() {
    #[cfg(feature = "tracing")]
    init_subscriber();

    let client = ApiClient::standard();

    if let Err(err) = block_on(client.get_public_keys()) {
        panic!("Failed to get public keys: {err}");
    }
}

#[test]
fn username_to_uuid() {
    #[cfg(feature = "tracing")]
    init_subscriber();

    let client = ApiClient::standard();

    for (username, expected) in
        [("jeb_", Uuid::parse_str("853c80ef3c3749fdaa49938b674adae6").unwrap())]
    {
        match block_on(client.get_uuid(username)) {
            Ok(response) => assert_eq!(
                response, expected,
                "Expected UUID for \"{username}\" to be {expected}, but got {response}"
            ),
            Err(err) => panic!("Failed to get UUID for \"{username}\": {err}"),
        }
    }
}

#[test]
fn uuid_to_username() {
    #[cfg(feature = "tracing")]
    init_subscriber();

    let client = ApiClient::standard();

    for (uuid, expected) in [(Uuid::parse_str("853c80ef3c3749fdaa49938b674adae6").unwrap(), "jeb_")]
    {
        match block_on(client.get_username(uuid)) {
            Ok(response) => assert_eq!(
                response, expected,
                "Expected Username for {uuid} to be \"{expected}\", but got \"{response}\""
            ),
            Err(err) => panic!("Failed to get Username for {uuid}: {err}"),
        }
    }
}

#[test]
fn uuid_to_profile() {
    #[cfg(feature = "tracing")]
    init_subscriber();

    let client = ApiClient::standard();

    for (uuid, expected_username, expected_skin, expected_cape) in [(
        Uuid::parse_str("853c80ef3c3749fdaa49938b674adae6").unwrap(),
        "jeb_",
        Some(
            "http://textures.minecraft.net/texture/7fd9ba42a7c81eeea22f1524271ae85a8e045ce0af5a6ae16c6406ae917e68b5",
        ),
        Some(
            "http://textures.minecraft.net/texture/9e507afc56359978a3eb3e32367042b853cddd0995d17d0da995662913fb00f7",
        ),
    )] {
        match block_on(client.get_profile(uuid)) {
            Ok(response) => {
                assert_eq!(
                    response.username, expected_username,
                    "Expected Username for {uuid} to be \"{expected_username}\", but got \"{}\"",
                    response.username
                );

                assert_eq!(
                    response.skin.as_deref(),
                    expected_skin,
                    "Expected Skin for \"{expected_username}\" to be \"{expected_skin:?}\", but got \"{:?}\"",
                    response.skin
                );
                assert_eq!(
                    response.cape.as_deref(),
                    expected_cape,
                    "Expected Cape for \"{expected_username}\" to be \"{expected_cape:?}\", but got \"{:?}\"",
                    response.cape
                );
            }
            Err(err) => panic!("Failed to get Profile for {uuid}: {err}"),
        }
    }
}
