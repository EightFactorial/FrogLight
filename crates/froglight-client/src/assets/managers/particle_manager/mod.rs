use bevy::prelude::*;
use froglight_assets::assets::{ParticleDefinition, ResourcePack};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

mod event;
pub use event::ParticleEvent;

mod particle;

use super::{AssetManager, LanguageManager, SoundManager};
use crate::{
    assets::{AssetLoading, ResourcePackSettings},
    systemsets::ClientPostUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ParticleManager>()
        .register_type::<ParticleManager>()
        .init_resource::<ParticleManagerState>()
        .register_type::<ParticleManagerState>()
        .add_event::<ParticleEvent>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        ParticleManager::reset_particle_manager.run_if(resource_exists::<ParticleManager>),
    );
    app.add_systems(
        Update,
        ParticleManager::populate_particle_manager
            .run_if(not(ParticleManager::is_finished))
            .run_if(resource_exists::<ParticleManager>)
            .ambiguous_with(AssetManager::populate_asset_manager)
            .ambiguous_with(LanguageManager::populate_language_manager)
            .ambiguous_with(SoundManager::populate_sound_manager)
            .in_set(AssetLoading::Processing),
    );

    app.add_systems(
        PostUpdate,
        ParticleManager::handle_particle_events
            .run_if(resource_exists::<ParticleManager>)
            .in_set(ClientPostUpdateSet),
    );
}

/// A [`Resource`] for managing particles.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ParticleManager {
    /// Particles
    pub particles: HashMap<ResourceKey, ParticleDefinition>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Default, Resource)]
#[allow(unreachable_pub)]
pub struct ParticleManagerState {
    finished: bool,
    current: usize,
}

impl ParticleManager {
    /// Handle vanilla particle events.
    ///
    /// TODO: Implement particles.
    ///
    /// Plugins can spawn particles by handling custom [`ParticleEvent`]s.
    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    pub fn handle_particle_events(
        manager: Res<ParticleManager>,
        mut events: EventReader<ParticleEvent>,
        mut _commands: Commands,
    ) {
        for event in events.read() {
            let Some(_definition) = manager.particles.get(&event.key) else {
                #[cfg(debug_assertions)]
                warn!("No particle definition found: \"{}\"", event.key);
                continue;
            };

            match event.key.as_str() {
                "minecraft:ambient_entity_effect" => {}
                "minecraft:angry_villager" => {}
                "minecraft:block" => {}
                "minecraft:block_marker" => {}
                "minecraft:bubble" => {}
                "minecraft:cloud" => {}
                "minecraft:crit" => {}
                "minecraft:damage_indicator" => {}
                "minecraft:dragon_breath" => {}
                "minecraft:dripping_lava" => {}
                "minecraft:falling_lava" => {}
                "minecraft:landing_lava" => {}
                "minecraft:dripping_water" => {}
                "minecraft:falling_water" => {}
                "minecraft:dust" => {}
                "minecraft:dust_color_transition" => {}
                "minecraft:effect" => {}
                "minecraft:elder_guardian" => {}
                "minecraft:enchanted_hit" => {}
                "minecraft:enchant" => {}
                "minecraft:end_rod" => {}
                "minecraft:entity_effect" => {}
                "minecraft:explosion_emitter" => {}
                "minecraft:explosion" => {}
                "minecraft:gust" => {}
                "minecraft:gust_emitter" => {}
                "minecraft:sonic_boom" => {}
                "minecraft:falling_dust" => {}
                "minecraft:firework" => {}
                "minecraft:fishing" => {}
                "minecraft:flame" => {}
                "minecraft:cherry_leaves" => {}
                "minecraft:sculk_soul" => {}
                "minecraft:sculk_charge" => {}
                "minecraft:sculk_charge_pop" => {}
                "minecraft:soul_fire_flame" => {}
                "minecraft:soul" => {}
                "minecraft:flash" => {}
                "minecraft:happy_villager" => {}
                "minecraft:composter" => {}
                "minecraft:heart" => {}
                "minecraft:instant_effect" => {}
                "minecraft:item" => {}
                "minecraft:vibration" => {}
                "minecraft:item_slime" => {}
                "minecraft:item_snowball" => {}
                "minecraft:large_smoke" => {}
                "minecraft:lava" => {}
                "minecraft:mycelium" => {}
                "minecraft:note" => {}
                "minecraft:poof" => {}
                "minecraft:portal" => {}
                "minecraft:rain" => {}
                "minecraft:smoke" => {}
                "minecraft:white_smoke" => {}
                "minecraft:sneeze" => {}
                "minecraft:spit" => {}
                "minecraft:squid_ink" => {}
                "minecraft:sweep_attack" => {}
                "minecraft:totem_of_undying" => {}
                "minecraft:underwater" => {}
                "minecraft:splash" => {}
                "minecraft:witch" => {}
                "minecraft:bubble_pop" => {}
                "minecraft:current_down" => {}
                "minecraft:bubble_column_up" => {}
                "minecraft:nautilus" => {}
                "minecraft:dolphin" => {}
                "minecraft:campfire_cosy_smoke" => {}
                "minecraft:campfire_signal_smoke" => {}
                "minecraft:dripping_honey" => {}
                "minecraft:falling_honey" => {}
                "minecraft:landing_honey" => {}
                "minecraft:falling_nectar" => {}
                "minecraft:falling_spore_blossom" => {}
                "minecraft:ash" => {}
                "minecraft:crimson_spore" => {}
                "minecraft:warped_spore" => {}
                "minecraft:spore_blossom_air" => {}
                "minecraft:dripping_obsidian_tear" => {}
                "minecraft:falling_obsidian_tear" => {}
                "minecraft:landing_obsidian_tear" => {}
                "minecraft:reverse_portal" => {}
                "minecraft:white_ash" => {}
                "minecraft:small_flame" => {}
                "minecraft:dripping_dripstone_lava" => {}
                "minecraft:falling_dripstone_lava" => {}
                "minecraft:dripping_dripstone_water" => {}
                "minecraft:falling_dripstone_water" => {}
                "minecraft:glow_squid_ink" => {}
                "minecraft:glow" => {}
                "minecraft:wax_on" => {}
                "minecraft:wax_off" => {}
                "minecraft:electric_spark" => {}
                "minecraft:scrape" => {}
                "minecraft:shriek" => {}
                "minecraft:egg_crack" => {}
                "minecraft:dust_plume" => {}
                "minecraft:trial_spawner_detection" => {}
                _ => {}
            }
        }
    }

    /// Returns `true` if the [`ParticleManager`] has finished loading all
    /// particles.
    #[must_use]
    pub fn is_finished(state: Res<ParticleManagerState>) -> bool { state.finished }

    /// Resets the [`ParticleManager`] to its initial state.
    fn reset_particle_manager(
        mut manager: ResMut<ParticleManager>,
        mut state: ResMut<ParticleManagerState>,
    ) {
        manager.particles.clear();
        state.finished = false;
        state.current = 0;
    }

    /// Populates the [`ParticleManager`] with particles.
    ///
    /// Does not rely on any other asset managers.
    pub(crate) fn populate_particle_manager(
        settings: Res<ResourcePackSettings>,
        mut manager: ResMut<ParticleManager>,
        mut state: ResMut<ParticleManagerState>,
        mut assets: ResMut<Assets<ResourcePack>>,
    ) {
        // Get the current `ResourcePack` from the list
        if let Some(pack_item) = settings.resourcepacks.get(state.current) {
            // If the `ResourcePack` has a handle
            if let Some(pack_handle) = pack_item.handle.as_ref() {
                // Access the `ResourcePack` data
                if let Some(resourcepack) = assets.get_mut(pack_handle) {
                    // Take the particles from the `ResourcePack`,
                    // if they don't already exist.
                    for (resourcekey, particle) in std::mem::take(&mut resourcepack.particles) {
                        manager.particles.entry(resourcekey).or_insert(particle);
                    }
                } else if let Some(path) = &pack_item.path {
                    error!("Failed to access ResourcePack: \"{path}\"");
                } else {
                    error!("Failed to access ResourcePack: #{}", state.current);
                }
            }
        }

        // Increment the current `ResourcePack` index
        state.current += 1;

        // Set the finished flag if all `ResourcePack`s have been loaded
        if state.current >= settings.resourcepacks.len() {
            #[cfg(debug_assertions)]
            debug!("Loaded \"{}\" particle defintions", manager.particles.len());

            state.finished = true;
        }
    }
}
