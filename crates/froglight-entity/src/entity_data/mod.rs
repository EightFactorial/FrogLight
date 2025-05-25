//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_ecs::reflect::ReflectBundle;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_macros::FrogNbt;

mod component;
pub use component::*;

#[derive(Debug, Clone, PartialEq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Bundle))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Bundle))]
pub struct EntityDataBundle {
    #[frog(name = "Air")]
    pub breath: EntityBreath,
    #[frog(default, name = "CustomName")]
    pub custom_name: CustomName,
    #[frog(default, name = "CustomNameVisible")]
    pub custom_name_visible: CustomNameVisible,
    #[frog(name = "NoGravity")]
    pub gravity: HasGravity,
    #[frog(name = "Glowing")]
    pub glowing: IsGlowing,
    #[frog(name = "Invulnerable")]
    pub invulnerable: IsInvulnerable,
    #[frog(default, name = "Silent")]
    pub silent: IsSilent,
    #[frog(name = "Fire")]
    pub on_fire: OnFire,
    #[frog(name = "HasVisualFire")]
    pub on_fire_visible: AppearsOnFire,
    #[frog(name = "PortalCooldown")]
    pub portal_cooldown: PortalCooldown,
    #[frog(default, name = "TicksFrozen")]
    pub ticks_frozen: TicksFrozen,
}
