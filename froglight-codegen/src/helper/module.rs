use std::{cmp::Ordering, path::PathBuf};

use miette::Result;

#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    name: String,
    parent: PathBuf,
    docs: String,
    precontent: String,
    imports: Vec<ModuleImport>,
    content: String,
}

#[derive(Debug, Default, Clone)]
pub struct SubModuleSettings {
    pub public: bool,
    pub import_from: Vec<String>,
    pub reexport: bool,
}

impl ModuleBuilder {
    /// Create a new [`ModuleBuilder`].
    #[must_use]
    pub fn new<S: ToString + ?Sized>(name: &S, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            parent: path,
            docs: String::new(),
            precontent: String::new(),
            imports: Vec::new(),
            content: String::new(),
        }
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
        let path = self.parent.join(&self.name);
        let mut submodule = ModuleBuilder::new(name, path);

        // Run the submodule configuration function and build it.
        let settings = f(&mut submodule, SubModuleSettings::default()).await?;
        submodule.build().await?;

        // Import the submodule into the current module
        self.imports.push(ModuleImport::Mod {
            name: name.to_string(),
            public: settings.public,
            import_from: settings.import_from,
            reexport: settings.reexport,
        });

        Ok(self)
    }

    /// Build the module and write it to the filesystem.
    pub async fn build(mut self) -> Result<()> {
        // Prepare the output buffer
        let mut output = String::new();

        // Sort imports to guaratee consistent order
        self.imports.sort();

        // Determine the file path for the module
        let mut path = self.parent.join(&self.name);
        if let Some(ModuleImport::Mod { .. }) = self.imports.last() {
            path = path.join("mod.rs");
        } else {
            path.set_extension("rs");
        }

        // Write module documentation to the output buffer
        if !self.docs.is_empty() {
            for line in self.docs.lines() {
                output.push_str("//! ");
                output.push_str(line);
                output.push('\n');
            }
            output.push('\n');
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
