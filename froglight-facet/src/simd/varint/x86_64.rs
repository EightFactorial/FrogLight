use crate::simd::varint::traits::VarIntType;

/// Encode integers using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
pub fn encode<T: X86_64Ext>(value: T) -> ([u8; 31], u8) { encode_inline::<T>(value) }

/// Encode integers using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
pub fn encode_inline<T: X86_64Ext>(value: T) -> ([u8; 31], u8) {
    match T::MAX_BYTES {
        0..=5 => unsafe { encode_small(value) },
        6..=32 => unsafe { encode_large(value) },
        _ => panic!("Encoding unsupported for types larger than 32 bytes!"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u8`]s, [`u16`]s, and [`u32`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
pub unsafe fn encode_small<T: X86_64Ext>(value: T) -> ([u8; 31], u8) {
    // Separate the bits into groups of 7 and shift them.
    let v = value.to_u64().to_le();

    (super::arr8_to_31(v.to_ne_bytes()), 0)
}

// -------------------------------------------------------------------------------------------------

/// Encode [`u64`]s and [`u128`]s using SIMD.
///
/// # Safety
///
/// TODO
#[must_use]
#[inline(always)]
pub unsafe fn encode_large<T: X86_64Ext>(_value: T) -> ([u8; 31], u8) {
    // Separate the bits into groups of 7 and shift them.
    let v = value.to_u128().to_le();

    (super::arr16_to_31(v.to_ne_bytes()), 0)
}

// -------------------------------------------------------------------------------------------------

trait X86_64Ext: VarIntType {}

impl X86_64Ext for u8 {}
impl X86_64Ext for u16 {}
impl X86_64Ext for u32 {}
impl X86_64Ext for u64 {}
impl X86_64Ext for u128 {}
