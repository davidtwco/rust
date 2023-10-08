# `msp430-none-elf`

**Tier: 3**

16-bit MSP430 microcontrollers

## Target maintainers

* Rust Embedded Working Group, [MSP430 team](https://github.com/rust-embedded/wg#the-msp430-team)

## Requirements

TI's [MSP430 GCC compiler], version 8.3.0 or greater, must be installed, with
`msp430-elf-gcc` visible on your path. You must specify a linker script; the
[`msp430-rt`] crate provides a suitable one. Refer to [`msp430-quickstart`]
for detailed instructions.

A nightly Rust is required for the forseeable future until at least [#3246]
and [#38487] are resolved.

[MSP430 GCC compiler]: http://www.ti.com/tool/MSP430-GCC-OPENSOURCE
[`msp430-rt`]: https://github.com/rust-embedded/msp430-rt
[`msp430-quickstart`]: https://github.com/rust-embedded/msp430-quickstart
[#3246]: https://github.com/rust-lang/rfcs/pull/3246
[#38487]: https://github.com/rust-lang/rust/issues/38487

## Building the target

Rust does not ship pre-built versions of libcore for this target. Using a
nightly Cargo version, add the following to your `.cargo/config.toml` file:

```toml
[unstable]
build-std = ["core"]
```

## Testing

This is a cross-compiled no-std target, which must be run either in a simulator
or by programming them onto suitable hardware. It is not possible to run the
Rust testsuite on this target.

## Cross-compilation toolchains and C code

This target supports interlinking with C code compiled using the TI MSP430 GCC
compiler, but cross-language LTO is not supported.
