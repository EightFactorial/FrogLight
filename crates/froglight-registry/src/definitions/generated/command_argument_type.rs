//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum CommandArgumentTypeKey {
    #[frog(key = "brigadier:bool")]
    BrigadierBool,
    #[frog(key = "brigadier:float")]
    BrigadierFloat,
    #[frog(key = "brigadier:double")]
    BrigadierDouble,
    #[frog(key = "brigadier:integer")]
    BrigadierInteger,
    #[frog(key = "brigadier:long")]
    BrigadierLong,
    #[frog(key = "brigadier:string")]
    BrigadierString,
    #[frog(key = "minecraft:entity")]
    Entity,
    #[frog(key = "minecraft:game_profile")]
    GameProfile,
    #[frog(key = "minecraft:block_pos")]
    BlockPos,
    #[frog(key = "minecraft:column_pos")]
    ColumnPos,
    #[frog(key = "minecraft:vec3")]
    Vec3,
    #[frog(key = "minecraft:vec2")]
    Vec2,
    #[frog(key = "minecraft:block_state")]
    BlockState,
    #[frog(key = "minecraft:block_predicate")]
    BlockPredicate,
    #[frog(key = "minecraft:item_stack")]
    ItemStack,
    #[frog(key = "minecraft:item_predicate")]
    ItemPredicate,
    #[frog(key = "minecraft:color")]
    Color,
    #[frog(key = "minecraft:component")]
    Component,
    #[frog(key = "minecraft:style")]
    Style,
    #[frog(key = "minecraft:message")]
    Message,
    #[frog(key = "minecraft:nbt_compound_tag")]
    NbtCompoundTag,
    #[frog(key = "minecraft:nbt_tag")]
    NbtTag,
    #[frog(key = "minecraft:nbt_path")]
    NbtPath,
    #[frog(key = "minecraft:objective")]
    Objective,
    #[frog(key = "minecraft:objective_criteria")]
    ObjectiveCriteria,
    #[frog(key = "minecraft:operation")]
    Operation,
    #[frog(key = "minecraft:particle")]
    Particle,
    #[frog(key = "minecraft:angle")]
    Angle,
    #[frog(key = "minecraft:rotation")]
    Rotation,
    #[frog(key = "minecraft:scoreboard_slot")]
    ScoreboardSlot,
    #[frog(key = "minecraft:score_holder")]
    ScoreHolder,
    #[frog(key = "minecraft:swizzle")]
    Swizzle,
    #[frog(key = "minecraft:team")]
    Team,
    #[frog(key = "minecraft:item_slot")]
    ItemSlot,
    #[frog(key = "minecraft:item_slots")]
    ItemSlots,
    #[frog(key = "minecraft:resource_location")]
    ResourceLocation,
    #[frog(key = "minecraft:function")]
    Function,
    #[frog(key = "minecraft:entity_anchor")]
    EntityAnchor,
    #[frog(key = "minecraft:int_range")]
    IntRange,
    #[frog(key = "minecraft:float_range")]
    FloatRange,
    #[frog(key = "minecraft:dimension")]
    Dimension,
    #[frog(key = "minecraft:gamemode")]
    Gamemode,
    #[frog(key = "minecraft:time")]
    Time,
    #[frog(key = "minecraft:resource_or_tag")]
    ResourceOrTag,
    #[frog(key = "minecraft:resource_or_tag_key")]
    ResourceOrTagKey,
    #[frog(key = "minecraft:resource")]
    Resource,
    #[frog(key = "minecraft:resource_key")]
    ResourceKey,
    #[frog(key = "minecraft:template_mirror")]
    TemplateMirror,
    #[frog(key = "minecraft:template_rotation")]
    TemplateRotation,
    #[frog(key = "minecraft:heightmap")]
    Heightmap,
    #[frog(key = "minecraft:loot_table")]
    LootTable,
    #[frog(key = "minecraft:loot_predicate")]
    LootPredicate,
    #[frog(key = "minecraft:loot_modifier")]
    LootModifier,
    #[frog(key = "minecraft:uuid")]
    Uuid,
}
