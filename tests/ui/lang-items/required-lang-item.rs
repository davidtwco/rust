//@ add-core-stubs
//@ edition: 2018

#![feature(lang_items, no_core)]
#![no_core]
#![no_main]

extern crate minicore;
use minicore::*;

async fn x() {} //~ ERROR requires `ResumeTy` lang_item
