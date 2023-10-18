#[cfg(test)]
use pretty_assertions::assert_eq;

#[cfg(test)]
use crate::buffer::VarDecode;

#[test]
fn var_decode_i32() {
    assert_eq!(
        i32::var_decode(&mut &[128, 128, 128, 128, 8][..]),
        Ok(-2147483648)
    );
    assert_eq!(i32::var_decode(&mut &[255, 255, 255, 255, 15][..]), Ok(-1));
    assert_eq!(i32::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(i32::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(i32::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(i32::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(i32::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(i32::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(i32::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(i32::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(i32::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        i32::var_decode(&mut &[255, 255, 255, 255, 7][..]),
        Ok(2147483647)
    );
}

#[test]
fn var_decode_u32() {
    assert_eq!(u32::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(u32::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(u32::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(u32::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(u32::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(u32::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(u32::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(u32::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(u32::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        u32::var_decode(&mut &[255, 255, 255, 255, 7][..]),
        Ok(2147483647)
    );
    assert_eq!(
        u32::var_decode(&mut &[255, 255, 255, 255, 15][..]),
        Ok(4294967295)
    );
}

#[test]
fn var_decode_i64() {
    assert_eq!(
        i64::var_decode(&mut &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1][..]),
        Ok(-9223372036854775808)
    );
    assert_eq!(
        i64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1][..]),
        Ok(-1)
    );
    assert_eq!(i64::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(i64::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(i64::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(i64::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(i64::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(i64::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(i64::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(i64::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(i64::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        i64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(9223372036854775807)
    );
}

#[test]
fn var_decode_u64() {
    assert_eq!(u64::var_decode(&mut &[0][..]), Ok(0));
    assert_eq!(u64::var_decode(&mut &[1][..]), Ok(1));
    assert_eq!(u64::var_decode(&mut &[2][..]), Ok(2));
    assert_eq!(u64::var_decode(&mut &[127][..]), Ok(127));
    assert_eq!(u64::var_decode(&mut &[128, 1][..]), Ok(128));
    assert_eq!(u64::var_decode(&mut &[254, 1][..]), Ok(254));
    assert_eq!(u64::var_decode(&mut &[255, 1][..]), Ok(255));
    assert_eq!(u64::var_decode(&mut &[221, 199, 1][..]), Ok(25565));
    assert_eq!(u64::var_decode(&mut &[255, 255, 127][..]), Ok(2097151));
    assert_eq!(
        u64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(9223372036854775807)
    );
    assert_eq!(
        u64::var_decode(&mut &[255, 255, 255, 255, 255, 255, 255, 255, 255, 127][..]),
        Ok(18446744073709551615)
    );
}
