use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    menus::MenuRoot,
    systems::{
        blocks::block_list::Blocks,
        states::application::{ApplicationState, InMenuSet, MenuSet},
    },
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
                .in_set(MenuSet),
        );

        app.add_systems(Startup, SplashPlugin::load);

        app.add_systems(
            OnEnter(ApplicationState::SplashScreen),
            SplashPlugin::create,
        );

        app.add_systems(
            Update,
            SplashPlugin::next_state
                .run_if(SplashPlugin::bar_finished)
                .in_set(SplashSet),
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
        blocks: Res<Blocks>,
        query: Query<Entity, With<MenuRoot>>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        // If the blocks are already loaded, use a timer
        // Otherwise, the main menu will be shown instantly
        let loaded = blocks.is_loaded();

        // Set the bar max value
        let max = if loaded {
            // Set the max to the elapsed time + 2 seconds
            BarMax(time.elapsed_seconds() + 2.)
        } else {
            // Set the max to the number of blocks with textures
            BarMax(blocks.blocks_with_textures_f32())
        };
        let entity = query.single();
        commands.entity(entity).insert((BarValue::default(), max));

        // Add the progress bar
        elements.select(".root").add_child(if loaded {
            // Bind to the elapsed time
            eml! {
                <div class="splash" s:padding="50px">
                    <div class="splash-text">"Loading..."</div>
                    <br />
                    <progressbar class="splash-bar" minimum=1. bind:maximum=from!(entity, BarMax:0)
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
                    <br />
                    <progressbar class="splash-bar" minimum=0. bind:maximum=from!(entity, BarMax:0)
                        bind:value=from!(Blocks:blocks_loaded_f32())
                        bind:value=to!(entity, BarValue:0)
                    />
                </div>
            }
        });
    }

    /// Check if the progress bar is finished
    fn bar_finished(query: Query<(&BarMax, &BarValue), With<MenuRoot>>) -> bool {
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

    /// Delete the splash screen
    fn delete(
        mut commands: Commands,
        mut elements: Elements,
        query: Query<Entity, With<MenuRoot>>,
    ) {
        // Remove the elements
        for entity in elements.select(".splash").entities() {
            commands.entity(entity).despawn_recursive();
        }

        // Remove the splash screen components
        commands.entity(query.single()).remove::<BarValue>();
        commands.entity(query.single()).remove::<BarMax>();
    }
}
