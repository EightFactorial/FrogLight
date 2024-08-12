//! @generated by `froglight-generator` #b0e1aa4

use froglight_macros::FrogRegistry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum EntityTypeKey {
    #[frog(key = "minecraft:allay")]
    Allay,
    #[frog(key = "minecraft:area_effect_cloud")]
    AreaEffectCloud,
    #[frog(key = "minecraft:armadillo")]
    Armadillo,
    #[frog(key = "minecraft:armor_stand")]
    ArmorStand,
    #[frog(key = "minecraft:arrow")]
    Arrow,
    #[frog(key = "minecraft:axolotl")]
    Axolotl,
    #[frog(key = "minecraft:bat")]
    Bat,
    #[frog(key = "minecraft:bee")]
    Bee,
    #[frog(key = "minecraft:blaze")]
    Blaze,
    #[frog(key = "minecraft:block_display")]
    BlockDisplay,
    #[frog(key = "minecraft:boat")]
    Boat,
    #[frog(key = "minecraft:bogged")]
    Bogged,
    #[frog(key = "minecraft:breeze")]
    Breeze,
    #[frog(key = "minecraft:breeze_wind_charge")]
    BreezeWindCharge,
    #[frog(key = "minecraft:camel")]
    Camel,
    #[frog(key = "minecraft:cat")]
    Cat,
    #[frog(key = "minecraft:cave_spider")]
    CaveSpider,
    #[frog(key = "minecraft:chest_boat")]
    ChestBoat,
    #[frog(key = "minecraft:chest_minecart")]
    ChestMinecart,
    #[frog(key = "minecraft:chicken")]
    Chicken,
    #[frog(key = "minecraft:cod")]
    Cod,
    #[frog(key = "minecraft:command_block_minecart")]
    CommandBlockMinecart,
    #[frog(key = "minecraft:cow")]
    Cow,
    #[frog(key = "minecraft:creeper")]
    Creeper,
    #[frog(key = "minecraft:dolphin")]
    Dolphin,
    #[frog(key = "minecraft:donkey")]
    Donkey,
    #[frog(key = "minecraft:dragon_fireball")]
    DragonFireball,
    #[frog(key = "minecraft:drowned")]
    Drowned,
    #[frog(key = "minecraft:egg")]
    Egg,
    #[frog(key = "minecraft:elder_guardian")]
    ElderGuardian,
    #[frog(key = "minecraft:end_crystal")]
    EndCrystal,
    #[frog(key = "minecraft:ender_dragon")]
    EnderDragon,
    #[frog(key = "minecraft:ender_pearl")]
    EnderPearl,
    #[frog(key = "minecraft:enderman")]
    Enderman,
    #[frog(key = "minecraft:endermite")]
    Endermite,
    #[frog(key = "minecraft:evoker")]
    Evoker,
    #[frog(key = "minecraft:evoker_fangs")]
    EvokerFangs,
    #[frog(key = "minecraft:experience_bottle")]
    ExperienceBottle,
    #[frog(key = "minecraft:experience_orb")]
    ExperienceOrb,
    #[frog(key = "minecraft:eye_of_ender")]
    EyeOfEnder,
    #[frog(key = "minecraft:falling_block")]
    FallingBlock,
    #[frog(key = "minecraft:firework_rocket")]
    FireworkRocket,
    #[frog(key = "minecraft:fox")]
    Fox,
    #[frog(key = "minecraft:frog")]
    Frog,
    #[frog(key = "minecraft:furnace_minecart")]
    FurnaceMinecart,
    #[frog(key = "minecraft:ghast")]
    Ghast,
    #[frog(key = "minecraft:giant")]
    Giant,
    #[frog(key = "minecraft:glow_item_frame")]
    GlowItemFrame,
    #[frog(key = "minecraft:glow_squid")]
    GlowSquid,
    #[frog(key = "minecraft:goat")]
    Goat,
    #[frog(key = "minecraft:guardian")]
    Guardian,
    #[frog(key = "minecraft:hoglin")]
    Hoglin,
    #[frog(key = "minecraft:hopper_minecart")]
    HopperMinecart,
    #[frog(key = "minecraft:horse")]
    Horse,
    #[frog(key = "minecraft:husk")]
    Husk,
    #[frog(key = "minecraft:illusioner")]
    Illusioner,
    #[frog(key = "minecraft:interaction")]
    Interaction,
    #[frog(key = "minecraft:iron_golem")]
    IronGolem,
    #[frog(key = "minecraft:item")]
    Item,
    #[frog(key = "minecraft:item_display")]
    ItemDisplay,
    #[frog(key = "minecraft:item_frame")]
    ItemFrame,
    #[frog(key = "minecraft:ominous_item_spawner")]
    OminousItemSpawner,
    #[frog(key = "minecraft:fireball")]
    Fireball,
    #[frog(key = "minecraft:leash_knot")]
    LeashKnot,
    #[frog(key = "minecraft:lightning_bolt")]
    LightningBolt,
    #[frog(key = "minecraft:llama")]
    Llama,
    #[frog(key = "minecraft:llama_spit")]
    LlamaSpit,
    #[frog(key = "minecraft:magma_cube")]
    MagmaCube,
    #[frog(key = "minecraft:marker")]
    Marker,
    #[frog(key = "minecraft:minecart")]
    Minecart,
    #[frog(key = "minecraft:mooshroom")]
    Mooshroom,
    #[frog(key = "minecraft:mule")]
    Mule,
    #[frog(key = "minecraft:ocelot")]
    Ocelot,
    #[frog(key = "minecraft:painting")]
    Painting,
    #[frog(key = "minecraft:panda")]
    Panda,
    #[frog(key = "minecraft:parrot")]
    Parrot,
    #[frog(key = "minecraft:phantom")]
    Phantom,
    #[frog(key = "minecraft:pig")]
    #[default]
    Pig,
    #[frog(key = "minecraft:piglin")]
    Piglin,
    #[frog(key = "minecraft:piglin_brute")]
    PiglinBrute,
    #[frog(key = "minecraft:pillager")]
    Pillager,
    #[frog(key = "minecraft:polar_bear")]
    PolarBear,
    #[frog(key = "minecraft:potion")]
    Potion,
    #[frog(key = "minecraft:pufferfish")]
    Pufferfish,
    #[frog(key = "minecraft:rabbit")]
    Rabbit,
    #[frog(key = "minecraft:ravager")]
    Ravager,
    #[frog(key = "minecraft:salmon")]
    Salmon,
    #[frog(key = "minecraft:sheep")]
    Sheep,
    #[frog(key = "minecraft:shulker")]
    Shulker,
    #[frog(key = "minecraft:shulker_bullet")]
    ShulkerBullet,
    #[frog(key = "minecraft:silverfish")]
    Silverfish,
    #[frog(key = "minecraft:skeleton")]
    Skeleton,
    #[frog(key = "minecraft:skeleton_horse")]
    SkeletonHorse,
    #[frog(key = "minecraft:slime")]
    Slime,
    #[frog(key = "minecraft:small_fireball")]
    SmallFireball,
    #[frog(key = "minecraft:sniffer")]
    Sniffer,
    #[frog(key = "minecraft:snow_golem")]
    SnowGolem,
    #[frog(key = "minecraft:snowball")]
    Snowball,
    #[frog(key = "minecraft:spawner_minecart")]
    SpawnerMinecart,
    #[frog(key = "minecraft:spectral_arrow")]
    SpectralArrow,
    #[frog(key = "minecraft:spider")]
    Spider,
    #[frog(key = "minecraft:squid")]
    Squid,
    #[frog(key = "minecraft:stray")]
    Stray,
    #[frog(key = "minecraft:strider")]
    Strider,
    #[frog(key = "minecraft:tadpole")]
    Tadpole,
    #[frog(key = "minecraft:text_display")]
    TextDisplay,
    #[frog(key = "minecraft:tnt")]
    Tnt,
    #[frog(key = "minecraft:tnt_minecart")]
    TntMinecart,
    #[frog(key = "minecraft:trader_llama")]
    TraderLlama,
    #[frog(key = "minecraft:trident")]
    Trident,
    #[frog(key = "minecraft:tropical_fish")]
    TropicalFish,
    #[frog(key = "minecraft:turtle")]
    Turtle,
    #[frog(key = "minecraft:vex")]
    Vex,
    #[frog(key = "minecraft:villager")]
    Villager,
    #[frog(key = "minecraft:vindicator")]
    Vindicator,
    #[frog(key = "minecraft:wandering_trader")]
    WanderingTrader,
    #[frog(key = "minecraft:warden")]
    Warden,
    #[frog(key = "minecraft:wind_charge")]
    WindCharge,
    #[frog(key = "minecraft:witch")]
    Witch,
    #[frog(key = "minecraft:wither")]
    Wither,
    #[frog(key = "minecraft:wither_skeleton")]
    WitherSkeleton,
    #[frog(key = "minecraft:wither_skull")]
    WitherSkull,
    #[frog(key = "minecraft:wolf")]
    Wolf,
    #[frog(key = "minecraft:zoglin")]
    Zoglin,
    #[frog(key = "minecraft:zombie")]
    Zombie,
    #[frog(key = "minecraft:zombie_horse")]
    ZombieHorse,
    #[frog(key = "minecraft:zombie_villager")]
    ZombieVillager,
    #[frog(key = "minecraft:zombified_piglin")]
    ZombifiedPiglin,
    #[frog(key = "minecraft:player")]
    Player,
    #[frog(key = "minecraft:fishing_bobber")]
    FishingBobber,
}
