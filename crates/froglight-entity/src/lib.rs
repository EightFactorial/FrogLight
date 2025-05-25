#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(const_type_id))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod entity_data;
pub mod entity_type;
pub mod player;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::StaticEntityType;

    pub use crate::{
        entity_data::{
            AppearsOnFire, CustomName, CustomNameVisible, EntityBreath, HasGravity, IsGlowing,
            IsInvulnerable, IsSilent, OnFire, PortalCooldown, TicksFrozen,
        },
        entity_type::{
            AppEntityTypeStorage, EntityCollider, EntityEyeHeight, EntityGravity, EntityTypeExt,
            EntityTypeStorage, EntityTypeTrait, StaticEntityType,
            generated::{self as entity, EntityType},
        },
        player::{profile::PlayerProfile, username::PlayerUsername, uuid::PlayerUuid},
    };
}
