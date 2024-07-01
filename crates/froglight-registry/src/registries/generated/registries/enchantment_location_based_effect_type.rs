//! @generated by `froglight-generator` #248246d

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum EnchantmentLocationBasedEffectTypeKey {
    #[frog(key = "minecraft:all_of")]
    AllOf,
    #[frog(key = "minecraft:apply_mob_effect")]
    ApplyMobEffect,
    #[frog(key = "minecraft:attribute")]
    Attribute,
    #[frog(key = "minecraft:damage_entity")]
    DamageEntity,
    #[frog(key = "minecraft:damage_item")]
    DamageItem,
    #[frog(key = "minecraft:explode")]
    Explode,
    #[frog(key = "minecraft:ignite")]
    Ignite,
    #[frog(key = "minecraft:play_sound")]
    PlaySound,
    #[frog(key = "minecraft:replace_block")]
    ReplaceBlock,
    #[frog(key = "minecraft:replace_disk")]
    ReplaceDisk,
    #[frog(key = "minecraft:run_function")]
    RunFunction,
    #[frog(key = "minecraft:set_block_properties")]
    SetBlockProperties,
    #[frog(key = "minecraft:spawn_particles")]
    SpawnParticles,
    #[frog(key = "minecraft:summon_entity")]
    SummonEntity,
}
