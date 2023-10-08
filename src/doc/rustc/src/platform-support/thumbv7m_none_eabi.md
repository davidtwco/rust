# `thumbv7{m,em}-none-eabi{,hf}`

**Tier: 2**

Bare-metal targets for ARMv7-M CPUs, including Cortex-M3, Cortex-M4, and
Cortex-M7. Three targets are available, depending on the use of the `E` DSP
extension (Cortex-M4 and Cortex-M7 only) and the presence of a hardware
floating-point unit.

| Target                  | Description              |
|-------------------------|--------------------------|
| `thumbv7em-none-eabi`   | Bare ARMv7E-M            |
| `thumbv7em-none-eabihf` | Bare ARMv7E-M, hardfloat |
| `thumbv7m-none-eabi`    | Bare ARMv7-M             |

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

The `thumbv7em-none-eabihf` target assumes that the device has a hardware FPU
and lowers all floating point operations to hardware instructions.
Additionally, this target uses the "hard" float ABI, where floating
point values are passed to/from subroutines via FPU registers.

To opt in to double precision hardware FPU support on Cortex-M7,
use the `-C target-feature=+fp64` or `-C target-cpu=cortex-m7` flags.

It is also possible to use the hardware FPU while using the soft float ABI, by
using the `thumbv7em-none-eabi` target and passing `-C target-cpu=cortex-m4`
(or `cortex-m7`). In this case the FPU must be enabled at startup, before
entering any function which takes floating-point arguments or uses any
floating-point variables. The [`cortex-m`] crate provides the
`SCB::enable_fpu()` convenience function to enable the FPU.

[`cortex-m`]: https://crates.io/crates/cortex-m

## Building the target

This target is included in Rust and can be installed via `rustup`.

## Testing

This is a cross-compiled no-std target, which must be run either in a simulator
or by programming them onto suitable hardware. It is not possible to run the
Rust testsuite on this target.

## Cross-compilation toolchains and C code

This target supports C code. If interlinking with C or C++, you may need to
use `arm-none-eabi-gcc` as a linker instead of the built-in LLD.
