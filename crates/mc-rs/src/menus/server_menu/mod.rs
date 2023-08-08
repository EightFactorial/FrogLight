use std::{fs, path::PathBuf};

use belly::prelude::*;
use bevy::{prelude::*, utils::HashMap};
use fastnbt::Value;
use serde::{Deserialize, Serialize};

use crate::{
    systems::app_state::{ApplicationState, InMenuSet},
    util::mc_dir::minecraft_dir,
};

use super::{main_menu::MainMenu, MenuRoot};

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        ServerMenu::create
            .run_if(not(any_with_component::<ServerMenu>()))
            .in_set(InMenuSet),
    );
}

/// A marker component for the server menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ServerMenu;

impl ServerMenu {
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
                                <button c:server-listing-button></button>
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
}

/// A list of servers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerList {
    pub servers: Vec<ServerListing>,
}

impl ServerList {
    /// Load the server list
    pub fn load() -> Result<ServerList, anyhow::Error> {
        Ok(fastnbt::from_bytes(&fs::read(Self::path()?)?)?)
    }

    /// Save the server list
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), anyhow::Error> {
        fs::write(Self::path()?, fastnbt::to_bytes(self)?)?;
        Ok(())
    }

    /// Get the path to the server list
    fn path() -> anyhow::Result<PathBuf> {
        Ok(minecraft_dir()
            .ok_or(anyhow::anyhow!("Unable to find Minecraft dir"))?
            .join("servers.dat"))
    }
}

/// A server in the server menu
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerListing {
    pub ip: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}
