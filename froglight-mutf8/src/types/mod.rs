//! TODO

pub mod str;
pub use str::MStr;

cfg_select! {
    feature = "alloc" => {
        pub mod string;
        pub use string::MString;
    }
}
