//@ compile-flags: --target=x86_64-unknown-none --crate-type=lib
//@ needs-llvm-components: x86
//@ build-pass
#![feature(no_core, lang_items, x87_target_feature)]
#![feature(trait_alias)] // `cfg(bootstrap)`: Remove this once `MetaSized_` has been removed
#![no_core]

#[lang = "metasized"]
pub trait MetaSized {}

// `cfg(bootstrap)`: Remove this once the real `MetaSized_` has been removed
#[lang = "metasized_alias"]
pub trait MetaSized_ = MetaSized;

#[lang = "sized"]
pub trait Sized: MetaSized {}

#[target_feature(enable = "x87")]
pub unsafe fn my_fun() {}
