use bevy::app::App;
use mc_rs_client::ClientPlugins;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() { App::new().add_plugins(ClientPlugins).run(); }
