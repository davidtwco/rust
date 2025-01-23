//@ compile-flags: --target=aarch64-unknown-linux-gnu --crate-type=lib
//@ needs-llvm-components: aarch64
//@ compile-flags: -Ctarget-feature=-neon
// For now this is just a warning.
//@ build-pass
#![feature(no_core, lang_items)]
#![feature(trait_alias)] // `cfg(bootstrap)`: Remove this once `MetaSized_` has been removed
#![no_core]

#[lang = "metasized"]
pub trait MetaSized {}

// `cfg(bootstrap)`: Remove this once the real `MetaSized_` has been removed
#[lang = "metasized_alias"]
pub trait MetaSized_ = MetaSized;

#[lang = "sized"]
pub trait Sized: MetaSized {}
