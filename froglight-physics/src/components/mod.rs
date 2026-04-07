//! TODO
#![expect(missing_docs, reason = "WIP")]

mod acceleration;
pub use acceleration::{Acceleration, PreviousAcceleration};

#[cfg(feature = "bevy")]
mod collision_bevy;
#[cfg(feature = "bevy")]
pub use collision_bevy::{CollidingWith, EntityCollisions};

mod collision;
pub use collision::{PreviousWorldCollision, WorldCollision};

mod controller;
pub use controller::PhysicsController;

mod fluid;
pub use fluid::{InFluid, PreviousInFluid};

mod ground;
pub use ground::{OnGround, PreviousOnGround};

mod velocity;
pub use velocity::{PreviousVelocity, Velocity};

mod state;
pub use state::PhysicsState;

mod transform;
pub use transform::{PreviousTransform, Transform};
