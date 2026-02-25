use cafebabe::constant_pool::MemberRef;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use miette::Result;
use tokio::sync::RwLock;

use crate::{
    common::{DATA, Version, VersionStorage},
    config::ConfigBundle,
    source::JarData,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    Profile,
    Tag,
    CompoundTag,
    ContainerId,
    RotationByte,
    OptionalVarInt,
    ByteArray,
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
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        JarData::get_for(version, storage, async |jar| {
            let mut states = IndexMap::new();

            for (name, class) in [
                ("handshake", "net/minecraft/network/protocol/handshake/HandshakeProtocols"),
                ("status", "net/minecraft/network/protocol/status/StatusProtocols"),
                ("login", "net/minecraft/network/protocol/login/LoginProtocols"),
                ("config", "net/minecraft/network/protocol/configuration/ConfigurationProtocols"),
                ("play", "net/minecraft/network/protocol/game/GameProtocols"),
            ] {
                if let Some(class) = jar.get_class(class) {
                    let data = Self::identify_packet_classes(class, jar).await?;
                    states.insert(name.to_case(Case::Pascal), data);
                } else {
                    miette::bail!(
                        "Failed to find state \"{class}\" in version \"{}\"",
                        version.as_str()
                    );
                }
            }

            Ok(Self { states })
        })
        .await
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate packets for all [`Version`]s.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    let guard = DATA.owned_guard();

    for version in &config.versions {
        let storage = DATA.get_or_insert_with(version.real.clone(), RwLock::default, &guard);
        let mut storage = storage.write().await;

        PacketData::get_for(&version.real, &mut storage, async |data| {
            tracing::info!("{data:#?}");
            Ok(())
        })
        .await?;
    }

    Ok(())
}
