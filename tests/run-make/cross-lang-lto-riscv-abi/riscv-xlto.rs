#![allow(internal_features)]
#![feature(no_core, lang_items, const_trait_impl)]
#![no_core]

#[lang = "pointee_sized"]
trait PointeeSized {}
#[lang = "meta_sized"]
#[const_trait]
trait MetaSized: PointeeSized {}
#[lang = "sized"]
#[const_trait]
trait Sized: ~const MetaSized {}

#[no_mangle]
pub fn hello() {}
