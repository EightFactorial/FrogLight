#[allow(unused_imports)]
use crate::buffer::VarEncode;

#[test]
fn var_encode_i32() {
    let mut buf = Vec::with_capacity(5);

    assert!((-2147483648i32).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 8]);
    buf.clear();

    assert!((-1i32).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(0i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647i32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();
}

#[test]
fn var_encode_u32() {
    let mut buf = Vec::with_capacity(5);

    assert!(0u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295u32.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();
}

#[test]
fn var_encode_i64() {
    let mut buf = Vec::with_capacity(10);

    assert!((-9223372036854775808i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    buf.clear();

    assert!((-2147483648i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!((-1i64).var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    buf.clear();

    assert!(0i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9223372036854775807i64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    buf.clear();
}

#[test]
fn var_encode_u64() {
    let mut buf = Vec::with_capacity(10);

    assert!(0u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(1u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(2u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![2]);
    buf.clear();

    assert!(127u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!(128u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 1]);
    buf.clear();

    assert!(254u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![254, 1]);
    buf.clear();

    assert!(255u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 1]);
    buf.clear();

    assert!(25565u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![221, 199, 1]);
    buf.clear();

    assert!(2097151u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 127]);
    buf.clear();

    assert!(2147483647u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 7]);
    buf.clear();

    assert!(4294967295u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 15]);
    buf.clear();

    assert!(9223372036854775807u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    buf.clear();

    assert!(18446744073709551615u64.var_encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    buf.clear();
}
