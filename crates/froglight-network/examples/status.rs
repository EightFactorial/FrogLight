//! TODO

use bevy_log::tracing_subscriber::{EnvFilter, fmt};
use bevy_tasks::{IoTaskPool, TaskPool, futures_lite::future::block_on};
use froglight_common::version::Version;
use froglight_network::{
    prelude::*,
    resolver::hickory::{ResolverConfig, ResolverOpts},
    types::ConnectionIntent,
    version::v1_21_4::prelude::*,
};
use smol_str::SmolStr;

/// The address of the server to request the status from.
static SERVER_ADDRESS: SmolStr = SmolStr::new_static("hypixel.net");

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize tracing and the `IoTaskPool`
    let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare DNS
    let resolver = FroglightResolver::new(ResolverConfig::cloudflare(), ResolverOpts::default());

    // Connect and send the handshake packet
    let mut conn = ClientConnection::<V1_21_4, _>::connect(&SERVER_ADDRESS, resolver).await?;
    conn.write(HandshakePacket {
        protocol: V1_21_4::PROTOCOL_ID as i32,
        address: SERVER_ADDRESS.clone(),
        port: 25565,
        intent: ConnectionIntent::Status,
    })
    .await?;

    // Enter the status state and request the server's status
    let mut conn = conn.status();
    conn.write(QueryRequestPacket).await?;

    // Print the response
    match conn.read().await? {
        ClientboundStatusPackets::PingResult(..) => panic!("Got a ping response?"),
        ClientboundStatusPackets::QueryResponse(response) => {
            println!("{response:#?}");
            Ok(())
        }
    }
}
