#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod component;
pub mod table;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::component::{
        direction::LookDirection,
        modifier::EntityPhysicsModifiers,
        state::{EntityGroundState, PlayerPhysicsState},
        vector::{CurrentAcceleration, CurrentPosition, CurrentVelocity},
    };
}
