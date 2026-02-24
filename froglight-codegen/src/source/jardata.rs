use std::collections::HashMap;

use cafebabe::{
    ClassFile, MethodInfo,
    attributes::{AttributeData, CodeData},
};
use miette::Result;

use crate::{
    common::{Version, VersionStorage},
    source::JarFile,
};

pub struct JarData {
    pub classes: HashMap<String, ClassFile<'static>>,
}

impl JarData {
    /// Get a [`ClassFile`] by its name.
    #[must_use]
    pub fn get_class(&self, class: &str) -> Option<&ClassFile<'static>> {
        // tracing::trace!("Looking up class \"{class}\"");
        self.classes.get(class.trim_end_matches(".class"))
    }

    /// Get a [`MethodInfo`] by its class and method name.
    #[must_use]
    pub fn get_class_method(&self, class: &str, method: &str) -> Option<&MethodInfo<'static>> {
        let class_file = self.get_class(class)?;
        class_file.methods.iter().find(|m| m.name == method)
    }

    /// Get the [`CodeData`] for a given class and method.
    #[must_use]
    pub fn get_class_method_code(&self, class: &str, method: &str) -> Option<&CodeData<'static>> {
        let Some(method_info) = self.get_class_method(class, method) else {
            if let Some(class_file) = self.get_class(class) {
                tracing::trace!(
                    "Available methods in class \"{class}\": {:?}",
                    class_file.methods.iter().map(|m| m.name.as_ref()).collect::<Vec<_>>()
                );
            }
            return None;
        };
        let Some(code_attr) = method_info.attributes.iter().find(|a| a.name == "Code") else {
            tracing::trace!(
                "Available attributes in method \"{method}\" of class \"{class}\": {:?}",
                method_info.attributes.iter().map(|a| a.name.as_ref()).collect::<Vec<_>>()
            );
            return None;
        };
        if let AttributeData::Code(code) = &code_attr.data { Some(code) } else { None }
    }

    /// Get the [`JarData`] for the given [`Version`], fetching it if necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `JarData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`JarData`] for the given [`Version`].
    #[expect(clippy::case_sensitive_file_extension_comparisons, reason = "It is case sensitive")]
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        JarFile::get_for(version, storage, async |file| {
            let mut reader = file.reader.clone();
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
                            classes.insert(filename.trim_end_matches(".class").to_string(), class);
                        }
                        Err(err) => {
                            tracing::error!("Failed to parse class file \"{filename}\", {err}");
                        }
                    }
                }
            }

            tracing::debug!("Parsed {} classes for \"{}\"", classes.len(), version.as_str());

            Ok(JarData { classes })
        })
        .await
    }
}
