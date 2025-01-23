//@ revisions: BADFLAGS BADFLAGSPC BADTARGET
//@ [BADFLAGS] compile-flags: --target=aarch64-unknown-linux-gnu -Zbranch-protection=leaf
//@ [BADFLAGS] check-fail
//@ [BADFLAGS] needs-llvm-components: aarch64
//@ [BADFLAGSPC] compile-flags: --target=aarch64-unknown-linux-gnu -Zbranch-protection=pc
//@ [BADFLAGSPC] check-fail
//@ [BADFLAGSPC] needs-llvm-components: aarch64
//@ [BADTARGET] compile-flags: --target=x86_64-unknown-linux-gnu -Zbranch-protection=bti
//@ [BADTARGET] check-fail
//@ [BADTARGET] needs-llvm-components: x86

#![crate_type = "lib"]
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
