use std::collections::HashMap;

use cafebabe::ClassFile;
use dashmap::Entry;
use miette::Result;

use crate::{
    common::{DATA, Version, VersionStorage},
    source::JarFile,
};

pub struct JarData {
    pub classes: HashMap<String, ClassFile<'static>>,
}

impl JarData {
    /// Get the [`JarData`] for the given [`Version`], fetching it if necessary.
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

        let jar_data = {
            if !version_data.contains::<Self>() {
                drop(version_data);
                tracing::info!("Fetching `JarData` for \"{}\"", version.as_str());
                let jardata = Self::fetch(version).await?;
                DATA.get_mut(version).unwrap().insert(jardata);
                version_data = DATA.get(version).unwrap();
            }
            version_data.get::<Self>().unwrap()
        };

        f(jar_data).await
    }

    /// Fetch the [`JarData`] for the given [`Version`].
    #[expect(clippy::case_sensitive_file_extension_comparisons, reason = "It is case sensitive")]
    pub async fn fetch(version: &Version) -> Result<Self> {
        JarFile::get_for(version, |file| {
            let mut reader = file.reader.clone();

            async move {
                let mut classes = HashMap::new();

                for entry in 0..reader.file().entries().len() {
                    if let Some(mut entry) = reader.reader_with_entry(entry).await.ok()
                        && entry.entry().filename().as_str().is_ok_and(|n| n.ends_with(".class"))
                    {
                        let buf = Box::leak(Box::new(Vec::new()));
                        if let Err(_err) = entry.read_to_end_checked(buf).await {
                            todo!()
                        }

                        let filename = entry.entry().filename().as_str().unwrap();
                        match cafebabe::parse_class(buf.as_slice()) {
                            Ok(class) => {
                                classes.insert(filename.to_string(), class);
                            }
                            Err(err) => {
                                tracing::error!("Failed to parse class file \"{filename}\", {err}");
                            }
                        }
                    }
                }

                tracing::debug!("Parsed {} classes for \"{}\"", classes.len(), version.as_str());

                Ok(JarData { classes })
            }
        })
        .await
    }
}
