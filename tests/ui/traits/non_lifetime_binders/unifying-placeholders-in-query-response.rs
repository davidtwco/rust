//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@ check-pass

#![feature(sized_hierarchy)]
#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

use std::marker::MetaSized;

pub trait Foo<T: ?MetaSized> {
    type Bar<K: ?MetaSized>: ?MetaSized;
}

impl Foo<usize> for () {
    type Bar<K: ?MetaSized> = K;
}

pub fn f<T1, T2>(a: T1, b: T2)
where
    T1: for<T> Foo<usize, Bar<T> = T>,
    T2: for<T> Foo<usize, Bar<T> = <T1 as Foo<usize>>::Bar<T>>,
{
}

fn it_works() {
    f((), ());
}

fn main() {}
