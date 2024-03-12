use std::any::TypeId;

use bevy_log::trace;
use froglight_protocol::traits::Version;
use hashbrown::HashMap;

use crate::biomes::{
    traits::{BiomeEnumTrait, BiomeRegistration},
    BiomeEnum, BiomeType,
};

/// The inner registry for the biome registry.
#[derive(Debug, Default)]
pub struct InnerBiomeRegistry<V: Version> {
    /// A collection of biomes inside the registry.
    pub(crate) dyn_biomes: Vec<Box<dyn BiomeType<V>>>,

    /// A map of biome type ids to biome ids.
    pub(crate) type_map: HashMap<TypeId, u32>,
}

impl<V: Version> InnerBiomeRegistry<V> {
    /// Creates a new empty registry.
    #[must_use]
    #[inline]
    pub fn new() -> Self { Self::default() }

    /// Get the biome id of a biome type.
    #[must_use]
    #[inline]
    pub fn id_of<T: BiomeType<V>>(&self) -> Option<u32> {
        self.type_map.get(&TypeId::of::<T>()).copied()
    }

    /// Gets a `dyn biome` from the registry.
    #[must_use]
    #[inline]
    pub fn get_dyn(&self, biome: u32) -> Option<&dyn BiomeType<V>> {
        let biome_id = usize::try_from(biome).ok()?;
        self.dyn_biomes.get(biome_id).map(AsRef::as_ref)
    }

    /// Gets a biome from the registry.
    #[must_use]
    #[inline]
    pub fn get_biome(&self, biome: u32) -> Option<BiomeEnum>
    where
        V: BiomeRegistration,
    {
        V::Biomes::get_biome(biome, self).map(Into::into)
    }

    /// Register a biome into the biome registry.
    #[allow(clippy::missing_panics_doc)]
    pub fn register_biome<B: BiomeType<V> + Default>(&mut self) -> &mut Self {
        let biome = B::default();

        #[cfg(debug_assertions)]
        {
            trace!("Registering biome `{}`", biome.resource_key());
        }

        // Insert the biome into the dyn_biomes list
        let index = self.dyn_biomes.len();
        self.dyn_biomes.push(Box::new(biome));

        // Insert the biome id into the type map
        self.type_map.insert(TypeId::of::<B>(), u32::try_from(index).expect("Biome id overflow"));

        self
    }
}
