//! Generated registries and their implementations
//!
//! @generated by `froglight-generator` #248246d
#![allow(missing_docs)]

mod generated;
mod v1_21_0;

pub use generated::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    generated::build(app);
    v1_21_0::build(app);
}
