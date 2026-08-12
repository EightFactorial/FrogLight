//! TODO

use froglight_snbt::prelude::*;
use yansi::Painted;

const TESTS: &str = include_str!("tests.txt");
const PASSED: Painted<&'static str> = Painted::new("Passed!").green();
const FAILED: Painted<&'static str> = Painted::new("Failed!").red();

#[test]
fn parse() -> Result<(), Vec<&'static str>> {
    let mut failed = Vec::new();

    for (index, (mut name, mut snbt)) in TESTS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once("=>"))
        .enumerate()
    {
        name = name.trim();
        snbt = snbt.trim();

        match IndexedSnbtSlice::new_ref(snbt) {
            Ok(snbt) => {
                println!("{index:02}. {PASSED} {name:?} => {snbt:?}");
            }
            Err(err) => {
                println!("{index:02}. {FAILED} {name:?}:\n{err:?}");
                // Append the failed test name.
                failed.push(name);
            }
        }
    }

    failed.is_empty().ok_or(failed)
}
