use std::{io::Cursor, path::Path};

use async_zip::tokio::read::seek::ZipFileReader;
use miette::Result;
use reqwest::Response;

use crate::{
    common::{CACHE_DIR, DATA, REQWEST, Version, VersionStorage},
    source::{Manifest, VersionData},
};

pub struct JarFile {
    pub raw: &'static Vec<u8>,
    pub reader: ZipFileReader<Cursor<&'static [u8]>>,
}

impl JarFile {
    /// Get the [`JarFile`] for the given [`Version`], fetching it if necessary.
    pub async fn get_for<F: FnOnce(&Self) -> Fut, Fut: Future<Output = Result<V>>, V>(
        version: &Version,
        f: F,
    ) -> Result<V> {
        let mut version_data = {
            if !DATA.contains_key(version) {
                DATA.insert(version.clone(), VersionStorage::default());
            }
            DATA.get(version).unwrap()
        };

        let jar_file = {
            if !version_data.contains::<Self>() {
                drop(version_data);
                tracing::info!("Fetching `JarFile` for \"{}\"", version.as_str());
                let jarfile = Self::fetch(version).await?;
                DATA.get_mut(version).unwrap().insert(jarfile);
                version_data = DATA.get(version).unwrap();
            }
            version_data.get::<Self>().unwrap()
        };

        f(jar_file).await
    }

    /// Fetch the [`JarFile`] for the given [`Version`].
    pub async fn fetch(version: &Version) -> Result<Self> {
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

            let content = VersionData::get_for(version, |data| {
                let url = data.downloads.client.url.clone();
                async move {
                    let response = match REQWEST.get(url).send().await {
                        Ok(response) => response,
                        Err(_err) => todo!(),
                    };
                    match response.bytes().await {
                        Ok(bytes) => Ok(bytes.to_vec()),
                        Err(_err) => todo!(),
                    }
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
