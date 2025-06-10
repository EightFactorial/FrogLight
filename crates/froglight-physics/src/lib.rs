#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(array_chunks, portable_simd))]

pub mod component;

pub mod math;
pub mod table;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::component::vector::{EntityVectors, EntityVectorsMut};
    pub use crate::component::{
        direction::LookDirection,
        state::{EntityGroundState, PlayerPhysicsState},
        vector::{EntityAcceleration, EntityPosition, EntityVelocity},
    };
}
