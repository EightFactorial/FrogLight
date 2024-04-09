//! Data structures used inside packets.

mod advancement_tab;
pub use advancement_tab::AdvancementTabAction;

mod block_hit;
pub use block_hit::BlockHit;

mod chat_suggestion;
pub use chat_suggestion::ChatSuggestionAction;

mod chunk_data;
pub use chunk_data::{BiomeDataPacket, BlockEntity, ChunkDataPacket, SectionDataPacket};

mod client;
pub use client::*;

mod entity_animation;
pub use entity_animation::EntityAnimation;

mod flags;
pub use flags::{
    ClientPlayerAbilityFlags, PlayerInputFlags, PlayerModelFlags, RelativePositionFlags,
    ServerPlayerAbilityFlags,
};

mod game_event;
pub use game_event::GameEvent;

mod item_slot;
pub use item_slot::{ItemSlot, ItemSlotAction, LegacyItemSlot};

mod look_anchor;
pub use look_anchor::{LookAnchor, LookEntity};

mod player_interaction;
pub use player_interaction::PlayerInteraction;

mod server_status;
pub use server_status::{ServerPlayers, ServerSamplePlayer, ServerStatus, ServerVersion};
