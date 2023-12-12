use std::ops::Deref;

use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::{
    components::player::{ControlledPlayer, ControlledPlayerHead},
    packets::sound::PacketSoundType,
    position::ChunkBlockPos,
    resources::client_information::ClientInformation,
    sounds::SoundEvent,
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
    world::{section::Section, tasks::DecodeChunkTask, Chunk},
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
            ClientboundPlayPackets::EntitySpawn(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received EntitySpawn: {:?}", p.entity_id);

                let transform = Transform::from_translation(p.position.into());
                world.spawn((p.entity_id, p.uuid, transform));
            }
            ClientboundPlayPackets::ExperienceOrbSpawn(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received ExperienceOrbSpawn: {:?}", p.entity_id);

                let transform = Transform::from_translation(p.position.into());
                world.spawn((p.entity_id, transform));
            }
            ClientboundPlayPackets::PlayerSpawn(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received PlayerSpawn: {:?}", p.entity_id);

                let mut transform = Transform::from_translation(p.position.into());
                transform.rotation =
                    Quat::from_rotation_y(p.yaw.into()) + Quat::from_rotation_x(p.pitch.into());

                world.spawn((p.entity_id, p.uuid, transform));
            }
            ClientboundPlayPackets::EntityAnimation(_) => {}
            ClientboundPlayPackets::Statistics(_) => {}
            ClientboundPlayPackets::PlayerActionResponse(_) => {}
            ClientboundPlayPackets::BlockBreakingProgress(_) => {}
            ClientboundPlayPackets::BlockEntityUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!("Received BlockEntityUpdate: {:?}", p.position);

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let chunk_worlds = world.resource::<Worlds>();
                if let Some(chunk_world) = chunk_worlds.get_world(&current) {
                    if let Some(chunk_entity) = chunk_world.get_entity(&p.position.into()) {
                        if let Some(mut _chunk) = world.entity_mut(*chunk_entity).get::<Chunk>() {
                            // TODO: Update the block entity
                        } else {
                            #[cfg(any(debug_assertions, feature = "debug"))]
                            error!(
                                "Error getting Chunk for BlockEntityUpdate: {:?}",
                                p.position
                            );
                        };
                    } else {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting Entity for Chunk: {:?}", p.position);
                    }
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Error getting current World: {current:?}");
                }
            }
            ClientboundPlayPackets::BlockEvent(_) => {}
            ClientboundPlayPackets::BlockUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received BlockUpdate: {:?}", p.position);

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        error!("Error getting CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let chunk_worlds = world.resource::<Worlds>();
                if let Some(chunk_world) = chunk_worlds.get_world(&current) {
                    if let Some(&chunk_entity) = chunk_world.get_entity(&p.position.into()) {
                        if let Some(mut chunk) = world.entity_mut(chunk_entity).get_mut::<Chunk>() {
                            chunk.set_block_id(p.block_state, ChunkBlockPos::from(p.position));
                        } else {
                            #[cfg(any(debug_assertions, feature = "debug"))]
                            error!("Error getting Chunk for BlockUpdate: {:?}", p.position);
                        };
                    } else {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting Entity for Chunk: {:?}", p.position);
                    }
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Error getting current World: {current:?}");
                }
            }
            ClientboundPlayPackets::BossBar(_) => {}
            ClientboundPlayPackets::Difficulty(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received Difficulty: {:?}", p);

                // TODO: Update the WorldDifficulty resource
            }
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
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    info!("Received CustomPayload: `{0} : {str}`", p.identifier);
                }
                Err(_) => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    info!(
                        "Received CustomPayload: `{0} : {1:?}`",
                        p.identifier, p.data
                    );
                }
            },
            ClientboundPlayPackets::EntityDamage(_) => {}
            ClientboundPlayPackets::RemoveMessage(_) => {}
            ClientboundPlayPackets::Disconnect(_) => {}
            ClientboundPlayPackets::ProfilelessChatMessage(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!("Received Server ChatMessage: {}", p.message.to_string());
            }
            ClientboundPlayPackets::EntityStatus(_) => {}
            ClientboundPlayPackets::Explosion(_) => {}
            ClientboundPlayPackets::UnloadChunk(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received UnloadChunk: {:?}", *p);

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let mut chunk_worlds = world.resource_mut::<Worlds>();
                if let Some(chunk_world) = chunk_worlds.get_world_mut(&current) {
                    if let Some(entity) = chunk_world.get_entity(&p).cloned() {
                        chunk_world.remove_entity(&p);
                        world.entity_mut(entity).despawn_recursive();
                    } else {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting Entity for Chunk: {:?}", *p);
                    }
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Error getting current World: {current:?}");
                }
            }
            ClientboundPlayPackets::GameStateChange(_) => {}
            ClientboundPlayPackets::OpenHorseScreen(_) => {}
            ClientboundPlayPackets::DamageTilt(_) => {}
            ClientboundPlayPackets::WorldBorderInitialize(_) => {}
            ClientboundPlayPackets::KeepAlive(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!("Received KeepAlive: {p:?}");

                let mut conn = world.resource_mut::<ConnectionChannel<Self>>();
                conn.send_play(ServerboundKeepAlivePacket::from(p));
            }
            ClientboundPlayPackets::ChunkData(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received ChunkData for: {:?}", p.position);

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let task = DecodeChunkTask::create(p.chunk_data);
                let transform = Transform::from_translation(p.position.into());
                let entity = world.spawn((task, p.position, transform)).id();

                let mut worlds = world.resource_mut::<Worlds>();
                worlds.insert_chunk_entity(current.into(), p.position, entity);
            }
            ClientboundPlayPackets::WorldEvent(_) => {}
            ClientboundPlayPackets::Particle(_) => {}
            ClientboundPlayPackets::LightUpdate(_) => {}
            ClientboundPlayPackets::GameJoin(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received GameInfo: {:?}", p);

                // Add the Players's EntityId
                let mut query = world.query_filtered::<Entity, With<ControlledPlayer>>();
                let player = query.single(world);
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
            ClientboundPlayPackets::EntityMoveRelative(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityMoveRelative: {:?}", p.entity_id);

                let mut query = world.query::<(&EntityId, &mut Transform)>();

                // Find the entity with the given id
                if let Some((_, mut transform)) =
                    query.iter_mut(world).find(|(id, _)| id == &&p.entity_id)
                {
                    transform.translation +=
                        Vec3::from_array([p.delta_x.into(), p.delta_y.into(), p.delta_z.into()]);
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityMoveRelative for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::EntityRotateAndMoveRelative(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityRotateAndMoveRelative: {:?}", p.entity_id);

                let mut query = world.query::<(&EntityId, &mut Transform)>();

                // Find the entity with the given id
                if let Some((_, mut transform)) =
                    query.iter_mut(world).find(|(id, _)| id == &&p.entity_id)
                {
                    transform.translation +=
                        Vec3::from_array([p.delta_x.into(), p.delta_y.into(), p.delta_z.into()]);

                    // TODO: Rotate the entity
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityRotateAndMoveRelative for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::EntityRotate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityRotate: {:?}", p.entity_id);

                let mut query = world.query::<(&EntityId, &mut Transform)>();

                // Find the entity with the given id
                if let Some((_, mut _transform)) =
                    query.iter_mut(world).find(|(id, _)| id == &&p.entity_id)
                {
                    // TODO: Rotate the entity
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityRotate for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::VehicleMove(_) => {}
            ClientboundPlayPackets::OpenWrittenBook(_) => {}
            ClientboundPlayPackets::OpenScreen(_) => {}
            ClientboundPlayPackets::SignEditorOpen(_) => {}
            ClientboundPlayPackets::PlayPing(_) => {}
            ClientboundPlayPackets::CraftFailedResponse(_) => {}
            ClientboundPlayPackets::PlayerAbilities(_) => {}
            ClientboundPlayPackets::ChatMessage(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!("Received ChatMessage from: {:?}", p.sender);
            }
            ClientboundPlayPackets::EndCombat(_) => {}
            ClientboundPlayPackets::EnterCombat(_) => {}
            ClientboundPlayPackets::DeathMessage(_) => {}
            ClientboundPlayPackets::PlayerRemove(_) => {}
            ClientboundPlayPackets::PlayerList(_) => {}
            ClientboundPlayPackets::LookAt(_) => {}
            ClientboundPlayPackets::PlayerPositionLook(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                info!(
                    "Received player PosLook: {:?}, ({:?}, {:?})",
                    p.position, p.yaw, p.pitch
                );

                // Send the teleport confirm packet
                let mut channel = world.resource_mut::<ConnectionChannel<Self>>();
                channel.send_play(ServerboundTeleportConfirmPacket::from(p.id));

                // Update the player's position and rotation
                let mut query = world.query_filtered::<&mut Transform, With<ControlledPlayer>>();
                let mut transform = query.single_mut(world);

                transform.translation = p.position.into();
                transform.rotation = Quat::from_rotation_x(p.pitch);

                // Update the player's head rotation
                let mut query =
                    world.query_filtered::<&mut Transform, With<ControlledPlayerHead>>();
                let mut transform = query.single_mut(world);

                transform.rotation = Quat::from_rotation_y(p.yaw) + Quat::from_rotation_y(p.yaw);
            }
            ClientboundPlayPackets::UnlockRecipes(_) => {}
            ClientboundPlayPackets::EntitiesDestroy(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received EntitiesDestroy: {:?}", &*p);

                let mut query = world.query::<(Entity, &EntityId)>();

                // Find the entity with the given id
                for entity_id in p.deref().iter() {
                    if let Some((entity, _)) = query.iter(world).find(|(_, id)| id == &entity_id) {
                        world.entity_mut(entity).despawn_recursive();
                    } else {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        warn!(
                            "Received EntitiesDestroy for unknown entity: {:?}",
                            entity_id
                        );
                    }
                }
            }
            ClientboundPlayPackets::RemoveEntityStatusEffect(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received RemoveEntityStatusEffect: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Remove the entity's status effect
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received RemoveEntityStatusEffect for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::ResourcePackSend(_) => {}
            ClientboundPlayPackets::PlayerRespawn(_) => {}
            ClientboundPlayPackets::EntitySetHeadYaw(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntitySetHeadYaw: {:?}", p.entity_id);

                let mut query = world.query::<(&EntityId, &mut Transform)>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|(id, _)| id == &p.entity_id) {
                    // TODO: Update the entity's head rotation
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntitySetHeadYaw for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::ChunkDeltaUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received ChunkDeltaUpdate: {:?}", p.position);

                let current = world
                    .get_resource::<CurrentWorld>()
                    .cloned()
                    .unwrap_or_else(|| {
                        error!("Error getting CurrentWorld");
                        CurrentWorld::from(WorldType::Overworld)
                    });

                let chunk_worlds = world.resource::<Worlds>();
                if let Some(chunk_world) = chunk_worlds.get_world(&current) {
                    if let Some(chunk_entity) = chunk_world.get_entity(&p.position.into()) {
                        if let Some(mut chunk) = world.entity_mut(*chunk_entity).get_mut::<Chunk>()
                        {
                            // TODO: Should this be a task?
                            for update in p.updates {
                                let block_pos = ChunkBlockPos::new(
                                    update.x,
                                    (p.position.y * Section::SECTION_HEIGHT as i32)
                                        + i32::from(update.y),
                                    update.z,
                                );

                                chunk.set_block_id(update.state, block_pos);
                            }
                        } else {
                            #[cfg(any(debug_assertions, feature = "debug"))]
                            error!("Error getting Chunk for ChunkDeltaUpdate: {:?}", p.position);
                        };
                    } else {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        error!("Error getting Entity for Chunk: {:?}", p.position);
                    }
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Error getting current World: {current:?}");
                }
            }
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
            ClientboundPlayPackets::ChunkRenderDistanceCenter(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received ChunkRenderDistanceCenter: {p:?}");

                // TODO: Update the ChunkRenderDistanceCenter resource
            }
            ClientboundPlayPackets::ChunkLoadDistance(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received ChunkLoadDistance: {p:?}");

                // TODO: Update the ChunkLoadDistance resource
            }
            ClientboundPlayPackets::PlayerSpawnPosition(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received PlayerSpawnPosition: {p:?}");

                // TODO: Update the PlayerSpawnPosition resource
            }
            ClientboundPlayPackets::ScoreboardDisplay(_) => {}
            ClientboundPlayPackets::EntityTrackerUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityTrackerUpdate: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Update the entity's trackers
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityTrackerUpdate for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::EntityAttach(_) => {}
            ClientboundPlayPackets::EntityVelocityUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityVelocityUpdate: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Update the entity's velocity
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityVelocityUpdate for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::EntityEquipmentUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityEquipmentUpdate: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                //
                // Only for this packet, spawn the entity if it doesn't exist,
                // sometimes the server sends this packet before the entity is spawned.
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Update the entity's equipment
                } else {
                    // TODO: Update the entity's equipment
                    world.spawn(p.entity_id);
                }
            }
            ClientboundPlayPackets::ExperienceBarUpdate(_) => {}
            ClientboundPlayPackets::HealthUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received HealthUpdate: {p:?}");

                // TODO: Update the PlayerHealth resource
            }
            ClientboundPlayPackets::ScoreboardObjectiveUpdate(_) => {}
            ClientboundPlayPackets::EntityPassengersSet(_) => {}
            ClientboundPlayPackets::Team(_) => {}
            ClientboundPlayPackets::ScoreboardPlayerUpdate(_) => {}
            ClientboundPlayPackets::SimulationDistance(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received SimulationDistance: {p:?}");

                // TODO: Update the SimulationDistance resource
            }
            ClientboundPlayPackets::Subtitle(_) => {}
            ClientboundPlayPackets::WorldTimeUpdate(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received WorldTimeUpdate: GameTime {}", p.game_time);

                // TODO: Update the WorldTime resource
            }
            ClientboundPlayPackets::Title(_) => {}
            ClientboundPlayPackets::TitleFade(_) => {}
            ClientboundPlayPackets::PlaySoundFromEntity(p) => {
                let mut query = world.query::<(&EntityId, &Transform)>();

                // Find the entity with the given id
                if let Some((_, transform)) =
                    query.iter_mut(world).find(|(id, _)| id == &&p.entity_id)
                {
                    match p.data {
                        PacketSoundType::SoundId(_id) => {
                            // let Some(registry) = SoundRegistry::try_from(id) else {
                            //     warn!("Received PlaySoundFromEntity for unknown sound: {id:?}");
                            //     return;
                            // };

                            // #[cfg(any(debug_assertions, feature = "debug"))]
                            // debug!("Received PlaySoundFromEntity: {registry:?}");

                            // let pos = transform.translation;
                            // world.send_event(SoundEvent {
                            //     asset: registry.into(),
                            //     kind: p.kind,
                            //     position: Some(pos),
                            // });
                        }
                        PacketSoundType::SoundName { registry, .. } => {
                            #[cfg(any(debug_assertions, feature = "debug"))]
                            debug!("Received PlaySoundFromEntity: {registry:?}");

                            let pos = transform.translation;
                            world.send_event(SoundEvent {
                                // TODO: Get the name from the registry
                                // asset: registry.into(),
                                asset: registry,
                                kind: p.kind,
                                position: Some(pos),
                            })
                        }
                    }
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received PlaySoundFromEntity for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::PlaySound(p) => {
                match p.data {
                    PacketSoundType::SoundId(_id) => {
                        // let Some(registry) = SoundRegistry::try_from(id) else {
                        //     warn!("Received PlaySound for unknown sound: {id:?}");
                        //     return;
                        // };

                        // #[cfg(any(debug_assertions, feature = "debug"))]
                        // debug!("Received PlaySound: {registry:?}");

                        // world.send_event(SoundEvent {
                        //     asset: registry.into(),
                        //     kind: p.kind,
                        //     position: None,
                        // });
                    }
                    PacketSoundType::SoundName { registry, .. } => {
                        #[cfg(any(debug_assertions, feature = "debug"))]
                        debug!("Received PlaySound: {registry:?}");

                        world.send_event(SoundEvent {
                            // TODO: Get the name from the registry
                            // asset: registry.into(),
                            asset: registry,
                            kind: p.kind,
                            // TODO: Decode the position
                            position: None,
                        })
                    }
                }
            }
            ClientboundPlayPackets::StopSound(_) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received StopSound");

                // TODO: Parse the packet
            }
            ClientboundPlayPackets::GameMessage(_) => {}
            ClientboundPlayPackets::PlayerListHeader(_) => {}
            ClientboundPlayPackets::NbtQueryResponse(_) => {}
            ClientboundPlayPackets::ItemPickupAnimation(_) => {}
            ClientboundPlayPackets::EntityPosition(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityPosition: {:?}", p.entity_id);

                let mut query = world.query::<(&EntityId, &mut Transform)>();

                // Find the entity with the given id
                if let Some((_, mut transform)) =
                    query.iter_mut(world).find(|(id, _)| id == &&p.entity_id)
                {
                    transform.translation = p.position.into();
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityPosition for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::AdvancementUpdate(_) => {}
            ClientboundPlayPackets::EntityAttributes(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityAttributes: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Update the entity's attributes
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityAttributes for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::Features(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Received Features: {p:?}");

                // TODO: Update the FeaturesList resource
            }
            ClientboundPlayPackets::EntityStatusEffect(p) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                trace!("Received EntityStatusEffect: {:?}", p.entity_id);

                let mut query = world.query::<&EntityId>();

                // Find the entity with the given id
                if query.iter_mut(world).any(|id| id == &p.entity_id) {
                    // TODO: Update the entity's status effects
                } else {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    warn!(
                        "Received EntityStatusEffect for unknown entity: {:?}",
                        p.entity_id
                    );
                }
            }
            ClientboundPlayPackets::SynchronizeRecipes(_) => {}
            ClientboundPlayPackets::SynchronizeTags(_) => {}
        }
    }
}
