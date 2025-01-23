//@ revisions: paca pacg
//@ compile-flags: --crate-type=rlib --target=aarch64-unknown-linux-gnu
//@ needs-llvm-components: aarch64
//@[paca] compile-flags: -Ctarget-feature=+paca
//@[paca] error-pattern: the target features paca, pacg must all be either enabled or disabled together
//@[pacg] compile-flags: -Ctarget-feature=+pacg
//@[paca] error-pattern: the target features paca, pacg must all be either enabled or disabled together
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

// In this test, demonstrate that +paca and +pacg both result in the tied feature error if there
// isn't something causing an error.
// See tied-features-no-implication.rs

#[cfg(target_feature = "pacg")]
pub unsafe fn foo() {
}
