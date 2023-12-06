use bevy::{ecs::system::SystemState, prelude::*};
use compact_str::CompactString;
use mc_rs_core::{
    components::player::{ControlledPlayer, ControlledPlayerHead},
    resources::client_information::ClientInformation,
};
use mc_rs_protocol::{
    types::EntityId,
    versions::v1_20_0::{
        configuration::ClientboundConfigurationPackets,
        play::{
            serverboundclientsettingspacket::ServerboundClientSettingsPacket,
            serverboundkeepalivepacket::ServerboundKeepAlivePacket,
            serverboundteleportconfirmpacket::ServerboundTeleportConfirmPacket,
            ClientboundPlayPackets,
        },
        V1_20_0,
    },
};
use mc_rs_world::{
    resources::{CurrentWorld, WorldType, Worlds},
    world::tasks::DecodeChunkTask,
};

use crate::{network::Network, task::ConnectionChannel};

impl Network for V1_20_0 {
    const HAS_CONFIGURATION_STATE: bool = false;

    fn config_packet(_world: &mut World, _packet: ClientboundConfigurationPackets) {
        unreachable!("This version does not have a configuration state");
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn play_packet(world: &mut World, packet: ClientboundPlayPackets) {
        match packet {
            ClientboundPlayPackets::Bundle(_) => {}
            ClientboundPlayPackets::EntitySpawn(_) => {}
            ClientboundPlayPackets::ExperienceOrbSpawn(_) => {}
            ClientboundPlayPackets::PlayerSpawn(_) => {}
            ClientboundPlayPackets::EntityAnimation(_) => {}
            ClientboundPlayPackets::Statistics(_) => {}
            ClientboundPlayPackets::PlayerActionResponse(_) => {}
            ClientboundPlayPackets::BlockBreakingProgress(_) => {}
            ClientboundPlayPackets::BlockEntityUpdate(_) => {}
            ClientboundPlayPackets::BlockEvent(_) => {}
            ClientboundPlayPackets::BlockUpdate(_) => {}
            ClientboundPlayPackets::BossBar(_) => {}
            ClientboundPlayPackets::Difficulty(_) => {}
            ClientboundPlayPackets::ChunkBiomeData(_) => {}
            ClientboundPlayPackets::ClearTitle(_) => {}
            ClientboundPlayPackets::CommandSuggestions(_) => {}
            ClientboundPlayPackets::CommandTree(_) => {}
            ClientboundPlayPackets::CloseScreen(_) => {}
            ClientboundPlayPackets::Inventory(_) => {}
            ClientboundPlayPackets::ScreenHandlerPropertyUpdate(_) => {}
            ClientboundPlayPackets::ScreenHandlerSlotUpdate(_) => {}
            ClientboundPlayPackets::CooldownUpdate(_) => {}
            ClientboundPlayPackets::ChatSuggestions(_) => {}
            ClientboundPlayPackets::CustomPayload(p) => match CompactString::from_utf8(&p.data) {
                Ok(str) => {
                    info!("Received custom payload: `{0} : {str}`", p.identifier);
                }
                Err(_) => {
                    info!(
                        "Received custom payload: `{0} : {1:?}`",
                        p.identifier, p.data
                    );
                }
            },
            ClientboundPlayPackets::EntityDamage(_) => {}
            ClientboundPlayPackets::RemoveMessage(_) => {}
            ClientboundPlayPackets::Disconnect(_) => {}
            ClientboundPlayPackets::ProfilelessChatMessage(_) => {}
            ClientboundPlayPackets::EntityStatus(_) => {}
            ClientboundPlayPackets::Explosion(_) => {}
            ClientboundPlayPackets::UnloadChunk(_) => {}
            ClientboundPlayPackets::GameStateChange(_) => {}
            ClientboundPlayPackets::OpenHorseScreen(_) => {}
            ClientboundPlayPackets::DamageTilt(_) => {}
            ClientboundPlayPackets::WorldBorderInitialize(_) => {}
            ClientboundPlayPackets::KeepAlive(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received keep alive: {p:?}");

                let mut state = SystemState::<ResMut<ConnectionChannel<Self>>>::new(world);
                state
                    .get_mut(world)
                    .send_play(ServerboundKeepAlivePacket::from(p));
            }
            ClientboundPlayPackets::ChunkData(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received chunk data: {p:?}");

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        error!("Unable to get CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let task = DecodeChunkTask::create(p.chunk_data);
                let entity = world.spawn(task).id();

                let mut worlds = world.resource_mut::<Worlds>();
                worlds.insert_chunk_entity(current.into(), p.position, entity);
            }
            ClientboundPlayPackets::WorldEvent(_) => {}
            ClientboundPlayPackets::Particle(_) => {}
            ClientboundPlayPackets::LightUpdate(_) => {}
            ClientboundPlayPackets::GameJoin(p) => {
                debug!("Joined game: {:?}", p);

                let mut state = SystemState::<Query<Entity, With<ControlledPlayer>>>::new(world);
                let player = state.get_mut(world);

                // Add the player's entity id
                let player = player.single();
                world.entity_mut(player).insert(EntityId(p.player_id));

                // Send the client settings packet
                let info = world.resource::<ClientInformation>().clone();
                let mut conn = world.resource_mut::<ConnectionChannel<Self>>();
                conn.send_play(ServerboundClientSettingsPacket::from(info));

                // Add all of the worlds to the worlds list
                let mut worlds = Worlds::default();
                for name in p.worlds {
                    worlds.insert_world(name.into(), Default::default());
                }
                world.insert_resource(worlds);

                // Set the current world
                world.insert_resource(CurrentWorld::from(p.world));

                // Add other information to the world
                world.insert_resource(p.game_mode);
            }
            ClientboundPlayPackets::MapUpdate(_) => {}
            ClientboundPlayPackets::SetTradeOffers(_) => {}
            ClientboundPlayPackets::EntityMoveRelative(_) => {}
            ClientboundPlayPackets::EntityRotateAndMoveRelative(_) => {}
            ClientboundPlayPackets::EntityRotate(_) => {}
            ClientboundPlayPackets::VehicleMove(_) => {}
            ClientboundPlayPackets::OpenWrittenBook(_) => {}
            ClientboundPlayPackets::OpenScreen(_) => {}
            ClientboundPlayPackets::SignEditorOpen(_) => {}
            ClientboundPlayPackets::PlayPing(_) => {}
            ClientboundPlayPackets::CraftFailedResponse(_) => {}
            ClientboundPlayPackets::PlayerAbilities(_) => {}
            ClientboundPlayPackets::ChatMessage(_) => {}
            ClientboundPlayPackets::EndCombat(_) => {}
            ClientboundPlayPackets::EnterCombat(_) => {}
            ClientboundPlayPackets::DeathMessage(_) => {}
            ClientboundPlayPackets::PlayerRemove(_) => {}
            ClientboundPlayPackets::PlayerList(_) => {}
            ClientboundPlayPackets::LookAt(_) => {}
            ClientboundPlayPackets::PlayerPositionLook(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!("Received player position and look: {:?}", p);

                let mut state = SystemState::<(
                    ResMut<ConnectionChannel<Self>>,
                    Query<&mut Transform, (With<ControlledPlayer>, Without<ControlledPlayerHead>)>,
                    Query<&mut Transform, (Without<ControlledPlayer>, With<ControlledPlayerHead>)>,
                )>::new(world);
                let (mut channel, mut player, mut head) = state.get_mut(world);
                channel.send_play(ServerboundTeleportConfirmPacket::from(p.id));

                // Update the player posiiton
                let mut transform = player.single_mut();

                #[allow(clippy::cast_possible_truncation)]
                if p.relative_flags.x {
                    transform.translation.x += p.position.x as f32;
                } else {
                    transform.translation.x = p.position.x as f32;
                }

                #[allow(clippy::cast_possible_truncation)]
                if p.relative_flags.y {
                    transform.translation.y += p.position.y as f32;
                } else {
                    transform.translation.y = p.position.y as f32;
                }

                #[allow(clippy::cast_possible_truncation)]
                if p.relative_flags.z {
                    transform.translation.z += p.position.z as f32;
                } else {
                    transform.translation.z = p.position.z as f32;
                }

                // Update the player rotation
                let mut transform = head.single_mut();

                if p.relative_flags.yaw {
                    transform.rotation *=
                        Quat::from_rotation_z(p.yaw * std::f32::consts::PI / 180.0);
                } else {
                    transform.rotation =
                        Quat::from_rotation_z(p.yaw * std::f32::consts::PI / 180.0);
                }

                if p.relative_flags.pitch {
                    transform.rotation *=
                        Quat::from_rotation_y(p.pitch * std::f32::consts::PI / 180.0);
                } else {
                    transform.rotation =
                        Quat::from_rotation_y(p.pitch * std::f32::consts::PI / 180.0);
                }
            }
            ClientboundPlayPackets::UnlockRecipes(_) => {}
            ClientboundPlayPackets::EntitiesDestroy(_) => {}
            ClientboundPlayPackets::RemoveEntityStatusEffect(_) => {}
            ClientboundPlayPackets::ResourcePackSend(_) => {}
            ClientboundPlayPackets::PlayerRespawn(_) => {}
            ClientboundPlayPackets::EntitySetHeadYaw(_) => {}
            ClientboundPlayPackets::ChunkDeltaUpdate(_) => {}
            ClientboundPlayPackets::SelectAdvancementTab(_) => {}
            ClientboundPlayPackets::ServerMetadata(_) => {}
            ClientboundPlayPackets::OverlayMessage(_) => {}
            ClientboundPlayPackets::WorldBorderCenterChanged(_) => {}
            ClientboundPlayPackets::WorldBorderInterpolateSize(_) => {}
            ClientboundPlayPackets::WorldBorderSizeChanged(_) => {}
            ClientboundPlayPackets::WorldBorderWarningTimeChanged(_) => {}
            ClientboundPlayPackets::WorldBorderWarningBlocksChanged(_) => {}
            ClientboundPlayPackets::SetCameraEntity(_) => {}
            ClientboundPlayPackets::UpdateSelectedSlot(_) => {}
            ClientboundPlayPackets::ChunkRenderDistanceCenter(_) => {}
            ClientboundPlayPackets::ChunkLoadDistance(_) => {}
            ClientboundPlayPackets::PlayerSpawnPosition(_) => {}
            ClientboundPlayPackets::ScoreboardDisplay(_) => {}
            ClientboundPlayPackets::EntityTrackerUpdate(_) => {}
            ClientboundPlayPackets::EntityAttach(_) => {}
            ClientboundPlayPackets::EntityVelocityUpdate(_) => {}
            ClientboundPlayPackets::EntityEquipmentUpdate(_) => {}
            ClientboundPlayPackets::ExperienceBarUpdate(_) => {}
            ClientboundPlayPackets::HealthUpdate(_) => {}
            ClientboundPlayPackets::ScoreboardObjectiveUpdate(_) => {}
            ClientboundPlayPackets::EntityPassengersSet(_) => {}
            ClientboundPlayPackets::Team(_) => {}
            ClientboundPlayPackets::ScoreboardPlayerUpdate(_) => {}
            ClientboundPlayPackets::SimulationDistance(_) => {}
            ClientboundPlayPackets::Subtitle(_) => {}
            ClientboundPlayPackets::WorldTimeUpdate(_) => {}
            ClientboundPlayPackets::Title(_) => {}
            ClientboundPlayPackets::TitleFade(_) => {}
            ClientboundPlayPackets::PlaySoundFromEntity(_) => {}
            ClientboundPlayPackets::PlaySound(_) => {}
            ClientboundPlayPackets::StopSound(_) => {}
            ClientboundPlayPackets::GameMessage(_) => {}
            ClientboundPlayPackets::PlayerListHeader(_) => {}
            ClientboundPlayPackets::NbtQueryResponse(_) => {}
            ClientboundPlayPackets::ItemPickupAnimation(_) => {}
            ClientboundPlayPackets::EntityPosition(p) => {
                let mut state = SystemState::<Query<(&EntityId, &mut Transform)>>::new(world);
                let mut query = state.get_mut(world);

                if let Some((_, mut transform)) =
                    query.iter_mut().find(|(id, _)| **id == p.entity_id)
                {
                    transform.translation = p.position.into();
                } else {
                    // warn!("Got EntityPosition for unknown entity {}", p.entity_id)
                }
            }
            ClientboundPlayPackets::AdvancementUpdate(_) => {}
            ClientboundPlayPackets::EntityAttributes(_) => {}
            ClientboundPlayPackets::Features(_) => {}
            ClientboundPlayPackets::EntityStatusEffect(_) => {}
            ClientboundPlayPackets::SynchronizeRecipes(_) => {}
            ClientboundPlayPackets::SynchronizeTags(_) => {}
        }
    }
}
