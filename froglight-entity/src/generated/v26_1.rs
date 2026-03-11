use froglight_common::version::V26_1;

#[expect(clippy::wildcard_imports, reason = "Generated code")]
use crate::{
    generated::{component::*, entity::*},
    types::*,
};

generate! {
    @version V26_1,
    datatypes: {
        Byte(u8) = 0,
        Int(EntityVarInt) = 1,
        Long(EntityVarLong) = 2,
        Float(f32) = 3,
        String(Cow<'static, str>) = 4,
        Component(()) = 5,
        OptionalComponent(Option<()>) = 6,
        ItemStack(()) = 7,
        BlockState(EntityBlockState) = 8,
        OptionalBlockState(EntityBlockState) = 9,
        Boolean(bool) = 10,
        Particle(()) = 11,
        Particles(()) = 12,
        Rotations(EntityRotation) = 13,
        BlockPos(EntityPosition) = 14,
        OptionalBlockPos(Option<EntityPosition>) = 15,
        Direction(EntityDirection) = 16,
        OptionalLivingEntityReference(Option<uuid::Uuid>) = 17,
        OptionalGlobalPos(Option<EntityGlobalPosition>) = 18,
        VillagerData(EntityVillagerData) = 19,
        OptionalUnsignedInt(EntityOptionalVarInt) = 20,
        Pose(()) = 21,
        CatVariant(EntityVarInt) = 22,
        CatSoundVariant(EntityVarInt) = 23,
        ChickenVariant(EntityVarInt) = 24,
        ChickenSoundVariant(EntityVarInt) = 25,
        CowVariant(EntityVarInt) = 26,
        CowSoundVariant(EntityVarInt) = 27,
        WolfVariant(EntityVarInt) = 28,
        WolfSoundVariant(EntityVarInt) = 29,
        FrogVariant(EntityVarInt) = 30,
        PigVariant(EntityVarInt) = 31,
        PigSoundVariant(EntityVarInt) = 32,
        ZombieNautilusVariant(EntityVarInt) = 33,
        PaintingVariant(()) = 34,
        ArmadilloState(()) = 35,
        SnifferState(()) = 36,
        WeatheringCopperState(()) = 37,
        CopperGolemState(()) = 38,
        Vector3(EntityVec3) = 39,
        Quaternion(EntityQuaternion) = 40,
        ResolvableProfile(()) = 41,
        HumanoidArm(()) = 42
    },
    AcaciaBoat => { ident: "minecraft:acacia_boat", global: 0,
        components: [ ]
    },
    AcaciaChestBoat => { ident: "minecraft:acacia_chest_boat", global: 1,
        components: [ ]
    },
    Allay => { ident: "minecraft:allay", global: 2,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AllayDancing = 16, AllayCanDuplicate = 17 ]
    },
    AreaEffectCloud => { ident: "minecraft:area_effect_cloud", global: 3,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, AreaEffectCloudRadius = 8, AreaEffectCloudWaiting = 9, AreaEffectCloudParticle = 10 ]
    },
    Armadillo => { ident: "minecraft:armadillo", global: 4,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, ArmadilloState = 18 ]
    },
    ArmorStand => { ident: "minecraft:armor_stand", global: 5,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, ArmorStandClientFlags = 15, ArmorStandHeadPose = 16, ArmorStandBodyPose = 17, ArmorStandLeftArmPose = 18, ArmorStandRightArmPose = 19, ArmorStandLeftLegPose = 20, ArmorStandRightLegPose = 21 ]
    },
    Arrow => { ident: "minecraft:arrow", global: 6,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, AbstractArrowIdFlags = 8, AbstractArrowPierceLevel = 9, AbstractArrowInGround = 10, ArrowIdEffectColor = 11 ]
    },
    Axolotl => { ident: "minecraft:axolotl", global: 7,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AxolotlVariant = 18, AxolotlPlayingDead = 19, AxolotlFromBucket = 20 ]
    },
    BambooChestRaft => { ident: "minecraft:bamboo_chest_raft", global: 8,
        components: [ ]
    },
    BambooRaft => { ident: "minecraft:bamboo_raft", global: 9,
        components: [ ]
    },
    Bat => { ident: "minecraft:bat", global: 10,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, BatIdFlags = 16 ]
    },
    Bee => { ident: "minecraft:bee", global: 11,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, BeeFlagsId = 18, BeeAngerEndTime = 19 ]
    },
    BirchBoat => { ident: "minecraft:birch_boat", global: 12,
        components: [ ]
    },
    BirchChestBoat => { ident: "minecraft:birch_chest_boat", global: 13,
        components: [ ]
    },
    Blaze => { ident: "minecraft:blaze", global: 14,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, BlazeFlagsId = 16 ]
    },
    BlockDisplay => { ident: "minecraft:block_display", global: 15,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, DisplayTransformationInterpolationStartDeltaTicksId = 8, DisplayTransformationInterpolationDurationId = 9, DisplayPosRotInterpolationDurationId = 10, DisplayTranslationId = 11, DisplayScaleId = 12, DisplayLeftRotationId = 13, DisplayRightRotationId = 14, DisplayBillboardRenderConstraintsId = 15, DisplayBrightnessOverrideId = 16, DisplayViewRangeId = 17, DisplayShadowRadiusId = 18, DisplayShadowStrengthId = 19, DisplayWidthId = 20, DisplayHeightId = 21, DisplayGlowColorOverrideId = 22, DisplayBlockStateId = 23 ]
    },
    Bogged => { ident: "minecraft:bogged", global: 16,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, BoggedSheared = 16 ]
    },
    Breeze => { ident: "minecraft:breeze", global: 17,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    BreezeWindCharge => { ident: "minecraft:breeze_wind_charge", global: 18,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Camel => { ident: "minecraft:camel", global: 19,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, CamelDash = 19, CamelLastPoseChangeTick = 20 ]
    },
    CamelHusk => { ident: "minecraft:camel_husk", global: 20,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, CamelDash = 19, CamelLastPoseChangeTick = 20 ]
    },
    Cat => { ident: "minecraft:cat", global: 21,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TamableAnimalFlagsId = 18, TamableAnimalOwneruuidId = 19, CatVariantId = 20, CatIsLying = 21, CatRelaxStateOne = 22, CatCollarColor = 23, CatSoundVariantId = 24 ]
    },
    CaveSpider => { ident: "minecraft:cave_spider", global: 22,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SpiderFlagsId = 16 ]
    },
    CherryBoat => { ident: "minecraft:cherry_boat", global: 23,
        components: [ ]
    },
    CherryChestBoat => { ident: "minecraft:cherry_chest_boat", global: 24,
        components: [ ]
    },
    ChestMinecart => { ident: "minecraft:chest_minecart", global: 25,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12 ]
    },
    Chicken => { ident: "minecraft:chicken", global: 26,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, ChickenVariantId = 18, ChickenSoundVariantId = 19 ]
    },
    Cod => { ident: "minecraft:cod", global: 27,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractFishFromBucket = 16 ]
    },
    CopperGolem => { ident: "minecraft:copper_golem", global: 28,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, CopperGolemWeatherState = 16, CopperGolemState = 17 ]
    },
    CommandBlockMinecart => { ident: "minecraft:command_block_minecart", global: 29,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12, MinecartCommandBlockIdCommandName = 13, MinecartCommandBlockIdLastOutput = 14 ]
    },
    Cow => { ident: "minecraft:cow", global: 30,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, CowVariantId = 18, CowSoundVariantId = 19 ]
    },
    Creaking => { ident: "minecraft:creaking", global: 31,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, CreakingCanMove = 16, CreakingIsActive = 17, CreakingIsTearingDown = 18, CreakingHomePos = 19 ]
    },
    Creeper => { ident: "minecraft:creeper", global: 32,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, CreeperSwellDir = 16, CreeperIsPowered = 17, CreeperIsIgnited = 18 ]
    },
    DarkOakBoat => { ident: "minecraft:dark_oak_boat", global: 33,
        components: [ ]
    },
    DarkOakChestBoat => { ident: "minecraft:dark_oak_chest_boat", global: 34,
        components: [ ]
    },
    Dolphin => { ident: "minecraft:dolphin", global: 35,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, DolphinGotFish = 18, DolphinMoistnessLevel = 19 ]
    },
    Donkey => { ident: "minecraft:donkey", global: 36,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, AbstractChestedHorseIdChest = 19 ]
    },
    DragonFireball => { ident: "minecraft:dragon_fireball", global: 37,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Drowned => { ident: "minecraft:drowned", global: 38,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZombieBabyId = 16, ZombieSpecialTypeId = 17, ZombieDrownedConversionId = 18 ]
    },
    Egg => { ident: "minecraft:egg", global: 39,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    ElderGuardian => { ident: "minecraft:elder_guardian", global: 40,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, GuardianIdMoving = 16, GuardianIdAttackTarget = 17 ]
    },
    Enderman => { ident: "minecraft:enderman", global: 41,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, EnderManCarryState = 16, EnderManCreepy = 17, EnderManStaredAt = 18 ]
    },
    Endermite => { ident: "minecraft:endermite", global: 42,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    EnderDragon => { ident: "minecraft:ender_dragon", global: 43,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, EnderDragonPhase = 16 ]
    },
    EnderPearl => { ident: "minecraft:ender_pearl", global: 44,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    EndCrystal => { ident: "minecraft:end_crystal", global: 45,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, EndCrystalBeamTarget = 8, EndCrystalShowBottom = 9 ]
    },
    Evoker => { ident: "minecraft:evoker", global: 46,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16, SpellcasterIllagerSpellCastingId = 17 ]
    },
    EvokerFangs => { ident: "minecraft:evoker_fangs", global: 47,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    ExperienceBottle => { ident: "minecraft:experience_bottle", global: 48,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    ExperienceOrb => { ident: "minecraft:experience_orb", global: 49,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ExperienceOrbValue = 8 ]
    },
    EyeOfEnder => { ident: "minecraft:eye_of_ender", global: 50,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, EyeOfEnderItemStack = 8 ]
    },
    FallingBlock => { ident: "minecraft:falling_block", global: 51,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, FallingBlockEntityStartPos = 8 ]
    },
    Fireball => { ident: "minecraft:fireball", global: 52,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, FireballItemStack = 8 ]
    },
    FireworkRocket => { ident: "minecraft:firework_rocket", global: 53,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, FireworkRocketEntityIdFireworksItem = 8, FireworkRocketEntityAttachedToTarget = 9, FireworkRocketEntityShotAtAngle = 10 ]
    },
    Fox => { ident: "minecraft:fox", global: 54,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, FoxTypeId = 18, FoxFlagsId = 19, FoxTrustedId0 = 20, FoxTrustedId1 = 21 ]
    },
    Frog => { ident: "minecraft:frog", global: 55,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, FrogVariantId = 18, FrogTongueTargetId = 19 ]
    },
    FurnaceMinecart => { ident: "minecraft:furnace_minecart", global: 56,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12, MinecartFurnaceIdFuel = 13 ]
    },
    Ghast => { ident: "minecraft:ghast", global: 57,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, GhastIsCharging = 16 ]
    },
    HappyGhast => { ident: "minecraft:happy_ghast", global: 58,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, HappyGhastIsLeashHolder = 18, HappyGhastStaysStill = 19 ]
    },
    Giant => { ident: "minecraft:giant", global: 59,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    GlowItemFrame => { ident: "minecraft:glow_item_frame", global: 60,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, HangingEntityDirection = 8, ItemFrameItem = 9, ItemFrameRotation = 10 ]
    },
    GlowSquid => { ident: "minecraft:glow_squid", global: 61,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, GlowSquidDarkTicksRemaining = 18 ]
    },
    Goat => { ident: "minecraft:goat", global: 62,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, GoatIsScreamingGoat = 18, GoatHasLeftHorn = 19, GoatHasRightHorn = 20 ]
    },
    Guardian => { ident: "minecraft:guardian", global: 63,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, GuardianIdMoving = 16, GuardianIdAttackTarget = 17 ]
    },
    Hoglin => { ident: "minecraft:hoglin", global: 64,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, HoglinImmuneToZombification = 18 ]
    },
    HopperMinecart => { ident: "minecraft:hopper_minecart", global: 65,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12 ]
    },
    Horse => { ident: "minecraft:horse", global: 66,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, HorseIdTypeVariant = 19 ]
    },
    Husk => { ident: "minecraft:husk", global: 67,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZombieBabyId = 16, ZombieSpecialTypeId = 17, ZombieDrownedConversionId = 18 ]
    },
    Illusioner => { ident: "minecraft:illusioner", global: 68,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16, SpellcasterIllagerSpellCastingId = 17 ]
    },
    Interaction => { ident: "minecraft:interaction", global: 69,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, InteractionWidthId = 8, InteractionHeightId = 9, InteractionResponseId = 10 ]
    },
    IronGolem => { ident: "minecraft:iron_golem", global: 70,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, IronGolemFlagsId = 16 ]
    },
    ItemEntity => { ident: "minecraft:item", global: 71,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ItemEntityItem = 8 ]
    },
    ItemDisplay => { ident: "minecraft:item_display", global: 72,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, DisplayTransformationInterpolationStartDeltaTicksId = 8, DisplayTransformationInterpolationDurationId = 9, DisplayPosRotInterpolationDurationId = 10, DisplayTranslationId = 11, DisplayScaleId = 12, DisplayLeftRotationId = 13, DisplayRightRotationId = 14, DisplayBillboardRenderConstraintsId = 15, DisplayBrightnessOverrideId = 16, DisplayViewRangeId = 17, DisplayShadowRadiusId = 18, DisplayShadowStrengthId = 19, DisplayWidthId = 20, DisplayHeightId = 21, DisplayGlowColorOverrideId = 22, DisplayItemStackId = 23, DisplayItemDisplayId = 24 ]
    },
    ItemFrame => { ident: "minecraft:item_frame", global: 73,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, HangingEntityDirection = 8, ItemFrameItem = 9, ItemFrameRotation = 10 ]
    },
    JungleBoat => { ident: "minecraft:jungle_boat", global: 74,
        components: [ ]
    },
    JungleChestBoat => { ident: "minecraft:jungle_chest_boat", global: 75,
        components: [ ]
    },
    LeashKnot => { ident: "minecraft:leash_knot", global: 76,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    LightningBolt => { ident: "minecraft:lightning_bolt", global: 77,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Llama => { ident: "minecraft:llama", global: 78,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, AbstractChestedHorseIdChest = 19, LlamaStrengthId = 20, LlamaVariantId = 21 ]
    },
    LlamaSpit => { ident: "minecraft:llama_spit", global: 79,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    MagmaCube => { ident: "minecraft:magma_cube", global: 80,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SlimeIdSize = 16 ]
    },
    MangroveBoat => { ident: "minecraft:mangrove_boat", global: 81,
        components: [ ]
    },
    MangroveChestBoat => { ident: "minecraft:mangrove_chest_boat", global: 82,
        components: [ ]
    },
    Mannequin => { ident: "minecraft:mannequin", global: 83,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MannequinAvatarPlayerMainHand = 15, MannequinAvatarPlayerModeCustomisation = 16, MannequinProfile = 17, MannequinImmovable = 18, MannequinDescription = 19 ]
    },
    Marker => { ident: "minecraft:marker", global: 84,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Minecart => { ident: "minecraft:minecart", global: 85,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12 ]
    },
    Mooshroom => { ident: "minecraft:mooshroom", global: 86,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, MushroomCowType = 18 ]
    },
    Mule => { ident: "minecraft:mule", global: 87,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, AbstractChestedHorseIdChest = 19 ]
    },
    Nautilus => { ident: "minecraft:nautilus", global: 88,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TamableAnimalFlagsId = 18, TamableAnimalOwneruuidId = 19, AbstractNautilusDash = 20 ]
    },
    OakBoat => { ident: "minecraft:oak_boat", global: 89,
        components: [ ]
    },
    OakChestBoat => { ident: "minecraft:oak_chest_boat", global: 90,
        components: [ ]
    },
    Ocelot => { ident: "minecraft:ocelot", global: 91,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, OcelotTrusting = 18 ]
    },
    OminousItemSpawner => { ident: "minecraft:ominous_item_spawner", global: 92,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, OminousItemSpawnerItem = 8 ]
    },
    Painting => { ident: "minecraft:painting", global: 93,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, HangingEntityDirection = 8, PaintingVariantId = 9 ]
    },
    PaleOakBoat => { ident: "minecraft:pale_oak_boat", global: 94,
        components: [ ]
    },
    PaleOakChestBoat => { ident: "minecraft:pale_oak_chest_boat", global: 95,
        components: [ ]
    },
    Panda => { ident: "minecraft:panda", global: 96,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, PandaUnhappyCounter = 18, PandaSneezeCounter = 19, PandaEatCounter = 20, PandaMainGeneId = 21, PandaHiddenGeneId = 22, PandaIdFlags = 23 ]
    },
    Parched => { ident: "minecraft:parched", global: 97,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    Parrot => { ident: "minecraft:parrot", global: 98,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TamableAnimalFlagsId = 18, TamableAnimalOwneruuidId = 19, ParrotVariantId = 20 ]
    },
    Phantom => { ident: "minecraft:phantom", global: 99,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, PhantomIdSize = 16 ]
    },
    Pig => { ident: "minecraft:pig", global: 100,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, PigBoostTime = 18, PigVariantId = 19, PigSoundVariantId = 20 ]
    },
    Piglin => { ident: "minecraft:piglin", global: 101,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractPiglinImmuneToZombification = 16, PiglinBabyId = 17, PiglinIsChargingCrossbow = 18, PiglinIsDancing = 19 ]
    },
    PiglinBrute => { ident: "minecraft:piglin_brute", global: 102,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractPiglinImmuneToZombification = 16 ]
    },
    Pillager => { ident: "minecraft:pillager", global: 103,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16, PillagerIsChargingCrossbow = 17 ]
    },
    PolarBear => { ident: "minecraft:polar_bear", global: 104,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, PolarBearStandingId = 18 ]
    },
    SplashPotion => { ident: "minecraft:splash_potion", global: 105,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    LingeringPotion => { ident: "minecraft:lingering_potion", global: 106,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    Pufferfish => { ident: "minecraft:pufferfish", global: 107,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractFishFromBucket = 16, PufferfishPuffState = 17 ]
    },
    Rabbit => { ident: "minecraft:rabbit", global: 108,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, RabbitTypeId = 18 ]
    },
    Ravager => { ident: "minecraft:ravager", global: 109,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16 ]
    },
    Salmon => { ident: "minecraft:salmon", global: 110,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractFishFromBucket = 16, SalmonType = 17 ]
    },
    Sheep => { ident: "minecraft:sheep", global: 111,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, SheepWoolId = 18 ]
    },
    Shulker => { ident: "minecraft:shulker", global: 112,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ShulkerAttachFaceId = 16, ShulkerPeekId = 17, ShulkerColorId = 18 ]
    },
    ShulkerBullet => { ident: "minecraft:shulker_bullet", global: 113,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Silverfish => { ident: "minecraft:silverfish", global: 114,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    Skeleton => { ident: "minecraft:skeleton", global: 115,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SkeletonStrayConversionId = 16 ]
    },
    SkeletonHorse => { ident: "minecraft:skeleton_horse", global: 116,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18 ]
    },
    Slime => { ident: "minecraft:slime", global: 117,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SlimeIdSize = 16 ]
    },
    SmallFireball => { ident: "minecraft:small_fireball", global: 118,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, FireballItemStack = 8 ]
    },
    Sniffer => { ident: "minecraft:sniffer", global: 119,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, SnifferState = 18, SnifferDropSeedAtTick = 19 ]
    },
    Snowball => { ident: "minecraft:snowball", global: 120,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, ThrowableItemProjectileItemStack = 8 ]
    },
    SnowGolem => { ident: "minecraft:snow_golem", global: 121,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SnowGolemPumpkinId = 16 ]
    },
    SpawnerMinecart => { ident: "minecraft:spawner_minecart", global: 122,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12 ]
    },
    SpectralArrow => { ident: "minecraft:spectral_arrow", global: 123,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, AbstractArrowIdFlags = 8, AbstractArrowPierceLevel = 9, AbstractArrowInGround = 10 ]
    },
    Spider => { ident: "minecraft:spider", global: 124,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, SpiderFlagsId = 16 ]
    },
    SpruceBoat => { ident: "minecraft:spruce_boat", global: 125,
        components: [ ]
    },
    SpruceChestBoat => { ident: "minecraft:spruce_chest_boat", global: 126,
        components: [ ]
    },
    Squid => { ident: "minecraft:squid", global: 127,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17 ]
    },
    Stray => { ident: "minecraft:stray", global: 128,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    Strider => { ident: "minecraft:strider", global: 129,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, StriderBoostTime = 18, StriderSuffocating = 19 ]
    },
    Tadpole => { ident: "minecraft:tadpole", global: 130,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractFishFromBucket = 16, TadpoleAgeLocked = 17 ]
    },
    TextDisplay => { ident: "minecraft:text_display", global: 131,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, DisplayTransformationInterpolationStartDeltaTicksId = 8, DisplayTransformationInterpolationDurationId = 9, DisplayPosRotInterpolationDurationId = 10, DisplayTranslationId = 11, DisplayScaleId = 12, DisplayLeftRotationId = 13, DisplayRightRotationId = 14, DisplayBillboardRenderConstraintsId = 15, DisplayBrightnessOverrideId = 16, DisplayViewRangeId = 17, DisplayShadowRadiusId = 18, DisplayShadowStrengthId = 19, DisplayWidthId = 20, DisplayHeightId = 21, DisplayGlowColorOverrideId = 22, DisplayTextId = 23, DisplayLineWidthId = 24, DisplayBackgroundColorId = 25, DisplayTextOpacityId = 26, DisplayStyleFlagsId = 27 ]
    },
    Tnt => { ident: "minecraft:tnt", global: 132,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, PrimedTntFuseId = 8, PrimedTntBlockStateId = 9 ]
    },
    TntMinecart => { ident: "minecraft:tnt_minecart", global: 133,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, VehicleEntityIdHurt = 8, VehicleEntityIdHurtdir = 9, VehicleEntityIdDamage = 10, AbstractMinecartIdCustomDisplayBlock = 11, AbstractMinecartIdDisplayOffset = 12 ]
    },
    TraderLlama => { ident: "minecraft:trader_llama", global: 134,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18, AbstractChestedHorseIdChest = 19, LlamaStrengthId = 20, LlamaVariantId = 21 ]
    },
    Trident => { ident: "minecraft:trident", global: 135,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, AbstractArrowIdFlags = 8, AbstractArrowPierceLevel = 9, AbstractArrowInGround = 10, ThrownTridentIdLoyalty = 11, ThrownTridentIdFoil = 12 ]
    },
    TropicalFish => { ident: "minecraft:tropical_fish", global: 136,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AbstractFishFromBucket = 16, TropicalFishIdTypeVariant = 17 ]
    },
    Turtle => { ident: "minecraft:turtle", global: 137,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TurtleHasEgg = 18, TurtleLayingEgg = 19 ]
    },
    Vex => { ident: "minecraft:vex", global: 138,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, VexFlagsId = 16 ]
    },
    Villager => { ident: "minecraft:villager", global: 139,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractVillagerUnhappyCounter = 18, VillagerData = 19, VillagerDataFinalized = 20 ]
    },
    Vindicator => { ident: "minecraft:vindicator", global: 140,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16 ]
    },
    WanderingTrader => { ident: "minecraft:wandering_trader", global: 141,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractVillagerUnhappyCounter = 18 ]
    },
    Warden => { ident: "minecraft:warden", global: 142,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, WardenClientAngerLevel = 16 ]
    },
    WindCharge => { ident: "minecraft:wind_charge", global: 143,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7 ]
    },
    Witch => { ident: "minecraft:witch", global: 144,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, RaiderIsCelebrating = 16, WitchUsingItem = 17 ]
    },
    Wither => { ident: "minecraft:wither", global: 145,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, WitherBossTargetA = 16, WitherBossTargetB = 17, WitherBossTargetC = 18, WitherBossIdInv = 19 ]
    },
    WitherSkeleton => { ident: "minecraft:wither_skeleton", global: 146,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15 ]
    },
    WitherSkull => { ident: "minecraft:wither_skull", global: 147,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, WitherSkullDangerous = 8 ]
    },
    Wolf => { ident: "minecraft:wolf", global: 148,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TamableAnimalFlagsId = 18, TamableAnimalOwneruuidId = 19, WolfInterestedId = 20, WolfCollarColor = 21, WolfAngerEndTime = 22, WolfVariantId = 23, WolfSoundVariantId = 24 ]
    },
    Zoglin => { ident: "minecraft:zoglin", global: 149,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZoglinBabyId = 16 ]
    },
    Zombie => { ident: "minecraft:zombie", global: 150,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZombieBabyId = 16, ZombieSpecialTypeId = 17, ZombieDrownedConversionId = 18 ]
    },
    ZombieHorse => { ident: "minecraft:zombie_horse", global: 151,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, AbstractHorseIdFlags = 18 ]
    },
    ZombieNautilus => { ident: "minecraft:zombie_nautilus", global: 152,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, AgeableMobBabyId = 16, AgeableMobAgeLocked = 17, TamableAnimalFlagsId = 18, TamableAnimalOwneruuidId = 19, AbstractNautilusDash = 20, ZombieNautilusVariantId = 21 ]
    },
    ZombieVillager => { ident: "minecraft:zombie_villager", global: 153,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZombieBabyId = 16, ZombieSpecialTypeId = 17, ZombieDrownedConversionId = 18, ZombieVillagerConvertingId = 19, ZombieVillagerVillagerData = 20, ZombieVillagerVillagerDataFinalized = 21 ]
    },
    ZombifiedPiglin => { ident: "minecraft:zombified_piglin", global: 154,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, LivingEntityFlags = 8, LivingEntityHealthId = 9, LivingEntityEffectParticles = 10, LivingEntityEffectAmbienceId = 11, LivingEntityArrowCountId = 12, LivingEntityStingerCountId = 13, LivingEntitySleepingPosId = 14, MobFlagsId = 15, ZombieBabyId = 16, ZombieSpecialTypeId = 17, ZombieDrownedConversionId = 18 ]
    },
    Player => { ident: "minecraft:player", global: 155,
        components: [ ]
    },
    FishingBobber => { ident: "minecraft:fishing_bobber", global: 156,
        components: [ EntitySharedFlagsId = 0, EntityAirSupplyId = 1, EntityCustomName = 2, EntityCustomNameVisible = 3, EntitySilent = 4, EntityNoGravity = 5, EntityPose = 6, EntityTicksFrozen = 7, FishingHookHookedEntity = 8, FishingHookBiting = 9 ]
    }
}
