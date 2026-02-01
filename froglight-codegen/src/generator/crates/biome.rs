use cafebabe::{
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use indexmap::IndexMap;
use miette::Result;

use crate::{
    common::{Version, VersionStorage},
    config::{ConfigBundle, VersionPair},
    source::JarData,
};

pub struct BiomeData {
    pub biomes: IndexMap<String, String>,
}

impl BiomeData {
    /// Get the [`BiomeData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `BiomeData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`BiomeData`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut biomes = IndexMap::new();

        JarData::get_for(version, storage, async |data| {
            let code = data
                .get_class_method_code("net/minecraft/world/level/biome/Biomes", "<clinit>")
                .unwrap();

            let mut constant = None;
            for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s))) => {
                        constant = Some(s.as_ref());
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type }) => {
                        if class_name != "net/minecraft/world/level/biome/Biomes" {
                            tracing::warn!(
                                "Unexpected class name in Biomes <clinit>: {class_name}"
                            );
                        }
                        if name_and_type.descriptor != "Lnet/minecraft/resources/ResourceKey;" {
                            tracing::warn!(
                                "Unexpected descriptor in Biomes <clinit>: {}",
                                name_and_type.descriptor
                            );
                        }

                        if let Some(constant) = constant.take() {
                            biomes.insert(
                                name_and_type.name.to_string(),
                                format!("minecraft:{constant}"),
                            );
                        } else {
                            tracing::warn!(
                                "Putstatic without preceding LDC in Biomes <clinit>: {}",
                                name_and_type.name
                            );
                        }
                    }
                    _ => {}
                }
            }

            Ok(())
        })
        .await?;

        tracing::debug!("Found {} biomes for \"{}\"", biomes.len(), version.as_str());

        Ok(BiomeData { biomes })
    }
}

// -------------------------------------------------------------------------------------------------

pub async fn generate(
    version: &VersionPair,
    storage: &mut VersionStorage,
    _config: &ConfigBundle,
) -> Result<()> {
    BiomeData::get_for(&version.real, storage, async |_data| Ok(())).await
}
