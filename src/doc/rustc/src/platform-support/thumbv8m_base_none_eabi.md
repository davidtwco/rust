# `thumbv8m.{base,main}-none-eabi{,hf}`

**Tier: 2**

Bare-metal target for ARMv8-M baseline and mainline CPUs, including Cortex-M23,
Cortex-M33, M35P, M55, and M85. The `-eabihf` target enables the hardware FPU
and uses the hard-float ABI.

| Target                      | Description                          |
|-----------------------------|--------------------------------------|
| `thumbv8m.base-none-eabi`   | Bare ARMv8-M baseline                |
| `thumbv8m.main-none-eabi`   | Bare ARMv8-M mainline                |
| `thumbv8m.main-none-eabihf` | Bare ARMv8-M mainline with hardfloat |

## Target maintainers

* Rust Embedded Working Group, [Cortex-M team](https://github.com/rust-embedded/wg#the-cortex-m-team)

## Requirements

The target is cross-compiled, and uses static linking. No external toolchain
is required and the default `rust-lld` linker works, but you must specify
a linker script. The [`cortex-m-rt`] crate provides a suitable one. The
[`cortex-m-quickstart`] repository gives an example of a Cortex-M project.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt
[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart

## Floating-point

The `thumbv8m.main-none-eabihf` target assumes that the device has a hardware
FPU and lowers all floating point operations to hardware instructions.
Additionally, this target uses the "hard" float ABI, where floating point
values are passed to/from subroutines via FPU registers.

## Building the target

This target is included in Rust and can be installed via `rustup`.

## Testing

This is a cross-compiled no-std target, which must be run either in a simulator
or by programming them onto suitable hardware. It is not possible to run the
Rust testsuite on this target.

## Cross-compilation toolchains and C code

This target supports C code. If interlinking with C or C++, you may need to
use `arm-none-eabi-gcc` as a linker instead of the built-in LLD.
