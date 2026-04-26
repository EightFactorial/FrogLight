//! TODO

mod str;
pub use str::MStr;

cfg_select! {
    feature = "alloc" => {
        mod string;
        pub use string::MString;
    }
}
