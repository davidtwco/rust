//@ run-pass
#![allow(dead_code)]
// Test that traits can be implemented for extern types.
#![feature(extern_types, sized_hierarchy)]
use std::marker::MetaSized;

extern "C" {
    type A;
}

trait Foo: ?MetaSized {
    fn foo(&self) {}
}

impl Foo for A {
    fn foo(&self) {}
}

fn assert_foo<T: ?MetaSized + Foo>() {}

fn use_foo<T: ?MetaSized + Foo>(x: &dyn Foo) {
    x.foo();
}

fn main() {
    assert_foo::<A>();
}
