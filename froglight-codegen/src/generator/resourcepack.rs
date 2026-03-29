use async_zip::{
    Compression, StringEncoding, ZipEntryBuilder, ZipString, tokio::write::ZipFileWriter,
};
use miette::Result;
use tokio::{fs::File, sync::RwLock};

use crate::{
    common::{DATA, REQWEST, Version},
    config::ConfigBundle,
    source::{JarFile, VersionAssets},
};

/// Generate a `vanilla.zip` resource pack.
pub async fn generate(config: &ConfigBundle) -> Result<()> {
    if !std::env::args().any(|arg| arg == "--resource-pack") {
        return Ok(());
    }

    let version =
        if let Some(arg) = std::env::args().find(|arg| arg.starts_with("--resource-pack=")) {
            Version::new(arg.trim_start_matches("--resource-pack="))
        } else {
            config.versions.last().unwrap().real.clone()
        };

    let pinned = DATA.pin_owned();
    let storage = pinned.get_or_insert_with(version.clone(), RwLock::default);
    let mut storage = storage.write().await;

    let mut contents = Vec::new();
    let mut writer = ZipFileWriter::with_tokio(File::create("vanilla.zip").await.unwrap());

    JarFile::get_for(&version, &mut storage, async |jar| {
        let mut reader = jar.reader.clone();
        let mut buffer = Vec::with_capacity(1024);

        for (index, file) in jar.reader.file().entries().iter().enumerate() {
            // Filter out non-asset files
            let Ok(filename) = file.filename().as_str() else { continue };
            if !(filename.starts_with("assets/") || filename.starts_with("pack")) {
                continue;
            }

            // Clear and reuse the buffer
            buffer.clear();
            let mut reader = reader.reader_with_entry(index).await.unwrap();
            reader.read_to_end_checked(&mut buffer).await.unwrap();

            // Add to the content list for future reference
            contents.push((filename.to_string(), buffer.len()));

            // Write the file to the zip
            let entry = ZipEntryBuilder::new(
                ZipString::new(String::from(filename).into_bytes(), StringEncoding::Utf8),
                Compression::Deflate,
            );
            writer.write_entry_whole(entry.build(), &buffer).await.unwrap();
        }

        Ok(())
    })
    .await?;

    VersionAssets::get_for(&version, &mut storage, async |assets| {
        for (index, (name, asset)) in assets.objects.iter().enumerate() {
            if !(name.starts_with("minecraft/") || name.starts_with("pack")) {
                continue;
            }

            if index % 100 == 0 {
                tracing::info!("ResourcePack: {index}/{}", assets.objects.len());
            }

            let mut path = name.clone();
            if !name.starts_with("pack") {
                path = format!("assets/{path}");
            }

            // Fetch from network
            let response = match REQWEST.get(asset.url()).send().await {
                Ok(response) => response,
                Err(_err) => todo!(),
            };
            let bytes = response.bytes().await.unwrap();

            // Write the file to the zip
            let entry = ZipEntryBuilder::new(
                ZipString::new(path.into_bytes(), StringEncoding::Utf8),
                Compression::Deflate,
            );
            writer.write_entry_whole(entry.build(), &bytes).await.unwrap();
        }

        Ok(())
    })
    .await?;

    writer.close().await.unwrap();
    Ok(())
}
