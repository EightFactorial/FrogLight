use std::{fs, io::Cursor, path::PathBuf};

use azalea_nbt::{Nbt, NbtList};
use belly::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::{
    schedule::{set::InMenuSet, state::ApplicationState},
    versions::DefaultVersion,
};
use mc_rs_network::{request::StatusRequest, ConnectionEvent};

use crate::util::mc_dir::minecraft_dir;

use super::{main_menu::MainMenu, MenuRoot};

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) {
    app.add_event::<ServerMenuPing>();

    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        ServerMenu::create
            .run_if(not(any_with_component::<ServerMenu>()))
            .in_set(InMenuSet),
    );

    app.add_systems(
        Update,
        ServerMenu::ping_servers
            .run_if(any_with_component::<ServerMenu>())
            .in_set(InMenuSet),
    );
}

/// A marker component for the server menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ServerMenu;

impl ServerMenu {
    #[allow(dead_code)]
    pub fn show(mut elements: Elements) {
        elements.select("div.server-menu").remove_class("hidden");
    }

    #[allow(dead_code)]
    pub fn hide(mut elements: Elements) { elements.select("div.server-menu").add_class("hidden"); }

    fn create(root: Res<MenuRoot>, mut elements: Elements, mut commands: Commands) {
        let servers = match ServerList::load() {
            Ok(servers) => servers,
            Err(e) => {
                error!("Unable to load server list: {}", e);
                return;
            }
        };

        commands.entity(**root).insert(ServerMenu);
        elements.select(".root").add_child(eml! {
            <div c:server-menu c:hidden>
                <div c:server-list>
                    <for server in=servers.servers>
                        <div c:server-listing>
                            <div c:server-listing-info>
                                // TODO: Get version from query
                                // TODO: Add description from query
                                // TODO: Add icon from file or query
                                <button c:server-listing-button on:press={
                                    let address = server.ip.clone();
                                    move |ctx| { ctx.send_event(ConnectionEvent::<DefaultVersion>::new(address.clone())) }
                                }></button>
                                <div c:server-listing-name>{ server.name }</div>
                                <div c:server-listing-ip>{ server.ip }</div>
                            </div>
                        </div>
                    </for>
                </div>
            </div>
        });
    }

    /// Handle the escape button
    pub fn handle_escape(mut elements: Elements) {
        elements.select("div.server-menu").add_class("hidden");
        MainMenu::show(elements);
    }

    /// Ping all servers in the server list when the event is sent
    pub fn ping_servers(
        query: Query<&Text>,
        children: Query<&Children>,
        mut events: EventReader<ServerMenuPing>,
        mut requests: EventWriter<StatusRequest<DefaultVersion>>,
        mut elements: Elements,
    ) {
        for _ in events.iter() {
            for entity in elements.select("div.server-listing-ip").entities() {
                if let Ok(children) = children.get(entity) {
                    for child in children {
                        if let Ok(Text { sections, .. }) = query.get(*child) {
                            if let Some(ip) = sections.first() {
                                debug!("Sending ping to {}", ip.value);
                                requests.send(StatusRequest::new(ip.value.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A list of servers
#[derive(Debug, Clone, PartialEq)]
pub struct ServerList {
    pub servers: Vec<ServerListing>,
}

impl ServerList {
    /// Load the server list
    pub fn load() -> Result<ServerList, anyhow::Error> {
        let file = fs::read(Self::path()?)?;
        let nbt = Nbt::read(&mut Cursor::new(file)).map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let root = nbt
            .as_compound()
            .ok_or(anyhow::anyhow!("Nbt is not a Nbt compound"))?
            .get("")
            .ok_or(anyhow::anyhow!("Nbt has no item at ``"))?
            .as_compound()
            .ok_or(anyhow::anyhow!("Nbt at `` not a Nbt compound"))?;

        let servers = root
            .get("servers")
            .ok_or(anyhow::anyhow!("Nbt has no `servers` item"))?
            .as_list()
            .ok_or(anyhow::anyhow!("`servers` not a Nbt list"))?;

        let NbtList::Compound(servers) = servers else {
            return Err(anyhow::anyhow!("`servers` not a Nbt list of Nbt compounds"));
        };

        let mut vec = Vec::with_capacity(servers.len());
        for server in servers {
            let mut listing = ServerListing::default();
            for (key, value) in server.iter() {
                match key.as_str() {
                    "name" => {
                        listing.name = value
                            .as_string()
                            .ok_or(anyhow::anyhow!("server `name` not a Nbt string"))?
                            .to_string();
                    }
                    "ip" => {
                        listing.ip = value
                            .as_string()
                            .ok_or(anyhow::anyhow!("server `ip` not a Nbt string"))?
                            .to_string();
                    }
                    "icon" => {
                        listing.icon = Some(
                            value
                                .as_string()
                                .ok_or(anyhow::anyhow!("server `icon` not a Nbt string"))?
                                .to_string(),
                        );
                    }
                    _ => {
                        listing.other.insert(key.to_string(), value.clone());
                    }
                }
            }
            vec.push(listing);
        }

        Ok(Self { servers: vec })
    }

    /// Save the server list
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), anyhow::Error> {
        todo!();
    }

    /// Get the path to the server list
    fn path() -> anyhow::Result<PathBuf> {
        Ok(minecraft_dir()
            .ok_or(anyhow::anyhow!("Unable to find Minecraft dir"))?
            .join("servers.dat"))
    }
}

/// A server in the server menu
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerListing {
    pub ip: String,
    pub name: String,
    pub icon: Option<String>,
    pub other: HashMap<String, Nbt>,
}

/// An event that triggers a status request to all known servers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ServerMenuPing;
