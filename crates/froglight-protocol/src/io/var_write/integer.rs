use super::FrogVarWrite;

macro_rules! impl_integer_var_write {
    ($ty:ty) => {
        impl FrogVarWrite for $ty {
            fn frog_var_write(
                &self,
                buf: &mut (impl std::io::Write + ?Sized),
            ) -> std::io::Result<()> {
                let mut value = *self;
                let mut byte = [0];

                if value == 0 {
                    buf.write_all(&byte)?;
                } else {
                    while value != 0 {
                        byte[0] = (value & 0b0111_1111) as u8;
                        value = (value >> 7) & (<$ty>::max_value() >> 6);
                        if value != 0 {
                            byte[0] |= 0b1000_0000;
                        }
                        buf.write_all(&byte)?;
                    }
                }

                Ok(())
            }
        }
    };
}

impl_integer_var_write!(u16);
impl_integer_var_write!(u32);
impl_integer_var_write!(u64);
impl_integer_var_write!(u128);
impl_integer_var_write!(usize);

impl_integer_var_write!(i16);
impl_integer_var_write!(i32);
impl_integer_var_write!(i64);
impl_integer_var_write!(i128);
impl_integer_var_write!(isize);

#[test]
fn proto_var_write_i32() {
    let mut buf = Vec::with_capacity(5);

    assert!((-2_147_483_648_i32).frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 8]);
    buf.clear();

    assert!((-1i32).frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(0i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2_097_151_i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2_147_483_647_i32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
}
#[test]
fn proto_var_write_i64() {
    let mut buf = Vec::with_capacity(10);

    assert!((-9_223_372_036_854_775_808_i64).frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    buf.clear();

    assert!((-2_147_483_648_i64).frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!((-1i64).frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!(0i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2_097_151_i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2_147_483_647_i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4_294_967_295_i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9_223_372_036_854_775_807_i64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
}

#[test]
fn proto_var_write_u32() {
    let mut buf = Vec::with_capacity(5);

    assert!(0u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2_097_151_u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2_147_483_647_u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4_294_967_295_u32.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
}
#[test]
fn proto_var_write_u64() {
    let mut buf = Vec::with_capacity(10);

    assert!(0u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2_097_151_u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2_147_483_647_u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4_294_967_295_u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9_223_372_036_854_775_807_u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    buf.clear();

    assert!(18_446_744_073_709_551_615_u64.frog_var_write(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
}
