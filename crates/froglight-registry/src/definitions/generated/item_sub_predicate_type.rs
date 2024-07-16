//! @generated by `froglight-generator` #3f83759

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum ItemSubPredicateTypeKey {
    #[frog(key = "minecraft:damage")]
    Damage,
    #[frog(key = "minecraft:enchantments")]
    Enchantments,
    #[frog(key = "minecraft:stored_enchantments")]
    StoredEnchantments,
    #[frog(key = "minecraft:potion_contents")]
    PotionContents,
    #[frog(key = "minecraft:custom_data")]
    CustomData,
    #[frog(key = "minecraft:container")]
    Container,
    #[frog(key = "minecraft:bundle_contents")]
    BundleContents,
    #[frog(key = "minecraft:firework_explosion")]
    FireworkExplosion,
    #[frog(key = "minecraft:fireworks")]
    Fireworks,
    #[frog(key = "minecraft:writable_book_content")]
    WritableBookContent,
    #[frog(key = "minecraft:written_book_content")]
    WrittenBookContent,
    #[frog(key = "minecraft:attribute_modifiers")]
    AttributeModifiers,
    #[frog(key = "minecraft:trim")]
    Trim,
    #[frog(key = "minecraft:jukebox_playable")]
    JukeboxPlayable,
}
