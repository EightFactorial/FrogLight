use std::{io::Cursor, path::PathBuf, process::Stdio};

use async_zip::tokio::read::seek::ZipFileReader;
use miette::Result;
use tokio::process::Command;

use crate::{
    common::{CACHE_DIR, REQWEST, Version, VersionStorage},
    source::VersionData,
};

pub struct JarFile {
    pub client: PathBuf,
    pub server: PathBuf,
    pub generated: PathBuf,

    pub raw: &'static Vec<u8>,
    pub reader: ZipFileReader<Cursor<&'static [u8]>>,
}

impl JarFile {
    /// Get the [`JarFile`] for the given [`Version`], fetching it if necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `JarFile` for \"{}\"", version.as_str());
            let data = Self::fetch(version, storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`JarFile`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut path = CACHE_DIR.clone();
        path.push(version.as_feature());

        let client = path.join(format!("{}_client.jar", version.as_str()));
        let server = path.join(format!("{}_server.jar", version.as_str()));

        let content = if client.exists() && server.exists() {
            tracing::debug!("Using cached `JarFile` for \"{}\"", version.as_str());

            match tokio::fs::read(&client).await {
                Ok(content) => content,
                Err(_err) => todo!(),
            }
        } else {
            tracing::debug!("Downloading `JarFile` for \"{}\"", version.as_str());

            VersionData::get_for(version, storage, async |data| {
                for (index, url) in
                    [&data.downloads.server.url, &data.downloads.client.url].into_iter().enumerate()
                {
                    let response = match REQWEST.get(url).send().await {
                        Ok(response) => response,
                        Err(_err) => todo!(),
                    };
                    let content = match response.bytes().await {
                        Ok(bytes) => bytes.to_vec(),
                        Err(_err) => todo!(),
                    };

                    // Ensure parent directory exists
                    let path = if index == 0 { &server } else { &client };
                    tracing::debug!("Caching `JarFile` for \"{}\" at {:?}", version.as_str(), path);
                    if let Some(parent) = path.parent()
                        && !parent.exists()
                        && let Err(_err) = tokio::fs::create_dir_all(parent).await
                    {
                        todo!()
                    }

                    // Write to cache
                    if let Err(_err) = tokio::fs::write(path, &content).await {
                        todo!()
                    }

                    if index == 1 {
                        return Ok(content);
                    }
                }
                unreachable!();
            })
            .await?
        };

        // Run the data generator if necessary.
        let mut generated = server.clone();
        generated.set_file_name("generated");

        if !generated.exists() {
            let mut command = Command::new("java");
            command.current_dir(server.parent().unwrap());
            command.args(["-DbundlerMainClass=net.minecraft.data.Main", "-jar"]);
            command.arg(&server).arg("--all");
            let mut task = command.stdout(Stdio::null()).spawn().unwrap();
            task.wait().await.unwrap();
        }

        let raw: &'static Vec<u8> = Box::leak(Box::new(content));
        match ZipFileReader::with_tokio(Cursor::new(raw.as_slice())).await {
            Ok(reader) => Ok(Self { client, server, generated, raw, reader }),
            Err(_err) => todo!(),
        }
    }
}
