use bevy::{ecs::system::SystemState, prelude::*};
use mc_rs_proto::{
    types::EntityId,
    versions::v1_20_0::{
        configuration::ClientboundConfigurationPackets,
        play::{
            serverboundkeepalivepacket::ServerboundKeepAlivePacket,
            serverboundteleportconfirmpacket::ServerboundTeleportConfirmPacket,
            ClientboundPlayPackets,
        },
        V1_20_0,
    },
};

use crate::{
    networking::{
        network::{LocalPlayer, Network},
        task::ConnectionChannel,
    },
    systems::{
        blocks::block_list::Blocks,
        world::{
            resources::{CurrentWorld, WorldSeed},
            WorldType, Worlds,
        },
    },
};

impl Network for V1_20_0 {
    const HAS_CONFIGURATION_STATE: bool = false;

    fn config_packet(_world: &mut World, _packet: ClientboundConfigurationPackets) {
        unreachable!("This version does not have a configuration state",);
    }

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
            ClientboundPlayPackets::CustomPayload(_) => {}
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
                log::info!("Received keep alive: {:?}", p);

                let mut state = SystemState::<ResMut<ConnectionChannel<Self>>>::new(world);
                let mut chan = state.get_mut(world);
                chan.send_play(ServerboundKeepAlivePacket { id: p.id });
            }
            ClientboundPlayPackets::ChunkData(p) => {
                let mut state =
                    SystemState::<(Res<Worlds>, Option<Res<CurrentWorld>>, Res<Blocks>)>::new(
                        world,
                    );

                let (worlds, current, blocks) = state.get(world);

                let mut worlds = worlds.clone();
                let blocks = blocks.clone();

                if let Some(current) = current {
                    if let Err(err) = worlds.insert_data::<V1_20_0>(
                        &current.clone(),
                        p.position,
                        p.chunk_data,
                        &blocks,
                        world,
                    ) {
                        log::error!("Failed to insert chunk {:?} : {err}", p.position);
                    }
                } else {
                    log::warn!("Received chunk data without a current world!");
                }

                {
                    let mut state = SystemState::<ResMut<Worlds>>::new(world);
                    let mut worlds = state.get_mut(world);
                    *worlds = worlds.clone();
                }
            }
            ClientboundPlayPackets::WorldEvent(_) => {}
            ClientboundPlayPackets::Particle(_) => {}
            ClientboundPlayPackets::LightUpdate(_) => {}
            ClientboundPlayPackets::GameJoin(p) => {
                debug!("Joined game: {:?}", p);

                let mut state = SystemState::<(Res<LocalPlayer>, ResMut<Worlds>)>::new(world);
                let (player, mut worlds) = state.get_mut(world);

                for world_type in p.worlds {
                    worlds.insert_empty(&WorldType::from(world_type));
                }

                let player = player.clone();
                world.entity_mut(*player).insert(EntityId(p.player_id));

                world.insert_resource(p.game_mode);
                world.insert_resource(CurrentWorld::new(p.world, p.world_type));
                world.insert_resource(WorldSeed(p.seed));
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
                info!("Received player position and look: {:?}", p);

                let mut state =
                    SystemState::<(ResMut<ConnectionChannel<Self>>, Res<LocalPlayer>)>::new(world);
                let (mut channel, player) = state.get_mut(world);
                channel.send_play(ServerboundTeleportConfirmPacket { id: p.id });

                let player = *player.clone();
                let mut player = world.entity_mut(player);
                let mut transform = player.get_mut::<Transform>().unwrap();

                if p.relative_flags.x {
                    transform.translation.x += p.position.x as f32;
                } else {
                    transform.translation.x = p.position.x as f32;
                }

                if p.relative_flags.y {
                    transform.translation.y += p.position.y as f32;
                } else {
                    transform.translation.y = p.position.y as f32;
                }

                if p.relative_flags.z {
                    transform.translation.z += p.position.z as f32;
                } else {
                    transform.translation.z = p.position.z as f32;
                }

                if p.relative_flags.yaw {
                    transform.rotation *=
                        Quat::from_rotation_y(p.yaw * std::f32::consts::PI / 180.0);
                } else {
                    transform.rotation =
                        Quat::from_rotation_y(p.yaw * std::f32::consts::PI / 180.0);
                }

                if p.relative_flags.pitch {
                    transform.rotation *=
                        Quat::from_rotation_x(p.pitch * std::f32::consts::PI / 180.0);
                } else {
                    transform.rotation =
                        Quat::from_rotation_x(p.pitch * std::f32::consts::PI / 180.0);
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
