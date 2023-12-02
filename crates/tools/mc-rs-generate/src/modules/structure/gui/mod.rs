use std::{collections::HashMap, future::Future, path::PathBuf, pin::Pin};

use git2::Repository;
use mc_rs_extract::ModuleData;
use serde::Deserialize;
use tracing::error;

use crate::modules::ModuleExt;

mod file;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GuiStructureModule;

impl GuiStructureModule {
    const MOD_LAYOUT: &'static str = include_str!("layout.json");
}

impl ModuleExt for GuiStructureModule {
    fn run(&self, _data: &ModuleData, repo: &Repository) -> Pin<Box<dyn Future<Output = ()>>> {
        // Get the path to the menus folder.
        let mut dir = repo.path().parent().unwrap().to_path_buf();
        dir.push("crates/mc-rs-gui/src/menus");

        Box::pin(async move {
            let menus = match serde_json::from_str::<Folder>(Self::MOD_LAYOUT) {
                Ok(layout) => layout,
                Err(err) => {
                    error!("Failed to parse layout.json: {err}");
                    return;
                }
            };

            if let Err(err) = menus.create(dir).await {
                error!("Failed to create Gui crate structure: {err}");
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct Folder(HashMap<String, Folder>);

impl Folder {
    fn create(self, dir: PathBuf) -> Pin<Box<dyn Future<Output = tokio::io::Result<()>>>> {
        Box::pin(async move {
            file::create_file(&dir, "mod", &self).await?;

            for (name, folder) in self.0 {
                if name.ends_with("_file") {
                    file::create_file(&dir, &name, &folder).await?;
                } else {
                    folder.create(dir.join(name)).await?;
                }
            }

            Ok(())
        })
    }
}

impl std::ops::Deref for Folder {
    type Target = HashMap<String, Folder>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
