#![feature(lang_items, no_core, auto_traits)]
#![feature(trait_alias)] // `cfg(bootstrap)`: Remove this once `MetaSized_` has been removed
#![no_core]

#[lang = "copy"]
trait Copy {}

#[lang = "metasized"]
trait MetaSized {}

// `cfg(bootstrap)`: Remove this once the real `MetaSized_` has been removed
#[lang = "metasized_alias"]
trait MetaSized_ = MetaSized;

#[lang = "sized"]
trait Sized: MetaSized {}

#[lang = "freeze"]
auto trait Freeze {}

#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
    0
}

extern "C" {
    fn _foo() -> [u8; 16];
}

fn _main() {
    let _a = unsafe { _foo() };
}
