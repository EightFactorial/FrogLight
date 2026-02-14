//! Biome data for [`V26_1`](froglight_common::version::V26_1).
//! 
//! @generated
#![allow(clippy::unreadable_literal, reason = "Generated code")]

#[cfg(feature = "std")]
use std::sync::LazyLock;

use froglight_common::version::V26_1;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
use once_cell::sync::OnceCell as LazyLock;

#[allow(clippy::wildcard_imports, reason = "Generated code")]
use crate::generated::{attribute::*, biome::*};

generate! {
    @version V26_1,
    TheVoid => { ident: "minecraft:the_void", global: 0, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { VisualSkyColor: "#7ba4ff" }
    },
    Plains => { ident: "minecraft:plains", global: 1, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.8f32, downfall: 0.4f32 },
        attr: { VisualSkyColor: "#78a7ff" }
    },
    SunflowerPlains => { ident: "minecraft:sunflower_plains", global: 2, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.8f32, downfall: 0.4f32 },
        attr: { VisualSkyColor: "#78a7ff" }
    },
    SnowyPlains => { ident: "minecraft:snowy_plains", global: 3, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0f32, downfall: 0.5f32 },
        attr: { VisualSkyColor: "#7fa1ff" }
    },
    IceSpikes => { ident: "minecraft:ice_spikes", global: 4, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0f32, downfall: 0.5f32 },
        attr: { VisualSkyColor: "#7fa1ff" }
    },
    Desert => { ident: "minecraft:desert", global: 5, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.desert" } }, GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    Swamp => { ident: "minecraft:swamp", global: 6, prop: { foliage: 6975545, dry_foliage: 8082228, grass: 0, water: 6388580, precip: true, temp: 0.8f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.swamp" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#78a7ff", VisualWaterFogColor: "#232317", VisualWaterFogEndDistance: { "argument": 0.85, "modifier": "multiply"} }
    },
    MangroveSwamp => { ident: "minecraft:mangrove_swamp", global: 7, prop: { foliage: 9285927, dry_foliage: 8082228, grass: 0, water: 3832426, precip: true, temp: 0.8f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.swamp" } }, GameplayIncreasedFireBurnout: true, VisualFogColor: "#c0d8ff", VisualSkyColor: "#78a7ff", VisualWaterFogColor: "#4d7a60", VisualWaterFogEndDistance: { "argument": 0.85, "modifier": "multiply"} }
    },
    Forest => { ident: "minecraft:forest", global: 8, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.7f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.forest" } }, VisualSkyColor: "#79a6ff" }
    },
    FlowerForest => { ident: "minecraft:flower_forest", global: 9, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.7f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.flower_forest" } }, VisualSkyColor: "#79a6ff" }
    },
    BirchForest => { ident: "minecraft:birch_forest", global: 10, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.6f32, downfall: 0.6f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.forest" } }, VisualSkyColor: "#7aa5ff" }
    },
    DarkForest => { ident: "minecraft:dark_forest", global: 11, prop: { foliage: 0, dry_foliage: 8082228, grass: 0, water: 4159204, precip: true, temp: 0.7f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.forest" } }, VisualSkyColor: "#79a6ff" }
    },
    PaleGarden => { ident: "minecraft:pale_garden", global: 12, prop: { foliage: 8883574, dry_foliage: 10528412, grass: 7832178, water: 7768221, precip: true, temp: 0.7f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: {}, AudioMusicVolume: 0, VisualFogColor: "#817770", VisualSkyColor: "#b9b9b9", VisualWaterFogColor: "#556980" }
    },
    OldGrowthBirchForest => { ident: "minecraft:old_growth_birch_forest", global: 13, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.6f32, downfall: 0.6f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.forest" } }, VisualSkyColor: "#7aa5ff" }
    },
    OldGrowthPineTaiga => { ident: "minecraft:old_growth_pine_taiga", global: 14, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.3f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.old_growth_taiga" } }, VisualSkyColor: "#7ca3ff" }
    },
    OldGrowthSpruceTaiga => { ident: "minecraft:old_growth_spruce_taiga", global: 15, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.25f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.old_growth_taiga" } }, VisualSkyColor: "#7da3ff" }
    },
    Taiga => { ident: "minecraft:taiga", global: 16, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.25f32, downfall: 0.8f32 },
        attr: { VisualSkyColor: "#7da3ff" }
    },
    SnowyTaiga => { ident: "minecraft:snowy_taiga", global: 17, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4020182, precip: true, temp: -0.5f32, downfall: 0.4f32 },
        attr: { VisualSkyColor: "#839eff" }
    },
    Savanna => { ident: "minecraft:savanna", global: 18, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    SavannaPlateau => { ident: "minecraft:savanna_plateau", global: 19, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    WindsweptHills => { ident: "minecraft:windswept_hills", global: 20, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.2f32, downfall: 0.3f32 },
        attr: { VisualSkyColor: "#7da2ff" }
    },
    WindsweptGravellyHills => { ident: "minecraft:windswept_gravelly_hills", global: 21, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.2f32, downfall: 0.3f32 },
        attr: { VisualSkyColor: "#7da2ff" }
    },
    WindsweptForest => { ident: "minecraft:windswept_forest", global: 22, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.2f32, downfall: 0.3f32 },
        attr: { VisualSkyColor: "#7da2ff" }
    },
    WindsweptSavanna => { ident: "minecraft:windswept_savanna", global: 23, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    Jungle => { ident: "minecraft:jungle", global: 24, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.95f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.jungle" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#77a8ff" }
    },
    SparseJungle => { ident: "minecraft:sparse_jungle", global: 25, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.95f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.sparse_jungle" } }, VisualSkyColor: "#77a8ff" }
    },
    BambooJungle => { ident: "minecraft:bamboo_jungle", global: 26, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.95f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.bamboo_jungle" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#77a8ff" }
    },
    Badlands => { ident: "minecraft:badlands", global: 27, prop: { foliage: 10387789, dry_foliage: 0, grass: 9470285, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.badlands" } }, GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    ErodedBadlands => { ident: "minecraft:eroded_badlands", global: 28, prop: { foliage: 10387789, dry_foliage: 0, grass: 9470285, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.badlands" } }, GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    WoodedBadlands => { ident: "minecraft:wooded_badlands", global: 29, prop: { foliage: 10387789, dry_foliage: 0, grass: 9470285, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.badlands" } }, GameplaySnowGolemMelts: true, VisualSkyColor: "#6eb1ff" }
    },
    Meadow => { ident: "minecraft:meadow", global: 30, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 937679, precip: true, temp: 0.5f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.meadow" } }, VisualSkyColor: "#7ba4ff" }
    },
    CherryGrove => { ident: "minecraft:cherry_grove", global: 31, prop: { foliage: 11983713, dry_foliage: 0, grass: 11983713, water: 6141935, precip: true, temp: 0.5f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.cherry_grove" } }, VisualSkyColor: "#7ba4ff", VisualWaterFogColor: "#5db7ef" }
    },
    Grove => { ident: "minecraft:grove", global: 32, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: -0.2f32, downfall: 0.8f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.grove" } }, VisualSkyColor: "#81a0ff" }
    },
    SnowySlopes => { ident: "minecraft:snowy_slopes", global: 33, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: -0.3f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.snowy_slopes" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#829fff" }
    },
    FrozenPeaks => { ident: "minecraft:frozen_peaks", global: 34, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: -0.7f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.frozen_peaks" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#859dff" }
    },
    JaggedPeaks => { ident: "minecraft:jagged_peaks", global: 35, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: -0.7f32, downfall: 0.9f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.jagged_peaks" } }, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#859dff" }
    },
    StonyPeaks => { ident: "minecraft:stony_peaks", global: 36, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 1f32, downfall: 0.3f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.stony_peaks" } }, VisualSkyColor: "#76a8ff" }
    },
    River => { ident: "minecraft:river", global: 37, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff" }
    },
    FrozenRiver => { ident: "minecraft:frozen_river", global: 38, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 3750089, precip: true, temp: 0f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7fa1ff" }
    },
    Beach => { ident: "minecraft:beach", global: 39, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.8f32, downfall: 0.4f32 },
        attr: { VisualSkyColor: "#78a7ff" }
    },
    SnowyBeach => { ident: "minecraft:snowy_beach", global: 40, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4020182, precip: true, temp: 0.05f32, downfall: 0.3f32 },
        attr: { VisualSkyColor: "#7fa1ff" }
    },
    StonyShore => { ident: "minecraft:stony_shore", global: 41, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.2f32, downfall: 0.3f32 },
        attr: { VisualSkyColor: "#7da2ff" }
    },
    WarmOcean => { ident: "minecraft:warm_ocean", global: 42, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4445678, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff", VisualWaterFogColor: "#041f33" }
    },
    LukewarmOcean => { ident: "minecraft:lukewarm_ocean", global: 43, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4566514, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff", VisualWaterFogColor: "#041633" }
    },
    DeepLukewarmOcean => { ident: "minecraft:deep_lukewarm_ocean", global: 44, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4566514, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff", VisualWaterFogColor: "#041633" }
    },
    Ocean => { ident: "minecraft:ocean", global: 45, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff" }
    },
    DeepOcean => { ident: "minecraft:deep_ocean", global: 46, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff" }
    },
    ColdOcean => { ident: "minecraft:cold_ocean", global: 47, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4020182, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff" }
    },
    DeepColdOcean => { ident: "minecraft:deep_cold_ocean", global: 48, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4020182, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "creative": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.creative" }, "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.game" }, "underwater": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.under_water" } }, VisualSkyColor: "#7ba4ff" }
    },
    FrozenOcean => { ident: "minecraft:frozen_ocean", global: 49, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 3750089, precip: true, temp: 0f32, downfall: 0.5f32 },
        attr: { VisualSkyColor: "#7fa1ff" }
    },
    DeepFrozenOcean => { ident: "minecraft:deep_frozen_ocean", global: 50, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 3750089, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { VisualSkyColor: "#7ba4ff" }
    },
    MushroomFields => { ident: "minecraft:mushroom_fields", global: 51, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.9f32, downfall: 1f32 },
        attr: { GameplayCanPillagerPatrolSpawn: false, GameplayIncreasedFireBurnout: true, VisualSkyColor: "#77a8ff" }
    },
    DripstoneCaves => { ident: "minecraft:dripstone_caves", global: 52, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.8f32, downfall: 0.4f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.dripstone_caves" } }, VisualSkyColor: "#78a7ff" }
    },
    LushCaves => { ident: "minecraft:lush_caves", global: 53, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.5f32, downfall: 0.5f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.lush_caves" } }, VisualSkyColor: "#7ba4ff" }
    },
    DeepDark => { ident: "minecraft:deep_dark", global: 54, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: true, temp: 0.8f32, downfall: 0.4f32 },
        attr: { AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.overworld.deep_dark" } }, VisualSkyColor: "#78a7ff" }
    },
    NetherWastes => { ident: "minecraft:nether_wastes", global: 55, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioAmbientSounds: { "additions": { "sound": "minecraft:ambient.nether_wastes.additions", "tick_chance": 0.0111 }, "loop": "minecraft:ambient.nether_wastes.loop", "mood": { "block_search_extent": 8, "offset": 2, "sound": "minecraft:ambient.nether_wastes.mood", "tick_delay": 6000 } }, AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.nether.nether_wastes" } }, VisualFogColor: "#330808" }
    },
    WarpedForest => { ident: "minecraft:warped_forest", global: 56, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioAmbientSounds: { "additions": { "sound": "minecraft:ambient.warped_forest.additions", "tick_chance": 0.0111 }, "loop": "minecraft:ambient.warped_forest.loop", "mood": { "block_search_extent": 8, "offset": 2, "sound": "minecraft:ambient.warped_forest.mood", "tick_delay": 6000 } }, AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.nether.warped_forest" } }, VisualAmbientParticles: [ { "particle": { "type": "minecraft:warped_spore" }, "probability": 0.01428 }], VisualFogColor: "#1a051a" }
    },
    CrimsonForest => { ident: "minecraft:crimson_forest", global: 57, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioAmbientSounds: { "additions": { "sound": "minecraft:ambient.crimson_forest.additions", "tick_chance": 0.0111 }, "loop": "minecraft:ambient.crimson_forest.loop", "mood": { "block_search_extent": 8, "offset": 2, "sound": "minecraft:ambient.crimson_forest.mood", "tick_delay": 6000 } }, AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.nether.crimson_forest" } }, VisualAmbientParticles: [ { "particle": { "type": "minecraft:crimson_spore" }, "probability": 0.025 }], VisualFogColor: "#330303" }
    },
    SoulSandValley => { ident: "minecraft:soul_sand_valley", global: 58, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioAmbientSounds: { "additions": { "sound": "minecraft:ambient.soul_sand_valley.additions", "tick_chance": 0.0111 }, "loop": "minecraft:ambient.soul_sand_valley.loop", "mood": { "block_search_extent": 8, "offset": 2, "sound": "minecraft:ambient.soul_sand_valley.mood", "tick_delay": 6000 } }, AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.nether.soul_sand_valley" } }, VisualAmbientParticles: [ { "particle": { "type": "minecraft:ash" }, "probability": 0.00625 }], VisualFogColor: "#1b4745" }
    },
    BasaltDeltas => { ident: "minecraft:basalt_deltas", global: 59, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 2f32, downfall: 0f32 },
        attr: { AudioAmbientSounds: { "additions": { "sound": "minecraft:ambient.basalt_deltas.additions", "tick_chance": 0.0111 }, "loop": "minecraft:ambient.basalt_deltas.loop", "mood": { "block_search_extent": 8, "offset": 2, "sound": "minecraft:ambient.basalt_deltas.mood", "tick_delay": 6000 } }, AudioBackgroundMusic: { "default": { "max_delay": 24000, "min_delay": 12000, "sound": "minecraft:music.nether.basalt_deltas" } }, VisualAmbientParticles: [ { "particle": { "type": "minecraft:white_ash" }, "probability": 0.118093334 }], VisualFogColor: "#685f70" }
    },
    TheEnd => { ident: "minecraft:the_end", global: 60, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { }
    },
    EndHighlands => { ident: "minecraft:end_highlands", global: 61, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { }
    },
    EndMidlands => { ident: "minecraft:end_midlands", global: 62, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { }
    },
    SmallEndIslands => { ident: "minecraft:small_end_islands", global: 63, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { }
    },
    EndBarrens => { ident: "minecraft:end_barrens", global: 64, prop: { foliage: 0, dry_foliage: 0, grass: 0, water: 4159204, precip: false, temp: 0.5f32, downfall: 0.5f32 },
        attr: { }
    }
}
