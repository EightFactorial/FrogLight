#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::{App, AppExit};
use froglight::ApplicationPlugins;

#[cfg(feature = "inspector")]
mod inspector;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(ApplicationPlugins);

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(inspector::InspectorPlugin);
    }

    app.run()
}
