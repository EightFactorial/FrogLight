use crate::data::ModuleData;

use crate::modules::{ExtractModule, ModuleExt};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStatesModule;

impl BlockStatesModule {}

impl ModuleExt for BlockStatesModule {
    fn deps(&self) -> &'static [ExtractModule] {
        &[ExtractModule::BlockList, ExtractModule::BlockAttributes]
    }

    fn run(&self, _data: &mut ModuleData) {}
}
