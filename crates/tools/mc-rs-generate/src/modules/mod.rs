use std::{future::Future, pin::Pin};

use clap::ValueEnum;
use git2::Repository;
use mc_rs_extract::{modules::ExtractModule, ModuleData};
use strum::{Display, EnumIter};

mod format;
pub(crate) use format::FormatModule;

/// Modules that can be run to generate data.
///
/// This enum is used to specify which modules to run on a given version of Minecraft.
///
/// The order of the variants in this enum is the order in which the modules will be run,
/// dependencies should be placed before the modules that depend on them.
#[derive(
    Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum, EnumIter,
)]
pub enum GenerateModule {
    Format,
}

impl GenerateModule {
    /// Get the dependencies of this module.
    pub fn deps(self) -> &'static [ExtractModule] { Box::<dyn ModuleExt>::from(self).deps() }

    /// Generate the data for this module.
    pub fn run(self, data: &ModuleData, repo: &Repository) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::<dyn ModuleExt>::from(self).run(data, repo)
    }
}

// Convert the module enum into a boxed module trait object.
impl From<GenerateModule> for Box<dyn ModuleExt> {
    fn from(value: GenerateModule) -> Self {
        match value {
            GenerateModule::Format => Box::<FormatModule>::default(),
        }
    }
}

/// A trait for modules.
trait ModuleExt {
    /// Returns the dependencies of the module.
    fn deps(&self) -> &'static [ExtractModule] { &[] }

    /// Runs the module's code generator.
    fn run(&self, data: &ModuleData, repo: &Repository) -> Pin<Box<dyn Future<Output = ()>>>;
}
