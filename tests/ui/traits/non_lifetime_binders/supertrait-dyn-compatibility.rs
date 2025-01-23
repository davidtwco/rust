#![feature(sized_hierarchy)]
#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

use std::marker::MetaSized;

trait Foo: for<T> Bar<T> + ?MetaSized {}

trait Bar<T: ?MetaSized>: ?MetaSized {
    fn method(&self) {}
}

fn needs_bar(x: &(impl Bar<i32> + ?MetaSized)) {
    x.method();
}

impl Foo for () {}

impl<T: ?MetaSized> Bar<T> for () {}

fn main() {
    let x: &dyn Foo = &();
    //~^ ERROR the trait `Foo` is not dyn compatible
    //~| ERROR the trait `Foo` is not dyn compatible
    needs_bar(x);
    //~^ ERROR the trait `Foo` is not dyn compatible
}
