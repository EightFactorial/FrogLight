//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum DataComponentTypeKey {
    #[frog(key = "minecraft:custom_data")]
    CustomData,
    #[frog(key = "minecraft:max_stack_size")]
    MaxStackSize,
    #[frog(key = "minecraft:max_damage")]
    MaxDamage,
    #[frog(key = "minecraft:damage")]
    Damage,
    #[frog(key = "minecraft:unbreakable")]
    Unbreakable,
    #[frog(key = "minecraft:custom_name")]
    CustomName,
    #[frog(key = "minecraft:item_name")]
    ItemName,
    #[frog(key = "minecraft:lore")]
    Lore,
    #[frog(key = "minecraft:rarity")]
    Rarity,
    #[frog(key = "minecraft:enchantments")]
    Enchantments,
    #[frog(key = "minecraft:can_place_on")]
    CanPlaceOn,
    #[frog(key = "minecraft:can_break")]
    CanBreak,
    #[frog(key = "minecraft:attribute_modifiers")]
    AttributeModifiers,
    #[frog(key = "minecraft:custom_model_data")]
    CustomModelData,
    #[frog(key = "minecraft:hide_additional_tooltip")]
    HideAdditionalTooltip,
    #[frog(key = "minecraft:hide_tooltip")]
    HideTooltip,
    #[frog(key = "minecraft:repair_cost")]
    RepairCost,
    #[frog(key = "minecraft:creative_slot_lock")]
    CreativeSlotLock,
    #[frog(key = "minecraft:enchantment_glint_override")]
    EnchantmentGlintOverride,
    #[frog(key = "minecraft:intangible_projectile")]
    IntangibleProjectile,
    #[frog(key = "minecraft:food")]
    Food,
    #[frog(key = "minecraft:fire_resistant")]
    FireResistant,
    #[frog(key = "minecraft:tool")]
    Tool,
    #[frog(key = "minecraft:stored_enchantments")]
    StoredEnchantments,
    #[frog(key = "minecraft:dyed_color")]
    DyedColor,
    #[frog(key = "minecraft:map_color")]
    MapColor,
    #[frog(key = "minecraft:map_id")]
    MapId,
    #[frog(key = "minecraft:map_decorations")]
    MapDecorations,
    #[frog(key = "minecraft:map_post_processing")]
    MapPostProcessing,
    #[frog(key = "minecraft:charged_projectiles")]
    ChargedProjectiles,
    #[frog(key = "minecraft:bundle_contents")]
    BundleContents,
    #[frog(key = "minecraft:potion_contents")]
    PotionContents,
    #[frog(key = "minecraft:suspicious_stew_effects")]
    SuspiciousStewEffects,
    #[frog(key = "minecraft:writable_book_content")]
    WritableBookContent,
    #[frog(key = "minecraft:written_book_content")]
    WrittenBookContent,
    #[frog(key = "minecraft:trim")]
    Trim,
    #[frog(key = "minecraft:debug_stick_state")]
    DebugStickState,
    #[frog(key = "minecraft:entity_data")]
    EntityData,
    #[frog(key = "minecraft:bucket_entity_data")]
    BucketEntityData,
    #[frog(key = "minecraft:block_entity_data")]
    BlockEntityData,
    #[frog(key = "minecraft:instrument")]
    Instrument,
    #[frog(key = "minecraft:ominous_bottle_amplifier")]
    OminousBottleAmplifier,
    #[frog(key = "minecraft:jukebox_playable")]
    JukeboxPlayable,
    #[frog(key = "minecraft:recipes")]
    Recipes,
    #[frog(key = "minecraft:lodestone_tracker")]
    LodestoneTracker,
    #[frog(key = "minecraft:firework_explosion")]
    FireworkExplosion,
    #[frog(key = "minecraft:fireworks")]
    Fireworks,
    #[frog(key = "minecraft:profile")]
    Profile,
    #[frog(key = "minecraft:note_block_sound")]
    NoteBlockSound,
    #[frog(key = "minecraft:banner_patterns")]
    BannerPatterns,
    #[frog(key = "minecraft:base_color")]
    BaseColor,
    #[frog(key = "minecraft:pot_decorations")]
    PotDecorations,
    #[frog(key = "minecraft:container")]
    Container,
    #[frog(key = "minecraft:block_state")]
    BlockState,
    #[frog(key = "minecraft:bees")]
    Bees,
    #[frog(key = "minecraft:lock")]
    Lock,
    #[frog(key = "minecraft:container_loot")]
    ContainerLoot,
}
