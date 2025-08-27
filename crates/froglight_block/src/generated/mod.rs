//! @generated block types, attribute types, and trait implementations

macro_rules! generate {
    // Generate a batch of structs for block types.
    (@types $($type:ident)+) => {
        $(
            pub struct $type;
        )+
    };

    // Generate a batch of `BlockAttribute` implementations.
    (@attrs $($type:ident $def:tt)+) => {
        $(
            generate!(@attr $type $def);
        )+
    };
    // Generate an enum-based `BlockAttribute` implementation.
    (@attr $type:ident { $(  $val:ident: $lit:literal  )+ }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $type {
            $( $val ),+
        }
        impl BlockAttribute for $type { const STATES: &'static [(&'static str, Self)] = &[ $( ($lit, Self::$val) ),+ ]; }
    };
    // Generate a struct-based `BlockAttribute` implementation.
    (@attr $type:ident (bool)) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $type(bool);
        impl BlockAttribute for $type { const STATES: &'static [(&'static str, Self)] = &[("true", Self(true)), ("false", Self(false))]; }
    };


    // Generate a batch of `BlockType` implementations and a `Blocks` implementation for a given version.
    (@blocks $version:ty, $($type:ty, $attr_names:expr => $attr:ty, $default:expr, $ident:expr, $settings:expr, $definition:expr)+) => {
        $(
            generate!(@block $version, $type, $attr_names => $attr, $default, $ident, $settings, $definition);
        )+

        impl Blocks for $version {
            fn blocks() -> &'static StaticBlockMap {
                static BLOCKS: Lazy<StaticBlockMap> = Lazy::new(|| {
                    let mut map = BlockMap::new_empty();
                    <$version as Blocks>::init_blocks(&mut map);
                    StaticBlockMap::new(map)
                });

                &BLOCKS
            }

            fn init_blocks(map: &mut BlockMap) {
                $( map.register::<$type, $version>(); )+
            }
        }
    };
    (@block $version:ty, $type:ty, $attr_names:expr => $attr:ty, $default:expr, $ident:expr, $settings:expr, $definition:expr) => {
        impl BlockType<$version> for $type {
            type Attributes = $attr;
            const ATTRIBUTE_NAMES: &'static [&'static str] = $attr_names;

            fn info() -> &'static BlockInfo {
                static INFO: BlockInfo = BlockInfo::new::<$type, $version>(
                    $default,
                    $ident,
                    $settings,
                    $definition,
                );

                &INFO
            }
        }
    };
}

pub mod attribute;
pub mod block;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

mod v1_21_8;
// mod v1_21_9;
