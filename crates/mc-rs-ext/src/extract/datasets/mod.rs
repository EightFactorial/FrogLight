use enum_dispatch::enum_dispatch;
use json::JsonValue;
use strum::{Display, EnumIter, EnumString};

use crate::types::{ClassMap, Manifest, Version};

mod diag;
mod info;

#[enum_dispatch]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Datasets {
    Diagnostics(diag::Diagnostics),
    Info(info::Info),
}

#[enum_dispatch(Datasets)]
pub trait Dataset {
    fn deps(&self) -> &'static [Datasets];
    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    );
}
