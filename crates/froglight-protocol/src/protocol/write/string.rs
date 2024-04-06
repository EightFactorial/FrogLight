use compact_str::CompactString;

use crate::protocol::{FrogVarWrite, FrogWrite, WriteError};

impl FrogWrite for String {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("String too long").fg_var_write(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl FrogWrite for CompactString {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("String too long").fg_var_write(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(512))]

    #[test]
    fn proto_write_string(data in ".*") {
        use crate::protocol::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 5);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        bytes.extend(data.as_bytes());

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_compact_string(data in ".*") {
        use crate::protocol::FrogVarWrite;

        let mut bytes = Vec::with_capacity(data.len() + 5);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        bytes.extend(data.as_bytes());

        assert_eq!(CompactString::from(data).fg_to_bytes(), bytes);
    }
}
