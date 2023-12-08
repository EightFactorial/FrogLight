use clap::ValueEnum;
use strum::{Display, EnumIter};

use crate::data::ModuleData;

mod debug;
pub use debug::DebugModule;

mod info;
pub use info::InfoModule;

mod registry;
pub use registry::{registries::RegistriesModule, serializable::SerializableRegistriesModule};

mod blocks;
pub use blocks::{block_list::BlockListModule, block_states::BlockStatesModule};

/// Modules that can be run to generate data.
///
/// This enum is used to specify which modules to run on a given version of Minecraft.
///
/// The order of the variants in this enum is the order in which the modules will be run,
/// dependencies should be placed before the modules that depend on them.
#[derive(
    Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum, EnumIter,
)]
pub enum ExtractModule {
    Debug,
    Info,
    Registries,
    SerializableRegistries,
    BlockList,
    BlockIds,
}

impl ExtractModule {
    /// Get the dependencies of this module.
    #[must_use]
    pub fn deps(self) -> &'static [ExtractModule] { Box::<dyn ModuleExt>::from(self).deps() }
    /// Generate the data for this module.
    pub fn run(self, data: &mut ModuleData) { Box::<dyn ModuleExt>::from(self).run(data); }
}

// Convert the module enum into a boxed module trait object.
impl From<ExtractModule> for Box<dyn ModuleExt> {
    fn from(value: ExtractModule) -> Self {
        match value {
            ExtractModule::Debug => Box::<DebugModule>::default(),
            ExtractModule::Info => Box::<InfoModule>::default(),
            ExtractModule::Registries => Box::<RegistriesModule>::default(),
            ExtractModule::SerializableRegistries => Box::<SerializableRegistriesModule>::default(),
            ExtractModule::BlockList => Box::<BlockListModule>::default(),
            ExtractModule::BlockIds => Box::<BlockStatesModule>::default(),
        }
    }
}

/// A trait for modules.
trait ModuleExt {
    /// Returns the dependencies of the module.
    fn deps(&self) -> &'static [ExtractModule] { &[] }

    /// Runs the module's data generator.
    fn run(&self, data: &mut ModuleData);
}
