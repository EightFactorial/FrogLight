//! Generated registry implementations
//!
//! @generated by `froglight-generator` #b0e1aa4
#![allow(missing_docs)]

mod generated;
mod v1_21_0;

pub use generated::*;

#[doc(hidden)]
#[cfg(feature = "bevy")]
pub(super) fn build(app: &mut bevy_app::App) { generated::build(app); }
