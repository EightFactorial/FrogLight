use std::fmt::Debug;

use super::regex::STRING_REGEX;
use crate::{
    mutf8::Mutf8String,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, NbtListTag, NbtTag,
        ShortArray,
    },
};

pub(super) trait WriteCompat {
    /// Write the content to a string.
    fn write_to_string(&self, content: &mut String);
}

impl WriteCompat for NbtCompound {
    fn write_to_string(&self, content: &mut String) {
        content.push('{');

        let last = self.len().saturating_sub(1);
        for (i, (key, value)) in self.iter().enumerate() {
            key.write_to_string(content);
            content.push(':');
            value.write_to_string(content);

            if i < last {
                content.push(',');
            }
        }

        content.push('}');
    }
}

// -------------------------------------------------------------------------------------------------

impl WriteCompat for NbtTag {
    fn write_to_string(&self, content: &mut String) {
        match self {
            NbtTag::Byte(item) => {
                content.push_str(&item.to_string());
                content.push('B');
            }
            NbtTag::Short(item) => {
                content.push_str(&item.to_string());
                content.push('S');
            }
            NbtTag::Int(item) => {
                content.push_str(&item.to_string());
            }
            NbtTag::Long(item) => {
                content.push_str(&item.to_string());
                content.push('L');
            }
            NbtTag::Float(item) => {
                content.push_str(&format!("{item:?}"));
                content.push('F');
            }
            NbtTag::Double(item) => {
                content.push_str(&format!("{item:?}"));
            }
            NbtTag::String(item) => item.write_to_string(content),
            NbtTag::List(item) => item.write_to_string(content),
            NbtTag::Compound(item) => item.write_to_string(content),
            NbtTag::ByteArray(array) => array.write_to_string(content),
            NbtTag::IntArray(array) => array.write_to_string(content),
            NbtTag::LongArray(array) => array.write_to_string(content),
        }
    }
}

#[test]
#[cfg(test)]
fn test_write_tag() {
    let mut bytes = String::new();
    NbtTag::Byte(1).write_to_string(&mut bytes);
    assert_eq!(bytes, "1B");

    let mut shorts = String::new();
    NbtTag::Short(1).write_to_string(&mut shorts);
    assert_eq!(shorts, "1S");

    let mut integers = String::new();
    NbtTag::Int(1).write_to_string(&mut integers);
    assert_eq!(integers, "1");

    let mut longs = String::new();
    NbtTag::Long(1).write_to_string(&mut longs);
    assert_eq!(longs, "1L");

    let mut floats = String::new();
    NbtTag::Float(1.0).write_to_string(&mut floats);
    assert_eq!(floats, "1.0F");

    let mut doubles = String::new();
    NbtTag::Double(1.0).write_to_string(&mut doubles);
    assert_eq!(doubles, "1.0");

    let mut strings = String::new();
    NbtTag::String("hello".into()).write_to_string(&mut strings);
    assert_eq!(strings, "hello");

    let mut compounds = String::new();
    NbtTag::Compound(NbtCompound::new()).write_to_string(&mut compounds);
    assert_eq!(compounds, "{}");

    let mut compounds = String::new();
    NbtTag::Compound(vec![(Mutf8String::from("hello"), NbtTag::Byte(1))].into())
        .write_to_string(&mut compounds);
    assert_eq!(compounds, "{hello:1B}");

    let mut compounds = String::new();
    NbtTag::Compound(
        vec![
            (Mutf8String::from("hello world"), NbtTag::Int(1)),
            (Mutf8String::from("empty_compound"), NbtTag::Compound(NbtCompound::new())),
            (Mutf8String::from("empty_list"), NbtTag::List(NbtListTag::Empty)),
        ]
        .into(),
    )
    .write_to_string(&mut compounds);
    assert_eq!(compounds, r#"{"hello world":1,empty_compound:{},empty_list:[]}"#);

    let mut bytes = String::new();
    NbtTag::ByteArray(ByteArray::from(vec![1, 2, 3])).write_to_string(&mut bytes);
    assert_eq!(bytes, "[B;1B,2B,3B]");

    let mut integers = String::new();
    NbtTag::IntArray(IntArray::from(vec![1, 2, 3])).write_to_string(&mut integers);
    assert_eq!(integers, "[I;1,2,3]");

    let mut longs = String::new();
    NbtTag::LongArray(LongArray::from(vec![1, 2, 3])).write_to_string(&mut longs);
    assert_eq!(longs, "[L;1L,2L,3L]");
}

// -------------------------------------------------------------------------------------------------

impl WriteCompat for NbtListTag {
    fn write_to_string(&self, content: &mut String) {
        match self {
            NbtListTag::Empty => content.push_str("[]"),
            NbtListTag::Byte(array) => write_array(None, Some('B'), array.iter(), content),
            NbtListTag::Short(array) => write_array(None, Some('S'), array.iter(), content),
            NbtListTag::Int(array) => write_array(None, None, array.iter(), content),
            NbtListTag::Long(array) => write_array(None, Some('L'), array.iter(), content),
            NbtListTag::Float(array) => write_array(None, Some('F'), array.iter(), content),
            NbtListTag::Double(array) => write_array(None, None, array.iter(), content),
            NbtListTag::String(array) => write_list(array.iter(), content),
            NbtListTag::List(array) => write_list(array.iter(), content),
            NbtListTag::Compound(array) => write_list(array.iter(), content),
            NbtListTag::ByteArray(array) => write_list(array.iter(), content),
            NbtListTag::IntArray(array) => write_list(array.iter(), content),
            NbtListTag::LongArray(array) => write_list(array.iter(), content),
        }
    }
}

fn write_list<'a, T: WriteCompat + 'a>(
    iter: impl ExactSizeIterator<Item = &'a T>,
    content: &mut String,
) {
    content.push('[');

    let last = iter.len().saturating_sub(1);
    for (i, item) in iter.enumerate() {
        item.write_to_string(content);

        if i < last {
            content.push(',');
        }
    }

    content.push(']');
}

#[test]
#[cfg(test)]
fn test_write_list() {
    let mut empty = String::new();
    NbtListTag::Empty.write_to_string(&mut empty);
    assert_eq!(empty, "[]");

    let mut bytes = String::new();
    NbtListTag::Byte(ByteArray::from(vec![1, 2, 3])).write_to_string(&mut bytes);
    assert_eq!(bytes, "[1B,2B,3B]");

    let mut shorts = String::new();
    NbtListTag::Short(ShortArray::from(vec![1, 2, 3])).write_to_string(&mut shorts);
    assert_eq!(shorts, "[1S,2S,3S]");

    let mut integers = String::new();
    NbtListTag::Int(IntArray::from(vec![1, 2, 3])).write_to_string(&mut integers);
    assert_eq!(integers, "[1,2,3]");

    let mut longs = String::new();
    NbtListTag::Long(LongArray::from(vec![1, 2, 3])).write_to_string(&mut longs);
    assert_eq!(longs, "[1L,2L,3L]");

    let mut floats = String::new();
    NbtListTag::Float(FloatArray::from(vec![1.0, 2.0, 3.0])).write_to_string(&mut floats);
    assert_eq!(floats, "[1.0F,2.0F,3.0F]");

    let mut doubles = String::new();
    NbtListTag::Double(DoubleArray::from(vec![1.0, 2.0, 3.0])).write_to_string(&mut doubles);
    assert_eq!(doubles, "[1.0,2.0,3.0]");

    let mut strings = String::new();
    NbtListTag::String(vec!["hello".into(), "world".into()]).write_to_string(&mut strings);
    assert_eq!(strings, "[hello,world]");

    let mut strings = String::new();
    NbtListTag::String(vec!["hello world".into(), "test-123".into()]).write_to_string(&mut strings);
    assert_eq!(strings, r#"["hello world",test-123]"#);

    let mut compounds = String::new();
    NbtListTag::Compound(vec![NbtCompound::new(), NbtCompound::new()])
        .write_to_string(&mut compounds);
    assert_eq!(compounds, "[{},{}]");

    let mut bytes = String::new();
    NbtListTag::ByteArray(vec![ByteArray::from(vec![1, 2, 3]), ByteArray::from(vec![4, 5, 6])])
        .write_to_string(&mut bytes);
    assert_eq!(bytes, "[[B;1B,2B,3B],[B;4B,5B,6B]]");

    let mut integers = String::new();
    NbtListTag::IntArray(vec![IntArray::from(vec![1, 2, 3]), IntArray::from(vec![4, 5, 6])])
        .write_to_string(&mut integers);
    assert_eq!(integers, "[[I;1,2,3],[I;4,5,6]]");

    let mut longs = String::new();
    NbtListTag::LongArray(vec![LongArray::from(vec![1, 2, 3]), LongArray::from(vec![4, 5, 6])])
        .write_to_string(&mut longs);
    assert_eq!(longs, "[[L;1L,2L,3L],[L;4L,5L,6L]]");
}

// -------------------------------------------------------------------------------------------------

/// ByteArray-specific format: `[B;({BYTE}B)?(,{BYTE}B)*]`
impl WriteCompat for ByteArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('B'), Some('B'), self.iter(), content);
    }
}

/// IntArray-specific format: `[I;{INT}?(,{INT})*]`
impl WriteCompat for IntArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('I'), None, self.iter(), content);
    }
}

/// LongArray-specific format: `[L;({LONG}L)?(,{LONG}L)*]`
impl WriteCompat for LongArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('L'), Some('L'), self.iter(), content);
    }
}

/// List-based format: `[({SHORT}S?)(,{SHORT}S)*]`
impl WriteCompat for ShortArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('S'), self.iter(), content);
    }
}

/// List-based format: `[({FLOAT}F)?(,{FLOAT}F)*]`
impl WriteCompat for FloatArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('F'), self.iter(), content);
    }
}

/// List-based format: `[{DOUBLE}?(,{DOUBLE})*]`
impl WriteCompat for DoubleArray {
    fn write_to_string(&self, content: &mut String) {
        write_array(None, None, self.iter(), content);
    }
}

/// Write an array to a string.
///
/// # Examples
/// - `(None, None, [1, 2, 3])` -> `"[1,2,3]"`
/// - `(None, Some('S'), [1, 2, 3])` -> `"[1S,2S,3S]"`
/// - `(Some('B'), Some('B'), [1, 2, 3])` -> `"[B;1B,2B,3B]"`
fn write_array<T: Debug>(
    prefix: Option<char>,
    suffix: Option<char>,
    iter: impl ExactSizeIterator<Item = T>,
    content: &mut String,
) {
    content.push('[');

    // Push the array prefix.
    if let Some(prefix) = prefix {
        content.push(prefix);
        content.push(';');
    }

    // Push the array items, separated by commas.
    let last = iter.len().saturating_sub(1);
    for (i, item) in iter.enumerate() {
        content.push_str(&format!("{item:?}"));
        if let Some(char) = suffix {
            content.push(char);
        }

        if i < last {
            content.push(',');
        }
    }

    content.push(']');
}

#[test]
#[cfg(test)]
fn test_write_array() {
    let mut bytes = String::new();
    ByteArray::from(vec![1, 2, 3]).write_to_string(&mut bytes);
    assert_eq!(bytes, "[B;1B,2B,3B]");

    let mut shorts = String::new();
    ShortArray::from(vec![1, 2, 3]).write_to_string(&mut shorts);
    assert_eq!(shorts, "[1S,2S,3S]");

    let mut integers = String::new();
    IntArray::from(vec![1, 2, 3]).write_to_string(&mut integers);
    assert_eq!(integers, "[I;1,2,3]");

    let mut longs = String::new();
    LongArray::from(vec![1, 2, 3]).write_to_string(&mut longs);
    assert_eq!(longs, "[L;1L,2L,3L]");

    let mut floats = String::new();
    FloatArray::from(vec![1.0, 2.0, 3.0]).write_to_string(&mut floats);
    assert_eq!(floats, "[1.0F,2.0F,3.0F]");

    let mut doubles = String::new();
    DoubleArray::from(vec![1.0, 2.0, 3.0]).write_to_string(&mut doubles);
    assert_eq!(doubles, "[1.0,2.0,3.0]");
}

// -------------------------------------------------------------------------------------------------

impl WriteCompat for Mutf8String {
    /// If a string only contains basic characters,
    /// it can be written without quotes.
    fn write_to_string(&self, content: &mut String) {
        let string = self.try_as_string().unwrap();
        if STRING_REGEX.is_match_at(&string, 0) {
            content.push_str(&string);
        } else {
            content.push('"');
            // Escape quotes.
            content.push_str(&string.replace('"', "\\\""));
            content.push('"');
        }
    }
}

#[test]
#[cfg(test)]
fn test_write_string() {
    let mut content = String::new();
    Mutf8String::from_string("hello123").write_to_string(&mut content);
    assert_eq!(content, "hello123");

    let mut content = String::new();
    Mutf8String::from_string("test-1").write_to_string(&mut content);
    assert_eq!(content, "test-1");

    let mut content = String::new();
    Mutf8String::from_string("123 \" 456").write_to_string(&mut content);
    assert_eq!(content, r#""123 \" 456""#);
}
