use async_net::TcpListener;
use futures_lite::{io::BufReader, AsyncBufReadExt, AsyncWriteExt};

use crate::{
    buffer::{Decode, Encode, VarDecode, VarEncode},
    types::enums::ConnectionIntent,
    versions::{
        state::Status,
        v1_20_0::{
            handshake::{
                serverboundhandshakepacket::ServerboundHandshakePacket, ServerboundHandshakePackets,
            },
            status::{
                clientboundqueryresponsepacket::{
                    ClientboundQueryResponsePacket, QueryPlayers, QueryVersion,
                },
                serverboundqueryrequestpacket::ServerboundQueryRequestPacket,
                ClientboundStatusPackets, ServerboundStatusPackets,
            },
            V1_20_0,
        },
    },
    Connection, Version,
};

#[test]
fn test_send_receive() {
    let read = std::thread::spawn(|| futures_lite::future::block_on(read_packets()));
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert_eq!(futures_lite::future::block_on(send_packets()), Ok(()));
    assert_eq!(read.join().unwrap(), Ok(()));
}

async fn send_packets() -> Result<(), ()> {
    let mut conn = Connection::new(V1_20_0, "localhost:25566").await.unwrap();

    // Send the handshake packet
    {
        let handshake = ServerboundHandshakePacket {
            protocol_version: V1_20_0::ID,
            hostname: "localhost".to_string(),
            port: 25566,
            intention: ConnectionIntent::Status,
        };

        conn.send_packet(handshake).await.unwrap();
    }

    let mut conn: Connection<V1_20_0, Status> = conn.into();

    // Send the status request packet
    {
        let status_request = ServerboundQueryRequestPacket {};
        conn.send_packet(status_request).await.unwrap();
    }

    // Read the status response packet
    {
        let packet = conn.receive_packet().await.unwrap();
        let ClientboundStatusPackets::QueryResponse(packet) = packet else {
            panic!("Expected status response packet");
        };

        assert_eq!(packet.version.name, "1.20.1".into());
        assert_eq!(packet.version.protocol, V1_20_0::ID);
        assert_eq!(packet.players.max, 0);
        assert_eq!(packet.players.online, 0);
        assert_eq!(packet.enforces_secure_chat, Some(false));
    }

    Ok(())
}

async fn read_packets() -> Result<(), ()> {
    let listen = TcpListener::bind("localhost:25566").await.unwrap();
    let Ok((mut stream, _addr)) = listen.accept().await else {
        panic!("Failed to accept connection");
    };
    let mut bufreader = BufReader::new(stream.clone());

    // Read the handshake packet
    {
        let buf = bufreader.fill_buf().await.unwrap();
        let mut cursor = std::io::Cursor::new(buf);

        read_handshake(&mut cursor);
        let pos = cursor.position() as usize;
        bufreader.consume(pos);
    }

    // Read the status request packet
    {
        let buf = bufreader.fill_buf().await.unwrap();
        let mut cursor = std::io::Cursor::new(buf);

        read_status_request(&mut cursor);
        let pos = cursor.position() as usize;
        bufreader.consume(pos);
    }

    // Send a status response packet
    {
        let status_response: ClientboundStatusPackets = ClientboundQueryResponsePacket {
            description: String::new().into(),
            favicon: None,
            players: QueryPlayers {
                max: 0,
                online: 0,
                sample: Vec::new(),
            },
            version: QueryVersion {
                name: "1.20.1".into(),
                protocol: V1_20_0::ID,
            },
            enforces_secure_chat: Some(false),
        }
        .into();

        let mut buf = Vec::new();
        status_response.encode(&mut buf).unwrap();

        // Prepend the length of the packet
        {
            let mut len_buf = Vec::with_capacity(buf.len() + 2);
            buf.len().var_encode(&mut len_buf).unwrap();
            len_buf.extend(buf);
            buf = len_buf;
        }

        stream.write_all(&buf).await.unwrap();
    }

    Ok(())
}

fn read_handshake(buf: &mut impl std::io::Read) {
    let packet_length = u32::var_decode(buf).unwrap();
    assert_eq!(packet_length, 16);

    let packet = ServerboundHandshakePackets::decode(buf).unwrap();
    let ServerboundHandshakePackets::Handshake(packet) = packet;

    assert_eq!(packet.protocol_version, V1_20_0::ID);
    assert_eq!(packet.intention, ConnectionIntent::Status);
    assert_eq!(packet.port, 25566);
}

fn read_status_request(buf: &mut impl std::io::Read) {
    let packet_length = u32::var_decode(buf).unwrap();
    assert_eq!(packet_length, 1);

    let packet = ServerboundStatusPackets::decode(buf).unwrap();
    let ServerboundStatusPackets::QueryRequest(packet) = packet else {
        panic!("Expected status request packet");
    };
}
