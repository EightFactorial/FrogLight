#[allow(unused_imports)]
use crate::buffer::Encode;

#[test]
fn encode_bool() {
    let mut buf = Vec::new();

    assert!(true.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1]);
    buf.clear();

    assert!(false.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
}

#[test]
fn encode_i8() {
    let mut buf = Vec::new();

    assert!(0i8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(127i8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127]);
    buf.clear();

    assert!((-128i8).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128]);
}

#[test]
fn encode_u8() {
    let mut buf = Vec::new();

    assert!(0u8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(255u8.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255]);
}

#[test]
fn encode_i16() {
    let mut buf = Vec::new();

    assert!(0i16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(32767i16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255]);
    buf.clear();

    assert!((-32768i16).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0]);
}

#[test]
fn encode_u16() {
    let mut buf = Vec::new();

    assert!(0u16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    assert!(65535u16.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255]);
}

#[test]
fn encode_i32() {
    let mut buf = Vec::new();

    assert!(0i32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(2147483647i32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255]);
    buf.clear();

    assert!((-2147483648i32).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0]);
}

#[test]
fn encode_u32() {
    let mut buf = Vec::new();

    assert!(0u32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(4294967295u32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255]);
}

#[test]
fn encode_i64() {
    let mut buf = Vec::new();

    assert!(0i64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(9223372036854775807i64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![127, 255, 255, 255, 255, 255, 255, 255]);
    buf.clear();

    assert!((-9223372036854775808i64).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn encode_u64() {
    let mut buf = Vec::new();

    assert!(0u64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(18446744073709551615u64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn encode_i128() {
    let mut buf = Vec::new();

    assert!(0i128.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(170141183460469231731687303715884105727i128
        .encode(&mut buf)
        .is_ok());
    assert_eq!(
        buf,
        vec![127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
    );
    buf.clear();
}

#[test]
fn encode_u128() {
    let mut buf = Vec::new();

    assert!(0u128.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0; 16]);
    buf.clear();

    assert!(340282366920938463463374607431768211455u128
        .encode(&mut buf)
        .is_ok());
    assert_eq!(buf, vec![255; 16]);
    buf.clear();
}

#[test]
fn encode_f32() {
    let mut buf = Vec::new();

    assert!(0f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 128, 0, 0]);
    buf.clear();

    assert!(1.5f32.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 192, 0, 0]);
    buf.clear();
}

#[test]
fn encode_f64() {
    let mut buf = Vec::new();

    assert!(0f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.0f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 240, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    assert!(1.5f64.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![63, 248, 0, 0, 0, 0, 0, 0]);
    buf.clear();
}

#[test]
fn encode_string() {
    let mut buf = Vec::new();

    assert!("".encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!("hello world".encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
    );
}

#[test]
fn encode_option() {
    let mut buf = Vec::new();

    assert!(None::<i32>.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0]);
    buf.clear();

    assert!(Some(42i32).encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1u8, 0u8, 0u8, 0u8, 42u8]);
}

#[test]
fn encode_vec() {
    let mut buf = Vec::new();

    assert!(vec![0u32].encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1u8, 0u8, 0u8, 0u8, 0u8]);
    buf.clear();

    assert!(vec![1u8, 2u8, 3u8].encode(&mut buf).is_ok());
    assert_eq!(buf, vec![3u8, 1u8, 2u8, 3u8]);
}
