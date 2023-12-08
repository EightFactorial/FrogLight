use crate::data::ModuleData;

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStatesModule;

impl BlockStatesModule {}

impl ModuleExt for BlockStatesModule {
    fn run(&self, _data: &mut ModuleData) {}
}
