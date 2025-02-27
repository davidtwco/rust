//@ compile-flags: --target=x86_64-unknown-linux-gnu --crate-type=lib
//@ needs-llvm-components: x86
//@ check-pass
#![feature(no_core, lang_items, const_trait_impl)]
#![no_core]
#![allow(unexpected_cfgs)]

#[lang = "pointee_sized"]
pub trait PointeeSized {}

#[lang = "meta_sized"]
#[const_trait]
pub trait MetaSized: PointeeSized {}

#[lang = "sized"]
#[const_trait]
pub trait Sized: ~const MetaSized {}

// The compile_error macro does not exist, so if the `cfg` evaluates to `true` this
// complains about the missing macro rather than showing the error... but that's good enough.
#[cfg(not(target_feature = "x87"))]
compile_error!("the x87 feature *should* be exposed in `cfg`");
