/// A macro that creates a [`Version`] subtrait.
///
/// # Example
///
/// ```rust
/// use froglight_registry_template::version_subtrait;
///
/// /// Defined anywhere, it doesn't matter.
/// pub trait Version {}
///
/// version_subtrait! {
///     pub trait MyTrait {
///         const GLOBAL: u32;
///         fn global();
///         fn new_value();
///     }
/// }
///
/// // Becomes (renamed due to name collision):
///
/// pub trait MyTraitExpanded: Version {
///     /// The [`u32`] for this [`Version`].
///     const GLOBAL: &'static froglight_registry_template::types::OnceLock<u32>;
///
///     /// Get the [`u32`] for this [`Version`].
///     #[inline]
///     #[must_use]
///     fn global() -> &'static u32 { Self::GLOBAL.get_or_init(Self::new_value) }
///
///     /// Create a new [`u32`] for this [`Version`].
///     ///
///     /// # Warning
///     ///
///     /// This will create a new [`u32`] each time it is called!
///     ///
///     /// Unless you are modifying the global,
///     /// you should probably be using [`MyTrait::global`]!
///     fn new_value() -> u32;
/// }
/// ```
#[macro_export]
macro_rules! version_subtrait {
    (
        $( #[$attr:meta] )*
        pub trait $name:ident $( : $($trait:ident +)+ )? {
            const $const:ident: $const_ty:ty;
            fn $const_fn:ident ();
            fn $new_fn:ident ();
        }
    ) => {
        $( #[$attr] )*
        #[doc = concat!("A [`", stringify!($const_ty), "`] for a [`Version`].")]
        pub trait $name: $($trait +)* Version {
            #[doc = concat!("The [`", stringify!($const_ty), "`] for this [`Version`].")]
            const $const: &'static $crate::types::OnceLock<$const_ty>;

            #[inline]
            #[must_use]
            #[doc = concat!("Get the [`", stringify!($const_ty), "`] for this [`Version`].")]
            fn $const_fn() -> &'static $const_ty { Self::$const.get_or_init(Self::$new_fn) }

            #[must_use]
            #[doc = concat!("Create a new [`", stringify!($const_ty), "`] for this [`Version`].")]
            #[doc = " # Warning"]
            #[doc = concat!(" This will create a new [`", stringify!($const_ty), "`] each time it is called!")]
            #[doc = ""]
            #[doc = concat!("Unless you are modifying the global, you should probably be using [`", stringify!($name), "::", stringify!($const_fn), "`]!")]
            fn $new_fn() -> $const_ty;
        }
    };
}

// -------------------------------------------------------------------------------------------------

/// A macro that implements a [`Version`] subtrait.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use froglight_registry_template::{version_subtrait, version_implement};
/// use froglight_registry_template::types::OnceLock;
///
/// pub trait Version {}
///
/// version_subtrait! {
///     pub trait MyTrait {
///         const GLOBAL: u32;
///         fn global();
///         fn new_value();
///     }
/// }
///
/// pub struct VersionA;
/// impl Version for VersionA {}
///
/// version_implement! {
///     impl MyTrait => VersionA {
///         const GLOBAL: u32;
///         fn new_value() => { 42 }
///     }
/// }
///
/// // Becomes (renamed due to name collision):
///
/// pub struct VersionB;
/// impl Version for VersionB {}
///
/// impl MyTrait for VersionB {
///     const GLOBAL: &'static OnceLock<u32> = {
///         static STATIC: OnceLock<u32> = OnceLock::new();
///         &STATIC
///     };
///
///     #[inline]
///     #[must_use]
///     fn global() -> &'static u32 { <VersionB as MyTrait>::GLOBAL.get_or_init(Self::new_value) }
///
///    #[must_use]
///     fn new_value() -> u32 { 42 }
/// }
#[macro_export]
macro_rules! version_implement {
    (
        impl $name:path => $version:path {
            const $const:ident: $const_ty:path;
            fn $new_fn:ident () => $new_block:block
        }
    ) => {
        impl $name for $version {
            const $const: &'static $crate::types::OnceLock<$const_ty> = {
                static STATIC: $crate::types::OnceLock<$const_ty> = $crate::types::OnceLock::new();
                &STATIC
            };

            #[must_use]
            fn $new_fn() -> $const_ty $new_block
        }
    };
}
