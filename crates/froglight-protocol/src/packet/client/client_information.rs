#![allow(missing_docs)]

use froglight_macros::FrogReadWrite;

use crate::{common::PlayerHand, packet::flags::PlayerModelFlags};

/// Client settings sent to the server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ClientSettings {
    pub language: String,
    pub view_distance: u8,
    pub chat_visibility: ChatVisibility,
    pub chat_colors: bool,
    pub model_customization: PlayerModelFlags,
    pub main_hand: PlayerHand,
    pub text_filtering_enabled: bool,
    pub allows_listing: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ChatVisibility {
    #[default]
    All,
    System,
    None,
}
