#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod assets;

pub mod systemsets;

mod layout;
pub use layout::{
    fade_animation::FadeAnimationMarker, loading_art::LoadingArt, progress_bar::ProgressBar,
    LoadingScreenCenter, LoadingScreenRoot,
};

mod plugin;
pub use plugin::LoadingPlugin;
