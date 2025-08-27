//! @generated item types and trait implementations

macro_rules! generate {
    // Generate a batch of structs for item types.
    (@types $($type:ident)+) => {
        $(
            pub struct $type;
        )+
    };

    // Generate a batch of `ItemType` implementations and an `Items` implementation for a given version.
    (@items $version:ty, $($type:ty, $ident:expr, $components:expr)+) => {
        $(
            generate!(@item $version, $type, $ident, $components);
        )+

        impl Items for $version {
            fn items() -> &'static StaticItemMap {
                static ITEMS: Lazy<StaticItemMap> = Lazy::new(|| {
                    let mut map = ItemMap::new_empty();
                    <$version as Items>::init_items(&mut map);
                    StaticItemMap::new(map)
                });

                &ITEMS
            }

            fn init_items(map: &mut ItemMap) {
                $( map.register::<$type, $version>(); )+
            }
        }
    };
    (@item $version:ty, $type:ty, $ident:expr, $components:expr) => {
        impl ItemType<$version> for $type {
            fn info() -> &'static ItemInfo {
                static INFO: ItemInfo = ItemInfo::new::<$type, $version>(
                    $ident,
                    $components,
                );

                &INFO
            }
        }
    };
}

mod item;
pub use item::*;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

mod v1_21_8;
// mod v1_21_9;
