use std::{path::PathBuf, sync::OnceLock};

use proc_macro::TokenStream;
use toml_edit::{DocumentMut, Item};

/// The project manifest.
///
/// This is initialized once and then cached for future use.
static MANIFEST: OnceLock<ProjectManifest> = OnceLock::new();

/// The project manifest.
///
/// I <3 Bevy
#[derive(Debug)]
pub(crate) struct ProjectManifest {
    manifest: DocumentMut,
}

impl ProjectManifest {
    /// Gets or initializes the project manifest.
    #[must_use]
    pub(crate) fn get() -> &'static ProjectManifest { MANIFEST.get_or_init(Self::init) }

    /// Initializes the project manifest.
    #[must_use]
    fn init() -> Self {
        // Get the path to the Cargo.toml file
        let Some(mut path) = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) else {
            panic!("Could not get environment variable `CARGO_MANIFEST_DIR`!");
        };
        path.push("Cargo.toml");

        // Make sure the Cargo.toml file exists
        assert!(path.exists(), "No `Cargo.toml` file found at `{}`!", path.display());

        // Read and parse the cargo manifest
        let manifest = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) => panic!("Failed to read Cargo.toml: {err}"),
        };
        let manifest = match manifest.parse::<DocumentMut>() {
            Ok(manifest) => manifest,
            Err(err) => panic!("Failed to parse Cargo.toml: {err}"),
        };

        Self { manifest }
    }

    /// The package name of the `froglight` crate.
    const FROGLIGHT: &'static str = "froglight";
    /// The package name of the `froglight_internal` crate.
    const FROGLIGHT_APP: &'static str = "froglight_internal";

    /// The prefix for all `froglight` crates.
    const PREFIX: &'static str = "froglight_";

    /// Attempt to retrieve the [path](syn::Path) of a particular package in
    /// the [manifest](Self) by [name](str).
    pub(crate) fn maybe_get_path(&self, name: &str) -> Option<syn::Path> {
        let find_in_deps = |deps: &Item| -> Option<syn::Path> {
            let package = if let Some(dep) = deps.get(name) {
                return Some(Self::parse_str(Self::dep_package(dep).unwrap_or(name)));
            } else if let Some(dep) = deps.get(Self::FROGLIGHT) {
                Self::dep_package(dep).unwrap_or(Self::FROGLIGHT)
            } else if let Some(dep) = deps.get(Self::FROGLIGHT_APP) {
                Self::dep_package(dep).unwrap_or(Self::FROGLIGHT_APP)
            } else {
                return None;
            };

            // Strip the prefix from the package name
            //
            // For example, if the package name is `froglight_protocol`, the
            // path will be `froglight::protocol`.
            let mut path = Self::parse_str::<syn::Path>(package);
            if let Some(module) = name.strip_prefix(Self::PREFIX) {
                path.segments.push(Self::parse_str(module));
            }

            Some(path)
        };

        let deps = self.manifest.get("dependencies");
        let deps_dev = self.manifest.get("dev-dependencies");

        deps.and_then(find_in_deps).or_else(|| deps_dev.and_then(find_in_deps))
    }

    /// Attempt to retrieve the package name from a [dependency](Item).
    fn dep_package(dep: &Item) -> Option<&str> {
        if dep.as_str().is_some() {
            None
        } else {
            dep.get("package").map(|name| name.as_str().unwrap())
        }
    }

    /// The path to the current crate.
    const CURRENT_CRATE: &'static str = "crate";

    /// Returns the path for the crate with the given name.
    pub(crate) fn get_path(&self, name: &str) -> syn::Path {
        // Check if the crate is imported as a dependency
        if let Some(path) = self.maybe_get_path(name) {
            return path;
        }

        // Get the name of the current crate
        let crate_name = self.crate_name();

        // Check if the current crate is `froglight`
        if crate_name == Self::FROGLIGHT {
            Self::parse_str::<syn::Path>(Self::FROGLIGHT)
            // Check if the current crate is `froglight_app`
        } else if crate_name == Self::FROGLIGHT_APP {
            Self::parse_str::<syn::Path>(Self::FROGLIGHT_APP)
        // Check if the current crate is the same as the package name
        } else if crate_name.replace('-', "_") == name {
            Self::parse_str::<syn::Path>(Self::CURRENT_CRATE)
        // If the package is not found, just return the name as a path
        } else {
            Self::parse_str::<syn::Path>(name)
        }
    }

    /// Returns the name of the current crate.
    fn crate_name(&self) -> &str {
        self.manifest
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or_default()
    }

    /// Attempt to parse provided [path](str) as a [syntax tree
    /// node](syn::parse::Parse).
    ///
    /// # Panics
    ///
    /// Will panic if the path is not able to be parsed. For a non-panicking
    /// option, see [`try_parse_str`]
    ///
    /// [`try_parse_str`]: Self::try_parse_str
    fn parse_str<T: syn::parse::Parse>(path: &str) -> T { Self::try_parse_str(path).unwrap() }

    /// Attempt to parse the provided [path](str) as a [syntax tree
    /// node](syn::parse::Parse)
    fn try_parse_str<T: syn::parse::Parse>(path: &str) -> Option<T> {
        syn::parse(path.parse::<TokenStream>().ok()?).ok()
    }
}
