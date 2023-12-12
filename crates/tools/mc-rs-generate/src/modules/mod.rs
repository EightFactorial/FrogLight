use std::{future::Future, pin::Pin};

use clap::ValueEnum;
use git2::Repository;
use mc_rs_extract::{modules::ExtractModule, ModuleData};
use strum::{Display, EnumIter};

mod format;
use format::FormatModule;

mod structure;
use structure::GuiStructureModule;

mod resourcepack;
use resourcepack::ResourcePackModule;

mod blocks;
use blocks::{block_impl::BlockVersionModule, block_list::BlockListModule};

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
    ResourcePack,
    GuiStructure,
    BlockList,
    BlockVersion,
}

impl GenerateModule {
    /// Get the dependencies of this module.
    pub fn deps(self) -> &'static [ExtractModule] { Box::<dyn ModuleExt>::from(self).deps() }

    /// Generate the data for this module.
    pub async fn run<'a>(self, data: &'a mut ModuleData, repo: &'a Repository) {
        Box::<dyn ModuleExt>::from(self).run(data, repo).await
    }
}

// Convert the module enum into a boxed module trait object.
impl From<GenerateModule> for Box<dyn ModuleExt> {
    fn from(value: GenerateModule) -> Self {
        match value {
            GenerateModule::Format => Box::<FormatModule>::default(),
            GenerateModule::ResourcePack => Box::<ResourcePackModule>::default(),
            GenerateModule::GuiStructure => Box::<GuiStructureModule>::default(),
            GenerateModule::BlockList => Box::<BlockListModule>::default(),
            GenerateModule::BlockVersion => Box::<BlockVersionModule>::default(),
        }
    }
}

/// A trait for modules.
trait ModuleExt {
    /// Returns the dependencies of the module.
    fn deps(&self) -> &'static [ExtractModule] { &[] }

    /// Runs the module's code generator.
    fn run<'a>(
        &'a self,
        data: &'a mut ModuleData,
        repo: &'a Repository,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>;
}
