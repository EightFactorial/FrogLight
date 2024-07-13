//! Generated blocks and block attributes
//!
//! Template: 1.21.0
//!
//! @generated by `froglight-generator` #cd8324b
#![allow(missing_docs)]

pub mod attributes;
pub mod blocks;
mod v1_21_0;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    attributes::build(app);
    blocks::build(app);

    app.init_resource::<crate::BlockRegistry<froglight_protocol::versions::v1_21_0::V1_21_0>>();
}
