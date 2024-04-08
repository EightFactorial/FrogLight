use super::{FrogWrite, WriteError};

macro_rules! impl_integer_write {
    ($($ty:ty),*) => {
        $(
            impl FrogWrite for $ty {
                fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                    buf.write_all(bytemuck::cast_ref::<$ty, [u8; std::mem::size_of::<$ty>()]>(&self.to_be())).map_err(WriteError::Io)
                }
            }
        )*
    };
}

impl_integer_write!(u8, u16, u32, u64, u128);
impl_integer_write!(i8, i16, i32, i64, i128);

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(32))]

    #[test]
    fn proto_write_u8(data in proptest::num::u8::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_u16(data in proptest::num::u16::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_u32(data in proptest::num::u32::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_u64(data in proptest::num::u64::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_u128(data in proptest::num::u128::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_i8(data in proptest::num::i8::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_i16(data in proptest::num::i16::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_i32(data in proptest::num::i32::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_i64(data in proptest::num::i64::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_i128(data in proptest::num::i128::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }
}
