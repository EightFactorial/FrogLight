#![expect(clippy::too_many_lines, reason = "Code generation")]

use std::fmt::Write;

use convert_case::{Case, Casing};
use indexmap::IndexMap;
use miette::Result;

use crate::{
    common::WORKSPACE_DIR,
    config::{ConfigBundle, VersionPair},
    generator::crates::packet::{GeneratedPacket, PacketInfo, VersionPackets, VersionState},
    helper::{ModuleBuilder, SubModuleSettings},
};

pub(super) async fn generate_packets(
    generator: IndexMap<VersionPair, VersionPackets>,
    _config: &ConfigBundle,
) -> Result<()> {
    let path = WORKSPACE_DIR.join("froglight-packet/src");
    let mut module = ModuleBuilder::new("generated", path.clone());

    module
        .with_docs(
            "Generated packet definitions

Do not edit anything other than packet fields and marked modules,
as everything else is automatically @generated and will be overwritten.",
        )
        .with_attribute(
            "#![allow(clippy::wildcard_imports, missing_docs, reason = \"Generated code\")]",
        );

    for (version, packets) in generator {
        module
            .with_submodule(&version.base.as_feature(), async |module, mut settings| {
                let mut path = path.join("generated");
                path.push(version.base.as_feature());

                module.with_docs(&format!(
                    "This file is auto-generated. Disable this by adding a `manual` tag.

@generated packets for {}",
                    version.base.as_str()
                ));

                for (state, packets) in packets.states {
                    module
                        .with_submodule(&state, async |module, settings| {
                            let mut settings =
                                generate_state(&version, &state, packets, module, settings).await?;

                            // Skip actually building the module if it is marked as @manual
                            let path = path.join(&state).join("mod.rs");
                            if path.exists()
                                && let Ok(content) = tokio::fs::read_to_string(&path).await
                                && content.contains("@manual")
                            {
                                settings.build = false;
                            }

                            Ok(settings.with_public(true))
                        })
                        .await?;
                }

                let version_feature = version.base.as_feature();
                let version_ident = version_feature.to_ascii_uppercase();
                module.with_content(&format!(
                    "#[cfg(feature = \"{version_feature}\")]
mod traits {{
    use froglight_common::version::{version_ident};

    use super::*;
    use crate::{{
        common::handshake::{{ConnectionIntent, HandshakeContent}},
        version::*,
    }};

    impl PacketVersion for {version_ident} {{
        type Config = Config;
        type Handshake = Handshake;
        type Login = Login;
        type Play = Play;
        type Status = Status;
    }}

    impl PacketState<{version_ident}> for Handshake {{
        type Clientbound = handshake::ClientboundPackets;
        type Serverbound = handshake::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {{
            let handshake::ServerboundPackets::Intention(handshake::IntentionC2SPacket(
                HandshakeContent {{ intent, .. }},
            )) = packet;

            match intent {{
                ConnectionIntent::Status => Some(PacketStateEnum::Status),
                ConnectionIntent::Login | ConnectionIntent::Transfer => {{
                    Some(PacketStateEnum::Login)
                }}
            }}
        }}
    }}

    impl PacketState<{version_ident}> for Status {{
        type Clientbound = status::ClientboundPackets;
        type Serverbound = status::ServerboundPackets;

        fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> {{ None }}
    }}

    impl PacketState<{version_ident}> for Login {{
        type Clientbound = login::ClientboundPackets;
        type Serverbound = login::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {{
            matches!(packet, login::ServerboundPackets::LoginAcknowledged(_))
                .then_some(PacketStateEnum::Config)
        }}
    }}

    impl PacketState<{version_ident}> for Config {{
        type Clientbound = configuration::ClientboundPackets;
        type Serverbound = configuration::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {{
            matches!(packet, configuration::ServerboundPackets::FinishConfiguration(_))
                .then_some(PacketStateEnum::Play)
        }}
    }}

    impl PacketState<{version_ident}> for Play {{
        type Clientbound = play::ClientboundPackets;
        type Serverbound = play::ServerboundPackets;

        fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {{
            matches!(packet, play::ServerboundPackets::ConfigurationAcknowledged(_))
                .then_some(PacketStateEnum::Play)
        }}
    }}
}}
"
                ));

                // Skip actually building the module if it is marked as @manual
                if path.exists()
                    && let Ok(content) = tokio::fs::read_to_string(&path).await
                    && content.contains("@manual")
                {
                    settings.build = false;
                }

                Ok(settings.with_public(true))
            })
            .await?;
    }

    module.build().await
}

async fn generate_state(
    version: &VersionPair,
    state: &str,
    packets: VersionState,
    module: &mut ModuleBuilder,
    settings: SubModuleSettings,
) -> Result<SubModuleSettings> {
    let version_feature = version.base.as_feature();
    let state_ident = state_to_ident(state);

    let mut imports = String::new();
    module.with_docs(&format!(
        "@generated [`{state_ident}`](crate::version::{state_ident}) packets for v{}",
        version.base.as_str()
    ));

    let mut clientbound_enum = String::new();
    for (index, (name, packet)) in packets.clientbound.into_iter().enumerate() {
        let variant = packet_name_to_variant(&name);
        let ident = packet_name_to_ident(&name, true);

        writeln!(clientbound_enum, "    {variant}({ident}) = 0x{index:02x},").unwrap();
        match packet {
            GeneratedPacket::Packet { info } => {
                let packet_ext = name.split_once(':').unwrap().1.replace('/', "_");
                let filename = format!("s2c_0x{index:02x}_{packet_ext}");

                writeln!(imports, "\nmod {filename};\npub use {filename}::{ident};").unwrap();
                module
                    .with_submodule(&filename, async |module, settings| {
                        let mut settings = generate_struct(info, true, module, settings)?;
                        settings.auto_import = false;

                        module.with_docs(&format!(
                            "This file is auto-generated. Disable this by adding a `manual` tag.

@generated packet for \"{name}\"",
                        ));

                        // Skip actually building the module if it is marked as @manual
                        let path = WORKSPACE_DIR
                            .join("froglight-packet/src/generated")
                            .join(&version_feature)
                            .join(state)
                            .join(&filename)
                            .with_extension("rs");
                        if path.exists()
                            && let Ok(content) = tokio::fs::read_to_string(&path).await
                            && content.contains("@manual")
                        {
                            settings.build = false;
                        }

                        Ok(settings)
                    })
                    .await?;
            }
            GeneratedPacket::Path { version, state, .. } => {
                let version = version.as_feature();
                writeln!(imports, "\npub use crate::generated::{version}::{state}::{ident};")
                    .unwrap();
            }
        }
    }

    let mut serverbound_enum = String::new();
    for (index, (name, packet)) in packets.serverbound.into_iter().enumerate() {
        let variant = packet_name_to_variant(&name);
        let ident = packet_name_to_ident(&name, false);

        writeln!(serverbound_enum, "    {variant}({ident}) = 0x{index:02x},").unwrap();
        match packet {
            GeneratedPacket::Packet { info } => {
                let filename = format!("c2s_0x{index:02x}_{}", name.split_once(':').unwrap().1);
                writeln!(imports, "\nmod {filename};\npub use {filename}::{ident};").unwrap();

                module
                    .with_submodule(&filename, async |module, settings| {
                        let mut settings = generate_struct(info, false, module, settings)?;
                        settings.auto_import = false;

                        module.with_docs(&format!(
                            "This file is auto-generated. Disable this by adding a `manual` tag.

@generated packet for \"{name}\"",
                        ));

                        // Skip actually building the module if it is marked as @manual
                        let path = WORKSPACE_DIR
                            .join("froglight-packet/src/generated")
                            .join(&version_feature)
                            .join(state)
                            .join(&filename)
                            .with_extension("rs");
                        if path.exists()
                            && let Ok(content) = tokio::fs::read_to_string(&path).await
                            && content.contains("@manual")
                        {
                            settings.build = false;
                        }

                        Ok(settings)
                    })
                    .await?;
            }
            GeneratedPacket::Path { version, state, .. } => {
                let version = version.as_feature();
                writeln!(imports, "\npub use crate::generated::{version}::{state}::{ident};")
                    .unwrap();
            }
        }
    }

    if clientbound_enum.is_empty() {
        clientbound_enum.push_str("    None(froglight_common::impossible::Impossible) = 0x00,\n");
    }
    if serverbound_enum.is_empty() {
        serverbound_enum.push_str("    None(froglight_common::impossible::Impossible) = 0x00,\n");
    }

    module.with_content(&format!(
        "{imports}
#[repr(u8)]
#[cfg(feature = \"{version_feature}\")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = \"bevy\", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = \"bevy\", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = \"facet\", derive(facet::Facet))]
pub enum ClientboundPackets {{
{clientbound_enum}}}

#[repr(u8)]
#[cfg(feature = \"{version_feature}\")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = \"bevy\", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = \"bevy\", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = \"facet\", derive(facet::Facet))]
pub enum ServerboundPackets {{
{serverbound_enum}}}"
    ));

    Ok(settings)
}

#[expect(clippy::unnecessary_wraps, reason = "May error in the future")]
fn generate_struct(
    packet: PacketInfo,
    direction: bool,
    module: &mut ModuleBuilder,
    settings: SubModuleSettings,
) -> Result<SubModuleSettings> {
    let packet_ident = packet_name_to_ident(&packet.packet_ident, direction);

    let mut content = format!(
        "
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = \"bevy\", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = \"bevy\", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = \"facet\", derive(facet::Facet))]
pub struct {packet_ident}"
    );

    if packet.read_ops.is_empty() {
        if packet.read_hash == 0 {
            // Unknown fields, leave an empty placeholder.
            content.push_str(" {}");
        } else {
            // No fields, create a unit struct.
            content.push(';');
        }
    }

    module.with_content(&content);
    Ok(settings)
}

fn state_to_ident(state: &str) -> String {
    match state {
        "configuration" => String::from("Config"),
        other => other.to_case(Case::Pascal),
    }
}

fn packet_name_to_variant(name: &str) -> String {
    let (_namespace, packet) = name.split_once(':').unwrap();
    packet.replace('/', "_").to_case(Case::Pascal)
}

/// `true` for clientbound, `false` for serverbound
fn packet_name_to_ident(name: &str, direction: bool) -> String {
    format!("{}{}", packet_name_to_variant(name), if direction { "S2CPacket" } else { "C2SPacket" })
}
