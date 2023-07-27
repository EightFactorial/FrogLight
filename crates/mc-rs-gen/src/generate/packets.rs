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

use crate::util::{create_file_with, create_module_with};

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
        let version_name = version.to_string().replace('.', "_");
        path.push(format!("src/versions/v{}", version_name));
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
            if let Err(e) = Self::generate_state(
                &data["packets"]["fields"],
                state,
                name,
                &version_name,
                path.clone(),
            ) {
                error!("Failed to generate state {}: {}", name, e);
            }
        }

        // Generate the module file
        let Ok(file) = create_module_with(&["crate::Version".to_string()], &path) else {
            error!("Failed to generate module file: {}", path.display());
            return;
        };
        let Some(mut file) = file else {
            warn!(
                "File {} already exists, skipping",
                path.join("mod.rs").display()
            );
            return;
        };

        // Write the version struct
        writeln!(
            file,
            "#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]"
        )
        .unwrap();
        writeln!(file, "pub struct V{};", version_name).unwrap();
        writeln!(file).unwrap();

        // Implement the Version trait
        writeln!(file, "impl Version for V{} {{", version_name).unwrap();
        writeln!(file, "    const ID: i32 = 0;").unwrap();
        writeln!(file, "}}").unwrap();
    }
}

impl Packets {
    /// Generate the connection state
    fn generate_state(
        fields: &JsonValue,
        state: &JsonValue,
        name: &str,
        version: &str,
        mut path: PathBuf,
    ) -> Result<(), std::io::Error> {
        path.push(name.to_ascii_lowercase());

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        // Generate the packets for each direction
        let mut packet_groups: Vec<(&str, Vec<(String, i32)>)> = Vec::new();
        for direction in ["clientbound", "serverbound"] {
            let packets = Self::generate_direction(fields, state, direction, &path)?;
            packet_groups.push((direction, packets));
        }

        let imports = vec![
            "mc_rs_macros::impl_state".to_owned(),
            format!("crate::versions::state::{}", name.to_case(Case::Pascal)),
            format!("super::V{}", version),
        ];

        // Generate the module file
        let Some(mut file) = create_module_with(&imports, &path)? else {
            warn!(
                "File {} already exists, skipping",
                &path.join("mod.rs").display()
            );
            return Ok(());
        };

        // Write the state macro
        writeln!(file, "impl_state!(")?;
        writeln!(file, "    {},", name.to_case(Case::Pascal))?;
        writeln!(file, "    V{},", version)?;
        for (direction, packets) in packet_groups {
            writeln!(file, "    {} => {{", direction.to_case(Case::Pascal))?;
            for (packet, id) in packets {
                writeln!(
                    file,
                    "        0x{id:x} => {}::{packet},",
                    packet.to_ascii_lowercase()
                )?;
            }
            writeln!(file, "    }},")?;
        }
        writeln!(file, ");")?;

        Ok(())
    }

    /// Generate the packet direction part of the state
    fn generate_direction(
        fields: &JsonValue,
        state: &JsonValue,
        direction: &str,
        path: &Path,
    ) -> Result<Vec<(String, i32)>, std::io::Error> {
        let mut packets = Vec::new();

        for (class_name, packet_id) in state[direction].entries() {
            let packet_name = Self::generate_packet(fields, class_name, direction, path.into())?;

            packets.push((packet_name, packet_id.as_i32().unwrap()));
        }

        Ok(packets)
    }

    /// The header for all generated packet structs
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

            "GameProfile" => Some("crate::types::GameProfile".to_string()),
            "BlockPos" => Some("crate::types::position::BlockPos".to_string()),
            "ResourceLocation" => Some("crate::types::ResourceLocation".to_string()),
            "UnsizedByteBuffer" => Some("crate::types::UnsizedByteBuffer".to_string()),
            //            "ResourceEntry" => Some("crate::types::ResourceEntry".to_string()),
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
        "net/minecraft/class_2604" => "EntitySpawnPacket",
        "net/minecraft/class_2606" => "ExperienceOrbSpawnPacket",
        "net/minecraft/class_2613" => "PlayerSpawnPacket",
        "net/minecraft/class_2616" => "EntityAnimationPacket",
        "net/minecraft/class_2617" => "StatisticsPacket",
        "net/minecraft/class_4463" => "PlayerActionResponsePacket",
        "net/minecraft/class_2620" => "BlockBreakingProgressPacket",
        "net/minecraft/class_2622" => "BlockEntityUpdatePacket",
        "net/minecraft/class_2623" => "BlockEventPacket",
        "net/minecraft/class_2626" => "BlockUpdatePacket",
        "net/minecraft/class_2629" => "BossBarPacket",
        "net/minecraft/class_8042" => "BundlePacket",
        "net/minecraft/class_2632" => "DifficultyPacket",
        "net/minecraft/class_8212" => "ChunkBiomeDataPacket",
        "net/minecraft/class_5888" => "ClearTitlePacket",
        "net/minecraft/class_2639" => "CommandSuggestionsPacket",
        "net/minecraft/class_2641" => "CommandTreePacket",
        "net/minecraft/class_2645" => "CloseScreenPacket",
        "net/minecraft/class_2649" => "InventoryPacket",
        "net/minecraft/class_2651" => "ScreenHandlerPropertyUpdatePacket",
        "net/minecraft/class_2653" => "ScreenHandlerSlotUpdatePacket",
        "net/minecraft/class_2656" => "CooldownUpdatePacket",
        "net/minecraft/class_7597" => "ChatSuggestionsPacket",
        "net/minecraft/class_2658" => "CustomPayloadPacket",
        "net/minecraft/class_8143" => "EntityDamagePacket",
        "net/minecraft/class_7617" => "RemoveMessagePacket",
        "net/minecraft/class_2661" => "DisconnectPacket",
        "net/minecraft/class_7827" => "ProfilelessChatMessagePacket",
        "net/minecraft/class_2663" => "EntityStatusPacket",
        "net/minecraft/class_2664" => "ExplosionPacket",
        "net/minecraft/class_2666" => "UnloadChunkPacket",
        "net/minecraft/class_2668" => "GameStateChangePacket",
        "net/minecraft/class_2648" => "OpenHorseScreenPacket",
        "net/minecraft/class_8043" => "DamageTiltPacket",
        "net/minecraft/class_5889" => "WorldBorderInitializePacket",
        "net/minecraft/class_2670" => "KeepAlivePacket",
        "net/minecraft/class_2672" => "ChunkDataPacket",
        "net/minecraft/class_2673" => "WorldEventPacket",
        "net/minecraft/class_2675" => "ParticlePacket",
        "net/minecraft/class_2676" => "LightUpdatePacket",
        "net/minecraft/class_2678" => "GameJoinPacket",
        "net/minecraft/class_2683" => "MapUpdatePacket",
        "net/minecraft/class_3943" => "SetTradeOffersPacket",
        "net/minecraft/class_2684" => "EntityPacket",
        "net/minecraft/class_2684$class_2685" => "EntityMoveRelativePacket",
        "net/minecraft/class_2684$class_2686" => "EntityRotateAndMoveRelativePacket",
        "net/minecraft/class_2684$class_2687" => "EntityRotatePacket",
        "net/minecraft/class_2692" => "VehicleMovePacket",
        "net/minecraft/class_3895" => "OpenWrittenBookPacket",
        "net/minecraft/class_3944" => "OpenScreenPacket",
        "net/minecraft/class_2693" => "SignEditorOpenPacket",
        "net/minecraft/class_6373" => "PlayPingPacket",
        "net/minecraft/class_2695" => "CraftFailedResponsePacket",
        "net/minecraft/class_2696" => "PlayerAbilitiesPacket",
        "net/minecraft/class_7438" => "ChatMessagePacket",
        "net/minecraft/class_5890" => "EndCombatPacket",
        "net/minecraft/class_5891" => "EnterCombatPacket",
        "net/minecraft/class_5892" => "DeathMessagePacket",
        "net/minecraft/class_7828" => "PlayerRemovePacket",
        "net/minecraft/class_2703" => "PlayerListPacket",
        "net/minecraft/class_2707" => "LookAtPacket",
        "net/minecraft/class_2708" => "PlayerPositionLookPacket",
        "net/minecraft/class_2713" => "UnlockRecipesPacket",
        "net/minecraft/class_2716" => "EntitiesDestroyPacket",
        "net/minecraft/class_2718" => "RemoveEntityStatusEffectPacket",
        "net/minecraft/class_2720" => "ResourcePackSendPacket",
        "net/minecraft/class_2724" => "PlayerRespawnPacket",
        "net/minecraft/class_2726" => "EntitySetHeadYawPacket",
        "net/minecraft/class_2637" => "ChunkDeltaUpdatePacket",
        "net/minecraft/class_2729" => "SelectAdvancementTabPacket",
        "net/minecraft/class_7495" => "ServerMetadataPacket",
        "net/minecraft/class_5894" => "OverlayMessagePacket",
        "net/minecraft/class_5895" => "WorldBorderCenterChangedPacket",
        "net/minecraft/class_5896" => "WorldBorderInterpolateSizePacket",
        "net/minecraft/class_5897" => "WorldBorderSizeChangedPacket",
        "net/minecraft/class_5898" => "WorldBorderWarningTimeChangedPacket",
        "net/minecraft/class_5899" => "WorldBorderWarningBlocksChangedPacket",
        "net/minecraft/class_2734" => "SetCameraEntityPacket",
        "net/minecraft/class_2735" => "UpdateSelectedSlotPacket",
        "net/minecraft/class_4282" => "ChunkRenderDistanceCenterPacket",
        "net/minecraft/class_4273" => "ChunkLoadDistancePacket",
        "net/minecraft/class_2759" => "PlayerSpawnPositionPacket",
        "net/minecraft/class_2736" => "ScoreboardDisplayPacket",
        "net/minecraft/class_2739" => "EntityTrackerUpdatePacket",
        "net/minecraft/class_2740" => "EntityAttachPacket",
        "net/minecraft/class_2743" => "EntityVelocityUpdatePacket",
        "net/minecraft/class_2744" => "EntityEquipmentUpdatePacket",
        "net/minecraft/class_2748" => "ExperienceBarUpdatePacket",
        "net/minecraft/class_2749" => "HealthUpdatePacket",
        "net/minecraft/class_2751" => "ScoreboardObjectiveUpdatePacket",
        "net/minecraft/class_2752" => "EntityPassengersSetPacket",
        "net/minecraft/class_5900" => "TeamPacket",
        "net/minecraft/class_2757" => "ScoreboardPlayerUpdatePacket",
        "net/minecraft/class_6682" => "SimulationDistancePacket",
        "net/minecraft/class_5903" => "SubtitlePacket",
        "net/minecraft/class_2761" => "WorldTimeUpdatePacket",
        "net/minecraft/class_5904" => "TitlePacket",
        "net/minecraft/class_5905" => "TitleFadePacket",
        "net/minecraft/class_2765" => "PlaySoundFromEntityPacket",
        "net/minecraft/class_2767" => "PlaySoundPacket",
        "net/minecraft/class_2770" => "StopSoundPacket",
        "net/minecraft/class_7439" => "GameMessagePacket",
        "net/minecraft/class_2772" => "PlayerListHeaderPacket",
        "net/minecraft/class_2774" => "NbtQueryResponsePacket",
        "net/minecraft/class_2775" => "ItemPickupAnimationPacket",
        "net/minecraft/class_2777" => "EntityPositionPacket",
        "net/minecraft/class_2779" => "AdvancementUpdatePacket",
        "net/minecraft/class_2781" => "EntityAttributesPacket",
        "net/minecraft/class_7832" => "FeaturesPacket",
        "net/minecraft/class_2783" => "EntityStatusEffectPacket",
        "net/minecraft/class_2788" => "SynchronizeRecipesPacket",
        "net/minecraft/class_2790" => "SynchronizeTagsPacket",
        "net/minecraft/class_2793" => "TeleportConfirmPacket",
        "net/minecraft/class_2795" => "QueryBlockNbtPacket",
        "net/minecraft/class_4210" => "UpdateDifficultyPacket",
        "net/minecraft/class_7640" => "MessageAcknowledgmentPacket",
        "net/minecraft/class_7472" => "CommandExecutionPacket",
        "net/minecraft/class_2797" => "ChatMessagePacket",
        "net/minecraft/class_7861" => "PlayerSessionPacket",
        "net/minecraft/class_2799" => "ClientStatusPacket",
        "net/minecraft/class_2803" => "ClientSettingsPacket",
        "net/minecraft/class_2805" => "RequestCommandCompletionsPacket",
        "net/minecraft/class_2811" => "ButtonClickPacket",
        "net/minecraft/class_2813" => "ClickSlotPacket",
        "net/minecraft/class_2815" => "CloseHandledScreenPacket",
        "net/minecraft/class_2817" => "CustomPayloadPacket",
        "net/minecraft/class_2820" => "BookUpdatePacket",
        "net/minecraft/class_2822" => "QueryEntityNbtPacket",
        "net/minecraft/class_2824" => "PlayerInteractEntityPacket",
        "net/minecraft/class_5194" => "JigsawGeneratingPacket",
        "net/minecraft/class_2827" => "KeepAlivePacket",
        "net/minecraft/class_4211" => "UpdateDifficultyLockPacket",
        "net/minecraft/class_2828" => "PlayerMovePacket",
        "net/minecraft/class_2828$class_2829" => "PlayerMovePositionAndOnGroundPacket",
        "net/minecraft/class_2828$class_2830" => "PlayerMoveFullPacket",
        "net/minecraft/class_2828$class_2831" => "PlayerMoveLookAndOnGroundPacket",
        "net/minecraft/class_2828$class_5911" => "PlayerMoveOnGroundOnlyPacket",
        "net/minecraft/class_2833" => "VehicleMovePacket",
        "net/minecraft/class_2836" => "BoatPaddleStatePacket",
        "net/minecraft/class_2838" => "PickFromInventoryPacket",
        "net/minecraft/class_2840" => "CraftRequestPacket",
        "net/minecraft/class_2842" => "UpdatePlayerAbilitiesPacket",
        "net/minecraft/class_2846" => "PlayerActionPacket",
        "net/minecraft/class_2848" => "ClientCommandPacket",
        "net/minecraft/class_2851" => "PlayerInputPacket",
        "net/minecraft/class_6374" => "PlayPongPacket",
        "net/minecraft/class_5427" => "RecipeCategoryOptionsPacket",
        "net/minecraft/class_2853" => "RecipeBookDataPacket",
        "net/minecraft/class_2855" => "RenameItemPacket",
        "net/minecraft/class_2856" => "ResourcePackStatusPacket",
        "net/minecraft/class_2859" => "AdvancementTabPacket",
        "net/minecraft/class_2863" => "SelectMerchantTradePacket",
        "net/minecraft/class_2866" => "UpdateBeaconPacket",
        "net/minecraft/class_2868" => "UpdateSelectedSlotPacket",
        "net/minecraft/class_2870" => "UpdateCommandBlockPacket",
        "net/minecraft/class_2871" => "UpdateCommandBlockMinecartPacket",
        "net/minecraft/class_2873" => "CreativeInventoryActionPacket",
        "net/minecraft/class_3753" => "UpdateJigsawPacket",
        "net/minecraft/class_2875" => "UpdateStructureBlockPacket",
        "net/minecraft/class_2877" => "UpdateSignPacket",
        "net/minecraft/class_2879" => "HandSwingPacket",
        "net/minecraft/class_2884" => "SpectatorTeleportPacket",
        "net/minecraft/class_2885" => "PlayerInteractBlockPacket",
        "net/minecraft/class_2886" => "PlayerInteractItemPacket",
        "net/minecraft/class_2889" => "HandshakePacket",
        "net/minecraft/class_2899" => "LoginQueryRequestPacket",
        "net/minecraft/class_2901" => "LoginSuccessPacket",
        "net/minecraft/class_2905" => "LoginHelloPacket",
        "net/minecraft/class_2907" => "LoginCompressionPacket",
        "net/minecraft/class_2909" => "LoginDisconnectPacket",
        "net/minecraft/class_2913" => "LoginQueryResponsePacket",
        "net/minecraft/class_2915" => "LoginHelloPacket",
        "net/minecraft/class_2917" => "LoginKeyPacket",
        "net/minecraft/class_2923" => "QueryPongPacket",
        "net/minecraft/class_2924" => "QueryResponsePacket",
        "net/minecraft/class_2935" => "QueryPingPacket",
        "net/minecraft/class_2937" => "QueryRequestPacket",
        _ => return None,
    };

    Some(string.to_owned())
}
