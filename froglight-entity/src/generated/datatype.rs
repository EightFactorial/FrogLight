//! Placeholder

use alloc::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "facet")]
use facet_minecraft as mc;

generate! {
    @datatypes
    as_armadillo_state => ArmadilloState(()),
    as_block_pos => BlockPos(()),
    as_block_state => BlockState(()),
    as_boolean => Boolean(bool),
    as_byte => Byte(u8),
    as_cat_sound_variant => CatSoundVariant(()),
    as_cat_variant => CatVariant(()),
    as_chicken_sound_variant => ChickenSoundVariant(()),
    as_chicken_variant => ChickenVariant(()),
    as_component => Component(()),
    as_copper_golem_state => CopperGolemState(()),
    as_cow_sound_variant => CowSoundVariant(()),
    as_cow_variant => CowVariant(()),
    as_direction => Direction(()),
    as_float => Float(f32),
    as_frog_variant => FrogVariant(()),
    as_humanoid_arm => HumanoidArm(()),
    as_int => Int(#[cfg_attr(feature = "facet", facet(mc::variable))] i32),
    as_item_stack => ItemStack(()),
    as_long => Long(#[cfg_attr(feature = "facet", facet(mc::variable))] i64),
    as_optional_block_pos => OptionalBlockPos(Option<()>),
    as_optional_block_state => OptionalBlockState(Option<()>),
    as_optional_component => OptionalComponent(Option<()>),
    as_optional_living_entity_reference => OptionalLivingEntityReference(Option<()>),
    as_optional_unsigned_int => OptionalUnsignedInt(#[cfg_attr(feature = "facet", facet(mc::variable))] Option<u32>),
    as_painting_variant => PaintingVariant(()),
    as_particle => Particle(()),
    as_particles => Particles(()),
    as_pig_sound_variant => PigSoundVariant(()),
    as_pig_variant => PigVariant(()),
    as_pose => Pose(()),
    as_quaternion => Quaternion(()),
    as_resolvable_profile => ResolvableProfile(()),
    as_rotations => Rotations(()),
    as_sniffer_state => SnifferState(()),
    as_string => String(Cow<'static, str>),
    as_vector3 => Vector3(()),
    as_villager_data => VillagerData(()),
    as_weathering_copper_state => WeatheringCopperState(()),
    as_wolf_sound_variant => WolfSoundVariant(()),
    as_wolf_variant => WolfVariant(()),
    as_zombie_nautilus_variant => ZombieNautilusVariant(())
}
