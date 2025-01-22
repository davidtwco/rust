// We previously mentioned other extern types in the error message here.
//
// Two extern types shouldn't really be considered similar just
// because they are both extern types.

#![feature(extern_types, sized_hierarchy)]

use std::marker::MetaSized;

extern "C" {
    type ShouldNotBeMentioned;
}

extern "C" {
    type Foo;
}

unsafe impl Send for ShouldNotBeMentioned {}

fn assert_send<T: Send + ?MetaSized>() {}

fn main() {
    assert_send::<Foo>()
    //~^ ERROR `Foo` cannot be sent between threads safely
}
