//! [`Play`](crate::states::Play) state packets for [`V1_20_2`](super::V1_20_2)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

pub use crate::versions::v1_20_0::play::{
    ClientSettingsC2SPacket as ClientOptionsC2SPacket, KeepAliveC2SPacket, KeepAliveS2CPacket,
};

mod bundles2cpacket;
pub use bundles2cpacket::*;

mod entityspawns2cpacket;
pub use entityspawns2cpacket::*;

mod experienceorbspawns2cpacket;
pub use experienceorbspawns2cpacket::*;

mod entityanimations2cpacket;
pub use entityanimations2cpacket::*;

mod statisticss2cpacket;
pub use statisticss2cpacket::*;

mod playeractionresponses2cpacket;
pub use playeractionresponses2cpacket::*;

mod blockbreakingprogresss2cpacket;
pub use blockbreakingprogresss2cpacket::*;

mod blockentityupdates2cpacket;
pub use blockentityupdates2cpacket::*;

mod blockevents2cpacket;
pub use blockevents2cpacket::*;

mod blockupdates2cpacket;
pub use blockupdates2cpacket::*;

mod bossbars2cpacket;
pub use bossbars2cpacket::*;

mod difficultys2cpacket;
pub use difficultys2cpacket::*;

mod chunksents2cpacket;
pub use chunksents2cpacket::*;

mod startchunksends2cpacket;
pub use startchunksends2cpacket::*;

mod chunkbiomedatas2cpacket;
pub use chunkbiomedatas2cpacket::*;

mod cleartitles2cpacket;
pub use cleartitles2cpacket::*;

mod commandsuggestionss2cpacket;
pub use commandsuggestionss2cpacket::*;

mod commandtrees2cpacket;
pub use commandtrees2cpacket::*;

mod closescreens2cpacket;
pub use closescreens2cpacket::*;

mod inventorys2cpacket;
pub use inventorys2cpacket::*;

mod screenhandlerpropertyupdates2cpacket;
pub use screenhandlerpropertyupdates2cpacket::*;

mod screenhandlerslotupdates2cpacket;
pub use screenhandlerslotupdates2cpacket::*;

mod cooldownupdates2cpacket;
pub use cooldownupdates2cpacket::*;

mod chatsuggestionss2cpacket;
pub use chatsuggestionss2cpacket::*;

mod custompayloads2cpacket;
pub use custompayloads2cpacket::*;

mod entitydamages2cpacket;
pub use entitydamages2cpacket::*;

mod removemessages2cpacket;
pub use removemessages2cpacket::*;

mod disconnects2cpacket;
pub use disconnects2cpacket::*;

mod profilelesschatmessages2cpacket;
pub use profilelesschatmessages2cpacket::*;

mod entitystatuss2cpacket;
pub use entitystatuss2cpacket::*;

mod explosions2cpacket;
pub use explosions2cpacket::*;

mod unloadchunks2cpacket;
pub use unloadchunks2cpacket::*;

mod gamestatechanges2cpacket;
pub use gamestatechanges2cpacket::*;

mod openhorsescreens2cpacket;
pub use openhorsescreens2cpacket::*;

mod damagetilts2cpacket;
pub use damagetilts2cpacket::*;

mod worldborderinitializes2cpacket;
pub use worldborderinitializes2cpacket::*;

mod chunkdatas2cpacket;
pub use chunkdatas2cpacket::*;

mod worldevents2cpacket;
pub use worldevents2cpacket::*;

mod particles2cpacket;
pub use particles2cpacket::*;

mod lightupdates2cpacket;
pub use lightupdates2cpacket::*;

mod gamejoins2cpacket;
pub use gamejoins2cpacket::*;

mod mapupdates2cpacket;
pub use mapupdates2cpacket::*;

mod settradeofferss2cpacket;
pub use settradeofferss2cpacket::*;

mod entitys2cpacketmoverelative;
pub use entitys2cpacketmoverelative::*;

mod entitys2cpacketrotateandmoverelative;
pub use entitys2cpacketrotateandmoverelative::*;

mod entitys2cpacketrotate;
pub use entitys2cpacketrotate::*;

mod vehiclemoves2cpacket;
pub use vehiclemoves2cpacket::*;

mod openwrittenbooks2cpacket;
pub use openwrittenbooks2cpacket::*;

mod openscreens2cpacket;
pub use openscreens2cpacket::*;

mod signeditoropens2cpacket;
pub use signeditoropens2cpacket::*;

mod commonpings2cpacket;
pub use commonpings2cpacket::*;

mod pingresults2cpacket;
pub use pingresults2cpacket::*;

mod craftfailedresponses2cpacket;
pub use craftfailedresponses2cpacket::*;

mod playerabilitiess2cpacket;
pub use playerabilitiess2cpacket::*;

mod chatmessages2cpacket;
pub use chatmessages2cpacket::*;

mod endcombats2cpacket;
pub use endcombats2cpacket::*;

mod entercombats2cpacket;
pub use entercombats2cpacket::*;

mod deathmessages2cpacket;
pub use deathmessages2cpacket::*;

mod playerremoves2cpacket;
pub use playerremoves2cpacket::*;

mod playerlists2cpacket;
pub use playerlists2cpacket::*;

mod lookats2cpacket;
pub use lookats2cpacket::*;

mod playerpositionlooks2cpacket;
pub use playerpositionlooks2cpacket::*;

mod unlockrecipess2cpacket;
pub use unlockrecipess2cpacket::*;

mod entitiesdestroys2cpacket;
pub use entitiesdestroys2cpacket::*;

mod removeentitystatuseffects2cpacket;
pub use removeentitystatuseffects2cpacket::*;

mod resourcepacksends2cpacket;
pub use resourcepacksends2cpacket::*;

mod playerrespawns2cpacket;
pub use playerrespawns2cpacket::*;

mod entitysetheadyaws2cpacket;
pub use entitysetheadyaws2cpacket::*;

mod chunkdeltaupdates2cpacket;
pub use chunkdeltaupdates2cpacket::*;

mod selectadvancementtabs2cpacket;
pub use selectadvancementtabs2cpacket::*;

mod servermetadatas2cpacket;
pub use servermetadatas2cpacket::*;

mod overlaymessages2cpacket;
pub use overlaymessages2cpacket::*;

mod worldbordercenterchangeds2cpacket;
pub use worldbordercenterchangeds2cpacket::*;

mod worldborderinterpolatesizes2cpacket;
pub use worldborderinterpolatesizes2cpacket::*;

mod worldbordersizechangeds2cpacket;
pub use worldbordersizechangeds2cpacket::*;

mod worldborderwarningtimechangeds2cpacket;
pub use worldborderwarningtimechangeds2cpacket::*;

mod worldborderwarningblockschangeds2cpacket;
pub use worldborderwarningblockschangeds2cpacket::*;

mod setcameraentitys2cpacket;
pub use setcameraentitys2cpacket::*;

mod updateselectedslots2cpacket;
pub use updateselectedslots2cpacket::*;

mod chunkrenderdistancecenters2cpacket;
pub use chunkrenderdistancecenters2cpacket::*;

mod chunkloaddistances2cpacket;
pub use chunkloaddistances2cpacket::*;

mod playerspawnpositions2cpacket;
pub use playerspawnpositions2cpacket::*;

mod scoreboarddisplays2cpacket;
pub use scoreboarddisplays2cpacket::*;

mod entitytrackerupdates2cpacket;
pub use entitytrackerupdates2cpacket::*;

mod entityattachs2cpacket;
pub use entityattachs2cpacket::*;

mod entityvelocityupdates2cpacket;
pub use entityvelocityupdates2cpacket::*;

mod entityequipmentupdates2cpacket;
pub use entityequipmentupdates2cpacket::*;

mod experiencebarupdates2cpacket;
pub use experiencebarupdates2cpacket::*;

mod healthupdates2cpacket;
pub use healthupdates2cpacket::*;

mod scoreboardobjectiveupdates2cpacket;
pub use scoreboardobjectiveupdates2cpacket::*;

mod entitypassengerssets2cpacket;
pub use entitypassengerssets2cpacket::*;

mod teams2cpacket;
pub use teams2cpacket::*;

mod scoreboardplayerupdates2cpacket;
pub use scoreboardplayerupdates2cpacket::*;

mod simulationdistances2cpacket;
pub use simulationdistances2cpacket::*;

mod subtitles2cpacket;
pub use subtitles2cpacket::*;

mod worldtimeupdates2cpacket;
pub use worldtimeupdates2cpacket::*;

mod titles2cpacket;
pub use titles2cpacket::*;

mod titlefades2cpacket;
pub use titlefades2cpacket::*;

mod playsoundfromentitys2cpacket;
pub use playsoundfromentitys2cpacket::*;

mod playsounds2cpacket;
pub use playsounds2cpacket::*;

mod enterreconfigurations2cpacket;
pub use enterreconfigurations2cpacket::*;

mod stopsounds2cpacket;
pub use stopsounds2cpacket::*;

mod gamemessages2cpacket;
pub use gamemessages2cpacket::*;

mod playerlistheaders2cpacket;
pub use playerlistheaders2cpacket::*;

mod nbtqueryresponses2cpacket;
pub use nbtqueryresponses2cpacket::*;

mod itempickupanimations2cpacket;
pub use itempickupanimations2cpacket::*;

mod entitypositions2cpacket;
pub use entitypositions2cpacket::*;

mod advancementupdates2cpacket;
pub use advancementupdates2cpacket::*;

mod entityattributess2cpacket;
pub use entityattributess2cpacket::*;

mod entitystatuseffects2cpacket;
pub use entitystatuseffects2cpacket::*;

mod synchronizerecipess2cpacket;
pub use synchronizerecipess2cpacket::*;

mod synchronizetagss2cpacket;
pub use synchronizetagss2cpacket::*;

mod teleportconfirmc2spacket;
pub use teleportconfirmc2spacket::*;

mod queryblocknbtc2spacket;
pub use queryblocknbtc2spacket::*;

mod updatedifficultyc2spacket;
pub use updatedifficultyc2spacket::*;

mod messageacknowledgmentc2spacket;
pub use messageacknowledgmentc2spacket::*;

mod commandexecutionc2spacket;
pub use commandexecutionc2spacket::*;

mod chatmessagec2spacket;
pub use chatmessagec2spacket::*;

mod playersessionc2spacket;
pub use playersessionc2spacket::*;

mod acknowledgechunksc2spacket;
pub use acknowledgechunksc2spacket::*;

mod clientstatusc2spacket;
pub use clientstatusc2spacket::*;

mod requestcommandcompletionsc2spacket;
pub use requestcommandcompletionsc2spacket::*;

mod acknowledgereconfigurationc2spacket;
pub use acknowledgereconfigurationc2spacket::*;

mod buttonclickc2spacket;
pub use buttonclickc2spacket::*;

mod clickslotc2spacket;
pub use clickslotc2spacket::*;

mod closehandledscreenc2spacket;
pub use closehandledscreenc2spacket::*;

mod custompayloadc2spacket;
pub use custompayloadc2spacket::*;

mod bookupdatec2spacket;
pub use bookupdatec2spacket::*;

mod queryentitynbtc2spacket;
pub use queryentitynbtc2spacket::*;

mod playerinteractentityc2spacket;
pub use playerinteractentityc2spacket::*;

mod jigsawgeneratingc2spacket;
pub use jigsawgeneratingc2spacket::*;

mod updatedifficultylockc2spacket;
pub use updatedifficultylockc2spacket::*;

mod playermovec2spacketpositionandonground;
pub use playermovec2spacketpositionandonground::*;

mod playermovec2spacketfull;
pub use playermovec2spacketfull::*;

mod playermovec2spacketlookandonground;
pub use playermovec2spacketlookandonground::*;

mod playermovec2spacketongroundonly;
pub use playermovec2spacketongroundonly::*;

mod vehiclemovec2spacket;
pub use vehiclemovec2spacket::*;

mod boatpaddlestatec2spacket;
pub use boatpaddlestatec2spacket::*;

mod pickfrominventoryc2spacket;
pub use pickfrominventoryc2spacket::*;

mod querypingc2spacket;
pub use querypingc2spacket::*;

mod craftrequestc2spacket;
pub use craftrequestc2spacket::*;

mod updateplayerabilitiesc2spacket;
pub use updateplayerabilitiesc2spacket::*;

mod playeractionc2spacket;
pub use playeractionc2spacket::*;

mod clientcommandc2spacket;
pub use clientcommandc2spacket::*;

mod playerinputc2spacket;
pub use playerinputc2spacket::*;

mod commonpongc2spacket;
pub use commonpongc2spacket::*;

mod recipecategoryoptionsc2spacket;
pub use recipecategoryoptionsc2spacket::*;

mod recipebookdatac2spacket;
pub use recipebookdatac2spacket::*;

mod renameitemc2spacket;
pub use renameitemc2spacket::*;

mod resourcepackstatusc2spacket;
pub use resourcepackstatusc2spacket::*;

mod advancementtabc2spacket;
pub use advancementtabc2spacket::*;

mod selectmerchanttradec2spacket;
pub use selectmerchanttradec2spacket::*;

mod updatebeaconc2spacket;
pub use updatebeaconc2spacket::*;

mod updateselectedslotc2spacket;
pub use updateselectedslotc2spacket::*;

mod updatecommandblockc2spacket;
pub use updatecommandblockc2spacket::*;

mod updatecommandblockminecartc2spacket;
pub use updatecommandblockminecartc2spacket::*;

mod creativeinventoryactionc2spacket;
pub use creativeinventoryactionc2spacket::*;

mod updatejigsawc2spacket;
pub use updatejigsawc2spacket::*;

mod updatestructureblockc2spacket;
pub use updatestructureblockc2spacket::*;

mod updatesignc2spacket;
pub use updatesignc2spacket::*;

mod handswingc2spacket;
pub use handswingc2spacket::*;

mod spectatorteleportc2spacket;
pub use spectatorteleportc2spacket::*;

mod playerinteractblockc2spacket;
pub use playerinteractblockc2spacket::*;

mod playerinteractitemc2spacket;
pub use playerinteractitemc2spacket::*;

frog_state! {
    Play,
    V1_20_2,
    Clientbound {
        0u32 => BundleS2CPacket,
        1u32 => EntitySpawnS2CPacket,
        2u32 => ExperienceOrbSpawnS2CPacket,
        3u32 => EntityAnimationS2CPacket,
        4u32 => StatisticsS2CPacket,
        5u32 => PlayerActionResponseS2CPacket,
        6u32 => BlockBreakingProgressS2CPacket,
        7u32 => BlockEntityUpdateS2CPacket,
        8u32 => BlockEventS2CPacket,
        9u32 => BlockUpdateS2CPacket,
        10u32 => BossBarS2CPacket,
        11u32 => DifficultyS2CPacket,
        12u32 => ChunkSentS2CPacket,
        13u32 => StartChunkSendS2CPacket,
        14u32 => ChunkBiomeDataS2CPacket,
        15u32 => ClearTitleS2CPacket,
        16u32 => CommandSuggestionsS2CPacket,
        17u32 => CommandTreeS2CPacket,
        18u32 => CloseScreenS2CPacket,
        19u32 => InventoryS2CPacket,
        20u32 => ScreenHandlerPropertyUpdateS2CPacket,
        21u32 => ScreenHandlerSlotUpdateS2CPacket,
        22u32 => CooldownUpdateS2CPacket,
        23u32 => ChatSuggestionsS2CPacket,
        24u32 => CustomPayloadS2CPacket,
        25u32 => EntityDamageS2CPacket,
        26u32 => RemoveMessageS2CPacket,
        27u32 => DisconnectS2CPacket,
        28u32 => ProfilelessChatMessageS2CPacket,
        29u32 => EntityStatusS2CPacket,
        30u32 => ExplosionS2CPacket,
        31u32 => UnloadChunkS2CPacket,
        32u32 => GameStateChangeS2CPacket,
        33u32 => OpenHorseScreenS2CPacket,
        34u32 => DamageTiltS2CPacket,
        35u32 => WorldBorderInitializeS2CPacket,
        36u32 => KeepAliveS2CPacket,
        37u32 => ChunkDataS2CPacket,
        38u32 => WorldEventS2CPacket,
        39u32 => ParticleS2CPacket,
        40u32 => LightUpdateS2CPacket,
        41u32 => GameJoinS2CPacket,
        42u32 => MapUpdateS2CPacket,
        43u32 => SetTradeOffersS2CPacket,
        44u32 => EntityS2CPacketMoveRelative,
        45u32 => EntityS2CPacketRotateAndMoveRelative,
        46u32 => EntityS2CPacketRotate,
        47u32 => VehicleMoveS2CPacket,
        48u32 => OpenWrittenBookS2CPacket,
        49u32 => OpenScreenS2CPacket,
        50u32 => SignEditorOpenS2CPacket,
        51u32 => CommonPingS2CPacket,
        52u32 => PingResultS2CPacket,
        53u32 => CraftFailedResponseS2CPacket,
        54u32 => PlayerAbilitiesS2CPacket,
        55u32 => ChatMessageS2CPacket,
        56u32 => EndCombatS2CPacket,
        57u32 => EnterCombatS2CPacket,
        58u32 => DeathMessageS2CPacket,
        59u32 => PlayerRemoveS2CPacket,
        60u32 => PlayerListS2CPacket,
        61u32 => LookAtS2CPacket,
        62u32 => PlayerPositionLookS2CPacket,
        63u32 => UnlockRecipesS2CPacket,
        64u32 => EntitiesDestroyS2CPacket,
        65u32 => RemoveEntityStatusEffectS2CPacket,
        66u32 => ResourcePackSendS2CPacket,
        67u32 => PlayerRespawnS2CPacket,
        68u32 => EntitySetHeadYawS2CPacket,
        69u32 => ChunkDeltaUpdateS2CPacket,
        70u32 => SelectAdvancementTabS2CPacket,
        71u32 => ServerMetadataS2CPacket,
        72u32 => OverlayMessageS2CPacket,
        73u32 => WorldBorderCenterChangedS2CPacket,
        74u32 => WorldBorderInterpolateSizeS2CPacket,
        75u32 => WorldBorderSizeChangedS2CPacket,
        76u32 => WorldBorderWarningTimeChangedS2CPacket,
        77u32 => WorldBorderWarningBlocksChangedS2CPacket,
        78u32 => SetCameraEntityS2CPacket,
        79u32 => UpdateSelectedSlotS2CPacket,
        80u32 => ChunkRenderDistanceCenterS2CPacket,
        81u32 => ChunkLoadDistanceS2CPacket,
        82u32 => PlayerSpawnPositionS2CPacket,
        83u32 => ScoreboardDisplayS2CPacket,
        84u32 => EntityTrackerUpdateS2CPacket,
        85u32 => EntityAttachS2CPacket,
        86u32 => EntityVelocityUpdateS2CPacket,
        87u32 => EntityEquipmentUpdateS2CPacket,
        88u32 => ExperienceBarUpdateS2CPacket,
        89u32 => HealthUpdateS2CPacket,
        90u32 => ScoreboardObjectiveUpdateS2CPacket,
        91u32 => EntityPassengersSetS2CPacket,
        92u32 => TeamS2CPacket,
        93u32 => ScoreboardPlayerUpdateS2CPacket,
        94u32 => SimulationDistanceS2CPacket,
        95u32 => SubtitleS2CPacket,
        96u32 => WorldTimeUpdateS2CPacket,
        97u32 => TitleS2CPacket,
        98u32 => TitleFadeS2CPacket,
        99u32 => PlaySoundFromEntityS2CPacket,
        100u32 => PlaySoundS2CPacket,
        101u32 => EnterReconfigurationS2CPacket,
        102u32 => StopSoundS2CPacket,
        103u32 => GameMessageS2CPacket,
        104u32 => PlayerListHeaderS2CPacket,
        105u32 => NbtQueryResponseS2CPacket,
        106u32 => ItemPickupAnimationS2CPacket,
        107u32 => EntityPositionS2CPacket,
        108u32 => AdvancementUpdateS2CPacket,
        109u32 => EntityAttributesS2CPacket,
        110u32 => EntityStatusEffectS2CPacket,
        111u32 => SynchronizeRecipesS2CPacket,
        112u32 => SynchronizeTagsS2CPacket,
    },
    Serverbound {
        0u32 => TeleportConfirmC2SPacket,
        1u32 => QueryBlockNbtC2SPacket,
        2u32 => UpdateDifficultyC2SPacket,
        3u32 => MessageAcknowledgmentC2SPacket,
        4u32 => CommandExecutionC2SPacket,
        5u32 => ChatMessageC2SPacket,
        6u32 => PlayerSessionC2SPacket,
        7u32 => AcknowledgeChunksC2SPacket,
        8u32 => ClientStatusC2SPacket,
        9u32 => ClientOptionsC2SPacket,
        10u32 => RequestCommandCompletionsC2SPacket,
        11u32 => AcknowledgeReconfigurationC2SPacket,
        12u32 => ButtonClickC2SPacket,
        13u32 => ClickSlotC2SPacket,
        14u32 => CloseHandledScreenC2SPacket,
        15u32 => CustomPayloadC2SPacket,
        16u32 => BookUpdateC2SPacket,
        17u32 => QueryEntityNbtC2SPacket,
        18u32 => PlayerInteractEntityC2SPacket,
        19u32 => JigsawGeneratingC2SPacket,
        20u32 => KeepAliveC2SPacket,
        21u32 => UpdateDifficultyLockC2SPacket,
        22u32 => PlayerMoveC2SPacketPositionAndOnGround,
        23u32 => PlayerMoveC2SPacketFull,
        24u32 => PlayerMoveC2SPacketLookAndOnGround,
        25u32 => PlayerMoveC2SPacketOnGroundOnly,
        26u32 => VehicleMoveC2SPacket,
        27u32 => BoatPaddleStateC2SPacket,
        28u32 => PickFromInventoryC2SPacket,
        29u32 => QueryPingC2SPacket,
        30u32 => CraftRequestC2SPacket,
        31u32 => UpdatePlayerAbilitiesC2SPacket,
        32u32 => PlayerActionC2SPacket,
        33u32 => ClientCommandC2SPacket,
        34u32 => PlayerInputC2SPacket,
        35u32 => CommonPongC2SPacket,
        36u32 => RecipeCategoryOptionsC2SPacket,
        37u32 => RecipeBookDataC2SPacket,
        38u32 => RenameItemC2SPacket,
        39u32 => ResourcePackStatusC2SPacket,
        40u32 => AdvancementTabC2SPacket,
        41u32 => SelectMerchantTradeC2SPacket,
        42u32 => UpdateBeaconC2SPacket,
        43u32 => UpdateSelectedSlotC2SPacket,
        44u32 => UpdateCommandBlockC2SPacket,
        45u32 => UpdateCommandBlockMinecartC2SPacket,
        46u32 => CreativeInventoryActionC2SPacket,
        47u32 => UpdateJigsawC2SPacket,
        48u32 => UpdateStructureBlockC2SPacket,
        49u32 => UpdateSignC2SPacket,
        50u32 => HandSwingC2SPacket,
        51u32 => SpectatorTeleportC2SPacket,
        52u32 => PlayerInteractBlockC2SPacket,
        53u32 => PlayerInteractItemC2SPacket,
    },
}