//! [`Play`](crate::states::Play) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

mod disconnect;
pub use disconnect::*;

mod acknowledge_chunks;
pub use acknowledge_chunks::*;

mod acknowledge_reconfiguration;
pub use acknowledge_reconfiguration::*;

mod advancement_tab;
pub use advancement_tab::*;

mod advancement_update;
pub use advancement_update::*;

mod block_breaking_progress;
pub use block_breaking_progress::*;

mod block_entity_update;
pub use block_entity_update::*;

mod block_event;
pub use block_event::*;

mod block_update;
pub use block_update::*;

mod boat_paddle_state;
pub use boat_paddle_state::*;

mod book_update;
pub use book_update::*;

mod boss_bar;
pub use boss_bar::*;

mod bundle_delimiter;
pub use bundle_delimiter::*;

mod button_click;
pub use button_click::*;

mod change_unlocked_recipes;
pub use change_unlocked_recipes::*;

mod chat_command_signed;
pub use chat_command_signed::*;

mod chat_message_c2s;
pub use chat_message_c2s::*;

mod chat_message_s2c;
pub use chat_message_s2c::*;

mod chat_suggestions;
pub use chat_suggestions::*;

mod chunk_biome_data;
pub use chunk_biome_data::*;

mod chunk_data;
pub use chunk_data::*;

mod chunk_delta_update;
pub use chunk_delta_update::*;

mod chunk_load_distance;
pub use chunk_load_distance::*;

mod chunk_render_distance_center;
pub use chunk_render_distance_center::*;

mod chunk_sent;
pub use chunk_sent::*;

mod clear_title;
pub use clear_title::*;

mod click_slot;
pub use click_slot::*;

mod client_command;
pub use client_command::*;

mod client_options;
pub use client_options::*;

mod client_status;
pub use client_status::*;

mod close_handled_screen;
pub use close_handled_screen::*;

mod close_screen;
pub use close_screen::*;

mod command_execution;
pub use command_execution::*;

mod command_suggestions;
pub use command_suggestions::*;

mod command_tree;
pub use command_tree::*;

mod common_ping;
pub use common_ping::*;

mod common_pong;
pub use common_pong::*;

mod cookie_request;
pub use cookie_request::*;

mod cookie_response;
pub use cookie_response::*;

mod cooldown_update;
pub use cooldown_update::*;

mod craft_failed_response;
pub use craft_failed_response::*;

mod craft_request;
pub use craft_request::*;

mod creative_inventory_action;
pub use creative_inventory_action::*;

mod custom_payload_c2s;
pub use custom_payload_c2s::*;

mod custom_payload_s2c;
pub use custom_payload_s2c::*;

mod custom_report_details;
pub use custom_report_details::*;

mod damage_tilt;
pub use damage_tilt::*;

mod death_message;
pub use death_message::*;

mod debug_sample;
pub use debug_sample::*;

mod debug_sample_subscription;
pub use debug_sample_subscription::*;

mod difficulty;
pub use difficulty::*;

mod end_combat;
pub use end_combat::*;

mod enter_combat;
pub use enter_combat::*;

mod enter_reconfiguration;
pub use enter_reconfiguration::*;

mod entities_destroy;
pub use entities_destroy::*;

mod entity_animation;
pub use entity_animation::*;

mod entity_attach;
pub use entity_attach::*;

mod entity_attributes;
pub use entity_attributes::*;

mod entity_damage;
pub use entity_damage::*;

mod entity_equipment_update;
pub use entity_equipment_update::*;

mod entity_move_relative;
pub use entity_move_relative::*;

mod entity_passengers_set;
pub use entity_passengers_set::*;

mod entity_position;
pub use entity_position::*;

mod entity_rotate;
pub use entity_rotate::*;

mod entity_rotate_and_move_relative;
pub use entity_rotate_and_move_relative::*;

mod entity_set_head_yaw;
pub use entity_set_head_yaw::*;

mod entity_spawn;
pub use entity_spawn::*;

mod entity_status;
pub use entity_status::*;

mod entity_status_effect;
pub use entity_status_effect::*;

mod entity_tracker_update;
pub use entity_tracker_update::*;

mod entity_velocity_update;
pub use entity_velocity_update::*;

mod experience_bar_update;
pub use experience_bar_update::*;

mod experience_orb_spawn;
pub use experience_orb_spawn::*;

mod explosion;
pub use explosion::*;

mod game_join;
pub use game_join::*;

mod game_message;
pub use game_message::*;

mod game_state_change;
pub use game_state_change::*;

mod hand_swing;
pub use hand_swing::*;

mod health_update;
pub use health_update::*;

mod inventory;
pub use inventory::*;

mod item_pickup_animation;
pub use item_pickup_animation::*;

mod jigsaw_generating;
pub use jigsaw_generating::*;

mod keep_alive_c2s;
pub use keep_alive_c2s::*;

mod keep_alive_s2c;
pub use keep_alive_s2c::*;

mod light_update;
pub use light_update::*;

mod look_at;
pub use look_at::*;

mod map_update;
pub use map_update::*;

mod message_acknowledgment;
pub use message_acknowledgment::*;

mod nbt_query_response;
pub use nbt_query_response::*;

mod open_horse_screen;
pub use open_horse_screen::*;

mod open_screen;
pub use open_screen::*;

mod open_written_book;
pub use open_written_book::*;

mod overlay_message;
pub use overlay_message::*;

mod particle;
pub use particle::*;

mod pick_from_inventory;
pub use pick_from_inventory::*;

mod ping_result;
pub use ping_result::*;

mod play_sound;
pub use play_sound::*;

mod play_sound_from_entity;
pub use play_sound_from_entity::*;

mod player_abilities;
pub use player_abilities::*;

mod player_action;
pub use player_action::*;

mod player_action_response;
pub use player_action_response::*;

mod player_input;
pub use player_input::*;

mod player_interact_block;
pub use player_interact_block::*;

mod player_interact_entity;
pub use player_interact_entity::*;

mod player_interact_item;
pub use player_interact_item::*;

mod player_list;
pub use player_list::*;

mod player_list_header;
pub use player_list_header::*;

mod player_move_full;
pub use player_move_full::*;

mod player_move_look_and_on_ground;
pub use player_move_look_and_on_ground::*;

mod player_move_on_ground_only;
pub use player_move_on_ground_only::*;

mod player_move_position_and_on_ground;
pub use player_move_position_and_on_ground::*;

mod player_position_look;
pub use player_position_look::*;

mod player_remove;
pub use player_remove::*;

mod player_respawn;
pub use player_respawn::*;

mod player_session;
pub use player_session::*;

mod player_spawn_position;
pub use player_spawn_position::*;

mod profileless_chat_message;
pub use profileless_chat_message::*;

mod projectile_power;
pub use projectile_power::*;

mod query_block_nbt;
pub use query_block_nbt::*;

mod query_entity_nbt;
pub use query_entity_nbt::*;

mod query_ping;
pub use query_ping::*;

mod recipe_book_data;
pub use recipe_book_data::*;

mod recipe_category_options;
pub use recipe_category_options::*;

mod remove_entity_status_effect;
pub use remove_entity_status_effect::*;

mod remove_message;
pub use remove_message::*;

mod rename_item;
pub use rename_item::*;

mod request_command_completions;
pub use request_command_completions::*;

mod resource_pack_remove;
pub use resource_pack_remove::*;

mod resource_pack_send;
pub use resource_pack_send::*;

mod resource_pack_status;
pub use resource_pack_status::*;

mod scoreboard_display;
pub use scoreboard_display::*;

mod scoreboard_objective_update;
pub use scoreboard_objective_update::*;

mod scoreboard_score_reset;
pub use scoreboard_score_reset::*;

mod scoreboard_score_update;
pub use scoreboard_score_update::*;

mod screen_handler_property_update;
pub use screen_handler_property_update::*;

mod screen_handler_slot_update;
pub use screen_handler_slot_update::*;

mod select_advancement_tab;
pub use select_advancement_tab::*;

mod select_merchant_trade;
pub use select_merchant_trade::*;

mod server_links;
pub use server_links::*;

mod server_metadata;
pub use server_metadata::*;

mod server_transfer;
pub use server_transfer::*;

mod set_camera_entity;
pub use set_camera_entity::*;

mod set_trade_offers;
pub use set_trade_offers::*;

mod sign_editor_open;
pub use sign_editor_open::*;

mod simulation_distance;
pub use simulation_distance::*;

mod slot_changed_state;
pub use slot_changed_state::*;

mod spectator_teleport;
pub use spectator_teleport::*;

mod start_chunk_send;
pub use start_chunk_send::*;

mod statistics;
pub use statistics::*;

mod stop_sound;
pub use stop_sound::*;

mod store_cookie;
pub use store_cookie::*;

mod subtitle;
pub use subtitle::*;

mod synchronize_recipes;
pub use synchronize_recipes::*;

mod synchronize_tags;
pub use synchronize_tags::*;

mod team;
pub use team::*;

mod teleport_confirm;
pub use teleport_confirm::*;

mod tick_step;
pub use tick_step::*;

mod title;
pub use title::*;

mod title_fade;
pub use title_fade::*;

mod unload_chunk;
pub use unload_chunk::*;

mod update_beacon;
pub use update_beacon::*;

mod update_command_block;
pub use update_command_block::*;

mod update_command_block_minecart;
pub use update_command_block_minecart::*;

mod update_difficulty;
pub use update_difficulty::*;

mod update_difficulty_lock;
pub use update_difficulty_lock::*;

mod update_jigsaw;
pub use update_jigsaw::*;

mod update_player_abilities;
pub use update_player_abilities::*;

mod update_selected_slot_c2s;
pub use update_selected_slot_c2s::*;

mod update_selected_slot_s2c;
pub use update_selected_slot_s2c::*;

mod update_sign;
pub use update_sign::*;

mod update_structure_block;
pub use update_structure_block::*;

mod update_tick_rate;
pub use update_tick_rate::*;

mod vehicle_move_c2s;
pub use vehicle_move_c2s::*;

mod vehicle_move_s2c;
pub use vehicle_move_s2c::*;

mod world_border_center_changed;
pub use world_border_center_changed::*;

mod world_border_initialize;
pub use world_border_initialize::*;

mod world_border_interpolate_size;
pub use world_border_interpolate_size::*;

mod world_border_size_changed;
pub use world_border_size_changed::*;

mod world_border_warning_blocks_changed;
pub use world_border_warning_blocks_changed::*;

mod world_border_warning_time_changed;
pub use world_border_warning_time_changed::*;

mod world_event;
pub use world_event::*;

mod world_time_update;
pub use world_time_update::*;

froglight_macros::frog_state! {
    Play,
    V1_21_0,
    Clientbound {
        0u32 => BundleDelimiterPacket,
        1u32 => EntitySpawnPacket,
        2u32 => ExperienceOrbSpawnPacket,
        3u32 => EntityAnimationPacket,
        4u32 => StatisticsPacket,
        5u32 => PlayerActionResponsePacket,
        6u32 => BlockBreakingProgressPacket,
        7u32 => BlockEntityUpdatePacket,
        8u32 => BlockEventPacket,
        9u32 => BlockUpdatePacket,
        10u32 => BossBarPacket,
        11u32 => DifficultyPacket,
        12u32 => ChunkSentPacket,
        13u32 => StartChunkSendPacket,
        14u32 => ChunkBiomeDataPacket,
        15u32 => ClearTitlePacket,
        16u32 => CommandSuggestionsPacket,
        17u32 => CommandTreePacket,
        18u32 => CloseScreenPacket,
        19u32 => InventoryPacket,
        20u32 => ScreenHandlerPropertyUpdatePacket,
        21u32 => ScreenHandlerSlotUpdatePacket,
        22u32 => CookieRequestPacket,
        23u32 => CooldownUpdatePacket,
        24u32 => ChatSuggestionsPacket,
        25u32 => CustomPayloadS2CPacket,
        26u32 => EntityDamagePacket,
        27u32 => DebugSamplePacket,
        28u32 => RemoveMessagePacket,
        29u32 => DisconnectPacket,
        30u32 => ProfilelessChatMessagePacket,
        31u32 => EntityStatusPacket,
        32u32 => ExplosionPacket,
        33u32 => UnloadChunkPacket,
        34u32 => GameStateChangePacket,
        35u32 => OpenHorseScreenPacket,
        36u32 => DamageTiltPacket,
        37u32 => WorldBorderInitializePacket,
        38u32 => KeepAliveS2CPacket,
        39u32 => ChunkDataPacket,
        40u32 => WorldEventPacket,
        41u32 => ParticlePacket,
        42u32 => LightUpdatePacket,
        43u32 => GameJoinPacket,
        44u32 => MapUpdatePacket,
        45u32 => SetTradeOffersPacket,
        46u32 => EntityMoveRelativePacket,
        47u32 => EntityRotateAndMoveRelativePacket,
        48u32 => EntityRotatePacket,
        49u32 => VehicleMoveS2CPacket,
        50u32 => OpenWrittenBookPacket,
        51u32 => OpenScreenPacket,
        52u32 => SignEditorOpenPacket,
        53u32 => CommonPingPacket,
        54u32 => PingResultPacket,
        55u32 => CraftFailedResponsePacket,
        56u32 => PlayerAbilitiesPacket,
        57u32 => ChatMessageS2CPacket,
        58u32 => EndCombatPacket,
        59u32 => EnterCombatPacket,
        60u32 => DeathMessagePacket,
        61u32 => PlayerRemovePacket,
        62u32 => PlayerListPacket,
        63u32 => LookAtPacket,
        64u32 => PlayerPositionLookPacket,
        65u32 => ChangeUnlockedRecipesPacket,
        66u32 => EntitiesDestroyPacket,
        67u32 => RemoveEntityStatusEffectPacket,
        68u32 => ScoreboardScoreResetPacket,
        69u32 => ResourcePackRemovePacket,
        70u32 => ResourcePackSendPacket,
        71u32 => PlayerRespawnPacket,
        72u32 => EntitySetHeadYawPacket,
        73u32 => ChunkDeltaUpdatePacket,
        74u32 => SelectAdvancementTabPacket,
        75u32 => ServerMetadataPacket,
        76u32 => OverlayMessagePacket,
        77u32 => WorldBorderCenterChangedPacket,
        78u32 => WorldBorderInterpolateSizePacket,
        79u32 => WorldBorderSizeChangedPacket,
        80u32 => WorldBorderWarningTimeChangedPacket,
        81u32 => WorldBorderWarningBlocksChangedPacket,
        82u32 => SetCameraEntityPacket,
        83u32 => UpdateSelectedSlotS2CPacket,
        84u32 => ChunkRenderDistanceCenterPacket,
        85u32 => ChunkLoadDistancePacket,
        86u32 => PlayerSpawnPositionPacket,
        87u32 => ScoreboardDisplayPacket,
        88u32 => EntityTrackerUpdatePacket,
        89u32 => EntityAttachPacket,
        90u32 => EntityVelocityUpdatePacket,
        91u32 => EntityEquipmentUpdatePacket,
        92u32 => ExperienceBarUpdatePacket,
        93u32 => HealthUpdatePacket,
        94u32 => ScoreboardObjectiveUpdatePacket,
        95u32 => EntityPassengersSetPacket,
        96u32 => TeamPacket,
        97u32 => ScoreboardScoreUpdatePacket,
        98u32 => SimulationDistancePacket,
        99u32 => SubtitlePacket,
        100u32 => WorldTimeUpdatePacket,
        101u32 => TitlePacket,
        102u32 => TitleFadePacket,
        103u32 => PlaySoundFromEntityPacket,
        104u32 => PlaySoundPacket,
        105u32 => EnterReconfigurationPacket,
        106u32 => StopSoundPacket,
        107u32 => StoreCookiePacket,
        108u32 => GameMessagePacket,
        109u32 => PlayerListHeaderPacket,
        110u32 => NbtQueryResponsePacket,
        111u32 => ItemPickupAnimationPacket,
        112u32 => EntityPositionPacket,
        113u32 => UpdateTickRatePacket,
        114u32 => TickStepPacket,
        115u32 => ServerTransferPacket,
        116u32 => AdvancementUpdatePacket,
        117u32 => EntityAttributesPacket,
        118u32 => EntityStatusEffectPacket,
        119u32 => SynchronizeRecipesPacket,
        120u32 => SynchronizeTagsPacket,
        121u32 => ProjectilePowerPacket,
        122u32 => CustomReportDetailsPacket,
        123u32 => ServerLinksPacket,
    },
    Serverbound {
        0u32 => TeleportConfirmPacket,
        1u32 => QueryBlockNbtPacket,
        2u32 => UpdateDifficultyPacket,
        3u32 => ChatMessageC2SPacket,
        4u32 => MessageAcknowledgmentPacket,
        5u32 => CommandExecutionPacket,
        6u32 => ChatCommandSignedPacket,
        7u32 => PlayerSessionPacket,
        8u32 => AcknowledgeChunksPacket,
        9u32 => ClientStatusPacket,
        10u32 => ClientOptionsPacket,
        11u32 => RequestCommandCompletionsPacket,
        12u32 => AcknowledgeReconfigurationPacket,
        13u32 => ButtonClickPacket,
        14u32 => ClickSlotPacket,
        15u32 => CloseHandledScreenPacket,
        16u32 => SlotChangedStatePacket,
        17u32 => CookieResponsePacket,
        18u32 => CustomPayloadC2SPacket,
        19u32 => DebugSampleSubscriptionPacket,
        20u32 => BookUpdatePacket,
        21u32 => QueryEntityNbtPacket,
        22u32 => PlayerInteractEntityPacket,
        23u32 => JigsawGeneratingPacket,
        24u32 => KeepAliveC2SPacket,
        25u32 => UpdateDifficultyLockPacket,
        26u32 => PlayerMovePositionAndOnGroundPacket,
        27u32 => PlayerMoveFullPacket,
        28u32 => PlayerMoveLookAndOnGroundPacket,
        29u32 => PlayerMoveOnGroundOnlyPacket,
        30u32 => VehicleMoveC2SPacket,
        31u32 => BoatPaddleStatePacket,
        32u32 => PickFromInventoryPacket,
        33u32 => QueryPingPacket,
        34u32 => CraftRequestPacket,
        35u32 => UpdatePlayerAbilitiesPacket,
        36u32 => PlayerActionPacket,
        37u32 => ClientCommandPacket,
        38u32 => PlayerInputPacket,
        39u32 => CommonPongPacket,
        40u32 => RecipeCategoryOptionsPacket,
        41u32 => RecipeBookDataPacket,
        42u32 => RenameItemPacket,
        43u32 => ResourcePackStatusPacket,
        44u32 => AdvancementTabPacket,
        45u32 => SelectMerchantTradePacket,
        46u32 => UpdateBeaconPacket,
        47u32 => UpdateSelectedSlotC2SPacket,
        48u32 => UpdateCommandBlockPacket,
        49u32 => UpdateCommandBlockMinecartPacket,
        50u32 => CreativeInventoryActionPacket,
        51u32 => UpdateJigsawPacket,
        52u32 => UpdateStructureBlockPacket,
        53u32 => UpdateSignPacket,
        54u32 => HandSwingPacket,
        55u32 => SpectatorTeleportPacket,
        56u32 => PlayerInteractItemPacket,
        57u32 => PlayerInteractBlockPacket,
    },
}
