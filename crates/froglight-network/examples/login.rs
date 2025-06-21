//! TODO

use async_lock::Mutex;
#[cfg(feature = "bevy")]
use bevy_tasks::{IoTaskPool, TaskPool};
use froglight_network::{
    packet::{common::ConnectionIntent, v1_21_4::prelude::*},
    prelude::*,
};
use froglight_packet::common::{
    PlayerProfile,
    profile::{PlayerProfileTextures, PlayerUsername, PlayerUuid},
};
use froglight_text::translate::TextTranslations;
use futures_lite::future::block_on;
use smol_str::SmolStr;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};
use uuid::Uuid;

/// The address of the server to request the status from.
static SERVER_ADDRESS: SmolStr = SmolStr::new_static("localhost");

/// The player's profile used for the connection.
static PLAYER_PROFILE: Mutex<PlayerProfile> = Mutex::new(PlayerProfile::new_with_textures(
    PlayerUsername::static_new("froglight"),
    PlayerUuid::const_new(Uuid::nil()),
    PlayerProfileTextures::new(),
));

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize the tracing subscriber
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        let _ = fmt().with_env_filter(filter).try_init();
    }

    // Initialize the `IoTaskPool`
    #[cfg(feature = "bevy")]
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

#[expect(clippy::too_many_lines)]
async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare's DNS servers
    let resolver = FroglightResolver::cloudflare();
    let translations = TextTranslations::new();

    // Create a new client connection to the server
    let mut conn = ClientConnection::<V1_21_4, _>::connect(&SERVER_ADDRESS, &resolver).await?;
    let peer = conn.peer_addr().await?;

    // Send a handshake packet to the server
    conn.handshake(&SERVER_ADDRESS, peer.port(), ConnectionIntent::Login).await?;

    // Enter the login state
    let mut conn = conn.login();
    conn.login_profile(&*PLAYER_PROFILE.lock().await).await?;

    // Complete the login process
    *PLAYER_PROFILE.lock().await = conn
        .login_handle::<_, ConnectionError>(async |conn, p| match p {
            ClientboundLoginPackets::LoginSuccess(p) => {
                info!("Login Success: \"{}\" ({})", p.username.as_ref(), p.uuid);
                conn.write(EnterConfigurationC2SPacket).await?;
                Ok(Some(p.profile))
            }
            ClientboundLoginPackets::LoginCompression(p) => {
                info!("Compression Threshold: {}", p.compression_threshold);
                conn.as_raw_mut().set_compression(Some(p.compression_threshold)).await;
                Ok(None)
            }
            ClientboundLoginPackets::LoginQueryRequest(p) => {
                info!("Query Request: \"{}\" ({})", p.identifier, p.query_id);
                conn.write(LoginQueryResponseC2SPacket { query_id: p.query_id, payload: None })
                    .await?;
                Ok(None)
            }
            ClientboundLoginPackets::CookieRequest(p) => {
                info!("Cookie Request: \"{}\"", p.key);
                conn.write(CookieResponseC2SPacket { key: p.key, payload: None }).await?;
                Ok(None)
            }
            ClientboundLoginPackets::LoginHello(_p) => {
                panic!("Server requested online encryption, which is not supported yet");
            }
            ClientboundLoginPackets::LoginDisconnect(mut p) => {
                let formatted = p.apply_legacy_formatting().as_message_ansi(&translations).unwrap();
                error!("Login Disconnect: {formatted}");
                panic!("Login Disconnect: {formatted}");
            }
        })
        .await?;

    // Enter the configuration state
    let mut conn = conn.config();

    // Complete the configuration process
    conn.config_handle::<_, ConnectionError>(async |conn, p| match p {
        ClientboundConfigPackets::Ready(..) => {
            info!("Finished configuration!");
            conn.write(ReadyC2SPacket).await?;
            Ok(true)
        }
        ClientboundConfigPackets::SelectKnownPacks(p) => {
            info!("Known Packs: {:?}", p.known);
            conn.write(SelectKnownPacksC2SPacket { known: p.known }).await?;
            Ok(false)
        }
        ClientboundConfigPackets::CookieRequest(p) => {
            info!("Cookie Request: \"{}\"", p.key);
            conn.write(CookieResponseC2SPacket { key: p.key, payload: None }).await?;
            Ok(false)
        }
        ClientboundConfigPackets::Features(p) => {
            info!("Features: {:?}", p.features);
            Ok(false)
        }
        ClientboundConfigPackets::DynamicRegistries(p) => {
            info!("Registry: \"{}\"", p.registry);
            Ok(false)
        }
        ClientboundConfigPackets::SynchronizeTags(..) => {
            info!("Synchronize Tags");
            Ok(false)
        }
        ClientboundConfigPackets::StoreCookie(p) => {
            info!("Cookie Store: \"{}\" -> {:?}", p.key, p.payload);
            Ok(false)
        }
        ClientboundConfigPackets::CustomPayload(p) => {
            info!("Custom Payload: \"{}\" -> {:?}", p.identifier, p.payload);
            Ok(false)
        }
        ClientboundConfigPackets::KeepAlive(p) => {
            info!("Keep Alive: {}", p.id);
            conn.write(KeepAliveC2SPacket { id: p.id }).await?;
            Ok(false)
        }
        ClientboundConfigPackets::CommonPing(p) => {
            info!("Common Ping: {}", p.id);
            conn.write(CommonPongC2SPacket { id: p.id }).await?;
            Ok(false)
        }
        ClientboundConfigPackets::ResetChat(..) => {
            info!("Reset Chat");
            Ok(false)
        }
        ClientboundConfigPackets::ResourcePackRemove(_p) => todo!(),
        ClientboundConfigPackets::ResourcePackSend(_p) => todo!(),
        ClientboundConfigPackets::ServerTransfer(_p) => todo!(),
        ClientboundConfigPackets::CustomReportDetails(_p) => todo!(),
        ClientboundConfigPackets::ServerLinks(_p) => todo!(),
        ClientboundConfigPackets::Disconnect(mut p) => {
            let formatted = p.apply_legacy_formatting().as_message_ansi(&translations).unwrap();
            error!("Configuration Disconnect: {formatted}");
            panic!("Configuration Disconnect: {formatted}");
        }
    })
    .await?;

    info!("Login complete, disconnecting...");

    Ok(())
}
