use std::sync::atomic::{AtomicBool, Ordering};

use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshake,
    traits::Version,
    versions::v1_21_0::{
        configuration::ConfigurationServerboundPackets,
        handshake::HandshakePacket,
        login::{LoginClientboundPackets, LoginHelloC2SPacket, LoginServerboundPackets},
        play::PlayServerboundPackets,
        V1_21_0,
    },
};

use super::PerformServerConnection;
use crate::{
    connection::{Connection, ConnectionError, Serverbound},
    network::{
        channel::{AsyncConnectionChannel, ConnectionHolder},
        ChannelSendPacket,
    },
};

impl PerformServerConnection for V1_21_0 {
    async fn perform_handshake(
        mut conn: Connection<Self, Handshake, Serverbound>,
    ) -> Result<Connection<Self, Handshake, Serverbound>, ConnectionError> {
        // Send a Handshake to the server.
        conn.send(HandshakePacket {
            protocol: V1_21_0::ID,
            address: conn.info.get_address(),
            port: conn.info.get_port(),
            intent: ConnectionIntent::Login,
        })
        .await?;

        Ok(conn)
    }

    async fn perform_login(
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<ConnectionHolder<Self, Serverbound>, ConnectionError> {
        let ConnectionHolder::Login(mut read, mut write) = conn else {
            panic!("Expected Login state, got something else!");
        };

        // Send the account information to the server.
        write
            .send(LoginHelloC2SPacket {
                username: read.account.read().await.username.clone(),
                uuid: read.account.read().await.uuid,
            })
            .await?;

        // Whether the connection has finished.
        let finished = AtomicBool::default();

        futures_lite::future::race(
            async {
                // Wait for the server to send a response
                while !finished.load(Ordering::Relaxed) {
                    let packet = read.recv().await?;
                    bevy_log::debug!("Received packet: {:?}", packet);

                    // Update the compression threshold if the server sends it.
                    if let LoginClientboundPackets::LoginCompression(packet) = &packet {
                        *read.compression.write().await =
                            Some(packet.threshold.try_into().unwrap());
                    }

                    if channel.send_login(packet).await.is_err() {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                // Wait for the client to send a response
                while !finished.load(Ordering::Relaxed) {
                    match channel.recv().await {
                        Ok(ChannelSendPacket::Login(packet)) => {
                            bevy_log::debug!("Sending packet: {:?}", packet);

                            // If the client acknowledges the connection is finished, stop the loop.
                            if matches!(
                                packet.as_ref(),
                                LoginServerboundPackets::EnterConfiguration(..)
                            ) {
                                finished.store(true, Ordering::Relaxed);
                                write.send_packet(&packet).await?;
                                return Ok(());
                            }

                            write.send_packet(&packet).await?;
                        }
                        Ok(other) => panic!("Expected Login packet, got {other:?}"),
                        Err(_) => break,
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(ConnectionHolder::Login(read, write))
    }

    async fn perform_configuration(
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<ConnectionHolder<Self, Serverbound>, ConnectionError> {
        let ConnectionHolder::Config(mut read, mut write) = conn else {
            panic!("Expected Config state, got something else!");
        };

        // Whether the connection has finished.
        let finished = AtomicBool::default();

        futures_lite::future::race(
            async {
                // Wait for the server to send a response
                while !finished.load(Ordering::Relaxed) {
                    let packet = read.recv().await?;
                    if channel.send_configuration(packet).await.is_err() {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                // Wait for the client to send a response
                while !finished.load(Ordering::Relaxed) {
                    match channel.recv().await {
                        Ok(ChannelSendPacket::Config(packet)) => {
                            // If the client acknowledges the connection is finished, stop the loop.
                            if matches!(packet.as_ref(), ConfigurationServerboundPackets::Ready(..))
                            {
                                finished.store(true, Ordering::Relaxed);
                                write.send_packet(&packet).await?;
                                return Ok(());
                            }

                            write.send_packet(&packet).await?;
                        }
                        Ok(other) => panic!("Expected Config packet, got {other:?}"),
                        Err(_) => break,
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(ConnectionHolder::Config(read, write))
    }

    async fn perform_play(
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<Option<ConnectionHolder<Self, Serverbound>>, ConnectionError> {
        let ConnectionHolder::Play(mut read, mut write) = conn else {
            panic!("Expected Play state, got something else!");
        };

        // Whether the connection has finished.
        let finished = AtomicBool::default();

        futures_lite::future::race(
            async {
                // Wait for the server to send a response
                while !finished.load(Ordering::Relaxed) {
                    let packet = read.recv().await?;
                    if channel.send_play(packet).await.is_err() {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                // Wait for the client to send a response
                while !finished.load(Ordering::Relaxed) {
                    match channel.recv().await {
                        Ok(ChannelSendPacket::Play(packet)) => {
                            // If the client acknowledges the connection is finished, stop the loop.
                            if matches!(
                                packet.as_ref(),
                                PlayServerboundPackets::AcknowledgeReconfiguration(..)
                            ) {
                                finished.store(true, Ordering::Relaxed);
                                write.send_packet(&packet).await?;
                                return Ok(());
                            }

                            write.send_packet(&packet).await?;
                        }
                        Ok(other) => panic!("Expected Play packet, got {other:?}"),
                        Err(_) => break,
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(Some(ConnectionHolder::Play(read, write)))
    }
}
