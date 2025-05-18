//! Inspired by [`BevyManifest`]
//!
//! [`BevyManifest`]: https://github.com/bevyengine/bevy/blob/main/crates/bevy_macro_utils/src/bevy_manifest.rs
#![allow(dead_code, unreachable_pub)]

use std::{collections::BTreeMap, path::PathBuf};

use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use proc_macro::TokenStream;

pub struct CrateManifest(toml_edit::DocumentMut);

impl CrateManifest {
    pub fn shared() -> MappedRwLockReadGuard<'static, CrateManifest> {
        static MANIFEST: RwLock<BTreeMap<PathBuf, CrateManifest>> = RwLock::new(BTreeMap::new());

        let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
        let mut path = manifest_dir.expect("CARGO_MANIFEST_DIR not defined!");
        path.push("Cargo.toml");

        // If the manifest hasn't been parsed yet, read and parse it.
        if !MANIFEST.read().contains_key(&path) {
            assert!(path.exists(), "Expected Cargo.toml at \"{}\"", path.display());

            let manifest = std::fs::read_to_string(&path).unwrap_or_else(|err| {
                panic!("Unable to read Cargo.toml at \"{}\": {err}", path.display());
            });

            let manifest = CrateManifest(manifest.parse().unwrap_or_else(|err| {
                panic!("Unable to parse Cargo.toml at \"{}\": {err}", path.display());
            }));
            MANIFEST.write().insert(path.clone(), manifest);
        }

        RwLockReadGuard::map(MANIFEST.read(), |manifests| manifests.get(&path).unwrap())
    }

    /// Find the path to a package in the manifest.
    #[must_use]
    pub fn find_package(&self, name: &str, series: &str) -> Option<syn::Path> {
        // If the package is the root package, return `crate`.
        if let Some(package) = self.0.get("package")
            && let Some(package_name) = package.get("name")
            && package_name.as_str() == Some(name)
        {
            return Some(syn::parse_quote!(crate));
        }

        // Find the package in `dependencies` or `dev-dependencies`.
        let deps = self.0.get("dependencies");
        let dev_deps = self.0.get("dev-dependencies");

        deps.and_then(|deps| Self::find_package_in_item(name, series, deps)).or_else(|| {
            dev_deps.and_then(|dev_deps| Self::find_package_in_item(name, series, dev_deps))
        })
    }

    /// Find a package in a dependency list.
    #[must_use]
    fn find_package_in_item(name: &str, series: &str, item: &toml_edit::Item) -> Option<syn::Path> {
        if let Some(dep) = item.get(name) {
            // Get the path to the package directly.
            Self::package_path(dep).map_or_else(|| Self::parse_path(name), Some)
        } else if let Some(dep) = item.get(series) {
            // Get the path to the `series` package.
            let mut path = Self::package_path(dep).map_or_else(|| Self::parse_path(series), Some);

            // Append a trimmed package name to the path.
            //
            // For example:
            //  - ("bevy_app", "bevy", ..) -> "bevy::app"
            //  - ("bevy_asset", "bevy", ..) -> "bevy::asset"
            //  - ("froglight-block", "froglight", ..) -> "froglight::block"
            //  - ("froglight-common", "froglight", ..) -> "froglight::common"
            if let Some(path) = path.as_mut() {
                path.segments.push(
                    syn::parse_str(name.trim_start_matches(series).trim_start_matches(['-', '_']))
                        .unwrap(),
                );
            }

            path
        } else {
            None
        }
    }

    /// Return the package path from it's TOML representation.
    ///
    /// If provided, the `prefix` will be removed from the package name.
    #[must_use]
    fn package_path(package: &toml_edit::Item) -> Option<syn::Path> {
        let mut package = package.get("package")?.as_str()?;

        if package.contains(['-', '_']) {
            package = package.split(['-', '_']).nth(1)?;
        }

        Self::parse_path(package)
    }

    fn parse_path(path: &str) -> Option<syn::Path> {
        syn::parse(path.replace('-', "_").parse::<TokenStream>().ok()?).ok()
    }
}

impl CrateManifest {
    pub fn try_find(name: &str, series: &str) -> Option<syn::Path> {
        Self::shared().find_package(name, series)
    }

    pub fn find(name: &str, series: &str) -> syn::Path {
        Self::try_find(name, series).unwrap_or_else(|| {
            panic!("Failed to find crate \"{name}\" or \"{series}\"!");
        })
    }

    /// A shortcut for finding `bevy` packages.
    #[inline]
    #[must_use]
    pub fn bevy(name: &str) -> syn::Path { Self::find(name, "bevy") }

    /// A shortcut for finding `froglight` packages.
    #[inline]
    #[must_use]
    pub fn froglight(name: &str) -> syn::Path { Self::find(name, "froglight") }
}
