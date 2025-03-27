//! This file is generated, do not modify it manually.
//!
//! TODO: Documentation
#![allow(missing_docs)]

froglight_macros::registry_values! {
    path = crate,
    feature = "v1_21_5",
    version = froglight_common::version::V1_21_5,
    inherit = super::v1_21_4,
    Attribute => {
        struct DimensionType { ambient_light: f32, bed_works: bool, coordinate_scale: f32, effects: String, fixed_time: i32, has_ceiling: bool, has_raids: bool, has_skylight: bool, height: i32, infiniburn: String, logical_height: i32, min_y: i32, monster_spawn_block_light_limit: i32, monster_spawn_light_level: MonsterSpawnLightLevel, natural: bool, piglin_safe: bool, respawn_anchor_works: bool, ultrawarm: bool }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ChatType { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TrimPattern { asset_id: String, decal: bool, description: TrimPatternDescription }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TrimMaterial { asset_name: String, description: TrimMaterialDescription, override_armor_assets: MaterialOverrideArmorAssets }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct WolfSoundVariant { ambient_sound: String, death_sound: String, growl_sound: String, hurt_sound: String, pant_sound: String, whine_sound: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PaintingVariant { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct DamageType { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BannerPattern { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct RaidEnchantmentProvider { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantmentProvider { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct JukeboxSong { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct Instrument { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TestEnvironment { type: String, definitions: Vec<Null> }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct TestInstance { type: String, environment: String, function: String, max_ticks: i32, required: bool, setup_ticks: i32, structure: String }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct MineableTagBlock { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BlockTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantableTagItem { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ItemTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct HasStructureTagWorldgenBiome { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BiomeTagWorldgen { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct StructureTagWorldgen { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct FlatLevelGeneratorPresetTagWorldgen { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct WorldPresetTagWorldgen { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PatternItemTagBannerPattern { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct BannerPatternTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct DamageTypeTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EntityTypeTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct FluidTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct GameEventTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct InstrumentTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PaintingVariantTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct PointOfInterestTypeTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct ExclusiveSetTagEnchantment { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
        struct EnchantmentTag { Inherited }
            => "minecraft:foo": todo!(),
            => "minecraft:bar": todo!(),
    },
    GeneratedType => {
        struct SpawnLightLevelItem { type: String, max_inclusive: i32, min_inclusive: i32 },
        enum MonsterSpawnLightLevel { Integer(i32), Item(SpawnLightLevelItem) },
        struct TrimPatternDescription { Inherited },
        struct TrimMaterialDescription { Inherited },
        struct MaterialOverrideArmorAssets { Inherited },
    }
}