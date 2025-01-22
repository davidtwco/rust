//@ add-core-stubs
#![feature(lang_items, no_core)]
#![no_core] // makes debugging this test *a lot* easier (during resolve)

extern crate minicore;
use minicore::*;

// Test to make sure that private items imported through globs remain private
// when  they're used.

mod bar {
    pub use self::glob::*;

    mod glob {
        fn gpriv() {}
    }
}

pub fn foo() {}

fn test2() {
    use bar::glob::gpriv; //~ ERROR: module `glob` is private
    gpriv();
}

fn main() {}
