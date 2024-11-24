//! Generated entities for all versions.
//!
//! @generated by 'TODO'
#[allow(clippy::wildcard_imports)]
use bevy_ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

use super::component::*;
use crate::EntitySize;

froglight_macros::impl_generated_entities! {
    Allay => { PassiveMobs, MobEntity, EntitySize(0.35f32,0.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Dancing, CanDuplicate },
    AreaEffectCloud => { Unknown, OtherEntity, EntitySize(6f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Radius, Waiting, Particle },
    Armadillo => { PassiveMobs, AnimalEntity, EntitySize(0.7f32,0.65f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, ArmadilloState },
    ArmorStand => { Immobile, LivingEntity, EntitySize(0.5f32,1.975f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, ClientFlags, HeadPose, BodyPose, LeftArmPose, RightArmPose, LeftLegPose, RightLegPose },
    Arrow => { Projectiles, ProjectileEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Flags, PierceLevel, EffectColor },
    Axolotl => { PassiveMobs, AnimalEntity, EntitySize(0.75f32,0.42f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Variant, PlayingDead, FromBucket },
    Bat => { PassiveMobs, AmbientEntity, EntitySize(0.5f32,0.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    Bee => { PassiveMobs, AnimalEntity, EntitySize(0.7f32,0.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, RemainingAngerTime },
    Blaze => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    BlockDisplay => { Immobile, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, TransformationInterpolationStartDeltaTicks, TransformationInterpolationDuration, PosRotInterpolationDuration, Translation, Scale, LeftRotation, RightRotation, BillboardRenderConstraints, BrightnessOverride, ViewRange, ShadowRadius, ShadowStrength, Width, Height, GlowColorOverride, BlockState },
    Boat => { Vehicles, OtherEntity, EntitySize(1.375f32,0.5625f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, Type, PaddleLeft, PaddleRight, BubbleTime },
    Bogged => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.99f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Sheared },
    Breeze => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.77f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    BreezeWindCharge => { Projectiles, ProjectileEntity, EntitySize(0.3125f32,0.3125f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Camel => { PassiveMobs, AnimalEntity, EntitySize(1.7f32,2.375f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, Dash, LastPoseChangeTick },
    Cat => { PassiveMobs, AnimalEntity, EntitySize(0.6f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, OwnerUuid, Variant, IsLying, RelaxStateOne, CollarColor },
    CaveSpider => { HostileMobs, HostileEntity, EntitySize(0.7f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    ChestBoat => { Vehicles, OtherEntity, EntitySize(1.375f32,0.5625f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, Type, PaddleLeft, PaddleRight, BubbleTime },
    ChestMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay },
    Chicken => { PassiveMobs, AnimalEntity, EntitySize(0.4f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby },
    Cod => { PassiveMobs, WaterCreatureEntity, EntitySize(0.5f32,0.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, FromBucket },
    CommandBlockMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay, CommandName, LastOutput },
    Cow => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby },
    Creeper => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsPowered, IsIgnited },
    Dolphin => { PassiveMobs, WaterCreatureEntity, EntitySize(0.9f32,0.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, TreasurePos, GotFish, MoistnessLevel },
    Donkey => { PassiveMobs, AnimalEntity, EntitySize(1.3964844f32,1.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, Chest },
    DragonFireball => { Projectiles, ProjectileEntity, EntitySize(1f32,1f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Drowned => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, SpecialType, DrownedConversion },
    Egg => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    ElderGuardian => { HostileMobs, HostileEntity, EntitySize(1.9975f32,1.9975f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Moving, AttackTarget },
    EndCrystal => { Immobile, OtherEntity, EntitySize(2f32,2f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, BeamTarget, ShowBottom },
    EnderDragon => { HostileMobs, MobEntity, EntitySize(16f32,8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Phase },
    EnderPearl => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    Enderman => { HostileMobs, HostileEntity, EntitySize(0.6f32,2.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, CarryState, Creepy, StaredAt },
    Endermite => { HostileMobs, HostileEntity, EntitySize(0.4f32,0.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    Evoker => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating, SpellCasting },
    EvokerFangs => { HostileMobs, OtherEntity, EntitySize(0.5f32,0.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    ExperienceBottle => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    ExperienceOrb => { Unknown, OtherEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    EyeOfEnder => { Unknown, OtherEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    FallingBlock => { Unknown, OtherEntity, EntitySize(0.98f32,0.98f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, StartPos },
    Fireball => { Projectiles, ProjectileEntity, EntitySize(1f32,1f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    FireworkRocket => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, FireworksItem, AttachedToTarget, ShotAtAngle },
    FishingBobber => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, HookedEntity, Biting },
    Fox => { PassiveMobs, AnimalEntity, EntitySize(0.6f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Type, Flags, Trusted0, Trusted1 },
    Frog => { PassiveMobs, AnimalEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Variant, TongueTarget },
    FurnaceMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay, Fuel },
    Ghast => { HostileMobs, MobEntity, EntitySize(4f32,4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCharging },
    Giant => { HostileMobs, HostileEntity, EntitySize(3.6f32,12f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    GlowItemFrame => { Immobile, OtherEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Item, Rotation },
    GlowSquid => { PassiveMobs, WaterCreatureEntity, EntitySize(0.8f32,0.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, DarkTicksRemaining },
    Goat => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, IsScreamingGoat, HasLeftHorn, HasRightHorn },
    Guardian => { HostileMobs, HostileEntity, EntitySize(0.85f32,0.85f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Moving, AttackTarget },
    Hoglin => { HostileMobs, AnimalEntity, EntitySize(1.3964844f32,1.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, ImmuneToZombification },
    HopperMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay },
    Horse => { PassiveMobs, AnimalEntity, EntitySize(1.3964844f32,1.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, TypeVariant },
    Husk => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, SpecialType, DrownedConversion },
    Illusioner => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating, SpellCasting },
    Interaction => { Immobile, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Width, Height, Response },
    IronGolem => { PassiveMobs, MobEntity, EntitySize(1.4f32,2.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    Item => { Unknown, OtherEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Item },
    ItemDisplay => { Immobile, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, TransformationInterpolationStartDeltaTicks, TransformationInterpolationDuration, PosRotInterpolationDuration, Translation, Scale, LeftRotation, RightRotation, BillboardRenderConstraints, BrightnessOverride, ViewRange, ShadowRadius, ShadowStrength, Width, Height, GlowColorOverride, ItemStack, ItemDisplay },
    ItemFrame => { Immobile, OtherEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Item, Rotation },
    LeashKnot => { Immobile, OtherEntity, EntitySize(0.375f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    LightningBolt => { Unknown, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Llama => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.87f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, Chest, Strength, Variant },
    LlamaSpit => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    MagmaCube => { HostileMobs, MobEntity, EntitySize(0.52f32,0.52f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Size },
    Marker => { Unknown, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Minecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay },
    Mooshroom => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Type },
    Mule => { PassiveMobs, AnimalEntity, EntitySize(1.3964844f32,1.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, Chest },
    Ocelot => { PassiveMobs, AnimalEntity, EntitySize(0.6f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Trusting },
    OminousItemSpawner => { Unknown, OtherEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Item },
    Painting => { Immobile, OtherEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, PaintingVariant },
    Panda => { PassiveMobs, AnimalEntity, EntitySize(1.3f32,1.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, UnhappyCounter, SneezeCounter, EatCounter, MainGene, HiddenGene, Flags },
    Parrot => { PassiveMobs, AnimalEntity, EntitySize(0.5f32,0.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, OwnerUuid, Variant },
    Phantom => { HostileMobs, MobEntity, EntitySize(0.9f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Size },
    Pig => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,0.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Saddle, BoostTime },
    Piglin => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, ImmuneToZombification, Baby, IsChargingCrossbow, IsDancing },
    PiglinBrute => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, ImmuneToZombification },
    Pillager => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating, IsChargingCrossbow },
    Player => { Unknown, PlayerEntity, EntitySize(0.6f32,1.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, PlayerAbsorption, Score, PlayerModeCustomisation, PlayerMainHand, ShoulderLeft, ShoulderRight },
    PolarBear => { PassiveMobs, AnimalEntity, EntitySize(1.4f32,1.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Standing },
    Potion => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    Pufferfish => { PassiveMobs, WaterCreatureEntity, EntitySize(0.7f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, FromBucket, PuffState },
    Rabbit => { PassiveMobs, AnimalEntity, EntitySize(0.4f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Type },
    Ravager => { HostileMobs, HostileEntity, EntitySize(1.95f32,2.2f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating },
    Salmon => { PassiveMobs, WaterCreatureEntity, EntitySize(0.7f32,0.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, FromBucket },
    Sheep => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Wool },
    Shulker => { HostileMobs, MobEntity, EntitySize(1f32,1f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, AttachFace, Peek, Color },
    ShulkerBullet => { Projectiles, ProjectileEntity, EntitySize(0.3125f32,0.3125f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Silverfish => { HostileMobs, HostileEntity, EntitySize(0.4f32,0.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    Skeleton => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.99f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, StrayConversion },
    SkeletonHorse => { HostileMobs, AnimalEntity, EntitySize(1.3964844f32,1.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags },
    Slime => { HostileMobs, MobEntity, EntitySize(0.52f32,0.52f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Size },
    SmallFireball => { Projectiles, ProjectileEntity, EntitySize(0.3125f32,0.3125f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    Sniffer => { PassiveMobs, AnimalEntity, EntitySize(1.9f32,1.75f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, State, DropSeedAtTick },
    SnowGolem => { PassiveMobs, MobEntity, EntitySize(0.7f32,1.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Pumpkin },
    Snowball => { Projectiles, ProjectileEntity, EntitySize(0.25f32,0.25f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, ItemStack },
    SpawnerMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay },
    SpectralArrow => { Projectiles, ProjectileEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Flags, PierceLevel },
    Spider => { HostileMobs, HostileEntity, EntitySize(1.4f32,0.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    Squid => { PassiveMobs, WaterCreatureEntity, EntitySize(0.8f32,0.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    Stray => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.99f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    Strider => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, BoostTime, Suffocating, Saddle },
    Tadpole => { PassiveMobs, WaterCreatureEntity, EntitySize(0.4f32,0.3f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, FromBucket },
    TextDisplay => { Immobile, OtherEntity, EntitySize(0f32,0f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, TransformationInterpolationStartDeltaTicks, TransformationInterpolationDuration, PosRotInterpolationDuration, Translation, Scale, LeftRotation, RightRotation, BillboardRenderConstraints, BrightnessOverride, ViewRange, ShadowRadius, ShadowStrength, Width, Height, GlowColorOverride, Text, LineWidth, BackgroundColor, TextOpacity, StyleFlags },
    Tnt => { Unknown, OtherEntity, EntitySize(0.98f32,0.98f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Fuse, BlockState },
    TntMinecart => { Vehicles, OtherEntity, EntitySize(0.98f32,0.7f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Hurt, Damage, DisplayBlock, DisplayOffset, CustomDisplay },
    TraderLlama => { PassiveMobs, AnimalEntity, EntitySize(0.9f32,1.87f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, Chest, Strength, Variant },
    Trident => { Projectiles, ProjectileEntity, EntitySize(0.5f32,0.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Flags, PierceLevel, Loyalty, Foil },
    TropicalFish => { PassiveMobs, WaterCreatureEntity, EntitySize(0.5f32,0.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, FromBucket, TypeVariant },
    Turtle => { PassiveMobs, AnimalEntity, EntitySize(1.2f32,0.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, HomePos, HasEgg, LayingEgg, TravelPos, GoingHome, Travelling },
    Vex => { HostileMobs, HostileEntity, EntitySize(0.4f32,0.8f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Flags },
    Villager => { PassiveMobs, PassiveEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, UnhappyCounter, VillagerData },
    Vindicator => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating },
    WanderingTrader => { PassiveMobs, PassiveEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, UnhappyCounter },
    Warden => { HostileMobs, HostileEntity, EntitySize(0.9f32,2.9f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, ClientAngerLevel },
    WindCharge => { Projectiles, ProjectileEntity, EntitySize(0.3125f32,0.3125f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen },
    Witch => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, IsCelebrating, UsingItem },
    Wither => { HostileMobs, HostileEntity, EntitySize(0.9f32,3.5f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, TargetA, TargetB, TargetC, Inv },
    WitherSkeleton => { HostileMobs, HostileEntity, EntitySize(0.7f32,2.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags },
    WitherSkull => { Projectiles, ProjectileEntity, EntitySize(0.3125f32,0.3125f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, Dangerous },
    Wolf => { PassiveMobs, AnimalEntity, EntitySize(0.6f32,0.85f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags, OwnerUuid, CollarColor, RemainingAngerTime, Variant },
    Zoglin => { HostileMobs, HostileEntity, EntitySize(1.3964844f32,1.4f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby },
    Zombie => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, SpecialType, DrownedConversion },
    ZombieHorse => { HostileMobs, AnimalEntity, EntitySize(1.3964844f32,1.6f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, Flags },
    ZombieVillager => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, SpecialType, DrownedConversion, Converting, VillagerData },
    ZombifiedPiglin => { HostileMobs, HostileEntity, EntitySize(0.6f32,1.95f32), SharedFlags, AirSupply, Silent, NoGravity, TicksFrozen, LivingEntityFlags, Health, EffectParticles, EffectAmbience, MobFlags, Baby, SpecialType, DrownedConversion }
}
