//! TODO
#![allow(clippy::all, clippy::pedantic, dead_code, missing_docs)]

mod builder;
pub use builder::FunctionBuilder;

mod function;
pub use function::BrigadierFunction;

mod world;
pub use world::WorldRef;
