//! TODO

#[cfg(feature = "bevy")]
use bevy_tasks::{IoTaskPool, TaskPool};
use froglight_common::version::Version;
use froglight_network::{
    packet::{common::ConnectionIntent, v1_21_4::prelude::*},
    prelude::*,
};
use froglight_text::translate::TextTranslations;
use futures_lite::future::block_on;
use smol_str::SmolStr;
use tracing_subscriber::{EnvFilter, fmt};

/// The address of the server to request the status from.
static SERVER_ADDRESS: SmolStr = SmolStr::new_static("hypixel.net");

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

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare's DNS servers
    let resolver = FroglightResolver::cloudflare();
    let translations = TextTranslations::default();

    // Connect and send the handshake packet
    let mut conn = ClientConnection::<V1_21_4, _>::connect(&SERVER_ADDRESS, &resolver).await?;
    let peer = conn.peer_addr().await?;
    conn.write(HandshakePacket {
        protocol: V1_21_4::PROTOCOL_ID as i32,
        address: SERVER_ADDRESS.clone(),
        port: peer.port(),
        intent: ConnectionIntent::Status,
    })
    .await?;

    // Enter the status state and request the server's status
    let mut conn = conn.status();
    conn.write(QueryRequestPacket).await?;

    // Print the response
    match conn.read().await? {
        ClientboundStatusPackets::PingResult(..) => panic!("Got a ping response?"),
        ClientboundStatusPackets::QueryResponse(mut response) => {
            match response.description.apply_legacy_formatting().as_message_ansi(&translations) {
                Ok(txt) => println!("Connected to \'{SERVER_ADDRESS}\' at \'{peer}\':\n{txt}"),
                Err(err) => panic!("Failed to parse description: {err}"),
            }
            Ok(())
        }
    }
}
