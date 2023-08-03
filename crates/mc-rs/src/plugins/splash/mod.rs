use belly::prelude::*;
use bevy::prelude::*;

use crate::systems::{
    blocks::block_list::Blocks,
    states::application::{ApplicationState, MenuSet},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SplashPlugin;

/// A system set that runs when the [ApplicationState] is
/// [SplashScreen](ApplicationState::SplashScreen)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct SplashSet;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            SplashSet
                .run_if(in_state(ApplicationState::SplashScreen))
                .ambiguous_with(MenuSet),
        );

        app.add_systems(Startup, SplashPlugin::load);

        app.add_systems(
            OnEnter(ApplicationState::SplashScreen),
            SplashPlugin::create_bar,
        );

        app.add_systems(
            Update,
            SplashPlugin::next_state
                .run_if(SplashPlugin::bar_finished)
                .in_set(SplashSet),
        );
    }
}

/// A marker component for the splash screen
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct SplashRoot;

/// The maximum value of the progress bar
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component)]
struct BarMax(pub f32);

/// The current value of the progress bar
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Component)]
struct BarValue(pub f32);

impl SplashPlugin {
    /// Load the splash screen stylesheet
    fn load(mut commands: Commands) {
        commands.add(StyleSheet::load("style/splash.ess"));
        commands.spawn((SplashRoot, BarValue::default()));
    }

    /// Create the progress bar
    fn create_bar(
        time: Res<Time>,
        blocks: Res<Blocks>,
        query: Query<Entity, With<SplashRoot>>,
        mut commands: Commands,
    ) {
        let entity = query.single();

        // If the blocks are already loaded, use a timer
        // Otherwise, the main menu will be shown instantly
        let loaded = blocks.is_loaded();

        if loaded {
            // Set the maximum value to the elapsed time + 2 seconds
            commands
                .entity(entity)
                .insert(BarMax(time.elapsed_seconds() + 2.));
        } else {
            // Set the maximum value to the number of blocks with textures
            commands
                .entity(entity)
                .insert(BarMax(blocks.blocks_with_textures_f32()));
        }

        if loaded {
            // Bind to the elapsed time
            commands.add(
                eml! {
                    <body class="splash" s:padding="50px">
                        <div class="splash-text">"Loading..."</div>
                        <br />
                        <progressbar class="splash-bar" s:width="400px" minimum=1. bind:maximum=from!(entity, BarMax:0)
                            bind:value=from!(Time:elapsed_seconds())
                            bind:value=to!(entity, BarValue:0)
                        />
                    </body>
                }
            );
        } else {
            // Bind to the loaded block count
            commands.add(
                eml! {
                    <body class="splash" s:padding="50px">
                        <div class="splash-text">"Loading..."</div>
                        <br />
                        <progressbar class="splash-bar" s:width="400px" minimum=0. bind:maximum=from!(entity, BarMax:0) 
                            bind:value=from!(Blocks:blocks_loaded_f32())
                            bind:value=to!(entity, BarValue:0)
                        />
                    </body>
                }
            );
        }
    }

    /// Check if the progress bar is finished
    fn bar_finished(query: Query<(&BarMax, &BarValue), With<SplashRoot>>) -> bool {
        if let Ok((max, value)) = query.get_single() {
            value.0 >= max.0
        } else {
            false
        }
    }

    /// Go to the main menu state
    fn next_state(mut state: ResMut<NextState<ApplicationState>>) {
        state.set(ApplicationState::InMenu);
    }
}
