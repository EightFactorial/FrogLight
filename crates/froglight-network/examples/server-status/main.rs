//! Send a status request to [`SERVER_ADDRESS`] and prints the response.

use bevy::{app::AppExit, prelude::*};
use bevy_log::LogPlugin;
use froglight_network::{
    network::{ConnectionTrait, NetworkErrorEvent, PolledTask, ServerStatusResponse},
    resolver::Resolver,
    versions::v1_21_0::V1_21_0,
    NetworkPlugins,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default(), NetworkPlugins.as_plugingroup()));

    app.add_systems(
        Update,
        (
            send_status_request.run_if(run_once()),
            print_status_response.run_if(on_event::<ServerStatusResponse>()),
            exit_on_error.run_if(on_event::<NetworkErrorEvent>()),
        )
            .chain(),
    );

    app.run();
}

const SERVER_ADDRESS: &str = "localhost";

/// Send a status request to [`SERVER_ADDRESS`].
fn send_status_request(mut commands: Commands, resolver: Res<Resolver>) {
    info!("Connecting to \"{SERVER_ADDRESS}\"...");
    let task = V1_21_0::status(SERVER_ADDRESS, &resolver);
    commands.spawn((task, PolledTask));
    info!("Waiting for status response...");
}

/// Print the status response and exit.
fn print_status_response(
    mut events: EventReader<ServerStatusResponse>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(event) = events.read().next() {
        info!("Ping: {:?}", event.ping);
        info!("Status:\n{}", serde_json::to_string_pretty(&event.status).unwrap());
        info!("Exiting...");
        exit.send(AppExit);
    }
}

/// Exit when a network error occurs.
///
/// The error will already be logged, so we just need to exit.
fn exit_on_error(mut events: EventReader<NetworkErrorEvent>, mut exit: EventWriter<AppExit>) {
    if let Some(error) = events.read().next() {
        error!("Error: {}", error.error);
        error!("Exiting...");
        exit.send(AppExit);
    }
}
