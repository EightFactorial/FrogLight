use std::{fmt::Debug, hash::Hash};

/// A Protocol version
pub trait Version: 'static + Debug + Default + Copy + Eq + Hash + Send + Sync {
    /// The protocol id
    const ID: i32;
}
