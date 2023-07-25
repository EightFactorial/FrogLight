use std::{
    io::Write,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use git2::Repository;
use itertools::Itertools;
use json::JsonValue;
use log::{error, warn};
use mc_rs_ext::{
    extract::datasets::{self, Datasets},
    types::Version,
};

use crate::util::{create_file_with, create_module};

use super::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packets;

impl Generator for Packets {
    fn deps(&self) -> &'static [Datasets] {
        &[
            Datasets::Packets(datasets::packet::Packets),
            Datasets::PacketFields(datasets::packet::PacketFields),
        ]
    }

    fn parse(&self, version: &Version, data: &JsonValue, repo: &Repository) {
        let mut path: PathBuf = repo.path().parent().unwrap().into();
        path.push("crates/mc-rs-proto");

        // Check if the mc-rs-proto crate exists
        if !path.exists() {
            error!("mc-rs-proto crate not found at {}", path.display());
            error!("Is this the right git repository?");
            return;
        }

        // Create the version directory
        path.push(format!(
            "src/versions/v{}",
            version.to_string().replace('.', "_")
        ));
        if !path.exists() {
            if let Err(err) = std::fs::create_dir_all(&path) {
                error!("Failed to create directory {}: {}", path.display(), err);
                return;
            };
        }

        let JsonValue::Array(names) = &data["packets"]["states"]["names"] else {
            error!("Failed to get packet names");
            return;
        };
        let names = names
            .iter()
            .map(|name| name.as_str().unwrap())
            .collect_vec();

        let states = names
            .iter()
            .map(|&name| &data["packets"]["states"]["data"][name])
            .collect_vec();

        // Generate submodules
        for (&state, &name) in states.iter().zip(names.iter()) {
            if let Err(e) =
                Self::generate_state(&data["packets"]["fields"], state, name, path.clone())
            {
                error!("Failed to generate state {}: {}", name, e);
            }
        }

        // Generate the module file
        if let Err(e) = create_module(&path) {
            error!("Failed to generate module file: {}", e);
        }
    }
}

impl Packets {
    /// Generate the connection state
    fn generate_state(
        fields: &JsonValue,
        state: &JsonValue,
        name: &str,
        mut path: PathBuf,
    ) -> Result<(), std::io::Error> {
        path.push(name.to_ascii_lowercase());

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        // Generate the packets for each direction
        let mut packet_groups: Vec<(&str, Vec<String>)> = Vec::new();
        for direction in ["clientbound", "serverbound"] {
            let packets = Self::generate_direction(fields, state, direction, &path)?;
            packet_groups.push((direction, packets));
        }

        // Generate the module file
        let Some(mut file) = create_module(&path)? else {
            warn!("File {} already exists, skipping", &path.display());
            return Ok(());
        };

        // Write the state macro
        writeln!(file, "// TODO: Write state macro")?;
        for (_direction, _packets) in packet_groups {
            // writeln!(file, "// TODO: Write part of state macro")?;
        }

        Ok(())
    }

    /// Generate the packet direction part of the state
    fn generate_direction(
        fields: &JsonValue,
        state: &JsonValue,
        direction: &str,
        path: &Path,
    ) -> Result<Vec<String>, std::io::Error> {
        let mut packets = Vec::new();

        for (class_name, _) in state[direction].entries() {
            packets.push(Self::generate_packet(
                fields,
                class_name,
                direction,
                path.into(),
            )?);
        }

        Ok(packets)
    }

    /// The header for all generated structs
    const PACKET_HEADER: &'static str = "#[derive(Debug, Clone, Transcode)]";

    /// Generate the packet struct
    fn generate_packet(
        fields: &JsonValue,
        class_name: &str,
        direction: &str,
        mut path: PathBuf,
    ) -> Result<String, std::io::Error> {
        // Get the packet name
        let packet_name = match get_packet_name(class_name) {
            Some(name) => format!("{}{}", direction.to_case(Case::Pascal), name),
            None => {
                error!("Failed to get packet name for {}", class_name);
                class_name.to_owned()
            }
        };

        // Get the file name
        let file_name = packet_name.to_ascii_lowercase();
        path.push(format!("{}.rs", file_name));

        // Get fields
        let JsonValue::Array(ref fields) = fields[class_name] else {
            panic!("Fields for {} is not an array!", class_name);
        };
        let fields = fields.iter().map(|v| v.as_str().unwrap()).collect_vec();

        // Create the file
        let file = create_file_with(&get_imports(&fields), &path)?;
        let Some(mut file) = file else {
            warn!("File {} already exists, skipping", &path.display());
            return Ok(packet_name);
        };

        // Write struct
        writeln!(file, "{}", Self::PACKET_HEADER)?;
        writeln!(file, "pub struct {} {{", packet_name)?;
        for (field, field_name) in fields.into_iter().zip("abcdefghijklmnopqrstuvwxyz".chars()) {
            writeln!(file, "    pub {}: {},", field_name, field)?;
        }
        writeln!(file, "}}")?;

        Ok(packet_name)
    }
}

/// Get imports for the packet
fn get_imports(fields: &[&str]) -> Vec<String> {
    let fields = fields.iter().cloned().unique().collect_vec();

    let mut imports = vec!["mc_rs_macros::Transcode".to_string()];
    for field in fields {
        let import = match field {
            "Uuid" => Some("uuid::Uuid".to_string()),
            "HashMap" => Some("hashbrown::HashMap".to_string()),
            "ResourceLocation" => Some("crate::types::ResourceLocation".to_string()),
            //            "ResourceEntry" => Some("crate::types::ResourceEntry".to_string()),
            //            "GameProfile" => Some("crate::types::GameProfile".to_string()),
            "UnsizedByteBuffer" => Some("crate::types::UnsizedByteBuffer".to_string()),
            _ => None,
        };

        if let Some(import) = import {
            imports.push(import);
        }
    }

    imports
}

// Generated with:
// grep '' ./yarn-1.20.1+build.10-tiny |
//     awk '/\/*Packet$/ {print "\"" $3 "\"" " => " "\"" $4 "\","}'
//
// Not perfect, requires a bit of manual formatting
// and misses some packets, but good enough

/// Get the packet name from the class name
fn get_packet_name(class: &str) -> Option<String> {
    let string = match class {
        "net/minecraft/class_8037" => "BundleSplitterPacket",
        "net/minecraft/class_8038" => "BundlePacket",
        "net/minecraft/class_2596" => "Packet",
        "net/minecraft/class_2604" => "EntitySpawnS2CPacket",
        "net/minecraft/class_2606" => "ExperienceOrbSpawnS2CPacket",
        "net/minecraft/class_2613" => "PlayerSpawnS2CPacket",
        "net/minecraft/class_2616" => "EntityAnimationS2CPacket",
        "net/minecraft/class_2617" => "StatisticsS2CPacket",
        "net/minecraft/class_4463" => "PlayerActionResponseS2CPacket",
        "net/minecraft/class_2620" => "BlockBreakingProgressS2CPacket",
        "net/minecraft/class_2622" => "BlockEntityUpdateS2CPacket",
        "net/minecraft/class_2623" => "BlockEventS2CPacket",
        "net/minecraft/class_2626" => "BlockUpdateS2CPacket",
        "net/minecraft/class_2629" => "BossBarS2CPacket",
        "net/minecraft/class_8042" => "BundleS2CPacket",
        "net/minecraft/class_2632" => "DifficultyS2CPacket",
        "net/minecraft/class_8212" => "ChunkBiomeDataS2CPacket",
        "net/minecraft/class_5888" => "ClearTitleS2CPacket",
        "net/minecraft/class_2639" => "CommandSuggestionsS2CPacket",
        "net/minecraft/class_2641" => "CommandTreeS2CPacket",
        "net/minecraft/class_2645" => "CloseScreenS2CPacket",
        "net/minecraft/class_2649" => "InventoryS2CPacket",
        "net/minecraft/class_2651" => "ScreenHandlerPropertyUpdateS2CPacket",
        "net/minecraft/class_2653" => "ScreenHandlerSlotUpdateS2CPacket",
        "net/minecraft/class_2656" => "CooldownUpdateS2CPacket",
        "net/minecraft/class_7597" => "ChatSuggestionsS2CPacket",
        "net/minecraft/class_2658" => "CustomPayloadS2CPacket",
        "net/minecraft/class_8143" => "EntityDamageS2CPacket",
        "net/minecraft/class_7617" => "RemoveMessageS2CPacket",
        "net/minecraft/class_2661" => "DisconnectS2CPacket",
        "net/minecraft/class_7827" => "ProfilelessChatMessageS2CPacket",
        "net/minecraft/class_2663" => "EntityStatusS2CPacket",
        "net/minecraft/class_2664" => "ExplosionS2CPacket",
        "net/minecraft/class_2666" => "UnloadChunkS2CPacket",
        "net/minecraft/class_2668" => "GameStateChangeS2CPacket",
        "net/minecraft/class_2648" => "OpenHorseScreenS2CPacket",
        "net/minecraft/class_8043" => "DamageTiltS2CPacket",
        "net/minecraft/class_5889" => "WorldBorderInitializeS2CPacket",
        "net/minecraft/class_2670" => "KeepAliveS2CPacket",
        "net/minecraft/class_2672" => "ChunkDataS2CPacket",
        "net/minecraft/class_2673" => "WorldEventS2CPacket",
        "net/minecraft/class_2675" => "ParticleS2CPacket",
        "net/minecraft/class_2676" => "LightUpdateS2CPacket",
        "net/minecraft/class_2678" => "GameJoinS2CPacket",
        "net/minecraft/class_2683" => "MapUpdateS2CPacket",
        "net/minecraft/class_3943" => "SetTradeOffersS2CPacket",
        "net/minecraft/class_2684" => "EntityS2CPacket",
        "net/minecraft/class_2684$class_2685" => "EntityMoveRelativeS2CPacket",
        "net/minecraft/class_2684$class_2686" => "EntityRotateAndMoveRelativeS2CPacket",
        "net/minecraft/class_2684$class_2687" => "EntityRotateS2CPacket",
        "net/minecraft/class_2692" => "VehicleMoveS2CPacket",
        "net/minecraft/class_3895" => "OpenWrittenBookS2CPacket",
        "net/minecraft/class_3944" => "OpenScreenS2CPacket",
        "net/minecraft/class_2693" => "SignEditorOpenS2CPacket",
        "net/minecraft/class_6373" => "PlayPingS2CPacket",
        "net/minecraft/class_2695" => "CraftFailedResponseS2CPacket",
        "net/minecraft/class_2696" => "PlayerAbilitiesS2CPacket",
        "net/minecraft/class_7438" => "ChatMessageS2CPacket",
        "net/minecraft/class_5890" => "EndCombatS2CPacket",
        "net/minecraft/class_5891" => "EnterCombatS2CPacket",
        "net/minecraft/class_5892" => "DeathMessageS2CPacket",
        "net/minecraft/class_7828" => "PlayerRemoveS2CPacket",
        "net/minecraft/class_2703" => "PlayerListS2CPacket",
        "net/minecraft/class_2707" => "LookAtS2CPacket",
        "net/minecraft/class_2708" => "PlayerPositionLookS2CPacket",
        "net/minecraft/class_2713" => "UnlockRecipesS2CPacket",
        "net/minecraft/class_2716" => "EntitiesDestroyS2CPacket",
        "net/minecraft/class_2718" => "RemoveEntityStatusEffectS2CPacket",
        "net/minecraft/class_2720" => "ResourcePackSendS2CPacket",
        "net/minecraft/class_2724" => "PlayerRespawnS2CPacket",
        "net/minecraft/class_2726" => "EntitySetHeadYawS2CPacket",
        "net/minecraft/class_2637" => "ChunkDeltaUpdateS2CPacket",
        "net/minecraft/class_2729" => "SelectAdvancementTabS2CPacket",
        "net/minecraft/class_7495" => "ServerMetadataS2CPacket",
        "net/minecraft/class_5894" => "OverlayMessageS2CPacket",
        "net/minecraft/class_5895" => "WorldBorderCenterChangedS2CPacket",
        "net/minecraft/class_5896" => "WorldBorderInterpolateSizeS2CPacket",
        "net/minecraft/class_5897" => "WorldBorderSizeChangedS2CPacket",
        "net/minecraft/class_5898" => "WorldBorderWarningTimeChangedS2CPacket",
        "net/minecraft/class_5899" => "WorldBorderWarningBlocksChangedS2CPacket",
        "net/minecraft/class_2734" => "SetCameraEntityS2CPacket",
        "net/minecraft/class_2735" => "UpdateSelectedSlotS2CPacket",
        "net/minecraft/class_4282" => "ChunkRenderDistanceCenterS2CPacket",
        "net/minecraft/class_4273" => "ChunkLoadDistanceS2CPacket",
        "net/minecraft/class_2759" => "PlayerSpawnPositionS2CPacket",
        "net/minecraft/class_2736" => "ScoreboardDisplayS2CPacket",
        "net/minecraft/class_2739" => "EntityTrackerUpdateS2CPacket",
        "net/minecraft/class_2740" => "EntityAttachS2CPacket",
        "net/minecraft/class_2743" => "EntityVelocityUpdateS2CPacket",
        "net/minecraft/class_2744" => "EntityEquipmentUpdateS2CPacket",
        "net/minecraft/class_2748" => "ExperienceBarUpdateS2CPacket",
        "net/minecraft/class_2749" => "HealthUpdateS2CPacket",
        "net/minecraft/class_2751" => "ScoreboardObjectiveUpdateS2CPacket",
        "net/minecraft/class_2752" => "EntityPassengersSetS2CPacket",
        "net/minecraft/class_5900" => "TeamS2CPacket",
        "net/minecraft/class_2757" => "ScoreboardPlayerUpdateS2CPacket",
        "net/minecraft/class_6682" => "SimulationDistanceS2CPacket",
        "net/minecraft/class_5903" => "SubtitleS2CPacket",
        "net/minecraft/class_2761" => "WorldTimeUpdateS2CPacket",
        "net/minecraft/class_5904" => "TitleS2CPacket",
        "net/minecraft/class_5905" => "TitleFadeS2CPacket",
        "net/minecraft/class_2765" => "PlaySoundFromEntityS2CPacket",
        "net/minecraft/class_2767" => "PlaySoundS2CPacket",
        "net/minecraft/class_2770" => "StopSoundS2CPacket",
        "net/minecraft/class_7439" => "GameMessageS2CPacket",
        "net/minecraft/class_2772" => "PlayerListHeaderS2CPacket",
        "net/minecraft/class_2774" => "NbtQueryResponseS2CPacket",
        "net/minecraft/class_2775" => "ItemPickupAnimationS2CPacket",
        "net/minecraft/class_2777" => "EntityPositionS2CPacket",
        "net/minecraft/class_2779" => "AdvancementUpdateS2CPacket",
        "net/minecraft/class_2781" => "EntityAttributesS2CPacket",
        "net/minecraft/class_7832" => "FeaturesS2CPacket",
        "net/minecraft/class_2783" => "EntityStatusEffectS2CPacket",
        "net/minecraft/class_2788" => "SynchronizeRecipesS2CPacket",
        "net/minecraft/class_2790" => "SynchronizeTagsS2CPacket",
        "net/minecraft/class_2793" => "TeleportConfirmC2SPacket",
        "net/minecraft/class_2795" => "QueryBlockNbtC2SPacket",
        "net/minecraft/class_4210" => "UpdateDifficultyC2SPacket",
        "net/minecraft/class_7640" => "MessageAcknowledgmentC2SPacket",
        "net/minecraft/class_7472" => "CommandExecutionC2SPacket",
        "net/minecraft/class_2797" => "ChatMessageC2SPacket",
        "net/minecraft/class_7861" => "PlayerSessionC2SPacket",
        "net/minecraft/class_2799" => "ClientStatusC2SPacket",
        "net/minecraft/class_2803" => "ClientSettingsC2SPacket",
        "net/minecraft/class_2805" => "RequestCommandCompletionsC2SPacket",
        "net/minecraft/class_2811" => "ButtonClickC2SPacket",
        "net/minecraft/class_2813" => "ClickSlotC2SPacket",
        "net/minecraft/class_2815" => "CloseHandledScreenC2SPacket",
        "net/minecraft/class_2817" => "CustomPayloadC2SPacket",
        "net/minecraft/class_2820" => "BookUpdateC2SPacket",
        "net/minecraft/class_2822" => "QueryEntityNbtC2SPacket",
        "net/minecraft/class_2824" => "PlayerInteractEntityC2SPacket",
        "net/minecraft/class_5194" => "JigsawGeneratingC2SPacket",
        "net/minecraft/class_2827" => "KeepAliveC2SPacket",
        "net/minecraft/class_4211" => "UpdateDifficultyLockC2SPacket",
        "net/minecraft/class_2828" => "PlayerMoveC2SPacket",
        "net/minecraft/class_2828$class_2829" => "PlayerMovePositionAndOnGroundC2SPacket",
        "net/minecraft/class_2828$class_2830" => "PlayerMoveFullC2SPacket",
        "net/minecraft/class_2828$class_2831" => "PlayerMoveLookAndOnGroundC2SPacket",
        "net/minecraft/class_2828$class_5911" => "PlayerMoveOnGroundOnlyC2SPacket",
        "net/minecraft/class_2833" => "VehicleMoveC2SPacket",
        "net/minecraft/class_2836" => "BoatPaddleStateC2SPacket",
        "net/minecraft/class_2838" => "PickFromInventoryC2SPacket",
        "net/minecraft/class_2840" => "CraftRequestC2SPacket",
        "net/minecraft/class_2842" => "UpdatePlayerAbilitiesC2SPacket",
        "net/minecraft/class_2846" => "PlayerActionC2SPacket",
        "net/minecraft/class_2848" => "ClientCommandC2SPacket",
        "net/minecraft/class_2851" => "PlayerInputC2SPacket",
        "net/minecraft/class_6374" => "PlayPongC2SPacket",
        "net/minecraft/class_5427" => "RecipeCategoryOptionsC2SPacket",
        "net/minecraft/class_2853" => "RecipeBookDataC2SPacket",
        "net/minecraft/class_2855" => "RenameItemC2SPacket",
        "net/minecraft/class_2856" => "ResourcePackStatusC2SPacket",
        "net/minecraft/class_2859" => "AdvancementTabC2SPacket",
        "net/minecraft/class_2863" => "SelectMerchantTradeC2SPacket",
        "net/minecraft/class_2866" => "UpdateBeaconC2SPacket",
        "net/minecraft/class_2868" => "UpdateSelectedSlotC2SPacket",
        "net/minecraft/class_2870" => "UpdateCommandBlockC2SPacket",
        "net/minecraft/class_2871" => "UpdateCommandBlockMinecartC2SPacket",
        "net/minecraft/class_2873" => "CreativeInventoryActionC2SPacket",
        "net/minecraft/class_3753" => "UpdateJigsawC2SPacket",
        "net/minecraft/class_2875" => "UpdateStructureBlockC2SPacket",
        "net/minecraft/class_2877" => "UpdateSignC2SPacket",
        "net/minecraft/class_2879" => "HandSwingC2SPacket",
        "net/minecraft/class_2884" => "SpectatorTeleportC2SPacket",
        "net/minecraft/class_2885" => "PlayerInteractBlockC2SPacket",
        "net/minecraft/class_2886" => "PlayerInteractItemC2SPacket",
        "net/minecraft/class_2889" => "HandshakeC2SPacket",
        "net/minecraft/class_2899" => "LoginQueryRequestS2CPacket",
        "net/minecraft/class_2901" => "LoginSuccessS2CPacket",
        "net/minecraft/class_2905" => "LoginHelloS2CPacket",
        "net/minecraft/class_2907" => "LoginCompressionS2CPacket",
        "net/minecraft/class_2909" => "LoginDisconnectS2CPacket",
        "net/minecraft/class_2913" => "LoginQueryResponseC2SPacket",
        "net/minecraft/class_2915" => "LoginHelloC2SPacket",
        "net/minecraft/class_2917" => "LoginKeyC2SPacket",
        "net/minecraft/class_2923" => "QueryPongS2CPacket",
        "net/minecraft/class_2924" => "QueryResponseS2CPacket",
        "net/minecraft/class_2935" => "QueryPingC2SPacket",
        "net/minecraft/class_2937" => "QueryRequestC2SPacket",
        _ => return None,
    };

    Some(string.to_owned())
}
