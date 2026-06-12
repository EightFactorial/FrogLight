//! TODO

use froglight_snbt::types::indexed::IndexedSnbt;

extern crate alloc;

#[test]
fn short() {
    static STRING: &str =
        "{ entry_name: entry_value, Short: [I; 0b, 10b, false, bool(1), bool(222uL)] }";

    let snbt = IndexedSnbt::new_ref(STRING).unwrap();

    std::println!("Raw SNBT: {STRING}");
    std::println!("Parsed SNBT: {:?}", snbt.root());

    std::println!("\nShort: {:?}", snbt.root().get("Short").unwrap());
}
