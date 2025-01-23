//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@ check-pass

#![feature(sized_hierarchy)]
#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

use std::marker::MetaSized;

trait Id: ?MetaSized {
    type Output: ?MetaSized;
}

impl<T: ?MetaSized> Id for T {
    type Output = T;
}

trait Everyone: ?MetaSized {}
impl<T: ?MetaSized> Everyone for T {}

fn hello() where for<T> <T as Id>::Output: Everyone {}

fn main() {
    hello();
}
