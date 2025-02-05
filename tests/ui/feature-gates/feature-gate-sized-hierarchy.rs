#![feature(extern_types)]
#![feature(sized_hierarchy)]

use std::marker::MetaSized;

fn needs_metasized<T: ?Sized>() {}
fn needs_sized<T: Sized>() {}

fn main() {
    needs_metasized::<u8>();
    needs_sized::<u8>();

    needs_metasized::<str>();
    needs_sized::<str>();
//~^ ERROR the size for values of type `str` cannot be known at compilation time

    extern "C" {
        type Foo;
    }

    needs_metasized::<Foo>();
//~^ ERROR the size for values of type `main::Foo` cannot be known
    needs_sized::<Foo>();
//~^ ERROR the size for values of type `main::Foo` cannot be known at compilation time
}
