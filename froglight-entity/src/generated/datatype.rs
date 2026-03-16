//! Placeholder

use alloc::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "facet")]
use facet_minecraft as mc;

#[expect(clippy::wildcard_imports, reason = "Generated code")]
use crate::types::*;

generate! {
    @datatypes
    as_armadillo_state => ArmadilloState(EntityVarInt),
    as_block_pos => BlockPos(EntityPosition),
    as_block_state => BlockState(EntityBlockState),
    as_boolean => Boolean(bool),
    as_byte => Byte(u8),
    as_cat_sound_variant => CatSoundVariant(EntityVarInt),
    as_cat_variant => CatVariant(EntityVarInt),
    as_chicken_sound_variant => ChickenSoundVariant(EntityVarInt),
    as_chicken_variant => ChickenVariant(EntityVarInt),
    as_component => Component(()),
    as_copper_golem_state => CopperGolemState(EntityVarInt),
    as_cow_sound_variant => CowSoundVariant(EntityVarInt),
    as_cow_variant => CowVariant(EntityVarInt),
    as_direction => Direction(EntityDirection),
    as_float => Float(f32),
    as_frog_variant => FrogVariant(EntityVarInt),
    as_humanoid_arm => HumanoidArm(EntityVarInt),
    as_int => Int(EntityVarInt),
    as_item_stack => ItemStack(()),
    as_long => Long(EntityVarLong),
    as_optional_block_pos => OptionalBlockPos(Option<EntityPosition>),
    as_optional_block_state => OptionalBlockState(EntityBlockState),
    as_optional_component => OptionalComponent(Option<()>),
    as_optional_global_pos => OptionalGlobalPos(Option<EntityGlobalPosition>),
    as_optional_living_entity_reference => OptionalLivingEntityReference(Option<uuid::Uuid>),
    as_optional_unsigned_int => OptionalUnsignedInt(EntityOptionalVarInt),
    as_painting_variant => PaintingVariant(()),
    as_particle => Particle(()),
    as_particles => Particles(()),
    as_pig_sound_variant => PigSoundVariant(EntityVarInt),
    as_pig_variant => PigVariant(EntityVarInt),
    as_pose => Pose(EntityVarInt),
    as_quaternion => Quaternion(EntityQuaternion),
    as_resolvable_profile => ResolvableProfile(froglight_player::profile::MaybePartialProfile),
    as_rotations => Rotations(EntityRotation),
    as_sniffer_state => SnifferState(EntityVarInt),
    as_string => String(Cow<'static, str>),
    as_vector3 => Vector3(EntityVec3),
    as_villager_data => VillagerData(EntityVillagerData),
    as_weathering_copper_state => WeatheringCopperState(EntityVarInt),
    as_wolf_sound_variant => WolfSoundVariant(EntityVarInt),
    as_wolf_variant => WolfVariant(EntityVarInt),
    as_zombie_nautilus_variant => ZombieNautilusVariant(EntityVarInt)
}
