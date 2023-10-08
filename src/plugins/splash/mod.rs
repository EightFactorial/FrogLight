use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    interface::menus::MenuRoot,
    systems::app_state::{ApplicationState, InMenuSet, MenuSet},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SplashPlugin;

/// A system set that runs when the [ApplicationState] is
/// [SplashScreen](ApplicationState::SplashScreen)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct SplashSet;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlocksLoaded>();
        app.add_systems(Startup, SplashPlugin::load);

        app.add_systems(
            OnEnter(ApplicationState::SplashScreen),
            SplashPlugin::create,
        );

        app.configure_set(
            Update,
            SplashSet
                .run_if(in_state(ApplicationState::SplashScreen))
                .in_set(MenuSet),
        );

        app.add_systems(
            Update,
            SplashPlugin::next_state
                .run_if(SplashPlugin::bar_finished)
                .in_set(SplashSet),
        );

        app.add_systems(
            Update,
            (
                BlocksLoaded::check_loaded.run_if(
                    resource_exists::<BlocksLoaded>().and_then(not(BlocksLoaded::is_loaded)),
                ),
                BlocksLoaded::destroy
                    .run_if(resource_exists::<BlocksLoaded>().and_then(BlocksLoaded::is_loaded)),
            )
                .in_set(MenuSet),
        );

        app.add_systems(
            OnEnter(ApplicationState::InMenu),
            SplashPlugin::delete
                .run_if(any_with_component::<BarMax>())
                .in_set(InMenuSet),
        );
    }
}

/// The maximum value of the progress bar
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component)]
pub struct BarMax(pub f32);

/// The current value of the progress bar
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component)]
pub struct BarValue(pub f32);

impl SplashPlugin {
    /// Load the splash screen stylesheet
    fn load(mut commands: Commands) { commands.add(StyleSheet::load("style/splash.ess")); }

    /// Create the progress bar
    fn create(
        time: Res<Time>,
        // blocks: Res<Blocks>,
        root: Res<MenuRoot>,
        // assets: Res<AssetServer>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        // If the blocks are already loaded, use a timer
        // Otherwise, the main menu will be shown instantly
        // let loaded = blocks.is_loaded(&assets);
        let loaded = true;

        // Set the bar max value
        let max = if loaded {
            // Set the max to the elapsed time + 2 seconds
            BarMax(time.elapsed_seconds() + 2.)
        } else {
            // Set the max to the number of blocks with textures
            BarMax(1.)
        };
        let entity = **root;
        commands.entity(entity).insert((BarValue::default(), max));

        // Add the progress bar
        elements.select(".root").add_child(if loaded {
            // Bind to the elapsed time
            eml! {
                <div class="splash">
                    <div class="splash-text">"Loading..."</div>
                    <progressbar class="splash-bar" minimum=1. maximum={max.0}
                        bind:value=from!(Time:elapsed_seconds())
                        bind:value=to!(entity, BarValue:0)
                    />
                </div>
            }
        } else {
            // Bind to the loaded block count
            eml! {
                <div class="splash">
                    <div class="splash-text">"Loading..."</div>
                    <progressbar class="splash-bar" minimum=0. maximum=1.
                        bind:value=from!(BlocksLoaded:get_percent())
                        bind:value=to!(entity, BarValue:0)
                    />
                </div>
            }
        });
    }

    /// Check if the progress bar is finished
    fn bar_finished(
        query: Query<(&BarMax, &BarValue)>,
        root: Res<MenuRoot>,
        // blocks: Res<Blocks>,
        // asset_server: Res<AssetServer>,
    ) -> bool {
        if let Ok((max, value)) = query.get(**root) {
            value.0 >= max.0
        } else {
            // blocks.is_loaded(&asset_server)
            true
        }
    }

    /// Go to the main menu state
    fn next_state(mut state: ResMut<NextState<ApplicationState>>) {
        state.set(ApplicationState::InMenu);
    }

    /// Delete the splash screen
    fn delete(mut commands: Commands, mut elements: Elements, root: Res<MenuRoot>) {
        // Remove the elements
        elements.select(".root div.splash").remove();

        // Remove the splash screen components
        commands.entity(**root).remove::<BarValue>();
        commands.entity(**root).remove::<BarMax>();
    }
}

/// A resource that is true when all blocks are loaded
#[derive(Clone, Default, PartialEq, Resource, Deref, DerefMut)]
pub struct BlocksLoaded {
    #[deref]
    pub bool: bool,
    pub percent: f32,
}

impl BlocksLoaded {
    /// A system that checks if all the block textures are loaded
    /// and replaces any broken textures with the fallback
    fn check_loaded(
        // mut blocks: ResMut<Blocks>,
        mut loaded: ResMut<BlocksLoaded>,
        // assets: Res<AssetServer>,
    ) {
        // if blocks.is_loaded(&assets) {
        if true {
            // TODO: Replace any failed textures with the error block texture

            // Set the blocks loaded resource to true
            loaded.bool = true;
            loaded.percent = 1.0;

            info!("Loaded all blocks");
            warn!("Fixed {} broken textures", 0);
        } else {
            // let per = blocks.progress(&assets);
            let per = 1.0;
            loaded.percent = per;

            info!("Loaded {}% of blocks", per);
        }
    }

    fn destroy(mut commands: Commands) { commands.remove_resource::<BlocksLoaded>(); }

    /// Get the percent of blocks loaded
    pub fn get_percent(&self) -> f32 { self.percent }

    /// Get if all blocks are loaded
    pub fn is_loaded(loaded: Res<BlocksLoaded>) -> bool { loaded.bool }
}
