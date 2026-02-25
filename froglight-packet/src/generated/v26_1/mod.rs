//! This file is auto-generated. Disable this by adding a `manual` tag.
//! 
//! @generated packets for 26.1
pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;
pub mod play;

// -------------------------------------------------------------------------------------------------
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
            let handshake::ServerboundPackets::Intention(handshake::IntentionC2SPacket(
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
            matches!(packet, login::ServerboundPackets::LoginAcknowledged(_))
                .then_some(PacketStateEnum::Config)
        }
    }

    impl PacketState<V26_1> for Config {
        type Clientbound = configuration::ClientboundPackets;
        type Serverbound = configuration::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
    }

    impl PacketState<V26_1> for Play {
        type Clientbound = play::ClientboundPackets;
        type Serverbound = play::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
    }
}

