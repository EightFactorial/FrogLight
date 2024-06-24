//! Send a status request to "localhost" and prints the response.

use bevy::{app::AppExit, prelude::*};
use froglight_internal::{prelude::*, HeadlessPlugins};
use froglight_network::{
    network::{ConnectionTrait, NetworkErrorEvent, ServerStatusResponse},
    versions::v1_21_0::V1_21_0,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

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

/// Send a status request to "localhost"
fn send_status_request(mut commands: Commands, resolver: Res<Resolver>) {
    let task = V1_21_0::request_status_of("localhost", &resolver);
    commands.spawn(task);
}

/// Print the status response and exit.
fn print_status_response(
    mut events: EventReader<ServerStatusResponse>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(event) = events.read().next() {
        println!("Entity {:?}", event.entity);
        println!("Ping: {:?}", event.ping);
        println!("Status: {:#?}", event.status);

        println!();
        println!("Exiting...");
        exit.send(AppExit);
    }
}

/// Exit when a network error occurs.
///
/// The error will already be logged, so we just need to exit.
fn exit_on_error(mut events: EventReader<NetworkErrorEvent>, mut exit: EventWriter<AppExit>) {
    if events.read().next().is_some() {
        error!("Exiting due to error...");
        exit.send(AppExit);
    }
}
