use std::path::PathBuf;

use miette::Result;

#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    name: String,
    parent: PathBuf,
    imports: Vec<ModuleImport>,
    content: String,
}

#[derive(Debug, Clone)]
pub enum ModuleImport {
    Use { name: String, public: bool },
    Mod { name: String, public: bool, import_from: Vec<String>, reexport: bool },
}

#[derive(Debug, Default, Clone)]
pub struct ModuleSettings {
    pub public: bool,
    pub import_from: Vec<String>,
    pub reexport: bool,
}

impl ModuleBuilder {
    /// Create a new [`ModuleBuilder`].
    pub fn new<S: ToString + ?Sized>(name: &S, path: PathBuf) -> Self {
        Self { name: name.to_string(), parent: path, imports: Vec::new(), content: String::new() }
    }

    /// Add a submodule to the current module.
    pub async fn with_submodule<
        S: ToString + ?Sized,
        F: FnOnce(&mut Self, ModuleSettings) -> Fut,
        Fut: Future<Output = Result<ModuleSettings>>,
    >(
        &mut self,
        name: &S,
        f: F,
    ) -> Result<&mut Self> {
        // Create a ModuleBuilder for the submodule
        let path = self.parent.join(&self.name);
        let mut submodule = ModuleBuilder::new(name, path);

        // Run the submodule function and build it.
        let settings = f(&mut submodule, ModuleSettings::default()).await?;
        submodule.build().await?;

        // Add the submodule import to the current module
        self.imports.push(ModuleImport::Mod {
            name: name.to_string(),
            public: settings.public,
            import_from: settings.import_from,
            reexport: settings.reexport,
        });

        Ok(self)
    }

    /// Build the module and write it to the filesystem.
    pub async fn build(self) -> Result<()> {
        let mut path = self.parent.join(&self.name);
        if self.imports.iter().any(|i| matches!(i, ModuleImport::Mod { .. })) {
            path = path.join("mod.rs");
        } else {
            path.set_extension("rs");
        }

        // Ensure the parent directory exists
        if let Some(parent) = path.parent()
            && !parent.exists()
            && let Err(_err) = tokio::fs::create_dir_all(parent).await
        {
            todo!()
        }

        // Write the module content to the file
        match tokio::fs::write(path, self.content).await {
            Ok(()) => Ok(()),
            Err(_err) => todo!(),
        }
    }
}
