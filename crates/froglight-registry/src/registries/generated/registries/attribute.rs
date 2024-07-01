//! @generated by `froglight-generator` #8ccbfa2

use bevy_reflect::Reflect;
use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FrogRegistry)]
pub enum AttributeKey {
    #[frog(key = "minecraft:generic.armor")]
    GenericArmor,
    #[frog(key = "minecraft:generic.armor_toughness")]
    GenericArmorToughness,
    #[frog(key = "minecraft:generic.attack_damage")]
    GenericAttackDamage,
    #[frog(key = "minecraft:generic.attack_knockback")]
    GenericAttackKnockback,
    #[frog(key = "minecraft:generic.attack_speed")]
    GenericAttackSpeed,
    #[frog(key = "minecraft:player.block_break_speed")]
    PlayerBlockBreakSpeed,
    #[frog(key = "minecraft:player.block_interaction_range")]
    PlayerBlockInteractionRange,
    #[frog(key = "minecraft:generic.burning_time")]
    GenericBurningTime,
    #[frog(key = "minecraft:generic.explosion_knockback_resistance")]
    GenericExplosionKnockbackResistance,
    #[frog(key = "minecraft:player.entity_interaction_range")]
    PlayerEntityInteractionRange,
    #[frog(key = "minecraft:generic.fall_damage_multiplier")]
    GenericFallDamageMultiplier,
    #[frog(key = "minecraft:generic.flying_speed")]
    GenericFlyingSpeed,
    #[frog(key = "minecraft:generic.follow_range")]
    GenericFollowRange,
    #[frog(key = "minecraft:generic.gravity")]
    GenericGravity,
    #[frog(key = "minecraft:generic.jump_strength")]
    GenericJumpStrength,
    #[frog(key = "minecraft:generic.knockback_resistance")]
    GenericKnockbackResistance,
    #[frog(key = "minecraft:generic.luck")]
    GenericLuck,
    #[frog(key = "minecraft:generic.max_absorption")]
    GenericMaxAbsorption,
    #[frog(key = "minecraft:generic.max_health")]
    GenericMaxHealth,
    #[frog(key = "minecraft:player.mining_efficiency")]
    PlayerMiningEfficiency,
    #[frog(key = "minecraft:generic.movement_efficiency")]
    GenericMovementEfficiency,
    #[frog(key = "minecraft:generic.movement_speed")]
    GenericMovementSpeed,
    #[frog(key = "minecraft:generic.oxygen_bonus")]
    GenericOxygenBonus,
    #[frog(key = "minecraft:generic.safe_fall_distance")]
    GenericSafeFallDistance,
    #[frog(key = "minecraft:generic.scale")]
    GenericScale,
    #[frog(key = "minecraft:player.sneaking_speed")]
    PlayerSneakingSpeed,
    #[frog(key = "minecraft:zombie.spawn_reinforcements")]
    ZombieSpawnReinforcements,
    #[frog(key = "minecraft:generic.step_height")]
    GenericStepHeight,
    #[frog(key = "minecraft:player.submerged_mining_speed")]
    PlayerSubmergedMiningSpeed,
    #[frog(key = "minecraft:player.sweeping_damage_ratio")]
    PlayerSweepingDamageRatio,
    #[frog(key = "minecraft:generic.water_movement_efficiency")]
    GenericWaterMovementEfficiency,
}
