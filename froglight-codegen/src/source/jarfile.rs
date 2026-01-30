use std::io::Cursor;

use async_zip::tokio::read::seek::ZipFileReader;
use miette::Result;

use crate::{
    common::{CACHE_DIR, REQWEST, Version, VersionStorage},
    source::VersionData,
};

pub struct JarFile {
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
        let jar = format!("{}.jar", version.as_str());
        path.push(version.as_feature());
        path.push(&jar);

        let content = if path.exists() {
            tracing::debug!("Using cached `JarFile` for \"{}\"", version.as_str());

            match tokio::fs::read(&path).await {
                Ok(content) => content,
                Err(_err) => todo!(),
            }
        } else {
            tracing::debug!("Downloading `JarFile` for \"{}\"", version.as_str());

            let content = VersionData::get_for(version, storage, async |data| {
                let response = match REQWEST.get(&data.downloads.client.url).send().await {
                    Ok(response) => response,
                    Err(_err) => todo!(),
                };
                match response.bytes().await {
                    Ok(bytes) => Ok(bytes.to_vec()),
                    Err(_err) => todo!(),
                }
            })
            .await?;

            // Ensure parent directory exists
            tracing::debug!("Caching `JarFile` for \"{}\" at {:?}", version.as_str(), path);
            if let Some(parent) = path.parent()
                && !parent.exists()
                && let Err(_err) = tokio::fs::create_dir_all(parent).await
            {
                todo!()
            }

            // Write to cache
            if let Err(_err) = tokio::fs::write(&path, &content).await {
                todo!()
            }

            content
        };

        let leaked: &'static Vec<u8> = Box::leak(Box::new(content));
        match ZipFileReader::with_tokio(Cursor::new(leaked.as_slice())).await {
            Ok(reader) => Ok(Self { raw: leaked, reader }),
            Err(_err) => todo!(),
        }
    }
}
