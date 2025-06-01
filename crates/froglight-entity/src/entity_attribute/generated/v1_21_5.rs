//! This file is generated, do not modify it manually.
//!
//! TODO: Documentation
#![allow(missing_docs)]

#[allow(clippy::wildcard_imports)]
use super::attribute::*;

froglight_macros::entity_attribute_properties! {
    path = crate,
    version = froglight_common::version::V1_21_5,
    ArmorAttribute => { properties: { ident: "minecraft:armor", key: "minecraft.attribute.name.armor", default: 0f64, range: 0f64..=30f64 } },
    ArmorToughnessAttribute => { properties: { ident: "minecraft:armor_toughness", key: "minecraft.attribute.name.armor_toughness", default: 0f64, range: 0f64..=20f64 } },
    AttackDamageAttribute => { properties: { ident: "minecraft:attack_damage", key: "minecraft.attribute.name.attack_damage", default: 2f64, range: 0f64..=2048f64 } },
    AttackKnockbackAttribute => { properties: { ident: "minecraft:attack_knockback", key: "minecraft.attribute.name.attack_knockback", default: 0f64, range: 0f64..=5f64 } },
    AttackSpeedAttribute => { properties: { ident: "minecraft:attack_speed", key: "minecraft.attribute.name.attack_speed", default: 4f64, range: 0f64..=1024f64 } },
    BlockBreakSpeedAttribute => { properties: { ident: "minecraft:block_break_speed", key: "minecraft.attribute.name.block_break_speed", default: 1f64, range: 0f64..=1024f64 } },
    BlockInteractionRangeAttribute => { properties: { ident: "minecraft:block_interaction_range", key: "minecraft.attribute.name.block_interaction_range", default: 4.5f64, range: 0f64..=64f64 } },
    BurningTimeAttribute => { properties: { ident: "minecraft:burning_time", key: "minecraft.attribute.name.burning_time", default: 1f64, range: 0f64..=1024f64 } },
    ExplosionKnockbackResistanceAttribute => { properties: { ident: "minecraft:explosion_knockback_resistance", key: "minecraft.attribute.name.explosion_knockback_resistance", default: 0f64, range: 0f64..=1f64 } },
    EntityInteractionRangeAttribute => { properties: { ident: "minecraft:entity_interaction_range", key: "minecraft.attribute.name.entity_interaction_range", default: 3f64, range: 0f64..=64f64 } },
    FallDamageMultiplierAttribute => { properties: { ident: "minecraft:fall_damage_multiplier", key: "minecraft.attribute.name.fall_damage_multiplier", default: 1f64, range: 0f64..=100f64 } },
    FlyingSpeedAttribute => { properties: { ident: "minecraft:flying_speed", key: "minecraft.attribute.name.flying_speed", default: 0.4f64, range: 0f64..=1024f64 } },
    FollowRangeAttribute => { properties: { ident: "minecraft:follow_range", key: "minecraft.attribute.name.follow_range", default: 32f64, range: 0f64..=2048f64 } },
    GravityAttribute => { properties: { ident: "minecraft:gravity", key: "minecraft.attribute.name.gravity", default: 0.08f64, range: -1f64..=1f64 } },
    JumpStrengthAttribute => { properties: { ident: "minecraft:jump_strength", key: "minecraft.attribute.name.jump_strength", default: 0.42f64, range: 0f64..=32f64 } },
    KnockbackResistanceAttribute => { properties: { ident: "minecraft:knockback_resistance", key: "minecraft.attribute.name.knockback_resistance", default: 0f64, range: 0f64..=1f64 } },
    LuckAttribute => { properties: { ident: "minecraft:luck", key: "minecraft.attribute.name.luck", default: 0f64, range: -1024f64..=1024f64 } },
    MaxAbsorptionAttribute => { properties: { ident: "minecraft:max_absorption", key: "minecraft.attribute.name.max_absorption", default: 0f64, range: 0f64..=2048f64 } },
    MaxHealthAttribute => { properties: { ident: "minecraft:max_health", key: "minecraft.attribute.name.max_health", default: 20f64, range: 1f64..=1024f64 } },
    MiningEfficiencyAttribute => { properties: { ident: "minecraft:mining_efficiency", key: "minecraft.attribute.name.mining_efficiency", default: 0f64, range: 0f64..=1024f64 } },
    MovementEfficiencyAttribute => { properties: { ident: "minecraft:movement_efficiency", key: "minecraft.attribute.name.movement_efficiency", default: 0f64, range: 0f64..=1f64 } },
    MovementSpeedAttribute => { properties: { ident: "minecraft:movement_speed", key: "minecraft.attribute.name.movement_speed", default: 0.7f64, range: 0f64..=1024f64 } },
    OxygenBonusAttribute => { properties: { ident: "minecraft:oxygen_bonus", key: "minecraft.attribute.name.oxygen_bonus", default: 0f64, range: 0f64..=1024f64 } },
    SafeFallDistanceAttribute => { properties: { ident: "minecraft:safe_fall_distance", key: "minecraft.attribute.name.safe_fall_distance", default: 3f64, range: -1024f64..=1024f64 } },
    ScaleAttribute => { properties: { ident: "minecraft:scale", key: "minecraft.attribute.name.scale", default: 1f64, range: 0.0625f64..=16f64 } },
    SneakingSpeedAttribute => { properties: { ident: "minecraft:sneaking_speed", key: "minecraft.attribute.name.sneaking_speed", default: 0.3f64, range: 0f64..=1f64 } },
    SpawnReinforcementsAttribute => { properties: { ident: "minecraft:spawn_reinforcements", key: "minecraft.attribute.name.spawn_reinforcements", default: 0f64, range: 0f64..=1f64 } },
    StepHeightAttribute => { properties: { ident: "minecraft:step_height", key: "minecraft.attribute.name.step_height", default: 0.6f64, range: 0f64..=10f64 } },
    SubmergedMiningSpeedAttribute => { properties: { ident: "minecraft:submerged_mining_speed", key: "minecraft.attribute.name.submerged_mining_speed", default: 0.2f64, range: 0f64..=20f64 } },
    SweepingDamageRatioAttribute => { properties: { ident: "minecraft:sweeping_damage_ratio", key: "minecraft.attribute.name.sweeping_damage_ratio", default: 0f64, range: 0f64..=1f64 } },
    TemptRangeAttribute => { properties: { ident: "minecraft:tempt_range", key: "minecraft.attribute.name.tempt_range", default: 10f64, range: 0f64..=2048f64 } },
    WaterMovementEfficiencyAttribute => { properties: { ident: "minecraft:water_movement_efficiency", key: "minecraft.attribute.name.water_movement_efficiency", default: 0f64, range: 0f64..=1f64 } },
}
