//! TODO
#![expect(missing_docs, reason = "WIP")]

mod acceleration;
pub use acceleration::{Acceleration, PreviousAcceleration};

#[cfg(feature = "bevy")]
mod collision;
#[cfg(feature = "bevy")]
pub use collision::{CollidingWith, EntityCollisions};

mod controller;
pub use controller::PhysicsController;

mod ground;
pub use ground::{OnGround, PreviousOnGround};

mod velocity;
pub use velocity::{PreviousVelocity, Velocity};

mod state;
pub use state::PhysicsState;

mod transform;
pub use transform::{PreviousTransform, Transform};
