//@ revisions: one two three four
//@ compile-flags: --crate-type=rlib --target=aarch64-unknown-linux-gnu
//@ needs-llvm-components: aarch64
//
//
//@ [one] check-fail
//@ [one] compile-flags: -C target-feature=+paca
//@ [two] check-fail
//@ [two] compile-flags: -C target-feature=-pacg,+pacg
//@ [three] check-fail
//@ [three] compile-flags: -C target-feature=+paca,+pacg,-paca
//@ [four] build-pass
//@ [four] compile-flags: -C target-feature=-paca,+pacg -C target-feature=+paca
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

fn main() {}
