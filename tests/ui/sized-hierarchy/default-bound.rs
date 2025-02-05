//@ check-fail
#![feature(extern_types, sized_hierarchy)]
use std::marker::MetaSized;

// This test checks that default bounds are appropriately added, the expected bounds are
// ultimately present regardless of ordering of explicit bounds, and that any errors/warnings
// related to sizedness bounds are emitted.

fn bare<T>() {} // `T: Sized`


fn pos_sized<T: Sized>() {} // `T: Sized`

fn neg_sized<T: ?Sized>() {} // `T: MetaSized`

fn pos_metasized<T: MetaSized>() {} // `T: Sized`
//~^ WARN `MetaSized` bound is redundant

fn neg_metasized<T: ?MetaSized>() {} // `T`


fn pos_sized_pos_metasized<T: Sized + MetaSized>() {} // `T: Sized`
//~^ WARN `MetaSized` bound is redundant

fn pos_sized_neg_metasized<T: Sized + ?MetaSized>() {}
//~^ ERROR `?MetaSized` bound cannot be used with `Sized` bound

fn neg_sized_pos_metasized<T: ?Sized + MetaSized>() {} // `T: MetaSized`
//~^ WARN `MetaSized` bound is redundant

fn neg_sized_neg_metasized<T: ?Sized + ?MetaSized>() {} // `T`
//~^ WARN `?Sized` bound is redundant

fn pos_metasized_pos_sized<T: MetaSized + Sized>() {} // `T: Sized`
//~^ WARN `MetaSized` bound is redundant

fn pos_metasized_neg_sized<T: MetaSized + ?Sized>() {} // `T: MetaSized`
//~^ WARN `MetaSized` bound is redundant

fn neg_metasized_pos_sized<T: ?MetaSized + Sized>() {}
//~^ ERROR `?MetaSized` bound cannot be used with `Sized` bound

fn neg_metasized_neg_sized<T: ?MetaSized + ?Sized>() {} // `T`
//~^ WARN `?Sized` bound is redundant


fn repeated_sized_bound<T: ?Sized + ?Sized>() {}
//~^ ERROR type parameter has more than one relaxed default bound, only one is supported

fn repeated_metasized_bound<T: ?MetaSized + ?MetaSized>() {}
//~^ ERROR type parameter has more than one relaxed default bound, only one is supported

fn additional_relaxed_bound<T: ?MetaSized + ?Copy>() {}
//~^ WARN relaxing a default bound only does something for `?Sized` and `?MetaSized`; all other traits are not bound by default

fn main() {
    // Functions which should have a `T: Sized` bound - check for an error given a non-Sized type:

    bare::<[u8]>();
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
    pos_sized::<[u8]>();
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
    pos_metasized::<[u8]>();
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
    pos_sized_pos_metasized::<[u8]>();
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
    pos_metasized_pos_sized::<[u8]>();
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time

    // Functions which should have a `T: MetaSized` bound - check for an error given a
    // non-MetaSized type:
    extern "C" {
        type Foo;
    }

    neg_sized::<Foo>();
    //~^ ERROR the size for values of type `main::Foo` cannot be known
    neg_sized_pos_metasized::<Foo>();
    //~^ ERROR the size for values of type `main::Foo` cannot be known
    pos_sized_neg_metasized::<Foo>();
    //~^ ERROR the size for values of type `main::Foo` cannot be known

    // Functions which have a `T` bound -- check that they can accept a extern type:

    neg_metasized::<Foo>();
    neg_sized_neg_metasized::<Foo>();
    neg_metasized_neg_sized::<Foo>();
}
