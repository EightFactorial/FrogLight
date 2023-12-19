use std::{future::Future, pin::Pin};

use git2::Repository;
use mc_rs_extract::ModuleData;
use tokio::fs::File;
use tracing::{error, info, trace};
use zip::ZipWriter;

use super::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResourcePackModule;

impl ModuleExt for ResourcePackModule {
    fn run<'a>(
        &self,
        data: &'a mut ModuleData,
        repo: &Repository,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        let resourcepack_path = repo
            .path()
            .parent()
            .unwrap()
            .to_path_buf()
            .join("minecraft.zip");

        Box::pin(async move {
            let Ok(resourcepack) = File::create(&resourcepack_path).await else {
                error!("Failed to create {}", resourcepack_path.display());
                return;
            };

            // Create the resourcepack zip file
            let mut resourcepack = ZipWriter::new(resourcepack.into_std().await);

            info!("Copying assets from jar...");
            for zip_index in 0..data.zip.len() {
                let zip_file = match data.zip.by_index(zip_index) {
                    Ok(zip_file) => zip_file,
                    Err(err) => {
                        error!("Failed to read zip file: {err}");
                        continue;
                    }
                };

                if !zip_file.name().starts_with("pack") && !zip_file.name().starts_with("assets") {
                    continue;
                }

                trace!("{}", zip_file.name());
                if let Err(err) = resourcepack.raw_copy_file(zip_file) {
                    error!("Failed to copy asset from jar: {err}");
                }
            }

            info!("TODO: Download assets from Mojang");
        })
    }
}
