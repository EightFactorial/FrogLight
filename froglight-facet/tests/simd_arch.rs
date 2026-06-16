//! TODO

use froglight_facet::simd::varint::{decode, encode};

macro_rules! assert {
    (@encode $val:expr, $out:pat, $len:expr) => {
        std::assert_matches!(encode($val), ($out, $len));
    };
    (@decode $out:literal, $val:expr, $len:expr) => {
        std::assert_matches!(decode(&$val), ($out, $len));
    };
}

#[test]
fn arch_u8() {
    assert!(@encode 0u8, [0, 0, ..], 1);
    assert!(@encode 1u8, [1, 0, ..], 1);
    assert!(@encode 2u8, [2, 0, ..], 1);

    assert!(@decode 0u8, [0], 1);
    assert!(@decode 1u8, [1], 1);
    assert!(@decode 2u8, [2], 1);

    assert!(@encode 126u8, [126, 0, 0, ..], 1);
    assert!(@encode 127u8, [127, 0, 0, ..], 1);
    assert!(@encode 128u8, [128, 1, 0, ..], 2);
    assert!(@encode 129u8, [129, 1, 0, ..], 2);
    assert!(@encode 130u8, [130, 1, 0, ..], 2);

    assert!(@decode 126u8, [126], 1);
    assert!(@decode 127u8, [127], 1);
    assert!(@decode 128u8, [128, 1], 2);
    assert!(@decode 129u8, [129, 1], 2);
    assert!(@decode 130u8, [130, 1], 2);

    assert!(@encode 253u8, [253, 1, 0, ..], 2);
    assert!(@encode 254u8, [254, 1, 0, ..], 2);
    assert!(@encode 255u8, [255, 1, 0, ..], 2);

    assert!(@decode 253u8, [253, 1], 2);
    assert!(@decode 254u8, [254, 1], 2);
    assert!(@decode 255u8, [255, 1], 2);
}

#[test]
fn arch_u16() {
    assert!(@encode 0u16, [0, 0, ..], 1);
    assert!(@encode 1u16, [1, 0, ..], 1);
    assert!(@encode 2u16, [2, 0, ..], 1);

    assert!(@decode 0u16, [0], 1);
    assert!(@decode 1u16, [1], 1);
    assert!(@decode 2u16, [2], 1);

    assert!(@encode 126u16, [126, 0, 0, ..], 1);
    assert!(@encode 127u16, [127, 0, 0, ..], 1);
    assert!(@encode 128u16, [128, 1, 0, ..], 2);
    assert!(@encode 129u16, [129, 1, 0, ..], 2);
    assert!(@encode 130u16, [130, 1, 0, ..], 2);

    assert!(@decode 126u16, [126], 1);
    assert!(@decode 127u16, [127], 1);
    assert!(@decode 128u16, [128, 1], 2);
    assert!(@decode 129u16, [129, 1], 2);
    assert!(@decode 130u16, [130, 1], 2);

    assert!(@encode 253u16, [253, 1, 0, ..], 2);
    assert!(@encode 254u16, [254, 1, 0, ..], 2);
    assert!(@encode 255u16, [255, 1, 0, ..], 2);
    assert!(@encode 256u16, [128, 2, 0, ..], 2);
    assert!(@encode 257u16, [129, 2, 0, ..], 2);

    assert!(@decode 253u16, [253, 1], 2);
    assert!(@decode 254u16, [254, 1], 2);
    assert!(@decode 255u16, [255, 1], 2);
    assert!(@decode 256u16, [128, 2], 2);
    assert!(@decode 257u16, [129, 2], 2);

    assert!(@encode 65533u16, [253, 255, 3, ..], 3);
    assert!(@encode 65534u16, [254, 255, 3, ..], 3);
    assert!(@encode 65535u16, [255, 255, 3, ..], 3);

    assert!(@decode 65533u16, [253, 255, 3], 3);
    assert!(@decode 65534u16, [254, 255, 3], 3);
    assert!(@decode 65535u16, [255, 255, 3], 3);
}

#[test]
fn arch_u32() {
    assert!(@encode 0u32, [0, 0, ..], 1);
    assert!(@encode 1u32, [1, 0, ..], 1);
    assert!(@encode 2u32, [2, 0, ..], 1);

    assert!(@decode 0u32, [0], 1);
    assert!(@decode 1u32, [1], 1);
    assert!(@decode 2u32, [2], 1);

    assert!(@encode 126u32, [126, 0, 0, ..], 1);
    assert!(@encode 127u32, [127, 0, 0, ..], 1);
    assert!(@encode 128u32, [128, 1, 0, ..], 2);
    assert!(@encode 129u32, [129, 1, 0, ..], 2);
    assert!(@encode 130u32, [130, 1, 0, ..], 2);

    assert!(@decode 126u32, [126], 1);
    assert!(@decode 127u32, [127], 1);
    assert!(@decode 128u32, [128, 1], 2);
    assert!(@decode 129u32, [129, 1], 2);
    assert!(@decode 130u32, [130, 1], 2);

    assert!(@encode 253u32, [253, 1, 0, ..], 2);
    assert!(@encode 254u32, [254, 1, 0, ..], 2);
    assert!(@encode 255u32, [255, 1, 0, ..], 2);
    assert!(@encode 256u32, [128, 2, 0, ..], 2);
    assert!(@encode 257u32, [129, 2, 0, ..], 2);

    assert!(@decode 253u32, [253, 1], 2);
    assert!(@decode 254u32, [254, 1], 2);
    assert!(@decode 255u32, [255, 1], 2);
    assert!(@decode 256u32, [128, 2], 2);
    assert!(@decode 257u32, [129, 2], 2);

    assert!(@encode 65533u32, [253, 255, 3, 0, ..], 3);
    assert!(@encode 65534u32, [254, 255, 3, 0, ..], 3);
    assert!(@encode 65535u32, [255, 255, 3, 0, ..], 3);
    assert!(@encode 65536u32, [128, 128, 4, 0, ..], 3);
    assert!(@encode 65537u32, [129, 128, 4, 0, ..], 3);

    assert!(@decode 65533u32, [253, 255, 3, 0], 3);
    assert!(@decode 65534u32, [254, 255, 3, 0], 3);
    assert!(@decode 65535u32, [255, 255, 3, 0], 3);
    assert!(@decode 65536u32, [128, 128, 4, 0], 3);
    assert!(@decode 65537u32, [129, 128, 4, 0], 3);

    assert!(@encode 2_097_150u32, [254, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_151u32, [255, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_152u32, [128, 128, 128, 1, 0, ..], 4);
    assert!(@encode 2_097_153u32, [129, 128, 128, 1, 0, ..], 4);
    assert!(@encode 2_097_154u32, [130, 128, 128, 1, 0, ..], 4);

    assert!(@decode 2_097_150u32, [254, 255, 127, 0], 3);
    assert!(@decode 2_097_151u32, [255, 255, 127, 0], 3);
    assert!(@decode 2_097_152u32, [128, 128, 128, 1, 0], 4);
    assert!(@decode 2_097_153u32, [129, 128, 128, 1, 0], 4);
    assert!(@decode 2_097_154u32, [130, 128, 128, 1, 0], 4);

    assert!(@encode 2_147_483_645u32, [253, 255, 255, 255, 7, ..], 5);
    assert!(@encode 2_147_483_646u32, [254, 255, 255, 255, 7, ..], 5);
    assert!(@encode 2_147_483_647u32, [255, 255, 255, 255, 7, ..], 5);
    assert!(@encode 2_147_483_648u32, [128, 128, 128, 128, 8, ..], 5);
    assert!(@encode 2_147_483_649u32, [129, 128, 128, 128, 8, ..], 5);

    assert!(@decode 2_147_483_645u32, [253, 255, 255, 255, 7], 5);
    assert!(@decode 2_147_483_646u32, [254, 255, 255, 255, 7], 5);
    assert!(@decode 2_147_483_647u32, [255, 255, 255, 255, 7], 5);
    assert!(@decode 2_147_483_648u32, [128, 128, 128, 128, 8], 5);
    assert!(@decode 2_147_483_649u32, [129, 128, 128, 128, 8], 5);

    assert!(@encode 4_294_967_293u32, [253, 255, 255, 255, 15, ..], 5);
    assert!(@encode 4_294_967_294u32, [254, 255, 255, 255, 15, ..], 5);
    assert!(@encode 4_294_967_295u32, [255, 255, 255, 255, 15, ..], 5);

    assert!(@decode 4_294_967_293u32, [253, 255, 255, 255, 15], 5);
    assert!(@decode 4_294_967_294u32, [254, 255, 255, 255, 15], 5);
    assert!(@decode 4_294_967_295u32, [255, 255, 255, 255, 15], 5);
}

#[test]
fn arch_u64() {
    assert!(@encode 0u64, [0, 0, ..], 1);
    assert!(@encode 1u64, [1, 0, ..], 1);
    assert!(@encode 2u64, [2, 0, ..], 1);

    assert!(@decode 0u64, [0], 1);
    assert!(@decode 1u64, [1], 1);
    assert!(@decode 2u64, [2], 1);

    assert!(@encode 126u64, [126, 0, 0, ..], 1);
    assert!(@encode 127u64, [127, 0, 0, ..], 1);
    assert!(@encode 128u64, [128, 1, 0, ..], 2);
    assert!(@encode 129u64, [129, 1, 0, ..], 2);
    assert!(@encode 130u64, [130, 1, 0, ..], 2);

    assert!(@decode 126u64, [126], 1);
    assert!(@decode 127u64, [127], 1);
    assert!(@decode 128u64, [128, 1], 2);
    assert!(@decode 129u64, [129, 1], 2);
    assert!(@decode 130u64, [130, 1], 2);

    assert!(@encode 253u64, [253, 1, 0, ..], 2);
    assert!(@encode 254u64, [254, 1, 0, ..], 2);
    assert!(@encode 255u64, [255, 1, 0, ..], 2);
    assert!(@encode 256u64, [128, 2, 0, ..], 2);
    assert!(@encode 257u64, [129, 2, 0, ..], 2);

    assert!(@decode 253u64, [253, 1], 2);
    assert!(@decode 254u64, [254, 1], 2);
    assert!(@decode 255u64, [255, 1], 2);
    assert!(@decode 256u64, [128, 2], 2);
    assert!(@decode 257u64, [129, 2], 2);

    assert!(@encode 65533u64, [253, 255, 3, 0, ..], 3);
    assert!(@encode 65534u64, [254, 255, 3, 0, ..], 3);
    assert!(@encode 65535u64, [255, 255, 3, 0, ..], 3);
    assert!(@encode 65536u64, [128, 128, 4, 0, ..], 3);
    assert!(@encode 65537u64, [129, 128, 4, 0, ..], 3);

    assert!(@decode 65533u64, [253, 255, 3, 0], 3);
    assert!(@decode 65534u64, [254, 255, 3, 0], 3);
    assert!(@decode 65535u64, [255, 255, 3, 0], 3);
    assert!(@decode 65536u64, [128, 128, 4, 0], 3);
    assert!(@decode 65537u64, [129, 128, 4, 0], 3);

    assert!(@encode 2_097_149u64, [253, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_150u64, [254, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_151u64, [255, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_152u64, [128, 128, 128, 1, 0, ..], 4);
    assert!(@encode 2_097_153u64, [129, 128, 128, 1, 0, ..], 4);

    assert!(@decode 2_097_149u64, [253, 255, 127, 0], 3);
    assert!(@decode 2_097_150u64, [254, 255, 127, 0], 3);
    assert!(@decode 2_097_151u64, [255, 255, 127, 0], 3);
    assert!(@decode 2_097_152u64, [128, 128, 128, 1, 0], 4);
    assert!(@decode 2_097_153u64, [129, 128, 128, 1, 0], 4);

    assert!(@encode 2_147_483_645u64, [253, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_646u64, [254, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_647u64, [255, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_648u64, [128, 128, 128, 128, 8, 0, ..], 5);
    assert!(@encode 2_147_483_649u64, [129, 128, 128, 128, 8, 0, ..], 5);

    assert!(@decode 2_147_483_645u64, [253, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_646u64, [254, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_647u64, [255, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_648u64, [128, 128, 128, 128, 8, 0], 5);
    assert!(@decode 2_147_483_649u64, [129, 128, 128, 128, 8, 0], 5);

    assert!(@encode 4_294_967_293u64, [253, 255, 255, 255, 15, 0, ..], 5);
    assert!(@encode 4_294_967_294u64, [254, 255, 255, 255, 15, 0, ..], 5);
    assert!(@encode 4_294_967_295u64, [255, 255, 255, 255, 15, 0, ..], 5);

    assert!(@decode 4_294_967_293u64, [253, 255, 255, 255, 15, 0], 5);
    assert!(@decode 4_294_967_294u64, [254, 255, 255, 255, 15, 0], 5);
    assert!(@decode 4_294_967_295u64, [255, 255, 255, 255, 15, 0], 5);
}

#[test]
#[allow(clippy::too_many_lines, reason = "Tests")]
fn arch_u128() {
    assert!(@encode 0u128, [0, 0, ..], 1);
    assert!(@encode 1u128, [1, 0, ..], 1);
    assert!(@encode 2u128, [2, 0, ..], 1);

    assert!(@decode 0u128, [0], 1);
    assert!(@decode 1u128, [1], 1);
    assert!(@decode 2u128, [2], 1);

    assert!(@encode 126u128, [126, 0, 0, ..], 1);
    assert!(@encode 127u128, [127, 0, 0, ..], 1);
    assert!(@encode 128u128, [128, 1, 0, ..], 2);
    assert!(@encode 129u128, [129, 1, 0, ..], 2);
    assert!(@encode 130u128, [130, 1, 0, ..], 2);

    assert!(@decode 126u128, [126], 1);
    assert!(@decode 127u128, [127], 1);
    assert!(@decode 128u128, [128, 1], 2);
    assert!(@decode 129u128, [129, 1], 2);
    assert!(@decode 130u128, [130, 1], 2);

    assert!(@encode 253u128, [253, 1, 0, ..], 2);
    assert!(@encode 254u128, [254, 1, 0, ..], 2);
    assert!(@encode 255u128, [255, 1, 0, ..], 2);
    assert!(@encode 256u128, [128, 2, 0, ..], 2);
    assert!(@encode 257u128, [129, 2, 0, ..], 2);

    assert!(@decode 253u128, [253, 1], 2);
    assert!(@decode 254u128, [254, 1], 2);
    assert!(@decode 255u128, [255, 1], 2);
    assert!(@decode 256u128, [128, 2], 2);
    assert!(@decode 257u128, [129, 2], 2);

    assert!(@encode 65533u128, [253, 255, 3, 0, ..], 3);
    assert!(@encode 65534u128, [254, 255, 3, 0, ..], 3);
    assert!(@encode 65535u128, [255, 255, 3, 0, ..], 3);
    assert!(@encode 65536u128, [128, 128, 4, 0, ..], 3);
    assert!(@encode 65537u128, [129, 128, 4, 0, ..], 3);

    assert!(@decode 65533u128, [253, 255, 3, 0], 3);
    assert!(@decode 65534u128, [254, 255, 3, 0], 3);
    assert!(@decode 65535u128, [255, 255, 3, 0], 3);
    assert!(@decode 65536u128, [128, 128, 4, 0], 3);
    assert!(@decode 65537u128, [129, 128, 4, 0], 3);

    assert!(@encode 2_097_149u128, [253, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_150u128, [254, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_151u128, [255, 255, 127, 0, ..], 3);
    assert!(@encode 2_097_152u128, [128, 128, 128, 1, 0, ..], 4);
    assert!(@encode 2_097_153u128, [129, 128, 128, 1, 0, ..], 4);

    assert!(@decode 2_097_149u128, [253, 255, 127, 0], 3);
    assert!(@decode 2_097_150u128, [254, 255, 127, 0], 3);
    assert!(@decode 2_097_151u128, [255, 255, 127, 0], 3);
    assert!(@decode 2_097_152u128, [128, 128, 128, 1, 0], 4);
    assert!(@decode 2_097_153u128, [129, 128, 128, 1, 0], 4);

    assert!(@encode 2_147_483_645u128, [253, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_646u128, [254, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_647u128, [255, 255, 255, 255, 7, 0, ..], 5);
    assert!(@encode 2_147_483_648u128, [128, 128, 128, 128, 8, 0, ..], 5);
    assert!(@encode 2_147_483_649u128, [129, 128, 128, 128, 8, 0, ..], 5);

    assert!(@decode 2_147_483_645u128, [253, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_646u128, [254, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_647u128, [255, 255, 255, 255, 7, 0], 5);
    assert!(@decode 2_147_483_648u128, [128, 128, 128, 128, 8, 0], 5);
    assert!(@decode 2_147_483_649u128, [129, 128, 128, 128, 8, 0], 5);

    assert!(@encode 4_294_967_293u128, [253, 255, 255, 255, 15, 0, ..], 5);
    assert!(@encode 4_294_967_294u128, [254, 255, 255, 255, 15, 0, ..], 5);
    assert!(@encode 4_294_967_295u128, [255, 255, 255, 255, 15, 0, ..], 5);
    assert!(@encode 4_294_967_296u128, [128, 128, 128, 128, 16, 0, ..], 5);
    assert!(@encode 4_294_967_297u128, [129, 128, 128, 128, 16, 0, ..], 5);

    assert!(@decode 4_294_967_293u128, [253, 255, 255, 255, 15, 0], 5);
    assert!(@decode 4_294_967_294u128, [254, 255, 255, 255, 15, 0], 5);
    assert!(@decode 4_294_967_295u128, [255, 255, 255, 255, 15, 0], 5);
    assert!(@decode 4_294_967_296u128, [128, 128, 128, 128, 16, 0], 5);
    assert!(@decode 4_294_967_297u128, [129, 128, 128, 128, 16, 0], 5);

    #[rustfmt::skip]
    assert!(@encode
        340_282_366_920_938_463_463_374_607_431_768_211_453_u128,
        [253, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3, ..],
        19
    );

    #[rustfmt::skip]
    assert!(@decode
        340_282_366_920_938_463_463_374_607_431_768_211_453_u128,
        [253, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3],
        19
    );

    #[rustfmt::skip]
    assert!(@encode
        340_282_366_920_938_463_463_374_607_431_768_211_454_u128,
        [254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3, ..],
        19
    );

    #[rustfmt::skip]
    assert!(@decode
        340_282_366_920_938_463_463_374_607_431_768_211_454_u128,
        [254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3],
        19
    );

    #[rustfmt::skip]
    assert!(@encode
        340_282_366_920_938_463_463_374_607_431_768_211_455_u128,
        [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3, ..],
        19
    );

    #[rustfmt::skip]
    assert!(@decode
        340_282_366_920_938_463_463_374_607_431_768_211_455_u128,
        [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3],
        19
    );
}
