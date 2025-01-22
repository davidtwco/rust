// Make sure we don't ICE in `normalize_erasing_regions` when normalizing
// an associated type in an impl with unconstrained non-lifetime params.

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
#![feature(sized_hierarchy)]
use std::marker::MetaSized;

struct Thing;

pub trait Every {
    type Assoc;
}
impl<T: ?MetaSized> Every for Thing {
//~^ ERROR the type parameter `T` is not constrained
    type Assoc = T;
}

static I: <Thing as Every>::Assoc = 3;

fn main() {}
