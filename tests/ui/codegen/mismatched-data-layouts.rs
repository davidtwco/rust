// This test checks that data layout mismatches emit an error.
//
//@ build-fail
//@ needs-llvm-components: x86
//@ compile-flags: --crate-type=lib --target={{src-base}}/codegen/mismatched-data-layout.json -Z unstable-options
//@ error-pattern: differs from LLVM target's
//@ normalize-stderr: "`, `[A-Za-z0-9-:]*`" -> "`, `normalized data layout`"
//@ normalize-stderr: "layout, `[A-Za-z0-9-:]*`" -> "layout, `normalized data layout`"

#![feature(lang_items, no_core, auto_traits)]
#![feature(trait_alias)] // `cfg(bootstrap)`: Remove this once `MetaSized_` has been removed
#![no_core]

#[lang = "metasized"]
pub trait MetaSized {}

// `cfg(bootstrap)`: Remove this once the real `MetaSized_` has been removed
#[lang = "metasized_alias"]
pub trait MetaSized_ = MetaSized;

#[lang = "sized"]
pub trait Sized: MetaSized {}
