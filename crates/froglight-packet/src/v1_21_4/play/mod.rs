//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};


pub(super) mod c2s_0x00_accept_teleportation;
pub use c2s_0x00_accept_teleportation::TeleportConfirmC2SPacket;

pub(super) mod c2s_0x01_block_entity_tag_query;
pub use c2s_0x01_block_entity_tag_query::QueryBlockNbtC2SPacket;

pub(super) mod c2s_0x02_bundle_item_selected;
pub use c2s_0x02_bundle_item_selected::BundleItemSelectedC2SPacket;

pub(super) mod c2s_0x03_change_difficulty;
pub use c2s_0x03_change_difficulty::UpdateDifficultyC2SPacket;

pub(super) mod c2s_0x04_chat_ack;
pub use c2s_0x04_chat_ack::MessageAcknowledgmentC2SPacket;

pub(super) mod c2s_0x05_chat_command;
pub use c2s_0x05_chat_command::CommandExecutionC2SPacket;

pub(super) mod c2s_0x06_chat_command_signed;
pub use c2s_0x06_chat_command_signed::ChatCommandSignedC2SPacket;

pub(super) mod c2s_0x07_chat;
pub use c2s_0x07_chat::ChatMessageC2SPacket;

pub(super) mod c2s_0x08_chat_session_update;
pub use c2s_0x08_chat_session_update::PlayerSessionC2SPacket;

pub(super) mod c2s_0x09_chunk_batch_received;
pub use c2s_0x09_chunk_batch_received::AcknowledgeChunksC2SPacket;

pub(super) mod c2s_0x0a_client_command;
pub use c2s_0x0a_client_command::ClientStatusC2SPacket;

pub(super) mod c2s_0x0b_client_tick_end;
pub use c2s_0x0b_client_tick_end::ClientTickEndC2SPacket;

pub(super) mod c2s_0x0c_client_information;
pub use c2s_0x0c_client_information::ClientOptionsC2SPacket;

pub(super) mod c2s_0x0d_command_suggestion;
pub use c2s_0x0d_command_suggestion::RequestCommandCompletionsC2SPacket;

pub(super) mod c2s_0x0e_configuration_acknowledged;
pub use c2s_0x0e_configuration_acknowledged::AcknowledgeReconfigurationC2SPacket;

pub(super) mod c2s_0x0f_container_button_click;
pub use c2s_0x0f_container_button_click::ButtonClickC2SPacket;

pub(super) mod c2s_0x10_container_click;
pub use c2s_0x10_container_click::ClickSlotC2SPacket;

pub(super) mod c2s_0x11_container_close;
pub use c2s_0x11_container_close::CloseHandledScreenC2SPacket;

pub(super) mod c2s_0x12_container_slot_state_changed;
pub use c2s_0x12_container_slot_state_changed::SlotChangedStateC2SPacket;

pub(super) mod c2s_0x13_cookie_response;
pub use c2s_0x13_cookie_response::CookieResponseC2SPacket;

pub(super) mod c2s_0x14_custom_payload;
pub use c2s_0x14_custom_payload::CustomPayloadC2SPacket;

pub(super) mod c2s_0x15_debug_sample_subscription;
pub use c2s_0x15_debug_sample_subscription::DebugSampleSubscriptionC2SPacket;

pub(super) mod c2s_0x16_edit_book;
pub use c2s_0x16_edit_book::BookUpdateC2SPacket;

pub(super) mod c2s_0x17_entity_tag_query;
pub use c2s_0x17_entity_tag_query::QueryEntityNbtC2SPacket;

pub(super) mod c2s_0x18_interact;
pub use c2s_0x18_interact::PlayerInteractEntityC2SPacket;

pub(super) mod c2s_0x19_jigsaw_generate;
pub use c2s_0x19_jigsaw_generate::JigsawGeneratingC2SPacket;

pub(super) mod c2s_0x1a_keep_alive;
pub use c2s_0x1a_keep_alive::KeepAliveC2SPacket;

pub(super) mod c2s_0x1b_lock_difficulty;
pub use c2s_0x1b_lock_difficulty::UpdateDifficultyLockC2SPacket;

pub(super) mod c2s_0x1c_move_player_pos;
pub use c2s_0x1c_move_player_pos::PlayerPositionAndOnGroundC2SPacket;

pub(super) mod c2s_0x1d_move_player_pos_rot;
pub use c2s_0x1d_move_player_pos_rot::PlayerFullC2SPacket;

pub(super) mod c2s_0x1e_move_player_rot;
pub use c2s_0x1e_move_player_rot::PlayerLookAndOnGroundC2SPacket;

pub(super) mod c2s_0x1f_move_player_status_only;
pub use c2s_0x1f_move_player_status_only::PlayerOnGroundOnlyC2SPacket;

pub(super) mod c2s_0x20_move_vehicle;
pub use c2s_0x20_move_vehicle::VehicleMoveC2SPacket;

pub(super) mod c2s_0x21_paddle_boat;
pub use c2s_0x21_paddle_boat::BoatPaddleStateC2SPacket;

pub(super) mod c2s_0x22_pick_item_from_block;
pub use c2s_0x22_pick_item_from_block::PickItemFromBlockC2SPacket;

pub(super) mod c2s_0x23_pick_item_from_entity;
pub use c2s_0x23_pick_item_from_entity::PickItemFromEntityC2SPacket;

pub(super) mod c2s_0x24_ping_request;
pub use c2s_0x24_ping_request::QueryPingC2SPacket;

pub(super) mod c2s_0x25_place_recipe;
pub use c2s_0x25_place_recipe::CraftRequestC2SPacket;

pub(super) mod c2s_0x26_player_abilities;
pub use c2s_0x26_player_abilities::UpdatePlayerAbilitiesC2SPacket;

pub(super) mod c2s_0x27_player_action;
pub use c2s_0x27_player_action::PlayerActionC2SPacket;

pub(super) mod c2s_0x28_player_command;
pub use c2s_0x28_player_command::ClientCommandC2SPacket;

pub(super) mod c2s_0x29_player_input;
pub use c2s_0x29_player_input::PlayerInputC2SPacket;

pub(super) mod c2s_0x2a_player_loaded;
pub use c2s_0x2a_player_loaded::PlayerLoadedC2SPacket;

pub(super) mod c2s_0x2b_pong;
pub use c2s_0x2b_pong::CommonPongC2SPacket;

pub(super) mod c2s_0x2c_recipe_book_change_settings;
pub use c2s_0x2c_recipe_book_change_settings::RecipeCategoryOptionsC2SPacket;

pub(super) mod c2s_0x2d_recipe_book_seen_recipe;
pub use c2s_0x2d_recipe_book_seen_recipe::RecipeBookDataC2SPacket;

pub(super) mod c2s_0x2e_rename_item;
pub use c2s_0x2e_rename_item::RenameItemC2SPacket;

pub(super) mod c2s_0x2f_resource_pack;
pub use c2s_0x2f_resource_pack::ResourcePackStatusC2SPacket;

pub(super) mod c2s_0x30_seen_advancements;
pub use c2s_0x30_seen_advancements::AdvancementTabC2SPacket;

pub(super) mod c2s_0x31_select_trade;
pub use c2s_0x31_select_trade::SelectMerchantTradeC2SPacket;

pub(super) mod c2s_0x32_set_beacon;
pub use c2s_0x32_set_beacon::UpdateBeaconC2SPacket;

pub(super) mod c2s_0x33_set_carried_item;
pub use c2s_0x33_set_carried_item::UpdateSelectedSlotC2SPacket;

pub(super) mod c2s_0x34_set_command_block;
pub use c2s_0x34_set_command_block::UpdateCommandBlockC2SPacket;

pub(super) mod c2s_0x35_set_command_minecart;
pub use c2s_0x35_set_command_minecart::UpdateCommandBlockMinecartC2SPacket;

pub(super) mod c2s_0x36_set_creative_mode_slot;
pub use c2s_0x36_set_creative_mode_slot::CreativeInventoryActionC2SPacket;

pub(super) mod c2s_0x37_set_jigsaw_block;
pub use c2s_0x37_set_jigsaw_block::UpdateJigsawC2SPacket;

pub(super) mod c2s_0x38_set_structure_block;
pub use c2s_0x38_set_structure_block::UpdateStructureBlockC2SPacket;

pub(super) mod c2s_0x39_sign_update;
pub use c2s_0x39_sign_update::UpdateSignC2SPacket;

pub(super) mod c2s_0x3a_swing;
pub use c2s_0x3a_swing::HandSwingC2SPacket;

pub(super) mod c2s_0x3b_teleport_to_entity;
pub use c2s_0x3b_teleport_to_entity::SpectatorTeleportC2SPacket;

pub(super) mod c2s_0x3c_use_item_on;
pub use c2s_0x3c_use_item_on::PlayerInteractBlockC2SPacket;

pub(super) mod c2s_0x3d_use_item;
pub use c2s_0x3d_use_item::PlayerInteractItemC2SPacket;

pub(super) mod s2c_0x00_bundle;
pub use s2c_0x00_bundle::BundleDelimiterS2CPacket;

pub(super) mod s2c_0x01_add_entity;
pub use s2c_0x01_add_entity::EntitySpawnS2CPacket;

pub(super) mod s2c_0x02_add_experience_orb;
pub use s2c_0x02_add_experience_orb::ExperienceOrbSpawnS2CPacket;

pub(super) mod s2c_0x03_animate;
pub use s2c_0x03_animate::EntityAnimationS2CPacket;

pub(super) mod s2c_0x04_award_stats;
pub use s2c_0x04_award_stats::StatisticsS2CPacket;

pub(super) mod s2c_0x05_block_changed_ack;
pub use s2c_0x05_block_changed_ack::PlayerActionResponseS2CPacket;

pub(super) mod s2c_0x06_block_destruction;
pub use s2c_0x06_block_destruction::BlockBreakingProgressS2CPacket;

pub(super) mod s2c_0x07_block_entity_data;
pub use s2c_0x07_block_entity_data::BlockEntityUpdateS2CPacket;

pub(super) mod s2c_0x08_block_event;
pub use s2c_0x08_block_event::BlockEventS2CPacket;

pub(super) mod s2c_0x09_block_update;
pub use s2c_0x09_block_update::BlockUpdateS2CPacket;

pub(super) mod s2c_0x0a_boss_event;
pub use s2c_0x0a_boss_event::BossBarS2CPacket;

pub(super) mod s2c_0x0b_change_difficulty;
pub use s2c_0x0b_change_difficulty::DifficultyS2CPacket;

pub(super) mod s2c_0x0c_chunk_batch_finished;
pub use s2c_0x0c_chunk_batch_finished::ChunkSentS2CPacket;

pub(super) mod s2c_0x0d_chunk_batch_start;
pub use s2c_0x0d_chunk_batch_start::StartChunkSendS2CPacket;

pub(super) mod s2c_0x0e_chunks_biomes;
pub use s2c_0x0e_chunks_biomes::ChunkBiomeDataS2CPacket;

pub(super) mod s2c_0x0f_clear_titles;
pub use s2c_0x0f_clear_titles::ClearTitleS2CPacket;

pub(super) mod s2c_0x10_command_suggestions;
pub use s2c_0x10_command_suggestions::CommandSuggestionsS2CPacket;

pub(super) mod s2c_0x11_commands;
pub use s2c_0x11_commands::CommandTreeS2CPacket;

pub(super) mod s2c_0x12_container_close;
pub use s2c_0x12_container_close::CloseScreenS2CPacket;

pub(super) mod s2c_0x13_container_set_content;
pub use s2c_0x13_container_set_content::InventoryS2CPacket;

pub(super) mod s2c_0x14_container_set_data;
pub use s2c_0x14_container_set_data::ScreenHandlerPropertyUpdateS2CPacket;

pub(super) mod s2c_0x15_container_set_slot;
pub use s2c_0x15_container_set_slot::ScreenHandlerSlotUpdateS2CPacket;

pub(super) mod s2c_0x16_cookie_request;
pub use s2c_0x16_cookie_request::CookieRequestS2CPacket;

pub(super) mod s2c_0x17_cooldown;
pub use s2c_0x17_cooldown::CooldownUpdateS2CPacket;

pub(super) mod s2c_0x18_custom_chat_completions;
pub use s2c_0x18_custom_chat_completions::ChatSuggestionsS2CPacket;

pub(super) mod s2c_0x19_custom_payload;
pub use s2c_0x19_custom_payload::CustomPayloadS2CPacket;

pub(super) mod s2c_0x1a_damage_event;
pub use s2c_0x1a_damage_event::EntityDamageS2CPacket;

pub(super) mod s2c_0x1b_debug_sample;
pub use s2c_0x1b_debug_sample::DebugSampleS2CPacket;

pub(super) mod s2c_0x1c_delete_chat;
pub use s2c_0x1c_delete_chat::RemoveMessageS2CPacket;

pub(super) mod s2c_0x1d_disconnect;
pub use s2c_0x1d_disconnect::DisconnectS2CPacket;

pub(super) mod s2c_0x1e_disguised_chat;
pub use s2c_0x1e_disguised_chat::ProfilelessChatMessageS2CPacket;

pub(super) mod s2c_0x1f_entity_event;
pub use s2c_0x1f_entity_event::EntityStatusS2CPacket;

pub(super) mod s2c_0x20_entity_position_sync;
pub use s2c_0x20_entity_position_sync::EntityPositionSyncS2CPacket;

pub(super) mod s2c_0x21_explode;
pub use s2c_0x21_explode::ExplosionS2CPacket;

pub(super) mod s2c_0x22_forget_level_chunk;
pub use s2c_0x22_forget_level_chunk::UnloadChunkS2CPacket;

pub(super) mod s2c_0x23_game_event;
pub use s2c_0x23_game_event::GameStateChangeS2CPacket;

pub(super) mod s2c_0x24_horse_screen_open;
pub use s2c_0x24_horse_screen_open::OpenHorseScreenS2CPacket;

pub(super) mod s2c_0x25_hurt_animation;
pub use s2c_0x25_hurt_animation::DamageTiltS2CPacket;

pub(super) mod s2c_0x26_initialize_border;
pub use s2c_0x26_initialize_border::WorldBorderInitializeS2CPacket;

pub(super) mod s2c_0x27_keep_alive;
pub use s2c_0x27_keep_alive::KeepAliveS2CPacket;

pub(super) mod s2c_0x28_level_chunk_with_light;
pub use s2c_0x28_level_chunk_with_light::ChunkDataS2CPacket;

pub(super) mod s2c_0x29_level_event;
pub use s2c_0x29_level_event::WorldEventS2CPacket;

pub(super) mod s2c_0x2a_level_particles;
pub use s2c_0x2a_level_particles::ParticleS2CPacket;

pub(super) mod s2c_0x2b_light_update;
pub use s2c_0x2b_light_update::LightUpdateS2CPacket;

pub(super) mod s2c_0x2c_login;
pub use s2c_0x2c_login::GameJoinS2CPacket;

pub(super) mod s2c_0x2d_map_item_data;
pub use s2c_0x2d_map_item_data::MapUpdateS2CPacket;

pub(super) mod s2c_0x2e_merchant_offers;
pub use s2c_0x2e_merchant_offers::SetTradeOffersS2CPacket;

pub(super) mod s2c_0x2f_move_entity_pos;
pub use s2c_0x2f_move_entity_pos::EntityMoveRelativeS2CPacket;

pub(super) mod s2c_0x30_move_entity_pos_rot;
pub use s2c_0x30_move_entity_pos_rot::EntityRotateAndMoveRelativeS2CPacket;

pub(super) mod s2c_0x31_move_minecart_along_track;
pub use s2c_0x31_move_minecart_along_track::MoveMinecartAlongTrackS2CPacket;

pub(super) mod s2c_0x32_move_entity_rot;
pub use s2c_0x32_move_entity_rot::EntityRotateS2CPacket;

pub(super) mod s2c_0x33_move_vehicle;
pub use s2c_0x33_move_vehicle::VehicleMoveS2CPacket;

pub(super) mod s2c_0x34_open_book;
pub use s2c_0x34_open_book::OpenWrittenBookS2CPacket;

pub(super) mod s2c_0x35_open_screen;
pub use s2c_0x35_open_screen::OpenScreenS2CPacket;

pub(super) mod s2c_0x36_open_sign_editor;
pub use s2c_0x36_open_sign_editor::SignEditorOpenS2CPacket;

pub(super) mod s2c_0x37_ping;
pub use s2c_0x37_ping::CommonPingS2CPacket;

pub(super) mod s2c_0x38_pong_response;
pub use s2c_0x38_pong_response::PingResultS2CPacket;

pub(super) mod s2c_0x39_place_ghost_recipe;
pub use s2c_0x39_place_ghost_recipe::CraftFailedResponseS2CPacket;

pub(super) mod s2c_0x3a_player_abilities;
pub use s2c_0x3a_player_abilities::PlayerAbilitiesS2CPacket;

pub(super) mod s2c_0x3b_player_chat;
pub use s2c_0x3b_player_chat::ChatMessageS2CPacket;

pub(super) mod s2c_0x3c_player_combat_end;
pub use s2c_0x3c_player_combat_end::EndCombatS2CPacket;

pub(super) mod s2c_0x3d_player_combat_enter;
pub use s2c_0x3d_player_combat_enter::EnterCombatS2CPacket;

pub(super) mod s2c_0x3e_player_combat_kill;
pub use s2c_0x3e_player_combat_kill::DeathMessageS2CPacket;

pub(super) mod s2c_0x3f_player_info_remove;
pub use s2c_0x3f_player_info_remove::PlayerRemoveS2CPacket;

pub(super) mod s2c_0x40_player_info_update;
pub use s2c_0x40_player_info_update::PlayerListS2CPacket;

pub(super) mod s2c_0x41_player_look_at;
pub use s2c_0x41_player_look_at::LookAtS2CPacket;

pub(super) mod s2c_0x42_player_position;
pub use s2c_0x42_player_position::PlayerPositionLookS2CPacket;

pub(super) mod s2c_0x43_player_rotation;
pub use s2c_0x43_player_rotation::PlayerRotationS2CPacket;

pub(super) mod s2c_0x44_recipe_book_add;
pub use s2c_0x44_recipe_book_add::RecipeBookAddS2CPacket;

pub(super) mod s2c_0x45_recipe_book_remove;
pub use s2c_0x45_recipe_book_remove::RecipeBookRemoveS2CPacket;

pub(super) mod s2c_0x46_recipe_book_settings;
pub use s2c_0x46_recipe_book_settings::RecipeBookSettingsS2CPacket;

pub(super) mod s2c_0x47_remove_entities;
pub use s2c_0x47_remove_entities::EntitiesDestroyS2CPacket;

pub(super) mod s2c_0x48_remove_mob_effect;
pub use s2c_0x48_remove_mob_effect::RemoveEntityStatusEffectS2CPacket;

pub(super) mod s2c_0x49_reset_score;
pub use s2c_0x49_reset_score::ScoreboardScoreResetS2CPacket;

pub(super) mod s2c_0x4a_resource_pack_pop;
pub use s2c_0x4a_resource_pack_pop::ResourcePackRemoveS2CPacket;

pub(super) mod s2c_0x4b_resource_pack_push;
pub use s2c_0x4b_resource_pack_push::ResourcePackSendS2CPacket;

pub(super) mod s2c_0x4c_respawn;
pub use s2c_0x4c_respawn::PlayerRespawnS2CPacket;

pub(super) mod s2c_0x4d_rotate_head;
pub use s2c_0x4d_rotate_head::EntitySetHeadYawS2CPacket;

pub(super) mod s2c_0x4e_section_blocks_update;
pub use s2c_0x4e_section_blocks_update::ChunkDeltaUpdateS2CPacket;

pub(super) mod s2c_0x4f_select_advancements_tab;
pub use s2c_0x4f_select_advancements_tab::SelectAdvancementTabS2CPacket;

pub(super) mod s2c_0x50_server_data;
pub use s2c_0x50_server_data::ServerMetadataS2CPacket;

pub(super) mod s2c_0x51_set_action_bar_text;
pub use s2c_0x51_set_action_bar_text::OverlayMessageS2CPacket;

pub(super) mod s2c_0x52_set_border_center;
pub use s2c_0x52_set_border_center::WorldBorderCenterChangedS2CPacket;

pub(super) mod s2c_0x53_set_border_lerp_size;
pub use s2c_0x53_set_border_lerp_size::WorldBorderInterpolateSizeS2CPacket;

pub(super) mod s2c_0x54_set_border_size;
pub use s2c_0x54_set_border_size::WorldBorderSizeChangedS2CPacket;

pub(super) mod s2c_0x55_set_border_warning_delay;
pub use s2c_0x55_set_border_warning_delay::WorldBorderWarningTimeChangedS2CPacket;

pub(super) mod s2c_0x56_set_border_warning_distance;
pub use s2c_0x56_set_border_warning_distance::WorldBorderWarningBlocksChangedS2CPacket;

pub(super) mod s2c_0x57_set_camera;
pub use s2c_0x57_set_camera::SetCameraEntityS2CPacket;

pub(super) mod s2c_0x58_set_chunk_cache_center;
pub use s2c_0x58_set_chunk_cache_center::ChunkRenderDistanceCenterS2CPacket;

pub(super) mod s2c_0x59_set_chunk_cache_radius;
pub use s2c_0x59_set_chunk_cache_radius::ChunkLoadDistanceS2CPacket;

pub(super) mod s2c_0x5a_set_cursor_item;
pub use s2c_0x5a_set_cursor_item::SetCursorItemS2CPacket;

pub(super) mod s2c_0x5b_set_default_spawn_position;
pub use s2c_0x5b_set_default_spawn_position::PlayerSpawnPositionS2CPacket;

pub(super) mod s2c_0x5c_set_display_objective;
pub use s2c_0x5c_set_display_objective::ScoreboardDisplayS2CPacket;

pub(super) mod s2c_0x5d_set_entity_data;
pub use s2c_0x5d_set_entity_data::EntityTrackerUpdateS2CPacket;

pub(super) mod s2c_0x5e_set_entity_link;
pub use s2c_0x5e_set_entity_link::EntityAttachS2CPacket;

pub(super) mod s2c_0x5f_set_entity_motion;
pub use s2c_0x5f_set_entity_motion::EntityVelocityUpdateS2CPacket;

pub(super) mod s2c_0x60_set_equipment;
pub use s2c_0x60_set_equipment::EntityEquipmentUpdateS2CPacket;

pub(super) mod s2c_0x61_set_experience;
pub use s2c_0x61_set_experience::ExperienceBarUpdateS2CPacket;

pub(super) mod s2c_0x62_set_health;
pub use s2c_0x62_set_health::HealthUpdateS2CPacket;

pub(super) mod s2c_0x63_set_held_slot;
pub use s2c_0x63_set_held_slot::UpdateSelectedSlotS2CPacket;

pub(super) mod s2c_0x64_set_objective;
pub use s2c_0x64_set_objective::ScoreboardObjectiveUpdateS2CPacket;

pub(super) mod s2c_0x65_set_passengers;
pub use s2c_0x65_set_passengers::EntityPassengersSetS2CPacket;

pub(super) mod s2c_0x66_set_player_inventory;
pub use s2c_0x66_set_player_inventory::SetPlayerInventoryS2CPacket;

pub(super) mod s2c_0x67_set_player_team;
pub use s2c_0x67_set_player_team::TeamS2CPacket;

pub(super) mod s2c_0x68_set_score;
pub use s2c_0x68_set_score::ScoreboardScoreUpdateS2CPacket;

pub(super) mod s2c_0x69_set_simulation_distance;
pub use s2c_0x69_set_simulation_distance::SimulationDistanceS2CPacket;

pub(super) mod s2c_0x6a_set_subtitle_text;
pub use s2c_0x6a_set_subtitle_text::SubtitleS2CPacket;

pub(super) mod s2c_0x6b_set_time;
pub use s2c_0x6b_set_time::WorldTimeUpdateS2CPacket;

pub(super) mod s2c_0x6c_set_title_text;
pub use s2c_0x6c_set_title_text::TitleS2CPacket;

pub(super) mod s2c_0x6d_set_titles_animation;
pub use s2c_0x6d_set_titles_animation::TitleFadeS2CPacket;

pub(super) mod s2c_0x6e_sound_entity;
pub use s2c_0x6e_sound_entity::PlaySoundFromEntityS2CPacket;

pub(super) mod s2c_0x6f_sound;
pub use s2c_0x6f_sound::PlaySoundS2CPacket;

pub(super) mod s2c_0x70_start_configuration;
pub use s2c_0x70_start_configuration::EnterReconfigurationS2CPacket;

pub(super) mod s2c_0x71_stop_sound;
pub use s2c_0x71_stop_sound::StopSoundS2CPacket;

pub(super) mod s2c_0x72_store_cookie;
pub use s2c_0x72_store_cookie::StoreCookieS2CPacket;

pub(super) mod s2c_0x73_system_chat;
pub use s2c_0x73_system_chat::GameMessageS2CPacket;

pub(super) mod s2c_0x74_tab_list;
pub use s2c_0x74_tab_list::PlayerListHeaderS2CPacket;

pub(super) mod s2c_0x75_tag_query;
pub use s2c_0x75_tag_query::NbtQueryResponseS2CPacket;

pub(super) mod s2c_0x76_take_item_entity;
pub use s2c_0x76_take_item_entity::ItemPickupAnimationS2CPacket;

pub(super) mod s2c_0x77_teleport_entity;
pub use s2c_0x77_teleport_entity::EntityPositionS2CPacket;

pub(super) mod s2c_0x78_ticking_state;
pub use s2c_0x78_ticking_state::UpdateTickRateS2CPacket;

pub(super) mod s2c_0x79_ticking_step;
pub use s2c_0x79_ticking_step::TickStepS2CPacket;

pub(super) mod s2c_0x7a_transfer;
pub use s2c_0x7a_transfer::ServerTransferS2CPacket;

pub(super) mod s2c_0x7b_update_advancements;
pub use s2c_0x7b_update_advancements::AdvancementUpdateS2CPacket;

pub(super) mod s2c_0x7c_update_attributes;
pub use s2c_0x7c_update_attributes::EntityAttributesS2CPacket;

pub(super) mod s2c_0x7d_update_mob_effect;
pub use s2c_0x7d_update_mob_effect::EntityStatusEffectS2CPacket;

pub(super) mod s2c_0x7e_update_recipes;
pub use s2c_0x7e_update_recipes::SynchronizeRecipesS2CPacket;

pub(super) mod s2c_0x7f_update_tags;
pub use s2c_0x7f_update_tags::SynchronizeTagsS2CPacket;

pub(super) mod s2c_0x80_projectile_power;
pub use s2c_0x80_projectile_power::ProjectilePowerS2CPacket;

pub(super) mod s2c_0x81_custom_report_details;
pub use s2c_0x81_custom_report_details::CustomReportDetailsS2CPacket;

pub(super) mod s2c_0x82_server_links;
pub use s2c_0x82_server_links::ServerLinksS2CPacket;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundPlayPackets {
    TeleportConfirm(TeleportConfirmC2SPacket) = 0x00,
    QueryBlockNbt(QueryBlockNbtC2SPacket) = 0x01,
    BundleItemSelected(BundleItemSelectedC2SPacket) = 0x02,
    UpdateDifficulty(UpdateDifficultyC2SPacket) = 0x03,
    MessageAcknowledgment(MessageAcknowledgmentC2SPacket) = 0x04,
    CommandExecution(CommandExecutionC2SPacket) = 0x05,
    ChatCommandSigned(ChatCommandSignedC2SPacket) = 0x06,
    ChatMessage(ChatMessageC2SPacket) = 0x07,
    PlayerSession(PlayerSessionC2SPacket) = 0x08,
    AcknowledgeChunks(AcknowledgeChunksC2SPacket) = 0x09,
    ClientStatus(ClientStatusC2SPacket) = 0x0a,
    ClientTickEnd(ClientTickEndC2SPacket) = 0x0b,
    ClientOptions(ClientOptionsC2SPacket) = 0x0c,
    RequestCommandCompletions(RequestCommandCompletionsC2SPacket) = 0x0d,
    AcknowledgeReconfiguration(AcknowledgeReconfigurationC2SPacket) = 0x0e,
    ButtonClick(ButtonClickC2SPacket) = 0x0f,
    ClickSlot(ClickSlotC2SPacket) = 0x10,
    CloseHandledScreen(CloseHandledScreenC2SPacket) = 0x11,
    SlotChangedState(SlotChangedStateC2SPacket) = 0x12,
    CookieResponse(CookieResponseC2SPacket) = 0x13,
    CustomPayload(CustomPayloadC2SPacket) = 0x14,
    DebugSampleSubscription(DebugSampleSubscriptionC2SPacket) = 0x15,
    BookUpdate(BookUpdateC2SPacket) = 0x16,
    QueryEntityNbt(QueryEntityNbtC2SPacket) = 0x17,
    PlayerInteractEntity(PlayerInteractEntityC2SPacket) = 0x18,
    JigsawGenerating(JigsawGeneratingC2SPacket) = 0x19,
    KeepAlive(KeepAliveC2SPacket) = 0x1a,
    UpdateDifficultyLock(UpdateDifficultyLockC2SPacket) = 0x1b,
    PlayerPositionAndOnGround(PlayerPositionAndOnGroundC2SPacket) = 0x1c,
    PlayerFull(PlayerFullC2SPacket) = 0x1d,
    PlayerLookAndOnGround(PlayerLookAndOnGroundC2SPacket) = 0x1e,
    PlayerOnGroundOnly(PlayerOnGroundOnlyC2SPacket) = 0x1f,
    VehicleMove(VehicleMoveC2SPacket) = 0x20,
    BoatPaddleState(BoatPaddleStateC2SPacket) = 0x21,
    PickItemFromBlock(PickItemFromBlockC2SPacket) = 0x22,
    PickItemFromEntity(PickItemFromEntityC2SPacket) = 0x23,
    QueryPing(QueryPingC2SPacket) = 0x24,
    CraftRequest(CraftRequestC2SPacket) = 0x25,
    UpdatePlayerAbilities(UpdatePlayerAbilitiesC2SPacket) = 0x26,
    PlayerAction(PlayerActionC2SPacket) = 0x27,
    ClientCommand(ClientCommandC2SPacket) = 0x28,
    PlayerInput(PlayerInputC2SPacket) = 0x29,
    PlayerLoaded(PlayerLoadedC2SPacket) = 0x2a,
    CommonPong(CommonPongC2SPacket) = 0x2b,
    RecipeCategoryOptions(RecipeCategoryOptionsC2SPacket) = 0x2c,
    RecipeBookData(RecipeBookDataC2SPacket) = 0x2d,
    RenameItem(RenameItemC2SPacket) = 0x2e,
    ResourcePackStatus(ResourcePackStatusC2SPacket) = 0x2f,
    AdvancementTab(AdvancementTabC2SPacket) = 0x30,
    SelectMerchantTrade(SelectMerchantTradeC2SPacket) = 0x31,
    UpdateBeacon(UpdateBeaconC2SPacket) = 0x32,
    UpdateSelectedSlot(UpdateSelectedSlotC2SPacket) = 0x33,
    UpdateCommandBlock(UpdateCommandBlockC2SPacket) = 0x34,
    UpdateCommandBlockMinecart(UpdateCommandBlockMinecartC2SPacket) = 0x35,
    CreativeInventoryAction(CreativeInventoryActionC2SPacket) = 0x36,
    UpdateJigsaw(UpdateJigsawC2SPacket) = 0x37,
    UpdateStructureBlock(UpdateStructureBlockC2SPacket) = 0x38,
    UpdateSign(UpdateSignC2SPacket) = 0x39,
    HandSwing(HandSwingC2SPacket) = 0x3a,
    SpectatorTeleport(SpectatorTeleportC2SPacket) = 0x3b,
    PlayerInteractBlock(PlayerInteractBlockC2SPacket) = 0x3c,
    PlayerInteractItem(PlayerInteractItemC2SPacket) = 0x3d,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundPlayPackets {
    BundleDelimiter(BundleDelimiterS2CPacket) = 0x00,
    EntitySpawn(EntitySpawnS2CPacket) = 0x01,
    ExperienceOrbSpawn(ExperienceOrbSpawnS2CPacket) = 0x02,
    EntityAnimation(EntityAnimationS2CPacket) = 0x03,
    Statistics(StatisticsS2CPacket) = 0x04,
    PlayerActionResponse(PlayerActionResponseS2CPacket) = 0x05,
    BlockBreakingProgress(BlockBreakingProgressS2CPacket) = 0x06,
    BlockEntityUpdate(BlockEntityUpdateS2CPacket) = 0x07,
    BlockEvent(BlockEventS2CPacket) = 0x08,
    BlockUpdate(BlockUpdateS2CPacket) = 0x09,
    BossBar(BossBarS2CPacket) = 0x0a,
    Difficulty(DifficultyS2CPacket) = 0x0b,
    ChunkSent(ChunkSentS2CPacket) = 0x0c,
    StartChunkSend(StartChunkSendS2CPacket) = 0x0d,
    ChunkBiomeData(ChunkBiomeDataS2CPacket) = 0x0e,
    ClearTitle(ClearTitleS2CPacket) = 0x0f,
    CommandSuggestions(CommandSuggestionsS2CPacket) = 0x10,
    CommandTree(CommandTreeS2CPacket) = 0x11,
    CloseScreen(CloseScreenS2CPacket) = 0x12,
    Inventory(InventoryS2CPacket) = 0x13,
    ScreenHandlerPropertyUpdate(ScreenHandlerPropertyUpdateS2CPacket) = 0x14,
    ScreenHandlerSlotUpdate(ScreenHandlerSlotUpdateS2CPacket) = 0x15,
    CookieRequest(CookieRequestS2CPacket) = 0x16,
    CooldownUpdate(CooldownUpdateS2CPacket) = 0x17,
    ChatSuggestions(ChatSuggestionsS2CPacket) = 0x18,
    CustomPayload(CustomPayloadS2CPacket) = 0x19,
    EntityDamage(EntityDamageS2CPacket) = 0x1a,
    DebugSample(DebugSampleS2CPacket) = 0x1b,
    RemoveMessage(RemoveMessageS2CPacket) = 0x1c,
    Disconnect(DisconnectS2CPacket) = 0x1d,
    ProfilelessChatMessage(ProfilelessChatMessageS2CPacket) = 0x1e,
    EntityStatus(EntityStatusS2CPacket) = 0x1f,
    EntityPositionSync(EntityPositionSyncS2CPacket) = 0x20,
    Explosion(ExplosionS2CPacket) = 0x21,
    UnloadChunk(UnloadChunkS2CPacket) = 0x22,
    GameStateChange(GameStateChangeS2CPacket) = 0x23,
    OpenHorseScreen(OpenHorseScreenS2CPacket) = 0x24,
    DamageTilt(DamageTiltS2CPacket) = 0x25,
    WorldBorderInitialize(WorldBorderInitializeS2CPacket) = 0x26,
    KeepAlive(KeepAliveS2CPacket) = 0x27,
    ChunkData(ChunkDataS2CPacket) = 0x28,
    WorldEvent(WorldEventS2CPacket) = 0x29,
    Particle(ParticleS2CPacket) = 0x2a,
    LightUpdate(LightUpdateS2CPacket) = 0x2b,
    GameJoin(GameJoinS2CPacket) = 0x2c,
    MapUpdate(MapUpdateS2CPacket) = 0x2d,
    SetTradeOffers(SetTradeOffersS2CPacket) = 0x2e,
    EntityMoveRelative(EntityMoveRelativeS2CPacket) = 0x2f,
    EntityRotateAndMoveRelative(EntityRotateAndMoveRelativeS2CPacket) = 0x30,
    MoveMinecartAlongTrack(MoveMinecartAlongTrackS2CPacket) = 0x31,
    EntityRotate(EntityRotateS2CPacket) = 0x32,
    VehicleMove(VehicleMoveS2CPacket) = 0x33,
    OpenWrittenBook(OpenWrittenBookS2CPacket) = 0x34,
    OpenScreen(OpenScreenS2CPacket) = 0x35,
    SignEditorOpen(SignEditorOpenS2CPacket) = 0x36,
    CommonPing(CommonPingS2CPacket) = 0x37,
    PingResult(PingResultS2CPacket) = 0x38,
    CraftFailedResponse(CraftFailedResponseS2CPacket) = 0x39,
    PlayerAbilities(PlayerAbilitiesS2CPacket) = 0x3a,
    ChatMessage(ChatMessageS2CPacket) = 0x3b,
    EndCombat(EndCombatS2CPacket) = 0x3c,
    EnterCombat(EnterCombatS2CPacket) = 0x3d,
    DeathMessage(DeathMessageS2CPacket) = 0x3e,
    PlayerRemove(PlayerRemoveS2CPacket) = 0x3f,
    PlayerList(PlayerListS2CPacket) = 0x40,
    LookAt(LookAtS2CPacket) = 0x41,
    PlayerPositionLook(PlayerPositionLookS2CPacket) = 0x42,
    PlayerRotation(PlayerRotationS2CPacket) = 0x43,
    RecipeBookAdd(RecipeBookAddS2CPacket) = 0x44,
    RecipeBookRemove(RecipeBookRemoveS2CPacket) = 0x45,
    RecipeBookSettings(RecipeBookSettingsS2CPacket) = 0x46,
    EntitiesDestroy(EntitiesDestroyS2CPacket) = 0x47,
    RemoveEntityStatusEffect(RemoveEntityStatusEffectS2CPacket) = 0x48,
    ScoreboardScoreReset(ScoreboardScoreResetS2CPacket) = 0x49,
    ResourcePackRemove(ResourcePackRemoveS2CPacket) = 0x4a,
    ResourcePackSend(ResourcePackSendS2CPacket) = 0x4b,
    PlayerRespawn(PlayerRespawnS2CPacket) = 0x4c,
    EntitySetHeadYaw(EntitySetHeadYawS2CPacket) = 0x4d,
    ChunkDeltaUpdate(ChunkDeltaUpdateS2CPacket) = 0x4e,
    SelectAdvancementTab(SelectAdvancementTabS2CPacket) = 0x4f,
    ServerMetadata(ServerMetadataS2CPacket) = 0x50,
    OverlayMessage(OverlayMessageS2CPacket) = 0x51,
    WorldBorderCenterChanged(WorldBorderCenterChangedS2CPacket) = 0x52,
    WorldBorderInterpolateSize(WorldBorderInterpolateSizeS2CPacket) = 0x53,
    WorldBorderSizeChanged(WorldBorderSizeChangedS2CPacket) = 0x54,
    WorldBorderWarningTimeChanged(WorldBorderWarningTimeChangedS2CPacket) = 0x55,
    WorldBorderWarningBlocksChanged(WorldBorderWarningBlocksChangedS2CPacket) = 0x56,
    SetCameraEntity(SetCameraEntityS2CPacket) = 0x57,
    ChunkRenderDistanceCenter(ChunkRenderDistanceCenterS2CPacket) = 0x58,
    ChunkLoadDistance(ChunkLoadDistanceS2CPacket) = 0x59,
    SetCursorItem(SetCursorItemS2CPacket) = 0x5a,
    PlayerSpawnPosition(PlayerSpawnPositionS2CPacket) = 0x5b,
    ScoreboardDisplay(ScoreboardDisplayS2CPacket) = 0x5c,
    EntityTrackerUpdate(EntityTrackerUpdateS2CPacket) = 0x5d,
    EntityAttach(EntityAttachS2CPacket) = 0x5e,
    EntityVelocityUpdate(EntityVelocityUpdateS2CPacket) = 0x5f,
    EntityEquipmentUpdate(EntityEquipmentUpdateS2CPacket) = 0x60,
    ExperienceBarUpdate(ExperienceBarUpdateS2CPacket) = 0x61,
    HealthUpdate(HealthUpdateS2CPacket) = 0x62,
    UpdateSelectedSlot(UpdateSelectedSlotS2CPacket) = 0x63,
    ScoreboardObjectiveUpdate(ScoreboardObjectiveUpdateS2CPacket) = 0x64,
    EntityPassengersSet(EntityPassengersSetS2CPacket) = 0x65,
    SetPlayerInventory(SetPlayerInventoryS2CPacket) = 0x66,
    Team(TeamS2CPacket) = 0x67,
    ScoreboardScoreUpdate(ScoreboardScoreUpdateS2CPacket) = 0x68,
    SimulationDistance(SimulationDistanceS2CPacket) = 0x69,
    Subtitle(SubtitleS2CPacket) = 0x6a,
    WorldTimeUpdate(WorldTimeUpdateS2CPacket) = 0x6b,
    Title(TitleS2CPacket) = 0x6c,
    TitleFade(TitleFadeS2CPacket) = 0x6d,
    PlaySoundFromEntity(PlaySoundFromEntityS2CPacket) = 0x6e,
    PlaySound(PlaySoundS2CPacket) = 0x6f,
    EnterReconfiguration(EnterReconfigurationS2CPacket) = 0x70,
    StopSound(StopSoundS2CPacket) = 0x71,
    StoreCookie(StoreCookieS2CPacket) = 0x72,
    GameMessage(GameMessageS2CPacket) = 0x73,
    PlayerListHeader(PlayerListHeaderS2CPacket) = 0x74,
    NbtQueryResponse(NbtQueryResponseS2CPacket) = 0x75,
    ItemPickupAnimation(ItemPickupAnimationS2CPacket) = 0x76,
    EntityPosition(EntityPositionS2CPacket) = 0x77,
    UpdateTickRate(UpdateTickRateS2CPacket) = 0x78,
    TickStep(TickStepS2CPacket) = 0x79,
    ServerTransfer(ServerTransferS2CPacket) = 0x7a,
    AdvancementUpdate(AdvancementUpdateS2CPacket) = 0x7b,
    EntityAttributes(EntityAttributesS2CPacket) = 0x7c,
    EntityStatusEffect(EntityStatusEffectS2CPacket) = 0x7d,
    SynchronizeRecipes(SynchronizeRecipesS2CPacket) = 0x7e,
    SynchronizeTags(SynchronizeTagsS2CPacket) = 0x7f,
    ProjectilePower(ProjectilePowerS2CPacket) = 0x80,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x81,
    ServerLinks(ServerLinksS2CPacket) = 0x82,
}
