//! TODO
#![no_std]

use froglight_snbt::types::indexed::IndexedSnbt;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[test]
fn short() {
    static STRING: &str = "{entry_name: entry_value, Short: 0x1234}";

    // Parse `STRING`

    let snbt = IndexedSnbt::new_str(STRING).unwrap();
    #[cfg(feature = "std")]
    std::println!("Parsed SNBT: {:?}", snbt.core());
}
