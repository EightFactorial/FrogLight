//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @manual @generated packets for v26.1.x

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "v26_1")]
mod traits {
    use froglight_common::version::V26_1;

    use super::*;
    use crate::{
        common::handshake::{ConnectionIntent, HandshakeContent},
        version::*,
    };

    impl PacketVersion for V26_1 {
        type Config = Config;
        type Handshake = Handshake;
        type Login = Login;
        type Play = Play;
        type Status = Status;
    }

    impl PacketState<V26_1> for Handshake {
        type Clientbound = handshake::ClientboundPackets;
        type Serverbound = handshake::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {
            let handshake::ServerboundPackets::Handshake(handshake::HandshakeC2SPacket(
                HandshakeContent { intent, .. },
            )) = packet;

            match intent {
                ConnectionIntent::Status => Some(PacketStateEnum::Status),
                ConnectionIntent::Login | ConnectionIntent::Transfer => {
                    Some(PacketStateEnum::Login)
                }
            }
        }
    }

    impl PacketState<V26_1> for Status {
        type Clientbound = status::ClientboundPackets;
        type Serverbound = status::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
    }

    impl PacketState<V26_1> for Login {
        type Clientbound = login::ClientboundPackets;
        type Serverbound = login::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {
            matches!(packet, login::ServerboundPackets::EnterConfiguration(_))
                .then_some(PacketStateEnum::Config)
        }
    }

    impl PacketState<V26_1> for Config {
        type Clientbound = config::ClientboundPackets;
        type Serverbound = config::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
    }

    impl PacketState<V26_1> for Play {
        type Clientbound = play::ClientboundPackets;
        type Serverbound = play::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
    }
}
