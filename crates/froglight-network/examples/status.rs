//! TODO

use bevy_tasks::{IoTaskPool, TaskPool, block_on};
use froglight_common::version::Version;
use froglight_network::{
    packet::{common::ConnectionIntent, v1_21_4::prelude::*},
    prelude::*,
};
use smol_str::SmolStr;

/// The address of the server to request the status from.
static SERVER_ADDRESS: SmolStr = SmolStr::new_static("hypixel.net");

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize the `IoTaskPool`
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare's DNS servers
    let resolver = FroglightResolver::cloudflare();

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
        ClientboundStatusPackets::QueryResponse(response) => {
            println!("Connected to {peer} ->\n{response:#?}");

            assert!(
                response.description.contains("Hypixel"),
                "Did not find \"Hypixel\" in the server description?"
            );

            Ok(())
        }
    }
}
