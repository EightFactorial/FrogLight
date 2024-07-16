//! @generated by `froglight-generator` #3f83759

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum LootFunctionTypeKey {
    #[frog(key = "minecraft:set_count")]
    SetCount,
    #[frog(key = "minecraft:set_item")]
    SetItem,
    #[frog(key = "minecraft:enchant_with_levels")]
    EnchantWithLevels,
    #[frog(key = "minecraft:enchant_randomly")]
    EnchantRandomly,
    #[frog(key = "minecraft:set_enchantments")]
    SetEnchantments,
    #[frog(key = "minecraft:set_custom_data")]
    SetCustomData,
    #[frog(key = "minecraft:set_components")]
    SetComponents,
    #[frog(key = "minecraft:furnace_smelt")]
    FurnaceSmelt,
    #[frog(key = "minecraft:enchanted_count_increase")]
    EnchantedCountIncrease,
    #[frog(key = "minecraft:set_damage")]
    SetDamage,
    #[frog(key = "minecraft:set_attributes")]
    SetAttributes,
    #[frog(key = "minecraft:set_name")]
    SetName,
    #[frog(key = "minecraft:exploration_map")]
    ExplorationMap,
    #[frog(key = "minecraft:set_stew_effect")]
    SetStewEffect,
    #[frog(key = "minecraft:copy_name")]
    CopyName,
    #[frog(key = "minecraft:set_contents")]
    SetContents,
    #[frog(key = "minecraft:modify_contents")]
    ModifyContents,
    #[frog(key = "minecraft:filtered")]
    Filtered,
    #[frog(key = "minecraft:limit_count")]
    LimitCount,
    #[frog(key = "minecraft:apply_bonus")]
    ApplyBonus,
    #[frog(key = "minecraft:set_loot_table")]
    SetLootTable,
    #[frog(key = "minecraft:explosion_decay")]
    ExplosionDecay,
    #[frog(key = "minecraft:set_lore")]
    SetLore,
    #[frog(key = "minecraft:fill_player_head")]
    FillPlayerHead,
    #[frog(key = "minecraft:copy_custom_data")]
    CopyCustomData,
    #[frog(key = "minecraft:copy_state")]
    CopyState,
    #[frog(key = "minecraft:set_banner_pattern")]
    SetBannerPattern,
    #[frog(key = "minecraft:set_potion")]
    SetPotion,
    #[frog(key = "minecraft:set_instrument")]
    SetInstrument,
    #[frog(key = "minecraft:reference")]
    Reference,
    #[frog(key = "minecraft:sequence")]
    Sequence,
    #[frog(key = "minecraft:copy_components")]
    CopyComponents,
    #[frog(key = "minecraft:set_fireworks")]
    SetFireworks,
    #[frog(key = "minecraft:set_firework_explosion")]
    SetFireworkExplosion,
    #[frog(key = "minecraft:set_book_cover")]
    SetBookCover,
    #[frog(key = "minecraft:set_written_book_pages")]
    SetWrittenBookPages,
    #[frog(key = "minecraft:set_writable_book_pages")]
    SetWritableBookPages,
    #[frog(key = "minecraft:toggle_tooltips")]
    ToggleTooltips,
    #[frog(key = "minecraft:set_ominous_bottle_amplifier")]
    SetOminousBottleAmplifier,
    #[frog(key = "minecraft:set_custom_model_data")]
    SetCustomModelData,
}
