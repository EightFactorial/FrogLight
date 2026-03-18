#![expect(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "Ignored"
)]
#![expect(clippy::many_single_char_names, clippy::unreadable_literal, reason = "Readability")]
#![expect(missing_docs, reason = "WIP")]

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
#[cfg(feature = "facet")]
use facet::{Facet, Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, bytes_to_variable, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError, variable_to_bytes},
};
use glam::{DVec3, Vec3};

/// A variable-length [`DVec3`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet), facet(opaque))]
#[cfg_attr(feature = "facet", facet(mc::serialize = LpDVec3::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = LpDVec3::DESERIALIZE))]
pub struct LpDVec3(LpDVec3Inner);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum LpDVec3Inner {
    #[default]
    Zero,
    Normal {
        a: u8,
        b: u8,
        c: u32,
    },
    Extended {
        a: u8,
        b: u8,
        c: u32,
        d: u32,
    },
}

impl LpDVec3 {
    // <3 Azalea

    /// Create a [`LpDVec3`] from a [`DVec3`].
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new(vec: DVec3) -> Self {
        let x = Self::sanitize(vec.x);
        let y = Self::sanitize(vec.y);
        let z = Self::sanitize(vec.z);
        let max = x.abs().max(y.abs()).max(z.abs());
        if max < 3.051944088384301E-5 {
            return Self(LpDVec3Inner::Zero);
        }

        let divisor = Self::ceil_long(max);
        let is_extended = divisor & 3 != divisor;
        let packed_divisor = if is_extended { (divisor as u64 & 3) | 4 } else { divisor as u64 };
        let packed_x = Self::pack(x / (divisor as f64)) << 3;
        let packed_y = Self::pack(y / (divisor as f64)) << 18;
        let packed_z = Self::pack(z / (divisor as f64)) << 33;
        let packed = packed_divisor | packed_x | packed_y | packed_z;

        let a = packed as u8;
        let b = (packed >> 8) as u8;
        let c = (packed >> 16) as u32;

        if is_extended {
            let d = ((divisor as u64) >> 2) as u32;
            Self(LpDVec3Inner::Extended { a, b, c, d })
        } else {
            Self(LpDVec3Inner::Normal { a, b, c })
        }
    }

    /// Create a [`LpDVec3`] from a [`DVec3`].
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    pub fn new(vec: DVec3) -> Self {
        let x = Self::sanitize(vec.x);
        let y = Self::sanitize(vec.y);
        let z = Self::sanitize(vec.z);
        let max = x.abs().max(y.abs()).max(z.abs());
        if max < 3.051944088384301E-5 {
            return Self(LpDVec3Inner::Zero);
        }

        let divisor = Self::ceil_long(max);
        let is_extended = divisor & 3 != divisor;
        let packed_divisor = if is_extended { (divisor as u64 & 3) | 4 } else { divisor as u64 };
        let packed_x = Self::pack(x / (divisor as f64)) << 3;
        let packed_y = Self::pack(y / (divisor as f64)) << 18;
        let packed_z = Self::pack(z / (divisor as f64)) << 33;
        let packed = packed_divisor | packed_x | packed_y | packed_z;

        let a = packed as u8;
        let b = (packed >> 8) as u8;
        let c = (packed >> 16) as u32;

        if is_extended {
            let d = ((divisor as u64) >> 2) as u32;
            Self(LpDVec3Inner::Extended { a, b, c, d })
        } else {
            Self(LpDVec3Inner::Normal { a, b, c })
        }
    }

    /// Handle `Nan` and out-of-range values.
    #[inline]
    #[must_use]
    const fn sanitize(val: f64) -> f64 {
        if val.is_nan() { 0.0 } else { val.clamp(-1.7179869183E10, 1.7179869183E10) }
    }

    const fn ceil_long(val: f64) -> i64 {
        let long = val as i64;
        if val > long as f64 { long + 1 } else { long }
    }

    /// Pack a [`f64`] into a [`u64`].
    #[cfg(feature = "std")]
    const fn pack(val: f64) -> u64 { f64::round((val * 0.5 + 0.5) * 32766.) as u64 }

    /// Pack a [`f64`] into a [`u64`].
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    fn pack(val: f64) -> u64 { libm::round((val * 0.5 + 0.5) * 32766.) as u64 }

    /// Create a [`Vec3`] from a [`LpDVec3`].
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        let dvec = self.as_dvec3();
        Vec3 { x: dvec.x as f32, y: dvec.y as f32, z: dvec.z as f32 }
    }

    /// Create a [`DVec3`] from a [`LpDVec3`].
    #[must_use]
    pub const fn as_dvec3(self) -> DVec3 {
        match self.0 {
            LpDVec3Inner::Zero => DVec3::ZERO,
            LpDVec3Inner::Normal { a, b, c } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64 as f64;

                DVec3 {
                    x: Self::unpack(packed >> 3) * multiplier,
                    y: Self::unpack(packed >> 18) * multiplier,
                    z: Self::unpack(packed >> 33) * multiplier,
                }
            }
            LpDVec3Inner::Extended { a, b, c, d } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64;
                let multiplier = multiplier | ((d as u64) << 2);
                let multiplier = multiplier as f64;

                DVec3 {
                    x: Self::unpack(packed >> 3) * multiplier,
                    y: Self::unpack(packed >> 18) * multiplier,
                    z: Self::unpack(packed >> 33) * multiplier,
                }
            }
        }
    }

    /// Unpack a [`u64`] into a [`f64`].
    #[inline]
    #[must_use]
    const fn unpack(val: u64) -> f64 { f64::min((val & 32767) as f64, 32766.) * 2. / 32766. - 1. }
}

#[cfg(feature = "facet")]
impl LpDVec3 {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    #[expect(clippy::cast_possible_truncation, reason = "Expected behavior")]
    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let a = cursor.take_array::<1>()?[0];
        if a == 0 {
            replace_with_or_abort(partial, |partial| {
                partial.set(Self(LpDVec3Inner::Zero)).unwrap()
            });
            return Ok(());
        }

        let b = cursor.take_array::<1>()?[0];
        let c = u32::from_be_bytes(*cursor.take_array::<4>()?);
        if a & 4 == 4 {
            let (len, d) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(len)?;

            let d = d as u32;
            replace_with_or_abort(partial, |partial| {
                partial.set(Self(LpDVec3Inner::Extended { a, b, c, d })).unwrap()
            });
        } else {
            replace_with_or_abort(partial, |partial| {
                partial.set(Self(LpDVec3Inner::Normal { a, b, c })).unwrap()
            });
        }

        Ok(())
    }

    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        macro_rules! write {
            ($data:expr) => {
                if !writer.write_data($data) {
                    return Err(SerializeIterError::new());
                }
            };
        }

        match peek.get::<Self>()?.0 {
            LpDVec3Inner::Zero => write!(&[0]),
            LpDVec3Inner::Normal { a, b, c } => {
                write!(&[a]);
                write!(&[b]);
                write!(&c.to_be_bytes());
            }
            LpDVec3Inner::Extended { a, b, c, d } => {
                write!(&[a]);
                write!(&[b]);
                write!(&c.to_be_bytes());

                let mut buffer = [0; _];
                let len = variable_to_bytes(u128::from(d), &mut buffer);
                write!(&buffer[len..]);
            }
        }

        Ok(())
    }
}

impl From<DVec3> for LpDVec3 {
    #[inline]
    fn from(value: DVec3) -> Self { LpDVec3::new(value) }
}
impl From<LpDVec3> for DVec3 {
    #[inline]
    fn from(value: LpDVec3) -> Self { value.as_dvec3() }
}

impl From<Vec3> for LpDVec3 {
    #[inline]
    fn from(value: Vec3) -> Self { LpDVec3::new(value.as_dvec3()) }
}
impl From<LpDVec3> for Vec3 {
    #[inline]
    fn from(value: LpDVec3) -> Self { value.as_vec3() }
}
