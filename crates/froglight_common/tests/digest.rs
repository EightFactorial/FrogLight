//! TODO

use froglight_common::digest::HexDigest;

macro_rules! digest {
    ($name:expr, $expected:expr) => {
        let output = HexDigest::digest($name.as_bytes());
        assert_eq!(
            output.as_str(),
            $expected,
            "Expected 1, got 2,\n{:?}\n{:?}",
            $expected.as_bytes(),
            output.as_str().as_bytes()
        );
    };
}

#[test]
fn hex_digest() {
    digest!("Notch", "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48");
    digest!("jeb_", "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1");
    digest!("simon", "88e16a1019277b15d58faf0541e11910eb756f6");
}
