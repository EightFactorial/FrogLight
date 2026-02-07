use std::{cmp::Ordering, path::PathBuf};

use miette::Result;

#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    name: String,
    folder: PathBuf,
    docs: String,
    precontent: String,
    imports: Vec<ModuleImport>,
    content: String,
}

#[derive(Debug, Default, Clone)]
pub struct SubModuleSettings {
    pub feature: Option<String>,
    pub public: bool,
    pub import_from: Vec<String>,
    pub reexport: bool,
}

impl ModuleBuilder {
    /// Create a new [`ModuleBuilder`].
    #[must_use]
    pub fn new<S: ToString + ?Sized>(name: &S, folder: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            folder,
            docs: String::new(),
            precontent: String::new(),
            imports: Vec::new(),
            content: String::new(),
        }
    }

    /// Create a new [`ModuleBuilder`], copying content from an existing module.
    ///
    /// ## Warning
    ///
    /// This will only copy content up to the `@generated` marker,
    /// meaning any content after that will be lost.
    pub async fn new_after_marker(mut path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            path.push("mod.rs");
        }

        let Some(filename) = path.file_name().and_then(|name| name.to_str()) else {
            miette::bail!("Invalid module file \"{}\"", path.display());
        };
        let Some(folder) = path.parent() else {
            miette::bail!("Invalid module file \"{}\"", path.display());
        };

        let Ok(contents) = tokio::fs::read_to_string(&path).await else {
            miette::bail!("Failed to read module file \"{}\"", path.display());
        };

        // Find the last occurrence of the @generated marker, plus a few lines.
        let Some(marker) = contents.lines().position(|l| l.contains("@generated")) else {
            miette::bail!("No `@generated` marker found in module file \"{}\"", path.display());
        };

        // Create a new `ModuleBuilder` with content up to the marker.
        let mut builder = Self::new(filename, folder.to_path_buf());
        for line in contents.lines().take(marker + 1) {
            if line.starts_with("//!") {
                builder.docs.push_str(line.trim_start_matches("//!").trim());
                builder.docs.push('\n');
            } else {
                builder.content.push_str(line);
                builder.content.push('\n');
            }
        }
        builder.content.push('\n');
        Ok(builder)
    }

    /// Add documentation to the module.
    pub fn with_docs<S: AsRef<str> + ?Sized>(&mut self, docs: &S) -> &mut Self {
        if !self.docs.is_empty() {
            self.docs.push('\n');
        }
        self.docs.push_str(docs.as_ref());
        self
    }

    /// Add content to the beginning of the module.
    pub fn with_precontent<S: AsRef<str> + ?Sized>(&mut self, content: &S) -> &mut Self {
        if !self.precontent.is_empty() {
            self.precontent.push('\n');
        }
        self.precontent.push_str(content.as_ref());
        self
    }

    /// Add content to the module.
    pub fn with_content<S: AsRef<str> + ?Sized>(&mut self, content: &S) -> &mut Self {
        if !self.content.is_empty() {
            self.content.push('\n');
            self.content.push_str("// -------------------------------------------------------------------------------------------------\n");
        }
        self.content.push_str(content.as_ref());
        self
    }

    /// Add a submodule to the current module.
    pub async fn with_submodule<
        S: ToString + ?Sized,
        F: AsyncFnOnce(&mut ModuleBuilder, SubModuleSettings) -> Result<SubModuleSettings>,
    >(
        &mut self,
        name: &S,
        f: F,
    ) -> Result<&mut Self> {
        // Create a ModuleBuilder for the submodule
        let path = self.folder.join(&self.name);
        let mut submodule = ModuleBuilder::new(name, path);

        // Run the submodule configuration function and build it.
        let settings = f(&mut submodule, SubModuleSettings::default()).await?;
        submodule.build().await?;

        // Add the submodule as an import to the parent module
        let import_name = name.to_string();
        let mut import = String::new();

        if let Some(feature) = settings.feature {
            import.push_str("#[cfg(feature = \"");
            import.push_str(&feature);
            import.push_str("\")]\n");
        }

        if settings.public {
            import.push_str("pub ");
        }
        import.push_str("mod ");
        import.push_str(&import_name);
        import.push_str(";\n");
        for item in &settings.import_from {
            if settings.reexport {
                import.push_str("pub ");
            }
            import.push_str("use ");
            import.push_str(&import_name);
            import.push_str("::");
            import.push_str(item);
            import.push_str(";\n");
        }
        if !settings.import_from.is_empty() {
            import.push('\n');
        }

        self.content.push_str(&import);

        Ok(self)
    }

    /// Build the module and write it to the filesystem.
    pub async fn build(mut self) -> Result<()> {
        // Prepare the output buffer
        let mut output = String::new();

        // Sort imports to guaratee consistent order
        self.imports.sort();

        // Determine the file path for the module
        let mut path;
        if self.folder.is_file() {
            path = self.folder;
            path.set_file_name(self.name);
        } else {
            path = self.folder.join(&self.name);
        }

        if let Some(ModuleImport::Mod { .. }) = self.imports.last()
            && path.is_dir()
        {
            path.push("mod.rs");
        } else {
            path.set_extension("rs");
        }

        // Write module documentation to the output buffer
        if !self.docs.is_empty() {
            for line in self.docs.lines() {
                output.push_str("//! ");
                output.push_str(line.trim());
                output.push('\n');
            }
        }

        // Write precontent to the output buffer
        if !self.precontent.is_empty() {
            output.push_str(&self.precontent);
            output.push_str("\n// -------------------------------------------------------------------------------------------------");
            output.push_str("\n// # Note: The following content is automatically @generated, do not edit this manually!\n\n");
        }

        // Write imports to the output buffer
        for import in self.imports {
            match import {
                ModuleImport::Use { name, public } => {
                    if public {
                        output.push_str("pub ");
                    }
                    output.push_str("use ");
                    output.push_str(&name);
                    output.push_str(";\n");
                }
                ModuleImport::Mod { name, public, import_from, reexport } => {
                    if public {
                        output.push_str("pub ");
                    }
                    output.push_str("mod ");
                    output.push_str(&name);
                    output.push_str(";\n");
                    for import in import_from {
                        if reexport {
                            output.push_str("pub ");
                        }
                        output.push_str("use ");
                        output.push_str(&name);
                        output.push_str("::");
                        output.push_str(&import);
                        output.push_str(";\n");
                    }
                    output.push('\n');
                }
            }
        }

        // Write content to the output buffer
        if !self.content.is_empty() {
            output.push_str(&self.content);
            output.push('\n');
        }

        // Ensure the parent directory exists
        if let Some(parent) = path.parent()
            && !parent.exists()
            && let Err(err) = tokio::fs::create_dir_all(parent).await
        {
            return Err(miette::miette!(
                "Failed to create module directory \"{}\", {err}",
                parent.display(),
            ));
        }

        // Write the module content to the file
        match tokio::fs::write(&path, output).await {
            Ok(()) => Ok(()),
            Err(err) => miette::bail!("Failed to write to module \"{}\", {err}", path.display()),
        }
    }
}

impl SubModuleSettings {
    /// Set the feature flag required to enable the submodule.
    #[must_use]
    pub fn with_feature(mut self, feature: String) -> Self {
        self.feature = Some(feature);
        self
    }

    /// Set whether the submodule should be public.
    #[must_use]
    pub fn with_public(mut self, public: bool) -> Self {
        self.public = public;
        self
    }

    /// Set the items the parent module should import from the submodule.
    #[must_use]
    pub fn with_import_from(mut self, import_from: Vec<String>) -> Self {
        self.import_from = import_from;
        self
    }

    /// Set whether the parent module should reexport the imported items from
    /// the submodule.
    #[must_use]
    pub fn with_reexport(mut self, reexport: bool) -> Self {
        self.reexport = reexport;
        self
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleImport {
    Use { name: String, public: bool },
    Mod { name: String, public: bool, import_from: Vec<String>, reexport: bool },
}

impl Ord for ModuleImport {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Ensure that `use` imports come before `mod` imports
            (ModuleImport::Use { .. }, ModuleImport::Mod { .. }) => Ordering::Greater,
            (ModuleImport::Mod { .. }, ModuleImport::Use { .. }) => Ordering::Less,
            // If both are of the same type, compare by name
            (ModuleImport::Use { name: name_a, .. }, ModuleImport::Use { name: name_b, .. })
            | (ModuleImport::Mod { name: name_a, .. }, ModuleImport::Mod { name: name_b, .. }) => {
                name_a.cmp(name_b)
            }
        }
    }
}
impl PartialOrd for ModuleImport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
