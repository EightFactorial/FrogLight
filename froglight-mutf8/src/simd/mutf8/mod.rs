//! TODO
#![expect(clippy::inline_always, reason = "Performance")]

cfg_select! {
    all(target_arch = "aarch64", not(feature = "simd_fallback")) => {
        /// The SIMD module currently being used.
        pub const ARCH: &str = "aarch64";

        pub mod aarch64;
        pub use aarch64::*;

        #[doc(hidden)]
        pub mod fallback;
    }
    all(target_arch = "x86_64", not(feature = "simd_fallback")) => {
        /// The SIMD module currently being used.
        pub const ARCH: &str = "x86_64";

        pub mod x86_64;
        pub use x86_64::*;

        #[doc(hidden)]
        pub mod fallback;
    }
    _ => {
        /// The SIMD module currently being used.
        pub const ARCH: &str = "fallback";

        pub mod fallback;
        pub use fallback::*;
    }
}
