use bevy::prelude::*;
use froglight::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FroglightPlugins)
        .add_plugins(BotPlugin)
        .run()
}

// -------------------------------------------------------------------------------------------------

struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, _app: &mut App) {}
}
