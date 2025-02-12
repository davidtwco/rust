//@ check-fail
//@ compile-flags: --crate-type=lib
//@ revisions: e2024 future
//@[e2024] edition: 2024
//@[future] compile-flags: -Zunstable-options
//@[future] edition: future

#![feature(sized_hierarchy)]

use std::marker::MetaSized;

pub fn metasized_migrated<T: MetaSized>() { }

pub fn metasized<T: ?Sized>() { }
//[future]~^ ERROR relaxing a default bound is disallowed in Rust Future

pub trait Tr {}

pub fn tr<T: ?Tr>() { }
//[future]~^ ERROR relaxing a default bound is disallowed in Rust Future
//[e2024]~^^ ERROR relaxing a default bound only does something for `?Sized`
