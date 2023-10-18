use mc_rs_macros::Transcode;

use crate::types::{inventory::ItemSlot, position::BlockPos};

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct Particle {
    #[var]
    pub id: i32,
    pub data: ParticleData,
}

#[derive(Debug, Default, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [21])]
pub enum ParticleData {
    AmbientEntityEffect,
    AngryVillager,
    Block(BlockParticle),
    BlockMarker(BlockParticle),
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust(DustParticle),
    DustColorTransition(DustColorTransitionParticle),
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    #[default]
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    FallingDust(BlockParticle),
    Firework,
    Fishing,
    Flame,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(ItemParticle),
    Vibration(VibrationParticle),
    ItemSlime,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCozySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    Snowflake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct BlockParticle {
    #[var]
    pub state: i32,
}
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct DustParticle {
    /// Red value, 0-1
    pub red: f32,
    /// Green value, 0-1
    pub green: f32,
    /// Blue value, 0-1
    pub blue: f32,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct DustColorTransitionParticle {
    pub from_red: f32,
    pub from_green: f32,
    pub from_blue: f32,
    pub scale: f32,
    pub to_red: f32,
    pub to_green: f32,
    pub to_blue: f32,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ItemParticle {
    pub item: ItemSlot,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct VibrationParticle {
    pub origin: BlockPos,
    pub position_type: String,
    pub block_position: BlockPos,
    #[var]
    pub entity_id: u32,
    #[var]
    pub ticks: u32,
}
