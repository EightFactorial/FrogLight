//! @generated [`Play`](crate::version::Play) packets for v26.1

mod s2c_0x00_bundle_delimiter;
pub use s2c_0x00_bundle_delimiter::BundleDelimiterS2CPacket;

mod s2c_0x01_add_entity;
pub use s2c_0x01_add_entity::AddEntityS2CPacket;

mod s2c_0x02_animate;
pub use s2c_0x02_animate::AnimateS2CPacket;

mod s2c_0x03_award_stats;
pub use s2c_0x03_award_stats::AwardStatsS2CPacket;

mod s2c_0x04_block_changed_ack;
pub use s2c_0x04_block_changed_ack::BlockChangedAckS2CPacket;

mod s2c_0x05_block_destruction;
pub use s2c_0x05_block_destruction::BlockDestructionS2CPacket;

mod s2c_0x06_block_entity_data;
pub use s2c_0x06_block_entity_data::BlockEntityDataS2CPacket;

mod s2c_0x07_block_event;
pub use s2c_0x07_block_event::BlockEventS2CPacket;

mod s2c_0x08_block_update;
pub use s2c_0x08_block_update::BlockUpdateS2CPacket;

mod s2c_0x09_boss_event;
pub use s2c_0x09_boss_event::BossEventS2CPacket;

mod s2c_0x0a_change_difficulty;
pub use s2c_0x0a_change_difficulty::ChangeDifficultyS2CPacket;

mod s2c_0x0b_chunk_batch_finished;
pub use s2c_0x0b_chunk_batch_finished::ChunkBatchFinishedS2CPacket;

mod s2c_0x0c_chunk_batch_start;
pub use s2c_0x0c_chunk_batch_start::ChunkBatchStartS2CPacket;

mod s2c_0x0d_chunks_biomes;
pub use s2c_0x0d_chunks_biomes::ChunksBiomesS2CPacket;

mod s2c_0x0e_clear_titles;
pub use s2c_0x0e_clear_titles::ClearTitlesS2CPacket;

mod s2c_0x0f_command_suggestions;
pub use s2c_0x0f_command_suggestions::CommandSuggestionsS2CPacket;

mod s2c_0x10_commands;
pub use s2c_0x10_commands::CommandsS2CPacket;

mod s2c_0x11_container_close;
pub use s2c_0x11_container_close::ContainerCloseS2CPacket;

mod s2c_0x12_container_set_content;
pub use s2c_0x12_container_set_content::ContainerSetContentS2CPacket;

mod s2c_0x13_container_set_data;
pub use s2c_0x13_container_set_data::ContainerSetDataS2CPacket;

mod s2c_0x14_container_set_slot;
pub use s2c_0x14_container_set_slot::ContainerSetSlotS2CPacket;

mod s2c_0x15_cookie_request;
pub use s2c_0x15_cookie_request::CookieRequestS2CPacket;

mod s2c_0x16_cooldown;
pub use s2c_0x16_cooldown::CooldownS2CPacket;

mod s2c_0x17_custom_chat_completions;
pub use s2c_0x17_custom_chat_completions::CustomChatCompletionsS2CPacket;

mod s2c_0x18_custom_payload;
pub use s2c_0x18_custom_payload::CustomPayloadS2CPacket;

mod s2c_0x19_damage_event;
pub use s2c_0x19_damage_event::DamageEventS2CPacket;

mod s2c_0x1a_debug_block_value;
pub use s2c_0x1a_debug_block_value::DebugBlockValueS2CPacket;

mod s2c_0x1b_debug_chunk_value;
pub use s2c_0x1b_debug_chunk_value::DebugChunkValueS2CPacket;

mod s2c_0x1c_debug_entity_value;
pub use s2c_0x1c_debug_entity_value::DebugEntityValueS2CPacket;

mod s2c_0x1d_debug_event;
pub use s2c_0x1d_debug_event::DebugEventS2CPacket;

mod s2c_0x1e_debug_sample;
pub use s2c_0x1e_debug_sample::DebugSampleS2CPacket;

mod s2c_0x1f_delete_chat;
pub use s2c_0x1f_delete_chat::DeleteChatS2CPacket;

mod s2c_0x20_disconnect;
pub use s2c_0x20_disconnect::DisconnectS2CPacket;

mod s2c_0x21_disguised_chat;
pub use s2c_0x21_disguised_chat::DisguisedChatS2CPacket;

mod s2c_0x22_entity_event;
pub use s2c_0x22_entity_event::EntityEventS2CPacket;

mod s2c_0x23_entity_position_sync;
pub use s2c_0x23_entity_position_sync::EntityPositionSyncS2CPacket;

mod s2c_0x24_explode;
pub use s2c_0x24_explode::ExplodeS2CPacket;

mod s2c_0x25_forget_level_chunk;
pub use s2c_0x25_forget_level_chunk::ForgetLevelChunkS2CPacket;

mod s2c_0x26_game_event;
pub use s2c_0x26_game_event::GameEventS2CPacket;

mod s2c_0x27_game_rule_values;
pub use s2c_0x27_game_rule_values::GameRuleValuesS2CPacket;

mod s2c_0x28_game_test_highlight_pos;
pub use s2c_0x28_game_test_highlight_pos::GameTestHighlightPosS2CPacket;

mod s2c_0x29_mount_screen_open;
pub use s2c_0x29_mount_screen_open::MountScreenOpenS2CPacket;

mod s2c_0x2a_hurt_animation;
pub use s2c_0x2a_hurt_animation::HurtAnimationS2CPacket;

mod s2c_0x2b_initialize_border;
pub use s2c_0x2b_initialize_border::InitializeBorderS2CPacket;

mod s2c_0x2c_keep_alive;
pub use s2c_0x2c_keep_alive::KeepAliveS2CPacket;

mod s2c_0x2d_level_chunk_with_light;
pub use s2c_0x2d_level_chunk_with_light::LevelChunkWithLightS2CPacket;

mod s2c_0x2e_level_event;
pub use s2c_0x2e_level_event::LevelEventS2CPacket;

mod s2c_0x2f_level_particles;
pub use s2c_0x2f_level_particles::LevelParticlesS2CPacket;

mod s2c_0x30_light_update;
pub use s2c_0x30_light_update::LightUpdateS2CPacket;

mod s2c_0x31_login;
pub use s2c_0x31_login::LoginS2CPacket;

mod s2c_0x32_low_disk_space_warning;
pub use s2c_0x32_low_disk_space_warning::LowDiskSpaceWarningS2CPacket;

mod s2c_0x33_map_item_data;
pub use s2c_0x33_map_item_data::MapItemDataS2CPacket;

mod s2c_0x34_merchant_offers;
pub use s2c_0x34_merchant_offers::MerchantOffersS2CPacket;

mod s2c_0x35_move_entity_pos;
pub use s2c_0x35_move_entity_pos::MoveEntityPosS2CPacket;

mod s2c_0x36_move_entity_pos_rot;
pub use s2c_0x36_move_entity_pos_rot::MoveEntityPosRotS2CPacket;

mod s2c_0x37_move_minecart_along_track;
pub use s2c_0x37_move_minecart_along_track::MoveMinecartAlongTrackS2CPacket;

mod s2c_0x38_move_entity_rot;
pub use s2c_0x38_move_entity_rot::MoveEntityRotS2CPacket;

mod s2c_0x39_move_vehicle;
pub use s2c_0x39_move_vehicle::MoveVehicleS2CPacket;

mod s2c_0x3a_open_book;
pub use s2c_0x3a_open_book::OpenBookS2CPacket;

mod s2c_0x3b_open_screen;
pub use s2c_0x3b_open_screen::OpenScreenS2CPacket;

mod s2c_0x3c_open_sign_editor;
pub use s2c_0x3c_open_sign_editor::OpenSignEditorS2CPacket;

mod s2c_0x3d_ping;
pub use s2c_0x3d_ping::PingS2CPacket;

mod s2c_0x3e_pong_response;
pub use s2c_0x3e_pong_response::PongResponseS2CPacket;

mod s2c_0x3f_place_ghost_recipe;
pub use s2c_0x3f_place_ghost_recipe::PlaceGhostRecipeS2CPacket;

mod s2c_0x40_player_abilities;
pub use s2c_0x40_player_abilities::PlayerAbilitiesS2CPacket;

mod s2c_0x41_player_chat;
pub use s2c_0x41_player_chat::PlayerChatS2CPacket;

mod s2c_0x42_player_combat_end;
pub use s2c_0x42_player_combat_end::PlayerCombatEndS2CPacket;

mod s2c_0x43_player_combat_enter;
pub use s2c_0x43_player_combat_enter::PlayerCombatEnterS2CPacket;

mod s2c_0x44_player_combat_kill;
pub use s2c_0x44_player_combat_kill::PlayerCombatKillS2CPacket;

mod s2c_0x45_player_info_remove;
pub use s2c_0x45_player_info_remove::PlayerInfoRemoveS2CPacket;

mod s2c_0x46_player_info_update;
pub use s2c_0x46_player_info_update::PlayerInfoUpdateS2CPacket;

mod s2c_0x47_player_look_at;
pub use s2c_0x47_player_look_at::PlayerLookAtS2CPacket;

mod s2c_0x48_player_position;
pub use s2c_0x48_player_position::PlayerPositionS2CPacket;

mod s2c_0x49_player_rotation;
pub use s2c_0x49_player_rotation::PlayerRotationS2CPacket;

mod s2c_0x4a_recipe_book_add;
pub use s2c_0x4a_recipe_book_add::RecipeBookAddS2CPacket;

mod s2c_0x4b_recipe_book_remove;
pub use s2c_0x4b_recipe_book_remove::RecipeBookRemoveS2CPacket;

mod s2c_0x4c_recipe_book_settings;
pub use s2c_0x4c_recipe_book_settings::RecipeBookSettingsS2CPacket;

mod s2c_0x4d_remove_entities;
pub use s2c_0x4d_remove_entities::RemoveEntitiesS2CPacket;

mod s2c_0x4e_remove_mob_effect;
pub use s2c_0x4e_remove_mob_effect::RemoveMobEffectS2CPacket;

mod s2c_0x4f_reset_score;
pub use s2c_0x4f_reset_score::ResetScoreS2CPacket;

mod s2c_0x50_resource_pack_pop;
pub use s2c_0x50_resource_pack_pop::ResourcePackPopS2CPacket;

mod s2c_0x51_resource_pack_push;
pub use s2c_0x51_resource_pack_push::ResourcePackPushS2CPacket;

mod s2c_0x52_respawn;
pub use s2c_0x52_respawn::RespawnS2CPacket;

mod s2c_0x53_rotate_head;
pub use s2c_0x53_rotate_head::RotateHeadS2CPacket;

mod s2c_0x54_section_blocks_update;
pub use s2c_0x54_section_blocks_update::SectionBlocksUpdateS2CPacket;

mod s2c_0x55_select_advancements_tab;
pub use s2c_0x55_select_advancements_tab::SelectAdvancementsTabS2CPacket;

mod s2c_0x56_server_data;
pub use s2c_0x56_server_data::ServerDataS2CPacket;

mod s2c_0x57_set_action_bar_text;
pub use s2c_0x57_set_action_bar_text::SetActionBarTextS2CPacket;

mod s2c_0x58_set_border_center;
pub use s2c_0x58_set_border_center::SetBorderCenterS2CPacket;

mod s2c_0x59_set_border_lerp_size;
pub use s2c_0x59_set_border_lerp_size::SetBorderLerpSizeS2CPacket;

mod s2c_0x5a_set_border_size;
pub use s2c_0x5a_set_border_size::SetBorderSizeS2CPacket;

mod s2c_0x5b_set_border_warning_delay;
pub use s2c_0x5b_set_border_warning_delay::SetBorderWarningDelayS2CPacket;

mod s2c_0x5c_set_border_warning_distance;
pub use s2c_0x5c_set_border_warning_distance::SetBorderWarningDistanceS2CPacket;

mod s2c_0x5d_set_camera;
pub use s2c_0x5d_set_camera::SetCameraS2CPacket;

mod s2c_0x5e_set_chunk_cache_center;
pub use s2c_0x5e_set_chunk_cache_center::SetChunkCacheCenterS2CPacket;

mod s2c_0x5f_set_chunk_cache_radius;
pub use s2c_0x5f_set_chunk_cache_radius::SetChunkCacheRadiusS2CPacket;

mod s2c_0x60_set_cursor_item;
pub use s2c_0x60_set_cursor_item::SetCursorItemS2CPacket;

mod s2c_0x61_set_default_spawn_position;
pub use s2c_0x61_set_default_spawn_position::SetDefaultSpawnPositionS2CPacket;

mod s2c_0x62_set_display_objective;
pub use s2c_0x62_set_display_objective::SetDisplayObjectiveS2CPacket;

mod s2c_0x63_set_entity_data;
pub use s2c_0x63_set_entity_data::SetEntityDataS2CPacket;

mod s2c_0x64_set_entity_link;
pub use s2c_0x64_set_entity_link::SetEntityLinkS2CPacket;

mod s2c_0x65_set_entity_motion;
pub use s2c_0x65_set_entity_motion::SetEntityMotionS2CPacket;

mod s2c_0x66_set_equipment;
pub use s2c_0x66_set_equipment::SetEquipmentS2CPacket;

mod s2c_0x67_set_experience;
pub use s2c_0x67_set_experience::SetExperienceS2CPacket;

mod s2c_0x68_set_health;
pub use s2c_0x68_set_health::SetHealthS2CPacket;

mod s2c_0x69_set_held_slot;
pub use s2c_0x69_set_held_slot::SetHeldSlotS2CPacket;

mod s2c_0x6a_set_objective;
pub use s2c_0x6a_set_objective::SetObjectiveS2CPacket;

mod s2c_0x6b_set_passengers;
pub use s2c_0x6b_set_passengers::SetPassengersS2CPacket;

mod s2c_0x6c_set_player_inventory;
pub use s2c_0x6c_set_player_inventory::SetPlayerInventoryS2CPacket;

mod s2c_0x6d_set_player_team;
pub use s2c_0x6d_set_player_team::SetPlayerTeamS2CPacket;

mod s2c_0x6e_set_score;
pub use s2c_0x6e_set_score::SetScoreS2CPacket;

mod s2c_0x6f_set_simulation_distance;
pub use s2c_0x6f_set_simulation_distance::SetSimulationDistanceS2CPacket;

mod s2c_0x70_set_subtitle_text;
pub use s2c_0x70_set_subtitle_text::SetSubtitleTextS2CPacket;

mod s2c_0x71_set_time;
pub use s2c_0x71_set_time::SetTimeS2CPacket;

mod s2c_0x72_set_title_text;
pub use s2c_0x72_set_title_text::SetTitleTextS2CPacket;

mod s2c_0x73_set_titles_animation;
pub use s2c_0x73_set_titles_animation::SetTitlesAnimationS2CPacket;

mod s2c_0x74_sound_entity;
pub use s2c_0x74_sound_entity::SoundEntityS2CPacket;

mod s2c_0x75_sound;
pub use s2c_0x75_sound::SoundS2CPacket;

mod s2c_0x76_start_configuration;
pub use s2c_0x76_start_configuration::StartConfigurationS2CPacket;

mod s2c_0x77_stop_sound;
pub use s2c_0x77_stop_sound::StopSoundS2CPacket;

mod s2c_0x78_store_cookie;
pub use s2c_0x78_store_cookie::StoreCookieS2CPacket;

mod s2c_0x79_system_chat;
pub use s2c_0x79_system_chat::SystemChatS2CPacket;

mod s2c_0x7a_tab_list;
pub use s2c_0x7a_tab_list::TabListS2CPacket;

mod s2c_0x7b_tag_query;
pub use s2c_0x7b_tag_query::TagQueryS2CPacket;

mod s2c_0x7c_take_item_entity;
pub use s2c_0x7c_take_item_entity::TakeItemEntityS2CPacket;

mod s2c_0x7d_teleport_entity;
pub use s2c_0x7d_teleport_entity::TeleportEntityS2CPacket;

mod s2c_0x7e_test_instance_block_status;
pub use s2c_0x7e_test_instance_block_status::TestInstanceBlockStatusS2CPacket;

mod s2c_0x7f_ticking_state;
pub use s2c_0x7f_ticking_state::TickingStateS2CPacket;

mod s2c_0x80_ticking_step;
pub use s2c_0x80_ticking_step::TickingStepS2CPacket;

mod s2c_0x81_transfer;
pub use s2c_0x81_transfer::TransferS2CPacket;

mod s2c_0x82_update_advancements;
pub use s2c_0x82_update_advancements::UpdateAdvancementsS2CPacket;

mod s2c_0x83_update_attributes;
pub use s2c_0x83_update_attributes::UpdateAttributesS2CPacket;

mod s2c_0x84_update_mob_effect;
pub use s2c_0x84_update_mob_effect::UpdateMobEffectS2CPacket;

mod s2c_0x85_update_recipes;
pub use s2c_0x85_update_recipes::UpdateRecipesS2CPacket;

mod s2c_0x86_update_tags;
pub use s2c_0x86_update_tags::UpdateTagsS2CPacket;

mod s2c_0x87_projectile_power;
pub use s2c_0x87_projectile_power::ProjectilePowerS2CPacket;

mod s2c_0x88_custom_report_details;
pub use s2c_0x88_custom_report_details::CustomReportDetailsS2CPacket;

mod s2c_0x89_server_links;
pub use s2c_0x89_server_links::ServerLinksS2CPacket;

mod s2c_0x8a_waypoint;
pub use s2c_0x8a_waypoint::WaypointS2CPacket;

mod s2c_0x8b_clear_dialog;
pub use s2c_0x8b_clear_dialog::ClearDialogS2CPacket;

mod s2c_0x8c_show_dialog;
pub use s2c_0x8c_show_dialog::ShowDialogS2CPacket;

mod c2s_0x00_accept_teleportation;
pub use c2s_0x00_accept_teleportation::AcceptTeleportationC2SPacket;

mod c2s_0x01_attack;
pub use c2s_0x01_attack::AttackC2SPacket;

mod c2s_0x02_block_entity_tag_query;
pub use c2s_0x02_block_entity_tag_query::BlockEntityTagQueryC2SPacket;

mod c2s_0x03_bundle_item_selected;
pub use c2s_0x03_bundle_item_selected::BundleItemSelectedC2SPacket;

mod c2s_0x04_change_difficulty;
pub use c2s_0x04_change_difficulty::ChangeDifficultyC2SPacket;

mod c2s_0x05_change_game_mode;
pub use c2s_0x05_change_game_mode::ChangeGameModeC2SPacket;

mod c2s_0x06_chat_ack;
pub use c2s_0x06_chat_ack::ChatAckC2SPacket;

mod c2s_0x07_chat_command;
pub use c2s_0x07_chat_command::ChatCommandC2SPacket;

mod c2s_0x08_chat_command_signed;
pub use c2s_0x08_chat_command_signed::ChatCommandSignedC2SPacket;

mod c2s_0x09_chat;
pub use c2s_0x09_chat::ChatC2SPacket;

mod c2s_0x0a_chat_session_update;
pub use c2s_0x0a_chat_session_update::ChatSessionUpdateC2SPacket;

mod c2s_0x0b_chunk_batch_received;
pub use c2s_0x0b_chunk_batch_received::ChunkBatchReceivedC2SPacket;

mod c2s_0x0c_client_command;
pub use c2s_0x0c_client_command::ClientCommandC2SPacket;

mod c2s_0x0d_client_tick_end;
pub use c2s_0x0d_client_tick_end::ClientTickEndC2SPacket;

mod c2s_0x0e_client_information;
pub use c2s_0x0e_client_information::ClientInformationC2SPacket;

mod c2s_0x0f_command_suggestion;
pub use c2s_0x0f_command_suggestion::CommandSuggestionC2SPacket;

mod c2s_0x10_configuration_acknowledged;
pub use c2s_0x10_configuration_acknowledged::ConfigurationAcknowledgedC2SPacket;

mod c2s_0x11_container_button_click;
pub use c2s_0x11_container_button_click::ContainerButtonClickC2SPacket;

mod c2s_0x12_container_click;
pub use c2s_0x12_container_click::ContainerClickC2SPacket;

mod c2s_0x13_container_close;
pub use c2s_0x13_container_close::ContainerCloseC2SPacket;

mod c2s_0x14_container_slot_state_changed;
pub use c2s_0x14_container_slot_state_changed::ContainerSlotStateChangedC2SPacket;

mod c2s_0x15_cookie_response;
pub use c2s_0x15_cookie_response::CookieResponseC2SPacket;

mod c2s_0x16_custom_payload;
pub use c2s_0x16_custom_payload::CustomPayloadC2SPacket;

mod c2s_0x17_debug_subscription_request;
pub use c2s_0x17_debug_subscription_request::DebugSubscriptionRequestC2SPacket;

mod c2s_0x18_edit_book;
pub use c2s_0x18_edit_book::EditBookC2SPacket;

mod c2s_0x19_entity_tag_query;
pub use c2s_0x19_entity_tag_query::EntityTagQueryC2SPacket;

mod c2s_0x1a_interact;
pub use c2s_0x1a_interact::InteractC2SPacket;

mod c2s_0x1b_jigsaw_generate;
pub use c2s_0x1b_jigsaw_generate::JigsawGenerateC2SPacket;

mod c2s_0x1c_keep_alive;
pub use c2s_0x1c_keep_alive::KeepAliveC2SPacket;

mod c2s_0x1d_lock_difficulty;
pub use c2s_0x1d_lock_difficulty::LockDifficultyC2SPacket;

mod c2s_0x1e_move_player_pos;
pub use c2s_0x1e_move_player_pos::MovePlayerPosC2SPacket;

mod c2s_0x1f_move_player_pos_rot;
pub use c2s_0x1f_move_player_pos_rot::MovePlayerPosRotC2SPacket;

mod c2s_0x20_move_player_rot;
pub use c2s_0x20_move_player_rot::MovePlayerRotC2SPacket;

mod c2s_0x21_move_player_status_only;
pub use c2s_0x21_move_player_status_only::MovePlayerStatusOnlyC2SPacket;

mod c2s_0x22_move_vehicle;
pub use c2s_0x22_move_vehicle::MoveVehicleC2SPacket;

mod c2s_0x23_paddle_boat;
pub use c2s_0x23_paddle_boat::PaddleBoatC2SPacket;

mod c2s_0x24_pick_item_from_block;
pub use c2s_0x24_pick_item_from_block::PickItemFromBlockC2SPacket;

mod c2s_0x25_pick_item_from_entity;
pub use c2s_0x25_pick_item_from_entity::PickItemFromEntityC2SPacket;

mod c2s_0x26_ping_request;
pub use c2s_0x26_ping_request::PingRequestC2SPacket;

mod c2s_0x27_place_recipe;
pub use c2s_0x27_place_recipe::PlaceRecipeC2SPacket;

mod c2s_0x28_player_abilities;
pub use c2s_0x28_player_abilities::PlayerAbilitiesC2SPacket;

mod c2s_0x29_player_action;
pub use c2s_0x29_player_action::PlayerActionC2SPacket;

mod c2s_0x2a_player_command;
pub use c2s_0x2a_player_command::PlayerCommandC2SPacket;

mod c2s_0x2b_player_input;
pub use c2s_0x2b_player_input::PlayerInputC2SPacket;

mod c2s_0x2c_player_loaded;
pub use c2s_0x2c_player_loaded::PlayerLoadedC2SPacket;

mod c2s_0x2d_pong;
pub use c2s_0x2d_pong::PongC2SPacket;

mod c2s_0x2e_recipe_book_change_settings;
pub use c2s_0x2e_recipe_book_change_settings::RecipeBookChangeSettingsC2SPacket;

mod c2s_0x2f_recipe_book_seen_recipe;
pub use c2s_0x2f_recipe_book_seen_recipe::RecipeBookSeenRecipeC2SPacket;

mod c2s_0x30_rename_item;
pub use c2s_0x30_rename_item::RenameItemC2SPacket;

mod c2s_0x31_resource_pack;
pub use c2s_0x31_resource_pack::ResourcePackC2SPacket;

mod c2s_0x32_seen_advancements;
pub use c2s_0x32_seen_advancements::SeenAdvancementsC2SPacket;

mod c2s_0x33_select_trade;
pub use c2s_0x33_select_trade::SelectTradeC2SPacket;

mod c2s_0x34_set_beacon;
pub use c2s_0x34_set_beacon::SetBeaconC2SPacket;

mod c2s_0x35_set_carried_item;
pub use c2s_0x35_set_carried_item::SetCarriedItemC2SPacket;

mod c2s_0x36_set_command_block;
pub use c2s_0x36_set_command_block::SetCommandBlockC2SPacket;

mod c2s_0x37_set_command_minecart;
pub use c2s_0x37_set_command_minecart::SetCommandMinecartC2SPacket;

mod c2s_0x38_set_creative_mode_slot;
pub use c2s_0x38_set_creative_mode_slot::SetCreativeModeSlotC2SPacket;

mod c2s_0x39_set_game_rule;
pub use c2s_0x39_set_game_rule::SetGameRuleC2SPacket;

mod c2s_0x3a_set_jigsaw_block;
pub use c2s_0x3a_set_jigsaw_block::SetJigsawBlockC2SPacket;

mod c2s_0x3b_set_structure_block;
pub use c2s_0x3b_set_structure_block::SetStructureBlockC2SPacket;

mod c2s_0x3c_set_test_block;
pub use c2s_0x3c_set_test_block::SetTestBlockC2SPacket;

mod c2s_0x3d_sign_update;
pub use c2s_0x3d_sign_update::SignUpdateC2SPacket;

mod c2s_0x3e_spectate_entity;
pub use c2s_0x3e_spectate_entity::SpectateEntityC2SPacket;

mod c2s_0x3f_swing;
pub use c2s_0x3f_swing::SwingC2SPacket;

mod c2s_0x40_teleport_to_entity;
pub use c2s_0x40_teleport_to_entity::TeleportToEntityC2SPacket;

mod c2s_0x41_test_instance_block_action;
pub use c2s_0x41_test_instance_block_action::TestInstanceBlockActionC2SPacket;

mod c2s_0x42_use_item_on;
pub use c2s_0x42_use_item_on::UseItemOnC2SPacket;

mod c2s_0x43_use_item;
pub use c2s_0x43_use_item::UseItemC2SPacket;

mod c2s_0x44_custom_click_action;
pub use c2s_0x44_custom_click_action::CustomClickActionC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    BundleDelimiter(BundleDelimiterS2CPacket) = 0x00,
    AddEntity(AddEntityS2CPacket) = 0x01,
    Animate(AnimateS2CPacket) = 0x02,
    AwardStats(AwardStatsS2CPacket) = 0x03,
    BlockChangedAck(BlockChangedAckS2CPacket) = 0x04,
    BlockDestruction(BlockDestructionS2CPacket) = 0x05,
    BlockEntityData(BlockEntityDataS2CPacket) = 0x06,
    BlockEvent(BlockEventS2CPacket) = 0x07,
    BlockUpdate(BlockUpdateS2CPacket) = 0x08,
    BossEvent(BossEventS2CPacket) = 0x09,
    ChangeDifficulty(ChangeDifficultyS2CPacket) = 0x0a,
    ChunkBatchFinished(ChunkBatchFinishedS2CPacket) = 0x0b,
    ChunkBatchStart(ChunkBatchStartS2CPacket) = 0x0c,
    ChunksBiomes(ChunksBiomesS2CPacket) = 0x0d,
    ClearTitles(ClearTitlesS2CPacket) = 0x0e,
    CommandSuggestions(CommandSuggestionsS2CPacket) = 0x0f,
    Commands(CommandsS2CPacket) = 0x10,
    ContainerClose(ContainerCloseS2CPacket) = 0x11,
    ContainerSetContent(ContainerSetContentS2CPacket) = 0x12,
    ContainerSetData(ContainerSetDataS2CPacket) = 0x13,
    ContainerSetSlot(ContainerSetSlotS2CPacket) = 0x14,
    CookieRequest(CookieRequestS2CPacket) = 0x15,
    Cooldown(CooldownS2CPacket) = 0x16,
    CustomChatCompletions(CustomChatCompletionsS2CPacket) = 0x17,
    CustomPayload(CustomPayloadS2CPacket) = 0x18,
    DamageEvent(DamageEventS2CPacket) = 0x19,
    DebugBlockValue(DebugBlockValueS2CPacket) = 0x1a,
    DebugChunkValue(DebugChunkValueS2CPacket) = 0x1b,
    DebugEntityValue(DebugEntityValueS2CPacket) = 0x1c,
    DebugEvent(DebugEventS2CPacket) = 0x1d,
    DebugSample(DebugSampleS2CPacket) = 0x1e,
    DeleteChat(DeleteChatS2CPacket) = 0x1f,
    Disconnect(DisconnectS2CPacket) = 0x20,
    DisguisedChat(DisguisedChatS2CPacket) = 0x21,
    EntityEvent(EntityEventS2CPacket) = 0x22,
    EntityPositionSync(EntityPositionSyncS2CPacket) = 0x23,
    Explode(ExplodeS2CPacket) = 0x24,
    ForgetLevelChunk(ForgetLevelChunkS2CPacket) = 0x25,
    GameEvent(GameEventS2CPacket) = 0x26,
    GameRuleValues(GameRuleValuesS2CPacket) = 0x27,
    GameTestHighlightPos(GameTestHighlightPosS2CPacket) = 0x28,
    MountScreenOpen(MountScreenOpenS2CPacket) = 0x29,
    HurtAnimation(HurtAnimationS2CPacket) = 0x2a,
    InitializeBorder(InitializeBorderS2CPacket) = 0x2b,
    KeepAlive(KeepAliveS2CPacket) = 0x2c,
    LevelChunkWithLight(LevelChunkWithLightS2CPacket) = 0x2d,
    LevelEvent(LevelEventS2CPacket) = 0x2e,
    LevelParticles(LevelParticlesS2CPacket) = 0x2f,
    LightUpdate(LightUpdateS2CPacket) = 0x30,
    Login(LoginS2CPacket) = 0x31,
    LowDiskSpaceWarning(LowDiskSpaceWarningS2CPacket) = 0x32,
    MapItemData(MapItemDataS2CPacket) = 0x33,
    MerchantOffers(MerchantOffersS2CPacket) = 0x34,
    MoveEntityPos(MoveEntityPosS2CPacket) = 0x35,
    MoveEntityPosRot(MoveEntityPosRotS2CPacket) = 0x36,
    MoveMinecartAlongTrack(MoveMinecartAlongTrackS2CPacket) = 0x37,
    MoveEntityRot(MoveEntityRotS2CPacket) = 0x38,
    MoveVehicle(MoveVehicleS2CPacket) = 0x39,
    OpenBook(OpenBookS2CPacket) = 0x3a,
    OpenScreen(OpenScreenS2CPacket) = 0x3b,
    OpenSignEditor(OpenSignEditorS2CPacket) = 0x3c,
    Ping(PingS2CPacket) = 0x3d,
    PongResponse(PongResponseS2CPacket) = 0x3e,
    PlaceGhostRecipe(PlaceGhostRecipeS2CPacket) = 0x3f,
    PlayerAbilities(PlayerAbilitiesS2CPacket) = 0x40,
    PlayerChat(PlayerChatS2CPacket) = 0x41,
    PlayerCombatEnd(PlayerCombatEndS2CPacket) = 0x42,
    PlayerCombatEnter(PlayerCombatEnterS2CPacket) = 0x43,
    PlayerCombatKill(PlayerCombatKillS2CPacket) = 0x44,
    PlayerInfoRemove(PlayerInfoRemoveS2CPacket) = 0x45,
    PlayerInfoUpdate(PlayerInfoUpdateS2CPacket) = 0x46,
    PlayerLookAt(PlayerLookAtS2CPacket) = 0x47,
    PlayerPosition(PlayerPositionS2CPacket) = 0x48,
    PlayerRotation(PlayerRotationS2CPacket) = 0x49,
    RecipeBookAdd(RecipeBookAddS2CPacket) = 0x4a,
    RecipeBookRemove(RecipeBookRemoveS2CPacket) = 0x4b,
    RecipeBookSettings(RecipeBookSettingsS2CPacket) = 0x4c,
    RemoveEntities(RemoveEntitiesS2CPacket) = 0x4d,
    RemoveMobEffect(RemoveMobEffectS2CPacket) = 0x4e,
    ResetScore(ResetScoreS2CPacket) = 0x4f,
    ResourcePackPop(ResourcePackPopS2CPacket) = 0x50,
    ResourcePackPush(ResourcePackPushS2CPacket) = 0x51,
    Respawn(RespawnS2CPacket) = 0x52,
    RotateHead(RotateHeadS2CPacket) = 0x53,
    SectionBlocksUpdate(SectionBlocksUpdateS2CPacket) = 0x54,
    SelectAdvancementsTab(SelectAdvancementsTabS2CPacket) = 0x55,
    ServerData(ServerDataS2CPacket) = 0x56,
    SetActionBarText(SetActionBarTextS2CPacket) = 0x57,
    SetBorderCenter(SetBorderCenterS2CPacket) = 0x58,
    SetBorderLerpSize(SetBorderLerpSizeS2CPacket) = 0x59,
    SetBorderSize(SetBorderSizeS2CPacket) = 0x5a,
    SetBorderWarningDelay(SetBorderWarningDelayS2CPacket) = 0x5b,
    SetBorderWarningDistance(SetBorderWarningDistanceS2CPacket) = 0x5c,
    SetCamera(SetCameraS2CPacket) = 0x5d,
    SetChunkCacheCenter(SetChunkCacheCenterS2CPacket) = 0x5e,
    SetChunkCacheRadius(SetChunkCacheRadiusS2CPacket) = 0x5f,
    SetCursorItem(SetCursorItemS2CPacket) = 0x60,
    SetDefaultSpawnPosition(SetDefaultSpawnPositionS2CPacket) = 0x61,
    SetDisplayObjective(SetDisplayObjectiveS2CPacket) = 0x62,
    SetEntityData(SetEntityDataS2CPacket) = 0x63,
    SetEntityLink(SetEntityLinkS2CPacket) = 0x64,
    SetEntityMotion(SetEntityMotionS2CPacket) = 0x65,
    SetEquipment(SetEquipmentS2CPacket) = 0x66,
    SetExperience(SetExperienceS2CPacket) = 0x67,
    SetHealth(SetHealthS2CPacket) = 0x68,
    SetHeldSlot(SetHeldSlotS2CPacket) = 0x69,
    SetObjective(SetObjectiveS2CPacket) = 0x6a,
    SetPassengers(SetPassengersS2CPacket) = 0x6b,
    SetPlayerInventory(SetPlayerInventoryS2CPacket) = 0x6c,
    SetPlayerTeam(SetPlayerTeamS2CPacket) = 0x6d,
    SetScore(SetScoreS2CPacket) = 0x6e,
    SetSimulationDistance(SetSimulationDistanceS2CPacket) = 0x6f,
    SetSubtitleText(SetSubtitleTextS2CPacket) = 0x70,
    SetTime(SetTimeS2CPacket) = 0x71,
    SetTitleText(SetTitleTextS2CPacket) = 0x72,
    SetTitlesAnimation(SetTitlesAnimationS2CPacket) = 0x73,
    SoundEntity(SoundEntityS2CPacket) = 0x74,
    Sound(SoundS2CPacket) = 0x75,
    StartConfiguration(StartConfigurationS2CPacket) = 0x76,
    StopSound(StopSoundS2CPacket) = 0x77,
    StoreCookie(StoreCookieS2CPacket) = 0x78,
    SystemChat(SystemChatS2CPacket) = 0x79,
    TabList(TabListS2CPacket) = 0x7a,
    TagQuery(TagQueryS2CPacket) = 0x7b,
    TakeItemEntity(TakeItemEntityS2CPacket) = 0x7c,
    TeleportEntity(TeleportEntityS2CPacket) = 0x7d,
    TestInstanceBlockStatus(TestInstanceBlockStatusS2CPacket) = 0x7e,
    TickingState(TickingStateS2CPacket) = 0x7f,
    TickingStep(TickingStepS2CPacket) = 0x80,
    Transfer(TransferS2CPacket) = 0x81,
    UpdateAdvancements(UpdateAdvancementsS2CPacket) = 0x82,
    UpdateAttributes(UpdateAttributesS2CPacket) = 0x83,
    UpdateMobEffect(UpdateMobEffectS2CPacket) = 0x84,
    UpdateRecipes(UpdateRecipesS2CPacket) = 0x85,
    UpdateTags(UpdateTagsS2CPacket) = 0x86,
    ProjectilePower(ProjectilePowerS2CPacket) = 0x87,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x88,
    ServerLinks(ServerLinksS2CPacket) = 0x89,
    Waypoint(WaypointS2CPacket) = 0x8a,
    ClearDialog(ClearDialogS2CPacket) = 0x8b,
    ShowDialog(ShowDialogS2CPacket) = 0x8c,
}

#[repr(u8)]
#[cfg(feature = "v26_1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    AcceptTeleportation(AcceptTeleportationC2SPacket) = 0x00,
    Attack(AttackC2SPacket) = 0x01,
    BlockEntityTagQuery(BlockEntityTagQueryC2SPacket) = 0x02,
    BundleItemSelected(BundleItemSelectedC2SPacket) = 0x03,
    ChangeDifficulty(ChangeDifficultyC2SPacket) = 0x04,
    ChangeGameMode(ChangeGameModeC2SPacket) = 0x05,
    ChatAck(ChatAckC2SPacket) = 0x06,
    ChatCommand(ChatCommandC2SPacket) = 0x07,
    ChatCommandSigned(ChatCommandSignedC2SPacket) = 0x08,
    Chat(ChatC2SPacket) = 0x09,
    ChatSessionUpdate(ChatSessionUpdateC2SPacket) = 0x0a,
    ChunkBatchReceived(ChunkBatchReceivedC2SPacket) = 0x0b,
    ClientCommand(ClientCommandC2SPacket) = 0x0c,
    ClientTickEnd(ClientTickEndC2SPacket) = 0x0d,
    ClientInformation(ClientInformationC2SPacket) = 0x0e,
    CommandSuggestion(CommandSuggestionC2SPacket) = 0x0f,
    ConfigurationAcknowledged(ConfigurationAcknowledgedC2SPacket) = 0x10,
    ContainerButtonClick(ContainerButtonClickC2SPacket) = 0x11,
    ContainerClick(ContainerClickC2SPacket) = 0x12,
    ContainerClose(ContainerCloseC2SPacket) = 0x13,
    ContainerSlotStateChanged(ContainerSlotStateChangedC2SPacket) = 0x14,
    CookieResponse(CookieResponseC2SPacket) = 0x15,
    CustomPayload(CustomPayloadC2SPacket) = 0x16,
    DebugSubscriptionRequest(DebugSubscriptionRequestC2SPacket) = 0x17,
    EditBook(EditBookC2SPacket) = 0x18,
    EntityTagQuery(EntityTagQueryC2SPacket) = 0x19,
    Interact(InteractC2SPacket) = 0x1a,
    JigsawGenerate(JigsawGenerateC2SPacket) = 0x1b,
    KeepAlive(KeepAliveC2SPacket) = 0x1c,
    LockDifficulty(LockDifficultyC2SPacket) = 0x1d,
    MovePlayerPos(MovePlayerPosC2SPacket) = 0x1e,
    MovePlayerPosRot(MovePlayerPosRotC2SPacket) = 0x1f,
    MovePlayerRot(MovePlayerRotC2SPacket) = 0x20,
    MovePlayerStatusOnly(MovePlayerStatusOnlyC2SPacket) = 0x21,
    MoveVehicle(MoveVehicleC2SPacket) = 0x22,
    PaddleBoat(PaddleBoatC2SPacket) = 0x23,
    PickItemFromBlock(PickItemFromBlockC2SPacket) = 0x24,
    PickItemFromEntity(PickItemFromEntityC2SPacket) = 0x25,
    PingRequest(PingRequestC2SPacket) = 0x26,
    PlaceRecipe(PlaceRecipeC2SPacket) = 0x27,
    PlayerAbilities(PlayerAbilitiesC2SPacket) = 0x28,
    PlayerAction(PlayerActionC2SPacket) = 0x29,
    PlayerCommand(PlayerCommandC2SPacket) = 0x2a,
    PlayerInput(PlayerInputC2SPacket) = 0x2b,
    PlayerLoaded(PlayerLoadedC2SPacket) = 0x2c,
    Pong(PongC2SPacket) = 0x2d,
    RecipeBookChangeSettings(RecipeBookChangeSettingsC2SPacket) = 0x2e,
    RecipeBookSeenRecipe(RecipeBookSeenRecipeC2SPacket) = 0x2f,
    RenameItem(RenameItemC2SPacket) = 0x30,
    ResourcePack(ResourcePackC2SPacket) = 0x31,
    SeenAdvancements(SeenAdvancementsC2SPacket) = 0x32,
    SelectTrade(SelectTradeC2SPacket) = 0x33,
    SetBeacon(SetBeaconC2SPacket) = 0x34,
    SetCarriedItem(SetCarriedItemC2SPacket) = 0x35,
    SetCommandBlock(SetCommandBlockC2SPacket) = 0x36,
    SetCommandMinecart(SetCommandMinecartC2SPacket) = 0x37,
    SetCreativeModeSlot(SetCreativeModeSlotC2SPacket) = 0x38,
    SetGameRule(SetGameRuleC2SPacket) = 0x39,
    SetJigsawBlock(SetJigsawBlockC2SPacket) = 0x3a,
    SetStructureBlock(SetStructureBlockC2SPacket) = 0x3b,
    SetTestBlock(SetTestBlockC2SPacket) = 0x3c,
    SignUpdate(SignUpdateC2SPacket) = 0x3d,
    SpectateEntity(SpectateEntityC2SPacket) = 0x3e,
    Swing(SwingC2SPacket) = 0x3f,
    TeleportToEntity(TeleportToEntityC2SPacket) = 0x40,
    TestInstanceBlockAction(TestInstanceBlockActionC2SPacket) = 0x41,
    UseItemOn(UseItemOnC2SPacket) = 0x42,
    UseItem(UseItemC2SPacket) = 0x43,
    CustomClickAction(CustomClickActionC2SPacket) = 0x44,
}
