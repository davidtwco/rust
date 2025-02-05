//@ check-pass
#![allow(internal_features)]
#![feature(extern_types, more_maybe_bounds, sized_hierarchy)]
use std::marker::MetaSized_;

pub fn hash<T: ?MetaSized_>(_: *const T) {
    unimplemented!();
}

extern "C" {
    type Foo;
}

fn get() -> *const Foo {
    unimplemented!()
}

fn main() {
    hash::<Foo>(get());
}
