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
use facet::Facet;
#[cfg(feature = "facet")]
use froglight_facet::facet::prelude::*;
#[cfg(feature = "glam")]
use glam::{DVec3, Vec3, Vec3A};

/// A variable-length [`DVec3`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet), facet(opaque))]
#[cfg_attr(feature = "facet", facet(mc::with = LpDVec3::WITH))]
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

    cfg_select! {
        feature = "std" => {
            /// Create a [`LpDVec3`] from a [`DVec3`].
            #[inline]
            #[must_use]
            #[cfg(feature = "glam")]
            pub const fn new(vec: DVec3) -> Self { Self::new_xyz(vec.x, vec.y, vec.z) }

            /// Create a [`LpDVec3`] from `x`, `y`, and `z` values.
            #[must_use]
            pub const fn new_xyz(mut x: f64, mut y: f64, mut z: f64) -> Self {
                x = Self::sanitize(x);
                y = Self::sanitize(y);
                z = Self::sanitize(z);

                let max = x.abs().max(y.abs()).max(z.abs());
                if max < 3.051944088384301E-5 {
                    return Self(LpDVec3Inner::Zero);
                }

                let divisor = Self::ceil_long(max);
                let is_extended = divisor & 3 != divisor;
                let packed_divisor =
                    if is_extended { (divisor as u64 & 3) | 4 } else { divisor as u64 };
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
        }
        feature = "libm" => {
            /// Create a [`LpDVec3`] from a [`DVec3`].
            #[inline]
            #[must_use]
            #[cfg(feature = "glam")]
            pub fn new(vec: DVec3) -> Self { Self::new_xyz(vec.x, vec.y, vec.z) }

            /// Create a [`LpDVec3`] from `x`, `y`, and `z` values.
            #[must_use]
            pub fn new_xyz(mut x: f64, mut y: f64, mut z: f64) -> Self {
                x = Self::sanitize(x);
                y = Self::sanitize(y);
                z = Self::sanitize(z);

                let max = x.abs().max(y.abs()).max(z.abs());
                if max < 3.051944088384301E-5 {
                    return Self(LpDVec3Inner::Zero);
                }

                let divisor = Self::ceil_long(max);
                let is_extended = divisor & 3 != divisor;
                let packed_divisor =
                    if is_extended { (divisor as u64 & 3) | 4 } else { divisor as u64 };
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
        }
        _ => {
            /// Create a [`LpDVec3`] from a [`DVec3`].
            #[must_use]
            #[cfg(feature = "glam")]
            pub fn new(_: DVec3) -> Self { unreachable!() }

            /// Create a [`LpDVec3`] from a [`DVec3`].
            #[must_use]
            pub fn new_xyz(_: f64, _: f64, _: f64) -> Self { unreachable!() }
        }
    }

    cfg_select! {
        feature = "std" => {
            /// Pack a [`f64`] into a [`u64`].
            #[inline]
            #[must_use]
            const fn pack(val: f64) -> u64 { f64::round((val * 0.5 + 0.5) * 32766.) as u64 }

            /// Unpack a [`u64`] into a [`f64`].
            #[inline]
            #[must_use]
            const fn unpack(val: u64) -> f64 {
                f64::min((val & 32767) as f64, 32766.) * 2. / 32766. - 1.
            }
        }
        feature = "libm" => {
            /// Pack a [`f64`] into a [`u64`].
            #[inline]
            #[must_use]
            fn pack(val: f64) -> u64 { libm::round((val * 0.5 + 0.5) * 32766.) as u64 }

            /// Unpack a [`u64`] into a [`f64`].
            #[inline]
            #[must_use]
            const fn unpack(val: u64) -> f64 {
                f64::min((val & 32767) as f64, 32766.) * 2. / 32766. - 1.
            }
        }
        _ => {
            /// Pack a [`f64`] into a [`u64`].
            #[inline]
            #[must_use]
            fn pack(_: f64) -> u64 {
                compile_error!("Either the `std` or `libm` feature must be enabled for `LpDVec3`.");
                unreachable!()
            }

            /// Unpack a [`u64`] into a [`f64`].
            #[inline]
            #[must_use]
            const fn unpack(val: u64) -> f64 {
                f64::min((val & 32767) as f64, 32766.) * 2. / 32766. - 1.
            }
        }
    }

    /// Handle `Nan` and out-of-range values.
    #[inline]
    #[must_use]
    const fn sanitize(val: f64) -> f64 {
        if val.is_nan() { 0.0 } else { val.clamp(-1.7179869183E10, 1.7179869183E10) }
    }

    /// Round up a [`f64`] to the nearest [`i64`].
    #[inline]
    #[must_use]
    const fn ceil_long(val: f64) -> i64 {
        let long = val as i64;
        if val > long as f64 { long + 1 } else { long }
    }

    /// Get the `x`, `y`, and `z` values of this [`LpDVec3`].
    #[must_use]
    pub const fn as_xyz(self) -> [f64; 3] {
        match self.0 {
            LpDVec3Inner::Zero => [0.0; 3],
            LpDVec3Inner::Normal { a, b, c } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64 as f64;

                [
                    Self::unpack(packed >> 3) * multiplier,
                    Self::unpack(packed >> 18) * multiplier,
                    Self::unpack(packed >> 33) * multiplier,
                ]
            }
            LpDVec3Inner::Extended { a, b, c, d } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64;
                let multiplier = multiplier | ((d as u64) << 2);
                let multiplier = multiplier as f64;

                [
                    Self::unpack(packed >> 3) * multiplier,
                    Self::unpack(packed >> 18) * multiplier,
                    Self::unpack(packed >> 33) * multiplier,
                ]
            }
        }
    }

    /// Create a [`Vec3`] from a [`LpDVec3`].
    #[must_use]
    #[cfg(feature = "glam")]
    pub const fn as_vec3(self) -> Vec3 {
        let [x, y, z] = self.as_xyz();
        Vec3::new(x as f32, y as f32, z as f32)
    }

    /// Create a [`Vec3A`] from a [`LpDVec3`].
    #[must_use]
    #[cfg(feature = "glam")]
    pub const fn as_vec3a(self) -> Vec3A {
        let [x, y, z] = self.as_xyz();
        Vec3A::new(x as f32, y as f32, z as f32)
    }

    /// Create a [`DVec3`] from a [`LpDVec3`].
    #[must_use]
    #[cfg(feature = "glam")]
    pub const fn as_dvec3(self) -> DVec3 {
        let [x, y, z] = self.as_xyz();
        DVec3::new(x, y, z)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
impl FacetTemplate for LpDVec3 {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        match item.get::<Self>()?.0 {
            LpDVec3Inner::Zero => writer.write_byte(0),
            LpDVec3Inner::Normal { a, b, c } => {
                writer.write_byte(a)?;
                writer.write_byte(b)?;
                writer.write_bytes(&c.to_be_bytes())
            }
            LpDVec3Inner::Extended { a, b, c, d } => {
                writer.write_byte(a)?;
                writer.write_byte(b)?;
                writer.write_bytes(&c.to_be_bytes())?;

                encode_u32_into(d, writer)
            }
        }
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let a = reader.read_byte()?;
        if a == 0 {
            return item.set(Self(LpDVec3Inner::Zero));
        }

        let b = reader.read_byte()?;
        let c = u32::from_be_bytes(*reader.read_array::<4>()?);

        if a & 4 == 4 {
            let d = decode_u32_from(reader)?;
            item.set(Self(LpDVec3Inner::Extended { a, b, c, d }))
        } else {
            item.set(Self(LpDVec3Inner::Normal { a, b, c }))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl From<[f32; 3]> for LpDVec3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        LpDVec3::new_xyz(f64::from(value[0]), f64::from(value[1]), f64::from(value[2]))
    }
}
impl From<LpDVec3> for [f32; 3] {
    #[inline]
    fn from(value: LpDVec3) -> Self { value.as_xyz().map(|v| v as f32) }
}

impl From<[f64; 3]> for LpDVec3 {
    #[inline]
    fn from(value: [f64; 3]) -> Self { LpDVec3::new_xyz(value[0], value[1], value[2]) }
}
impl From<LpDVec3> for [f64; 3] {
    #[inline]
    fn from(value: LpDVec3) -> Self { value.as_xyz() }
}

#[cfg(feature = "glam")]
impl From<Vec3> for LpDVec3 {
    #[inline]
    fn from(value: Vec3) -> Self { LpDVec3::new(value.as_dvec3()) }
}
#[cfg(feature = "glam")]
impl From<LpDVec3> for Vec3 {
    #[inline]
    fn from(value: LpDVec3) -> Self { DVec3::from(value.as_xyz()).as_vec3() }
}

#[cfg(feature = "glam")]
impl From<Vec3A> for LpDVec3 {
    #[inline]
    fn from(value: Vec3A) -> Self { LpDVec3::new(value.as_dvec3()) }
}
#[cfg(feature = "glam")]
impl From<LpDVec3> for Vec3A {
    #[inline]
    fn from(value: LpDVec3) -> Self { Vec3A::from_array(value.as_xyz().map(|v| v as f32)) }
}

#[cfg(feature = "glam")]
impl From<DVec3> for LpDVec3 {
    #[inline]
    fn from(value: DVec3) -> Self { LpDVec3::new(value) }
}
#[cfg(feature = "glam")]
impl From<LpDVec3> for DVec3 {
    #[inline]
    fn from(value: LpDVec3) -> Self { DVec3::from(value.as_xyz()) }
}
