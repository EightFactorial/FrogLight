//! TODO

cfg_select! {
    feature = "std" => {
        use std::sync::LazyLock;
    }
    feature = "once_cell" => {
        use once_cell::sync::Lazy as LazyLock;
    }
    _ => {
        compile_error!("Either the `std` or `once_cell` feature must be enabled!");
    }
}

type HashMap<K, V> = hashbrown::HashMap<K, V, foldhash::fast::RandomState>;
type LazyMap<K, V> = LazyLock<parking_lot::RwLock<HashMap<K, V>>>;

pub mod schema;
pub mod typeplan;
