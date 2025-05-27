//! TODO

#[cfg(feature = "bevy")]
use bevy_tasks::{IoTaskPool, TaskPool};
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
    let translations = TextTranslations::new();

    // Create a new client connection to the server
    let mut conn = ClientConnection::<V1_21_4, _>::connect(&SERVER_ADDRESS, &resolver).await?;

    // Get the peer's address and send a handshake packet to the server
    let peer = conn.peer_addr().await?;
    conn.handshake(&SERVER_ADDRESS, peer.port(), ConnectionIntent::Status).await?;

    // Enter the status state
    let mut conn = conn.status();

    // Request the server's status, apply any legacy formatting codes,
    // and print the server's description
    let mut status = conn.query_status().await?;
    match status.description.apply_legacy_formatting().as_message_ansi(&translations) {
        Ok(txt) => println!("Connected to \'{SERVER_ADDRESS}\' at \'{peer}\':\n{txt}"),
        Err(err) => panic!("Failed to parse description: {err}"),
    }

    #[cfg(feature = "std")]
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Get current time and the number of milliseconds since the UNIX epoch
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

        // Send a ping request to the server and wait for the response
        let response = conn.query_ping(millis).await?;

        #[cfg(feature = "trace")]
        if millis != response {
            // Warn if the ping response does not match the expected value
            tracing::warn!("Ping mismatch: expected '{millis}', got '{response}'?");
        }

        // Print how long the ping took
        let duration = now.elapsed().unwrap();
        println!("Ping: {}ms", duration.as_millis());
    }

    Ok(())
}
