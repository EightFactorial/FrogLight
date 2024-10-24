//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum LootConditionTypeKey {
    #[frog(key = "minecraft:inverted")]
    Inverted,
    #[frog(key = "minecraft:any_of")]
    AnyOf,
    #[frog(key = "minecraft:all_of")]
    AllOf,
    #[frog(key = "minecraft:random_chance")]
    RandomChance,
    #[frog(key = "minecraft:random_chance_with_enchanted_bonus")]
    RandomChanceWithEnchantedBonus,
    #[frog(key = "minecraft:entity_properties")]
    EntityProperties,
    #[frog(key = "minecraft:killed_by_player")]
    KilledByPlayer,
    #[frog(key = "minecraft:entity_scores")]
    EntityScores,
    #[frog(key = "minecraft:block_state_property")]
    BlockStateProperty,
    #[frog(key = "minecraft:match_tool")]
    MatchTool,
    #[frog(key = "minecraft:table_bonus")]
    TableBonus,
    #[frog(key = "minecraft:survives_explosion")]
    SurvivesExplosion,
    #[frog(key = "minecraft:damage_source_properties")]
    DamageSourceProperties,
    #[frog(key = "minecraft:location_check")]
    LocationCheck,
    #[frog(key = "minecraft:weather_check")]
    WeatherCheck,
    #[frog(key = "minecraft:reference")]
    Reference,
    #[frog(key = "minecraft:time_check")]
    TimeCheck,
    #[frog(key = "minecraft:value_check")]
    ValueCheck,
    #[frog(key = "minecraft:enchantment_active_check")]
    EnchantmentActiveCheck,
}
