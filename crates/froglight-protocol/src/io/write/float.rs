use super::{FrogWrite, WriteError};

macro_rules! impl_float_write {
    ($($ty:ty),*) => {
        $(
            impl FrogWrite for $ty {
                #[inline]
                fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
                    Ok(buf.write_all(&self.to_be_bytes())?)
                }
            }
        )*
    };
}
impl_float_write!(f32, f64);

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

    #[test]
    fn proto_write_f32(data in proptest::num::f32::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }

    #[test]
    fn proto_write_f64(data in proptest::num::f64::ANY) {
        assert_eq!(data.fg_to_bytes(), data.to_be_bytes());
    }
}
