use cafebabe::constant_pool::MemberRef;
use facet::Facet;
use indexmap::IndexMap;
use miette::Result;
use tokio::sync::RwLock;

use crate::{
    common::{DATA, Version, VersionStorage},
    config::{ConfigBundle, VersionPair},
    source::{JarData, JarFile, Manifest},
};

mod analyze;
mod identify;

#[derive(Debug, Clone, PartialEq)]
pub struct PacketData {
    pub states: IndexMap<String, PacketStateData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketStateData {
    pub clientbound: IndexMap<String, PacketInfo>,
    pub serverbound: IndexMap<String, PacketInfo>,
}

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub packet_ident: String,
    pub protocol_id: u64,

    pub packet_type: MemberRef<'static>,
    pub packet_codec: Option<MemberRef<'static>>,

    pub read_ops: Vec<PacketField>,
    pub read_hash: u64,
    pub write_ops: Vec<PacketField>,
    pub write_hash: u64,
}

#[derive(Debug, Clone)]
pub struct PacketField {
    pub name: Option<String>,
    pub ty: FieldType,
}

impl PartialEq for PacketInfo {
    fn eq(&self, other: &Self) -> bool {
        self.read_hash == other.read_hash && self.write_hash == other.write_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Bool,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    VarShort,
    VarInt,
    VarLong,

    Vec(Box<FieldType>),
    Option(Box<FieldType>),
}

impl PacketData {
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `PacketData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`JarData`] for the given [`Version`].
    #[allow(clippy::too_many_lines, reason = "Parses both bytecode and json")]
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut states = JarData::get_for(version, storage, async |jar| {
            let mut states = IndexMap::new();

            for (name, class) in [
                ("handshake", "net/minecraft/network/protocol/handshake/HandshakeProtocols"),
                ("status", "net/minecraft/network/protocol/status/StatusProtocols"),
                ("login", "net/minecraft/network/protocol/login/LoginProtocols"),
                (
                    "configuration",
                    "net/minecraft/network/protocol/configuration/ConfigurationProtocols",
                ),
                ("play", "net/minecraft/network/protocol/game/GameProtocols"),
            ] {
                if let Some(class) = jar.get_class(class) {
                    let data = Self::identify_packet_classes(class, jar).await?;
                    states.insert(String::from(name), data);
                } else {
                    miette::bail!(
                        "Failed to find state \"{class}\" in version \"{}\"",
                        version.as_str()
                    );
                }
            }

            Ok(states)
        })
        .await?;

        let report = JarFile::get_for(version, storage, async |jar| {
            #[repr(transparent)]
            #[derive(Debug, Clone, PartialEq, Eq, Facet)]
            #[facet(transparent)]
            struct PacketReport(IndexMap<String, PacketReportState>);

            #[derive(Debug, Clone, PartialEq, Eq, Facet)]
            struct PacketReportState {
                pub clientbound: IndexMap<String, PacketReportPacket>,
                pub serverbound: IndexMap<String, PacketReportPacket>,
            }

            #[derive(Debug, Clone, PartialEq, Eq, Facet)]
            struct PacketReportPacket {
                pub protocol_id: u64,
            }

            let path = jar.generated.join("reports/packets.json");
            let Ok(contents) = tokio::fs::read_to_string(&path).await else {
                miette::bail!(
                    "Failed to read \"packets.json\" for version \"{}\"",
                    version.as_str(),
                );
            };

            match facet_json::from_str::<PacketReport>(&contents) {
                Ok(report) => Ok(report),
                Err(err) => Err(miette::miette!(
                    "Failed to parse \"packets.json\" for version \"{}\", {err}",
                    version.as_str(),
                )),
            }
        })
        .await?;

        for (state, rstate) in report.0 {
            let Some(pstate) = states.get_mut(&state) else {
                miette::bail!(
                    "Failed to find packet state \"{state}\" for version \"{}\"",
                    version.as_str()
                );
            };

            for (name, rpacket) in &rstate.clientbound {
                let Some(ppacket) = pstate.clientbound.get_mut(name) else {
                    miette::bail!(
                        "Failed to find clientbound packet \"{state}::{name}\" for version \"{}\"",
                        version.as_str()
                    );
                };
                ppacket.protocol_id = rpacket.protocol_id;
            }
            for (name, rpacket) in &rstate.serverbound {
                let Some(ppacket) = pstate.serverbound.get_mut(name) else {
                    miette::bail!(
                        "Failed to find serverbound packet \"{state}::{name}\" for version \"{}\"",
                        version.as_str()
                    );
                };
                ppacket.protocol_id = rpacket.protocol_id;
            }
        }

        let mut any = false;
        for packet in states
            .values()
            .flat_map(|state| state.clientbound.values().chain(state.serverbound.values()))
        {
            if packet.protocol_id == u64::MAX {
                any = true;
                tracing::error!(
                    "Packet \"{}\" has no ID for version \"{}\"",
                    packet.packet_ident,
                    version.as_str()
                );
            }
        }
        if any {
            miette::bail!(
                "One or more packets have no ID for version \"{}\", see above for details",
                version.as_str()
            );
        }

        Ok(Self { states })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate packets for all [`Version`]s.
#[allow(clippy::too_many_lines, reason = "Complex multi-version comparison logic")]
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    // Collect and sort versions by release time
    let mut versions = config.versions.clone();
    {
        let manifest = Manifest::get().await;
        versions.sort_by_key(|v| manifest.version(&v.real).unwrap().release_time);
    }

    let mut generator =
        IndexMap::<VersionPair, VersionPackets>::with_capacity(config.versions.len());

    let mut last = Option::<VersionPair>::None;
    for current in versions {
        let guard = DATA.owned_guard();
        let storage = DATA.get_or_insert_with(current.real.clone(), RwLock::default, &guard);

        let mut storage = storage.write().await;
        PacketData::get_for(&current.real, &mut storage, async |data| {
            if last.is_none() {
                tracing::info!(
                    "Generating packets starting from version \"{}\"",
                    current.base.as_str()
                );
                generator.insert(current.clone(), data.clone().into());
                return Ok(());
            }

            let last = last.take().unwrap();
            let last_data = generator.get(&last).unwrap();

            let mut current_data = VersionPackets { states: IndexMap::new() };
            for (state_name, state) in &data.states {
                let Some(last_state) = last_data.states.get(state_name) else {
                    // If the current state doesn't exist in the previous version copy everything.
                    current_data
                        .states
                        .insert(state_name.clone(), VersionState::from(state.clone()));
                    continue;
                };

                let mut current_state =
                    VersionState { clientbound: IndexMap::new(), serverbound: IndexMap::new() };

                for (packet_name, packet) in &state.clientbound {
                    match last_state.clientbound.get(packet_name) {
                        Some(GeneratedPacket::Packet { info }) => {
                            if packet == info {
                                // If the packet is the same, create a path to the previous packet.
                                current_state.clientbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::path(info, &last.base, state_name),
                                );
                            } else {
                                // If the packet is different, create a new packet.
                                current_state.clientbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::packet(packet.clone()),
                                );
                            }
                        }
                        Some(path @ GeneratedPacket::Path { info, .. }) => {
                            if packet == info {
                                // If the packet is the same, copy the previous path.
                                current_state.clientbound.insert(packet_name.clone(), path.clone());
                            } else {
                                // If the packet is different, create a new packet.
                                current_state.clientbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::packet(packet.clone()),
                                );
                            }
                        }
                        None => {
                            // If the packet doesn't exist, create a new packet.
                            current_state.clientbound.insert(
                                packet_name.clone(),
                                GeneratedPacket::packet(packet.clone()),
                            );
                        }
                    }
                }
                for (packet_name, packet) in &state.serverbound {
                    match last_state.serverbound.get(packet_name) {
                        Some(GeneratedPacket::Packet { info }) => {
                            if packet == info {
                                // If the packet is the same, create a path to the previous packet.
                                current_state.serverbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::path(info, &last.base, state_name),
                                );
                            } else {
                                // If the packet is different, create a new packet.
                                current_state.serverbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::packet(packet.clone()),
                                );
                            }
                        }
                        Some(path @ GeneratedPacket::Path { info, .. }) => {
                            if packet == info {
                                // If the packet is the same, copy the previous path.
                                current_state.serverbound.insert(packet_name.clone(), path.clone());
                            } else {
                                // If the packet is different, create a new packet.
                                current_state.serverbound.insert(
                                    packet_name.clone(),
                                    GeneratedPacket::packet(packet.clone()),
                                );
                            }
                        }
                        None => {
                            // If the packet doesn't exist, create a new packet.
                            current_state.serverbound.insert(
                                packet_name.clone(),
                                GeneratedPacket::packet(packet.clone()),
                            );
                        }
                    }
                }

                current_data.states.insert(state_name.clone(), current_state);
            }

            generator.insert(current.clone(), current_data);

            Ok(())
        })
        .await?;
        last = Some(current);
    }

    tracing::debug!("{generator:#?}");

    Ok(())
}

#[derive(Debug, Clone)]
struct VersionPackets {
    states: IndexMap<String, VersionState>,
}

#[derive(Debug, Clone)]
struct VersionState {
    clientbound: IndexMap<String, GeneratedPacket>,
    serverbound: IndexMap<String, GeneratedPacket>,
}

#[derive(Clone)]
enum GeneratedPacket {
    Packet { info: PacketInfo },
    Path { info: PacketInfo, version: Version, state: String },
}

impl GeneratedPacket {
    /// Create a new [`GeneratedPacket::Packet`].
    #[must_use]
    const fn packet(info: PacketInfo) -> Self { Self::Packet { info } }

    /// Create a new [`GeneratedPacket::Path`].
    #[must_use]
    fn path(info: &PacketInfo, version: &Version, state: &str) -> Self {
        Self::Path { info: info.clone(), version: version.clone(), state: state.to_string() }
    }
}

impl core::fmt::Debug for GeneratedPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Packet { info } => f.debug_tuple("Packet").field(&info.packet_ident).finish(),
            Self::Path { info, version, state, .. } => f
                .debug_tuple("Path")
                .field(&format!("{}::{state}::{}", version.as_str(), info.packet_ident))
                .finish(),
        }
    }
}

impl From<PacketData> for VersionPackets {
    fn from(value: PacketData) -> Self {
        VersionPackets {
            states: value.states.into_iter().map(|(state, data)| (state, data.into())).collect(),
        }
    }
}
impl From<PacketStateData> for VersionState {
    fn from(value: PacketStateData) -> Self {
        VersionState {
            clientbound: value
                .clientbound
                .into_iter()
                .map(|(name, info)| (name, GeneratedPacket::packet(info)))
                .collect(),
            serverbound: value
                .serverbound
                .into_iter()
                .map(|(name, info)| (name, GeneratedPacket::packet(info)))
                .collect(),
        }
    }
}
