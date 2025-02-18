//! This file is generated, do not modify it manually.
//!
//! TODO: Documentation
#![allow(missing_docs)]

froglight_macros::registry_values! {
    path = crate,
    feature = "v1_21_4",
    version = froglight_common::version::V1_21_4,
    Attribute => {
        struct DimensionType { ambient_light: f32, bed_works: bool, coordinate_scale: f32, effects: String, fixed_time: i32, has_ceiling: bool, has_raids: bool, has_skylight: bool, height: i32, infiniburn: String, logical_height: i32, min_y: i32, monster_spawn_block_light_limit: i32, monster_spawn_light_level: MonsterSpawnLightLevel, natural: bool, piglin_safe: bool, respawn_anchor_works: bool, ultrawarm: bool }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ChatType { chat: ChatTypeChat, narration: ChatTypeNarration }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TrimPattern { asset_id: String, decal: bool, description: TrimPatternDescription, template_item: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TrimMaterial { asset_name: String, description: TrimMaterialDescription, ingredient: String, override_armor_assets: MaterialOverrideArmorAssets }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PaintingVariant { asset_id: String, author: PaintingVariantAuthor, height: i32, title: PaintingVariantTitle, width: i32 }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct DamageType { effects: String, exhaustion: f32, message_id: String, scaling: String, death_message_type: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BannerPattern { asset_id: String, translation_key: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct RaidEnchantmentProvider { type: String, enchantment: String, level: i32 }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantmentProvider { type: String, enchantment: String, level: i32, enchantments: String, max_cost_span: i32, min_cost: i32 }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct JukeboxSong { comparator_output: i32, description: JukeboxSongDescription, length_in_seconds: f32, sound_event: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct Instrument { description: InstrumentDescription, range: f32, sound_event: String, use_duration: f32 }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct MineableTagBlock { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BlockTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantableTagItem { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ItemTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct HasStructureTagWorldgenBiome { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BiomeTagWorldgen { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct StructureTagWorldgen { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct FlatLevelGeneratorPresetTagWorldgen { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct WorldPresetTagWorldgen { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PatternItemTagBannerPattern { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BannerPatternTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct CatVariantTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct DamageTypeTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EntityTypeTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct FluidTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct GameEventTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct InstrumentTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PaintingVariantTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PointOfInterestTypeTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ExclusiveSetTagEnchantment { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantmentTag { values: Vec<String> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
    },
    GeneratedType => {
        struct SpawnLightLevelItem { type: String, max_inclusive: i32, min_inclusive: i32 },
        enum MonsterSpawnLightLevel { Item(SpawnLightLevelItem), Integer(i32) },
        struct ChatTypeChatStyle { color: String, italic: bool },
        struct ChatTypeChat { parameters: Vec<String>, translation_key: String, style: ChatTypeChatStyle },
        struct ChatTypeNarration { parameters: Vec<String>, translation_key: String },
        struct TrimPatternDescription { translate: String },
        struct TrimMaterialDescription { color: String, translate: String },
        struct MaterialOverrideArmorAssets { minecraft_diamond: String, minecraft_gold: String, minecraft_iron: String, minecraft_netherite: String },
        struct PaintingVariantAuthor { color: String, translate: String },
        struct PaintingVariantTitle { color: String, translate: String },
        struct JukeboxSongDescription { translate: String },
        struct InstrumentDescription { translate: String },
    }
}