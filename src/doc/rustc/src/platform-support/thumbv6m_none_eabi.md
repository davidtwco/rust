# `thumbv6m-none-eabi`

**Tier: 2**

Bare-metal target for ARMv6-M CPUs, including Cortex-M0, Cortex-M0+, and
Cortex-M1.

## Target maintainers

* Rust Embedded Working Group, [Cortex-M team](https://github.com/rust-embedded/wg#the-cortex-m-team)

## Requirements

The target is cross-compiled, and uses static linking. No external toolchain
is required and the default `rust-lld` linker works, but you must specify
a linker script. The [`cortex-m-rt`] crate provides a suitable one. The
[`cortex-m-quickstart`] repository gives an example of a Cortex-M project.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt
[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart

## Atomics

Rust enables 32-bit atomics with only load and store operations. There is no
hardware support for compare-and-swap. These atomics are ABI incompatible with
atomics backed by libatomic in C or C++. You may find the [portable-atomic]
crate useful to provide atomic compare-and-set operations.

[portable-atomic]: https://crates.io/crates/portable-atomic

## Building the target

This target is included in Rust and can be installed via `rustup`.

## Testing

This is a cross-compiled no-std target, which must be run either in a simulator
or by programming them onto suitable hardware. It is not possible to run the
Rust testsuite on this target.

## Cross-compilation toolchains and C code

This target supports C code. If interlinking with C or C++, you may need to
use `arm-none-eabi-gcc` as a linker instead of `rust-lld`.
