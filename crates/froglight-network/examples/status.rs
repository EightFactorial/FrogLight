//! TODO

use bevy_tasks::{IoTaskPool, TaskPool, block_on};
use froglight_common::version::Version;
use froglight_network::{
    packet::{common::ConnectionIntent, v1_21_4::prelude::*},
    prelude::*,
};
use froglight_text::prelude::TextTranslations;
use smol_str::SmolStr;

/// The address of the server to request the status from.
static SERVER_ADDRESS: SmolStr = SmolStr::new_static("mbs.playskyward.gg");

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize the `IoTaskPool`
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
        ClientboundStatusPackets::QueryResponse(response) => {
            match response.description.as_message_ansi(&translations) {
                Ok(txt) => println!("Connected to \'{SERVER_ADDRESS}\' at \'{peer}\':\n{txt}"),
                Err(err) => panic!("Failed to parse description: {err}"),
            }
            Ok(())
        }
    }
}
