// #[cfg(not(feature = "std"))]
// use alloc::{format, string::String, vec, vec::Vec};

use once_cell::sync::Lazy;
use regex::RegexSet;

use super::Compat;

#[rustfmt::skip]
impl Compat {
    const UNQUOTED_STRING_REGEX: &'static str = r"[\w\-\.\+]+";

    const BYTE_REGEX: &'static str = r"(-?\d+(b|B))|true|false";
    const SHORT_REGEX: &'static str = r"-?\d+(s|S)";
    const INT_REGEX: &'static str = r"-?\d+";
    const LONG_REGEX: &'static str = r"-?\d+(l|L)";
    const FLOAT_REGEX: &'static str = r"-?\d+\.\d+(f|F)";
    const DOUBLE_REGEX: &'static str = r"-?\d+\.\d+(d|D)?";
}

pub(super) static FIELD_REGEX: Lazy<RegexSet> = Lazy::new(|| {
    RegexSet::new([STRING_REGEXES.iter(), NUMBER_REGEXES.iter()].into_iter().flatten()).unwrap()
});
pub(super) static STRING_REGEX: Lazy<RegexSet> =
    Lazy::new(|| RegexSet::new(STRING_REGEXES.iter()).unwrap());

/// Match unquoted strings.
///
/// Examples:
/// - `hello123`
/// - `test-1`
/// - `1` / `+1` / `-1` / `1.0`
static STRING_REGEXES: Lazy<Vec<String>> =
    Lazy::new(|| vec![format!("^({})$", Compat::UNQUOTED_STRING_REGEX)]);

/// Match all number types.
///
/// Examples:
/// - `0b` / `-1B` / `true` / `false`
/// - `0s` / `1s` / `2S` / `-3S`
/// - `0` / `1` / `2` / `-3`
/// - `0l` / `1l` / `2L` / `-3L`
/// - `0.0f` / `1.0f` / `2.0F` / `-3.0F`
/// - `0.0d` / `1.0d` / `2.0D` / `-3.0D`
static NUMBER_REGEXES: Lazy<Vec<String>> = Lazy::new(|| {
    [
        Compat::BYTE_REGEX,
        Compat::SHORT_REGEX,
        Compat::INT_REGEX,
        Compat::LONG_REGEX,
        Compat::FLOAT_REGEX,
        Compat::DOUBLE_REGEX,
    ]
    .map(|regex| format!("^({regex})$"))
    .to_vec()
});
