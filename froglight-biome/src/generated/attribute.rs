//! Biome attributes for all [`Version`](froglight_common::version::Version)s.
//!
//! @generated

use alloc::{string::String, vec::Vec};

use facet::Facet;

generate! {
    @attributes
    @object "minecraft:audio/ambient_sounds" AudioAmbientSounds { additions: AudioAmbientSoundsAdditions, r#loop: String, mood: AudioAmbientSoundsMood } => { AudioAmbientSoundsAdditions { sound: String, tick_chance: f64 }, AudioAmbientSoundsMood { block_search_extent: f64, offset: f64, sound: String, tick_delay: f64 } },
    @object "minecraft:audio/background_music" AudioBackgroundMusic { default: AudioBackgroundMusicDefault, creative: AudioBackgroundMusicCreative, underwater: AudioBackgroundMusicUnderwater } => { AudioBackgroundMusicDefault { max_delay: f64, min_delay: f64, sound: String }, AudioBackgroundMusicCreative { max_delay: f64, min_delay: f64, sound: String }, AudioBackgroundMusicUnderwater { max_delay: f64, min_delay: f64, sound: String } },
    @newtype "minecraft:audio/music_volume" AudioMusicVolume f64,
    @newtype "minecraft:gameplay/can_pillager_patrol_spawn" GameplayCanPillagerPatrolSpawn bool,
    @newtype "minecraft:gameplay/increased_fire_burnout" GameplayIncreasedFireBurnout bool,
    @newtype "minecraft:gameplay/snow_golem_melts" GameplaySnowGolemMelts bool,
    @newtype "minecraft:visual/ambient_particles" VisualAmbientParticles Vec<VisualAmbientParticlesItem> => { VisualAmbientParticlesItem { particle: VisualAmbientParticlesItemParticle, probability: f64 }, VisualAmbientParticlesItemParticle { r#type: String } },
    @newtype "minecraft:visual/fog_color" VisualFogColor String,
    @newtype "minecraft:visual/sky_color" VisualSkyColor String,
    @newtype "minecraft:visual/water_fog_color" VisualWaterFogColor String,
    @object "minecraft:visual/water_fog_end_distance" VisualWaterFogEndDistance { argument: f64, modifier: String },
}
